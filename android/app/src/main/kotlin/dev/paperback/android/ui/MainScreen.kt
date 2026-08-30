package dev.paperback.android.ui

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.os.Environment
import android.provider.Settings
import android.view.accessibility.AccessibilityManager
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.core.content.ContextCompat
import androidx.core.net.toUri
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.NavKey
import dev.paperback.android.AllDocumentsRoute
import dev.paperback.android.SettingsRoute
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File

/** True once Android enforces scoped storage (R+) and the app still lacks "All files access". */
internal fun needsAllFilesAccessPermission(): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && !Environment.isExternalStorageManager()

/** True only on R+ devices where "All files access" has already been granted. */
internal fun hasAllFilesAccessOnR(): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && Environment.isExternalStorageManager()

/** True once Android requires a runtime prompt (Tiramisu+) and notifications aren't yet allowed. */
internal fun needsNotificationPermission(context: Context): Boolean =
	Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
		ContextCompat.checkSelfPermission(context, Manifest.permission.POST_NOTIFICATIONS) !=
		PackageManager.PERMISSION_GRANTED

@OptIn(ExperimentalMaterial3Api::class, FlowPreview::class)
@Composable
fun MainScreen(
	modifier: Modifier = Modifier,
	onItemClick: (NavKey) -> Unit = {},
	viewModel: MainScreenViewModel = viewModel()
) {
	val context = LocalContext.current
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	val listStates = remember { mutableStateMapOf<String, LazyListState>() }
	var exportDocumentDialogOpen by remember { mutableStateOf(false) }
	var selectedExportFormat by remember { mutableStateOf<uniffi.paperback.ExportFormat?>(null) }
	val goToDialogOpen by viewModel.showGoToDialog.collectAsStateWithLifecycle()
	val goToInitialMode by viewModel.goToInitialMode.collectAsStateWithLifecycle()
	val findDialogOpen by viewModel.findDialog.isOpen.collectAsStateWithLifecycle()
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	val settings = viewModel.settings
	val restorePreviousDocuments by settings.restorePreviousDocuments.state.collectAsStateWithLifecycle()
	val useInAppFileBrowser by settings.useInAppFileBrowser.state.collectAsStateWithLifecycle()
	// Guards the one-time auto-switch to the in-app browser right after All Files
	// Access is first granted, so it doesn't keep re-enabling itself on every later
	// resume (e.g. after using the system picker) and fight the user's own toggle.
	var hasAutoEnabledInAppFileBrowser by remember {
		mutableStateOf(viewModel.configManager.getAppBool("auto_enabled_in_app_file_browser", false))
	}
	val swipeUpMovesForward by settings.swipeUpMovesForward.state.collectAsStateWithLifecycle()
	var onboardingCompleted by remember {
		mutableStateOf(viewModel.configManager.getAppBool("permissions_onboarding_shown", false))
	}
	// Bumped whenever the activity resumes (e.g. returning from the All Files Access
	// settings screen), so permission checks below re-read the live OS state instead
	// of the value from whenever this composable last recomposed for another reason.
	var permissionResumeTrigger by remember { mutableStateOf(0) }
	var notificationRequested by remember { mutableStateOf(false) }
	val notificationPermissionLauncher = rememberLauncherForActivityResult(
		ActivityResultContracts.RequestPermission()
	) { notificationRequested = true }
	val notificationsSectionApplicable = Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU
	val allFilesAccessSectionApplicable = Build.VERSION.SDK_INT >= Build.VERSION_CODES.R
	val notificationsGranted = remember(permissionResumeTrigger, notificationRequested) {
		!needsNotificationPermission(context)
	}
	val allFilesAccessGranted = remember(permissionResumeTrigger) { !needsAllFilesAccessPermission() }
	val showOnboarding = !onboardingCompleted &&
		(
			(notificationsSectionApplicable && !notificationsGranted) ||
				(allFilesAccessSectionApplicable && !allFilesAccessGranted)
		)
	val activeSearchQuery by viewModel.activeSearchQuery.collectAsStateWithLifecycle()
	val activeSearchOptions by viewModel.activeSearchOptions.collectAsStateWithLifecycle()
	var isTextMode by rememberSaveable { mutableStateOf(false) }

	// An audio-only tab has no real text spine to show in Text Mode (its top-bar toggle is
	// hidden for the same reason), so switching to one from a Text Mode session falls back to
	// Read-Aloud mode instead of stranding the user on a blank text view with no way back.
	LaunchedEffect((state as? MainScreenUiState.Success)?.activeTab?.documentUri) {
		if ((state as? MainScreenUiState.Success)?.activeTab?.isAudioOnly == true) {
			isTextMode = false
		}
	}

	// F3/Shift+F3 (MainActivity) trigger this. In Read-Aloud mode, Find is a nav unit, so this
	// just steps it the same way the nav-unit slider's Previous/Next buttons do; only Text mode
	// (which has no nav-unit slider) still needs this handler's own search-and-scroll logic.
	LaunchedEffect(Unit) {
		viewModel.performSearchEvent.collect { forward ->
			if (!isTextMode) {
				// No active search means Find isn't the nav unit, so there's nothing for F3 to
				// step through here; without this guard it would fall through to whatever unit
				// currently is selected (Section, a Time seek, ordinary Paragraph...), silently
				// doing the wrong thing instead of the no-op F3 has always been without a search.
				if (activeSearchQuery != null && activeSearchOptions != null) {
					val speaking = viewModel.ttsManager.isSpeaking.value
					if (forward) {
						viewModel.playNextSegment(speak = speaking, announce = !speaking)
					} else {
						viewModel.playPrevSegment(speak = speaking, announce = !speaking)
					}
				}
				return@collect
			}
			if (activeSearchQuery != null && activeSearchOptions != null) {
				val state = viewModel.uiState.value
				if (state is MainScreenUiState.Success) {
					val tab = state.activeTab
					if (tab != null) {
						val listState = listStates[tab.documentUri]
						val searchPos = if (listState != null) {
							val nextLine = (listState.firstVisibleItemIndex + if (forward) 2 else 1).toLong()
							tab.session.positionFromLine(nextLine)
						} else {
							viewModel.ttsPosition.value
						}
						val res = tab.session.searchFfi(activeSearchQuery!!, searchPos, activeSearchOptions!!.copy(forward = forward))
						if (res.found) {
							val line = tab.session.lineFromPosition(res.position)
							val indexToScroll = (line - 1).toInt().coerceAtLeast(0)
							listState?.scrollToItem(indexToScroll)
						}
					}
				}
			}
		}
	}
	val isSpeaking by viewModel.ttsManager.isSpeaking.collectAsStateWithLifecycle()
	val currentNavUnit by viewModel.currentNavUnit.collectAsStateWithLifecycle()
	val ttsPosition by viewModel.ttsPosition.collectAsStateWithLifecycle()
	val currentSegmentText by viewModel.currentSegmentText.collectAsStateWithLifecycle()
	val textScalePercent by settings.textScalePercent.state.collectAsStateWithLifecycle()
	val lineSpacing by settings.lineSpacing.state.collectAsStateWithLifecycle()
	val paragraphSpacing by settings.paragraphSpacing.state.collectAsStateWithLifecycle()
	val textAlignment by settings.textAlignment.state.collectAsStateWithLifecycle()
	val readability = rememberReadabilityStyle(textScalePercent, lineSpacing, paragraphSpacing, textAlignment)
	var ttsConfigDialogOpen by remember { mutableStateOf(false) }
	val sleepTimerRemaining by viewModel.sleepTimerRemaining.collectAsStateWithLifecycle()
	val showElementsDialog by viewModel.showElementsDialog.collectAsStateWithLifecycle()
	val currentHeadings by viewModel.currentHeadings.collectAsStateWithLifecycle()
	val currentLinks by viewModel.currentLinks.collectAsStateWithLifecycle()

	val view = androidx.compose.ui.platform.LocalView.current
	LaunchedEffect(Unit) {
		viewModel.accessibilityAnnouncement.collect { message ->
			@Suppress("DEPRECATION")
			view.announceForAccessibility(message)
		}
	}

	LaunchedEffect(Unit) {
		viewModel.sleepTimerExpired.collect {
			(context as? Activity)?.moveTaskToBack(true)
		}
	}
	val accessibilityManager =
		remember(context) {
			context.getSystemService(Context.ACCESSIBILITY_SERVICE) as AccessibilityManager
		}
	var isTouchExplorationEnabled by remember { mutableStateOf(accessibilityManager.isTouchExplorationEnabled) }
	DisposableEffect(accessibilityManager) {
		val listener = AccessibilityManager.TouchExplorationStateChangeListener { enabled ->
			isTouchExplorationEnabled = enabled
		}
		accessibilityManager.addTouchExplorationStateChangeListener(listener)
		onDispose {
			accessibilityManager.removeTouchExplorationStateChangeListener(listener)
		}
	}
	val activity = context as? Activity
	DisposableEffect(activity) {
		val listener = androidx.core.util.Consumer<Intent> { newIntent ->
			val uri = newIntent.data
			if (uri != null && newIntent.action == Intent.ACTION_VIEW) {
				viewModel.openDocument(uri)
				newIntent.action = Intent.ACTION_MAIN
			}
		}
		if (activity is androidx.activity.ComponentActivity) {
			activity.addOnNewIntentListener(listener)
		}
		onDispose {
			if (activity is androidx.activity.ComponentActivity) {
				activity.removeOnNewIntentListener(listener)
			}
		}
	}
	LaunchedEffect(Unit) {
		val intent = activity?.intent
		val uri = intent?.data
		if (uri != null && intent.action == Intent.ACTION_VIEW) {
			viewModel.openDocument(uri)
			intent.action = Intent.ACTION_MAIN
		}
	}
	val supportedMimeTypes by viewModel.supportedMimeTypes.collectAsStateWithLifecycle()

	val filePickerLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.OpenDocument(),
		onResult = { uri -> uri?.let { viewModel.openDocument(it) } }
	)

	var locateTargetUri by remember { mutableStateOf<String?>(null) }
	val locateFilePickerLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.OpenDocument(),
		onResult = { uri ->
			val target = locateTargetUri
			if (uri != null && target != null) {
				viewModel.locateRecentDocument(target, uri)
			}
			locateTargetUri = null
		}
	)
	val onLocateRecentDocument: (String) -> Unit = { uri ->
		locateTargetUri = uri
		locateFilePickerLauncher.launch(supportedMimeTypes)
	}

	var showFileManager by remember { mutableStateOf(false) }
	var showFileManagerForImport by remember { mutableStateOf(false) }

	val importSettingsLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.OpenDocument(),
		onResult = { uri ->
			if (uri != null) {
				scope.launch(Dispatchers.IO) {
					val success = viewModel.importSettingsFromUri(context, uri)
					// TRANSLATORS: Toast confirming a .paperback settings file was imported successfully, or the failure message if not
					val message = if (success) t("Settings imported") else t("Failed to import settings")
					withContext(Dispatchers.Main) {
						Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
					}
				}
			}
		}
	)

	val exportSettingsLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.CreateDocument("*/*"),
		onResult = { uri ->
			if (uri != null) {
				scope.launch(Dispatchers.IO) {
					val success = viewModel.exportSettingsToUri(context, uri)
					// TRANSLATORS: Toast confirming the document's settings were exported to a .paperback file, or the failure message if not
					val message = if (success) t("Settings exported") else t("Failed to export settings")
					withContext(Dispatchers.Main) {
						Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
					}
				}
			}
		}
	)

	val exportDocumentLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.CreateDocument("*/*"),
		onResult = { uri ->
			if (uri != null) {
				selectedExportFormat?.let { format ->
					scope.launch(Dispatchers.IO) {
						val success = viewModel.exportDocumentToUri(context, uri, format)
						val message = if (success) t("Document exported") else t("Failed to export document")
						withContext(Dispatchers.Main) {
							Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
						}
					}
				}
			}
		}
	)

	Box(modifier = Modifier.fillMaxSize()) {
		Scaffold(
			// safeDrawing rather than the default systemBars so text keeps clear of a landscape
			// display cutout. The bars each pad themselves, so letting the Scaffold be the one
			// place that applies these is what stops them being counted twice.
			contentWindowInsets = WindowInsets.safeDrawing,
			topBar = {
				MainScreenTopBar(
					state = state,
					isTextMode = isTextMode,
					isSpeaking = isSpeaking,
					onOpenBook = {
						if (useInAppFileBrowser) {
							if (needsAllFilesAccessPermission()) {
								viewModel.permissionRationaleDialog.open()
							} else {
								showFileManager = true
							}
						} else {
							filePickerLauncher.launch(supportedMimeTypes)
						}
					},
					onTocOpen = { viewModel.tocRequest.request() },
					onTabSelect = { viewModel.setActiveTab(it) },
					onTabClose = { viewModel.closeTab(it) },
					onToggleTextMode = { isTextMode = !isTextMode },
					onTogglePlayPause = { viewModel.togglePlayPause() },
					onRecentsOpen = { onItemClick(AllDocumentsRoute) },
					onGoToOpen = { viewModel.openGoToDialog() },
					onFindOpen = { viewModel.findDialog.open() },
					onWordCountOpen = { viewModel.openWordCountDialog() },
					onDocumentInfoOpen = { viewModel.documentInfoDialog.open() },
					onSettingsOpen = { onItemClick(SettingsRoute) },
					onSleepTimerOpen = { viewModel.sleepTimerDialog.open() },
					onElementsOpen = { viewModel.openElementsDialog() },
					onExportDocumentOpen = { exportDocumentDialogOpen = true },
					onExportSettings = {
						val activeDocUri = (state as? MainScreenUiState.Success)?.activeTab?.documentUri
						if (activeDocUri != null) {
							if (activeDocUri.startsWith("content://")) {
								exportSettingsLauncher.launch("document.paperback")
							} else {
								if (viewModel.exportCurrentSettings()) {
									Toast.makeText(context, t("Settings exported"), Toast.LENGTH_SHORT).show()
								} else {
									Toast.makeText(context, t("Failed to export settings"), Toast.LENGTH_SHORT).show()
								}
							}
						}
					},
					onImportSettings = {
						if (useInAppFileBrowser) {
							if (needsAllFilesAccessPermission()) {
								viewModel.permissionRationaleDialog.open()
							} else {
								showFileManagerForImport = true
							}
						} else {
							importSettingsLauncher.launch(arrayOf("*/*"))
						}
					},
					onHelpOpen = {
						viewModel.openHelpDocument()
					}
				)
			},
			bottomBar = {
				val searchDocState = if (
					isTextMode && activeSearchQuery != null && activeSearchOptions != null && !isTouchExplorationEnabled
				) {
					(state as? MainScreenUiState.Success)?.activeTab
				} else {
					null
				}
				val searchListState = searchDocState?.let { listStates[it.documentUri] }
				if (searchDocState != null && searchListState != null) {
					SearchBottomBar(
						docState = searchDocState,
						listState = searchListState,
						activeSearchQuery = activeSearchQuery!!,
						activeSearchOptions = activeSearchOptions!!,
						onClose = { viewModel.clearSearch() },
						onNavigate = { lineIndexToFocus = it }
					)
				} else if (!isTextMode &&
					state is MainScreenUiState.Success &&
					(state as MainScreenUiState.Success).activeTab != null
				) {
					val activeTab = (state as MainScreenUiState.Success).activeTab!!
					val baseNavUnits = remember(activeTab.session) { viewModel.navUnitsFor(activeTab) }
					// Find is only offered as a nav unit once a search is active; it steps through
					// that search's matches instead of opening a separate find bar.
					val navUnits = if (activeSearchQuery != null) baseNavUnits + NavUnit.Find else baseNavUnits
					LaunchedEffect(navUnits) {
						viewModel.ensureNavUnitSupported(navUnits)
					}
					TtsBottomBar(
						isSpeaking = isSpeaking,
						onPlayPause = { viewModel.togglePlayPause() },
						onPrev = { viewModel.playPrevSegment(speak = isSpeaking, announce = !isSpeaking) },
						onNext = { viewModel.playNextSegment(speak = isSpeaking, announce = !isSpeaking) },
						onPrevButton = { viewModel.playPrevSegment(speak = isSpeaking) },
						onNextButton = { viewModel.playNextSegment(speak = isSpeaking) },
						currentUnit = currentNavUnit,
						navUnits = navUnits,
						onNavUnitChange = { viewModel.setNavUnit(it) },
						swipeUpMovesForward = swipeUpMovesForward
					)
				}
			}
		) { padding ->
			Column(modifier = modifier.fillMaxSize().padding(padding)) {
				when (state) {
					MainScreenUiState.Idle -> {
						Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
							// TRANSLATORS: Shown on the main screen before the app has finished loading any document state
							Text(t("No document open. Please open a book."))
						}
					}
					MainScreenUiState.Loading -> {
						Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
							CircularProgressIndicator()
						}
					}
					is MainScreenUiState.Success -> {
						val successState = state as MainScreenUiState.Success
						val docState = successState.activeTab
						if (docState == null) {
							NoDocumentPane(
								recentDocuments = successState.recentDocuments,
								onOpenDocument = { viewModel.openDocument(it.toUri()) },
								onRemoveDocument = { viewModel.removeRecentDocument(it) },
								onLocateDocument = onLocateRecentDocument,
								onShowAllDocuments = { onItemClick(AllDocumentsRoute) }
							)
						} else {
							val listState = listStates.getOrPut(docState.documentUri) {
								LazyListState(firstVisibleItemIndex = docState.initialScrollIndex)
							}
							// What every "take me there" control in this view does once it has worked
							// out which line it wants: record the new position, switch to Text Mode so
							// the destination is actually on screen, and put the focus on it.
							val jumpToLine: (Int) -> Unit = { indexToScroll ->
								viewModel.savePosition(docState.session, docState.documentUri, indexToScroll)
								viewModel.refreshSegmentPreview()
								isTextMode = true
								scope.launch {
									listState.scrollToItem(indexToScroll)
									lineIndexToFocus = indexToScroll
								}
							}
							LaunchedEffect(docState.documentUri) {
								if (docState.initialScrollIndex > 0) {
									lineIndexToFocus = docState.initialScrollIndex
								}
							}
							LaunchedEffect(isTextMode) {
								if (isTextMode) {
									val line = docState.session.lineFromPosition(ttsPosition)
									val index = (line - 1).toInt().coerceAtLeast(0)
									listState.scrollToItem(index)
									lineIndexToFocus = index
								} else {
									viewModel.savePosition(docState.session, docState.documentUri, listState.firstVisibleItemIndex)
									viewModel.refreshSegmentPreview()
								}
							}
							LaunchedEffect(ttsPosition) {
								if (isTextMode) {
									val line = docState.session.lineFromPosition(ttsPosition)
									val index = (line - 1).toInt().coerceAtLeast(0)
									listState.scrollToItem(index)
									lineIndexToFocus = index
								}
							}
							LaunchedEffect(docState.documentUri) {
								snapshotFlow { listState.firstVisibleItemIndex }
									.distinctUntilChanged()
									.debounce(500)
									.collect { index -> viewModel.savePosition(docState.session, docState.documentUri, index) }
							}
							if (!isTextMode) {
								ReadAloudPane(
									segmentText = currentSegmentText,
									textStyle = readability.textStyle,
									sleepTimerRemaining = sleepTimerRemaining,
									onCancelSleepTimer = { viewModel.cancelSleepTimer() }
								)
							} else {
								DocumentTextView(
									docState = docState,
									listState = listState,
									readability = readability,
									lineIndexToFocus = lineIndexToFocus,
									onLineIndexChange = { lineIndexToFocus = it },
									activeSearchQuery = activeSearchQuery,
									activeSearchOptions = activeSearchOptions,
									onCloseSearch = {
										viewModel.clearSearch()
									}
								)
							}
							if (goToDialogOpen) {
								GoToDialog(
									docState = docState,
									onDismiss = { viewModel.closeGoToDialog() },
									initialMode = goToInitialMode,
									onGoTo = jumpToLine
								)
							}
							if (findDialogOpen) {
								FindDialog(
									configManager = viewModel.configManager,
									initialQuery = activeSearchQuery ?: "",
									onDismiss = { viewModel.findDialog.close() },
									onSearch = { query, options ->
										val wasSpeaking = viewModel.ttsManager.isSpeaking.value
										if (wasSpeaking) {
											viewModel.pauseTts()
										}
										val isSameQuery = activeSearchQuery == query &&
											activeSearchOptions?.matchCase == options.matchCase &&
											activeSearchOptions?.wholeWord == options.wholeWord &&
											activeSearchOptions?.regex == options.regex
										viewModel.startSearch(query, options)
										if (!isTextMode) {
											viewModel.setNavUnit(NavUnit.Find)
										}
										val searchPos = if (isTextMode) {
											val nextLineOffset = if (isSameQuery) 2 else 1
											docState.session.positionFromLine((listState.firstVisibleItemIndex + nextLineOffset).toLong())
										} else {
											val currentPos = viewModel.ttsPosition.value
											if (isSameQuery) currentPos + 1L else currentPos
										}
										val res = docState.session.searchFfi(query, searchPos, options)
										if (res.found) {
											if (isTextMode) {
												val targetLine = docState.session.lineFromPosition(res.position)
												val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
												scope.launch {
													listState.scrollToItem(targetIndex)
													lineIndexToFocus = targetIndex
												}
											} else {
												viewModel.jumpToFoundPosition(res.position, resume = wasSpeaking)
											}
										}
									}
								)
							}
							if (showElementsDialog) {
								ElementsDialog(
									headings = currentHeadings,
									links = currentLinks,
									onNavigate = { offset ->
										val line = docState.session.lineFromPosition(offset)
										jumpToLine((line - 1).toInt().coerceAtLeast(0))
									},
									onDismiss = { viewModel.closeElementsDialog() }
								)
							}
						}
						if (exportDocumentDialogOpen) {
							(state as? MainScreenUiState.Success)?.activeTab?.let { docState ->
								ExportDocumentDialog(
									supportedFormats = docState.session.getSupportedExportFormatsFfi(),
									onFormatSelected = { format ->
										selectedExportFormat = format
										exportDocumentDialogOpen = false
										val extension = when (format) {
											uniffi.paperback.ExportFormat.TEXT -> "txt"
											uniffi.paperback.ExportFormat.HTML -> "html"
											uniffi.paperback.ExportFormat.MARKDOWN -> "md"
										}
										val baseName = docState.fileName.substringBeforeLast(".")
										exportDocumentLauncher.launch("$baseName.$extension")
									},
									onDismiss = { exportDocumentDialogOpen = false }
								)
							} ?: run {
								exportDocumentDialogOpen = false
							}
						}
						DocumentToolDialogs(docState = docState, viewModel = viewModel)
					}
					is MainScreenUiState.Error -> {
						Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
							// TRANSLATORS: Shown in place of the document when it could not be opened; {} is the reason
							Text(t("Error loading document: {}", (state as MainScreenUiState.Error).message))
						}
					}
				}
			}
		}
		DocumentPromptDialogs(viewModel)
		val lifecycleOwner = LocalLifecycleOwner.current
		DisposableEffect(lifecycleOwner) {
			val observer = LifecycleEventObserver { _, event ->
				if (event == Lifecycle.Event.ON_RESUME) {
					permissionResumeTrigger++
					if (hasAllFilesAccessOnR() && !useInAppFileBrowser && !hasAutoEnabledInAppFileBrowser) {
						settings.useInAppFileBrowser.set(true)
						hasAutoEnabledInAppFileBrowser = true
						viewModel.configManager.setAppBool("auto_enabled_in_app_file_browser", true)
						viewModel.configManager.flush()
					}
				}
			}
			lifecycleOwner.lifecycle.addObserver(observer)
			onDispose { lifecycleOwner.lifecycle.removeObserver(observer) }
		}
		val showPermissionRationale by viewModel.permissionRationaleDialog.isOpen.collectAsStateWithLifecycle()
		if (showPermissionRationale) {
			PermissionRationaleDialog(
				onGrantClick = {
					viewModel.permissionRationaleDialog.close()
					val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
					intent.data = "package:${context.packageName}".toUri()
					context.startActivity(intent)
				},
				onDismiss = {
					viewModel.permissionRationaleDialog.close()
				}
			)
		}
		if (showFileManager) {
			val extensions = remember(viewModel.configManager) { viewModel.configManager.getSupportedExtensions() }
			val initialDirPath = remember {
				val savedPath = viewModel.configManager.getAppString("last_file_manager_directory", "")
				if (savedPath.isNotEmpty()) {
					savedPath
				} else {
					Environment.getExternalStorageDirectory().absolutePath
				}
			}
			FileManagerDialog(
				supportedExtensions = extensions.toList(),
				initialDirectory = File(initialDirPath),
				onDirectoryChanged = { dir ->
					scope.launch(Dispatchers.IO) {
						viewModel.configManager.setAppString("last_file_manager_directory", dir.absolutePath)
						viewModel.configManager.flush()
					}
				},
				onFileSelected = { file ->
					showFileManager = false
					viewModel.openDocument(Uri.fromFile(file))
				},
				onDismiss = { showFileManager = false }
			)
		}
		if (showFileManagerForImport) {
			val extensions = listOf("paperback")
			val initialDirPath = remember {
				val savedPath = viewModel.configManager.getAppString("last_file_manager_directory", "")
				if (savedPath.isNotEmpty()) {
					savedPath
				} else {
					Environment.getExternalStorageDirectory().absolutePath
				}
			}
			FileManagerDialog(
				supportedExtensions = extensions,
				initialDirectory = File(initialDirPath),
				onDirectoryChanged = { dir ->
					scope.launch(Dispatchers.IO) {
						viewModel.configManager.setAppString("last_file_manager_directory", dir.absolutePath)
						viewModel.configManager.flush()
					}
				},
				onFileSelected = { file ->
					showFileManagerForImport = false
					val uri = Uri.fromFile(file)
					scope.launch(Dispatchers.IO) {
						if (viewModel.importSettingsFromUri(context, uri)) {
							launch(Dispatchers.Main) {
								Toast.makeText(context, t("Settings imported"), Toast.LENGTH_SHORT).show()
							}
						} else {
							launch(Dispatchers.Main) {
								Toast.makeText(context, t("Failed to import settings"), Toast.LENGTH_SHORT).show()
							}
						}
					}
				},
				onDismiss = { showFileManagerForImport = false }
			)
		}
		if (showOnboarding) {
			PermissionsOnboardingScreen(
				showNotificationsSection = notificationsSectionApplicable,
				notificationsGranted = notificationsGranted,
				onEnableNotifications = { notificationPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS) },
				showAllFilesAccessSection = allFilesAccessSectionApplicable,
				allFilesAccessGranted = allFilesAccessGranted,
				onEnableAllFilesAccess = {
					val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
					intent.data = "package:${context.packageName}".toUri()
					context.startActivity(intent)
				},
				onContinue = {
					onboardingCompleted = true
					viewModel.configManager.setAppBool("permissions_onboarding_shown", true)
					viewModel.configManager.flush()
				}
			)
		}
	}
}
