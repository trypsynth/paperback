package dev.paperback.android.ui.screens

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.Uri
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
import androidx.compose.ui.platform.LocalView
import androidx.core.net.toUri
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.NavKey
import dev.paperback.android.AllDocumentsRoute
import dev.paperback.android.SettingsRoute
import dev.paperback.android.t
import dev.paperback.android.ui.components.DocumentTextView
import dev.paperback.android.ui.components.MainScreenTopBar
import dev.paperback.android.ui.components.NoDocumentPane
import dev.paperback.android.ui.components.ReadAloudPane
import dev.paperback.android.ui.components.SearchBottomBar
import dev.paperback.android.ui.components.TtsBottomBar
import dev.paperback.android.ui.components.rememberReadabilityStyle
import dev.paperback.android.ui.dialogs.DocumentPromptDialogs
import dev.paperback.android.ui.dialogs.DocumentTextDialogs
import dev.paperback.android.ui.dialogs.DocumentToolDialogs
import dev.paperback.android.ui.dialogs.FileManagerDialog
import dev.paperback.android.ui.dialogs.PermissionRationaleDialog
import dev.paperback.android.ui.state.MainScreenUiState
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.NavUnit
import dev.paperback.android.ui.state.OnScreenRequest
import dev.paperback.android.ui.state.activeTab
import dev.paperback.android.ui.state.lineIndexFor
import dev.paperback.android.ui.state.scrollsWorthSaving
import dev.paperback.android.ui.state.shouldSyncPositionFromList
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ExportFormat
import java.io.File

private const val LAST_FILE_MANAGER_DIRECTORY_KEY = "last_file_manager_directory"

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
	modifier: Modifier = Modifier,
	onItemClick: (NavKey) -> Unit = {},
	viewModel: MainScreenViewModel = viewModel()
) {
	val context = LocalContext.current
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val pendingJumpOffset by viewModel.pendingJumpOffset.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	val listStates = remember { mutableStateMapOf<String, LazyListState>() }
	var selectedExportFormat by remember { mutableStateOf<ExportFormat?>(null) }
	val goToInitialMode by viewModel.goToInitialMode.collectAsStateWithLifecycle()
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	val settings = viewModel.settings
	val restorePreviousDocuments by settings.restorePreviousDocuments.state.collectAsStateWithLifecycle()
	val useInAppFileBrowser by settings.useInAppFileBrowser.state.collectAsStateWithLifecycle()
	val swipeUpMovesForward by settings.swipeUpMovesForward.state.collectAsStateWithLifecycle()
	val activeSearchQuery by viewModel.search.query.collectAsStateWithLifecycle()
	val activeSearchOptions by viewModel.search.options.collectAsStateWithLifecycle()
	var isTextMode by rememberSaveable { mutableStateOf(false) }

	// An audio-only tab has no real text spine to show in Text Mode (its top-bar toggle is
	// hidden for the same reason), so switching to one from a Text Mode session falls back to
	// Read-Aloud mode instead of stranding the user on a blank text view with no way back.
	LaunchedEffect(state.activeTab?.documentUri) {
		if (state.activeTab?.isAudioOnly == true) {
			isTextMode = false
		}
	}

	// F3/Shift+F3 (MainActivity) trigger this. In Read-Aloud mode, Find is a nav unit, so this
	// just steps it the same way the nav-unit slider's Previous/Next buttons do; only Text mode
	// (which has no nav-unit slider) still needs this handler's own search-and-scroll logic.
	LaunchedEffect(Unit) {
		viewModel.search.stepRequests.collect { forward ->
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
			val query = activeSearchQuery ?: return@collect
			val options = activeSearchOptions ?: return@collect
			val tab = viewModel.uiState.value.activeTab ?: return@collect
			val listState = listStates[tab.documentUri]
			val searchPos = if (listState != null) {
				val nextLine = (listState.firstVisibleItemIndex + if (forward) 2 else 1).toLong()
				tab.session.positionFromLine(nextLine)
			} else {
				viewModel.ttsPosition.value
			}
			val res = tab.session.searchFfi(query, searchPos, options.copy(forward = forward))
			if (res.found) {
				val line = tab.session.lineFromPosition(res.position)
				val indexToScroll = lineIndexFor(line)
				listState?.scrollToItem(indexToScroll)
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
	val sleepTimerRemaining by viewModel.sleepTimer.remaining.collectAsStateWithLifecycle()

	val view = LocalView.current
	LaunchedEffect(Unit) {
		viewModel.accessibilityAnnouncement.collect { message ->
			@Suppress("DEPRECATION")
			view.announceForAccessibility(message)
		}
	}

	LaunchedEffect(Unit) {
		viewModel.sleepTimer.expired.collect {
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
						// TRANSLATORS: Toast confirming the document was exported successfully, or the failure message if not
						val message = if (success) t("Document exported") else t("Failed to export document")
						withContext(Dispatchers.Main) {
							Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
						}
					}
				}
			}
		}
	)

	// The three actions that pick a file. Which picker each one uses depends on a setting and on
	// where the document came from, so the top bar's menu items and the keyboard shortcuts share
	// these rather than each deciding for themselves.
	val openBook: () -> Unit = {
		if (useInAppFileBrowser) {
			if (needsAllFilesAccessPermission()) {
				viewModel.permissionRationaleDialog.open()
			} else {
				showFileManager = true
			}
		} else {
			filePickerLauncher.launch(supportedMimeTypes)
		}
	}
	val exportSettings: () -> Unit = {
		val activeDocUri = state.activeTab?.documentUri
		if (activeDocUri != null) {
			if (activeDocUri.startsWith("content://")) {
				exportSettingsLauncher.launch("document.paperback")
			} else {
				// TRANSLATORS: Toast confirming the document's settings were exported to a .paperback file, or the failure message if not
				if (viewModel.exportCurrentSettings()) {
					Toast.makeText(context, t("Settings exported"), Toast.LENGTH_SHORT).show()
				} else {
					Toast.makeText(context, t("Failed to export settings"), Toast.LENGTH_SHORT).show()
				}
			}
		}
	}
	val importSettings: () -> Unit = {
		if (useInAppFileBrowser) {
			if (needsAllFilesAccessPermission()) {
				viewModel.permissionRationaleDialog.open()
			} else {
				showFileManagerForImport = true
			}
		} else {
			importSettingsLauncher.launch(arrayOf("*/*"))
		}
	}

	OnScreenRequest(viewModel.openBookRequest, openBook)
	OnScreenRequest(viewModel.exportSettingsRequest, exportSettings)
	OnScreenRequest(viewModel.importSettingsRequest, importSettings)

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
					onOpenBook = openBook,
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
					onElementsOpen = { viewModel.openElements() },
					onExportDocumentOpen = { viewModel.openExportDocumentDialog() },
					onExportSettings = exportSettings,
					onImportSettings = importSettings,
					onHelpOpen = {
						viewModel.openHelpDocument()
					}
				)
			},
			bottomBar = {
				val activeTab = state.activeTab
				val searchDocState = if (
					isTextMode && activeSearchQuery != null && activeSearchOptions != null && !isTouchExplorationEnabled
				) {
					activeTab
				} else {
					null
				}
				val searchListState = searchDocState?.let { listStates[it.documentUri] }
				val searchQuery = activeSearchQuery
				val searchOptions = activeSearchOptions
				if (searchDocState != null && searchListState != null && searchQuery != null && searchOptions != null) {
					SearchBottomBar(
						docState = searchDocState,
						listState = searchListState,
						activeSearchQuery = searchQuery,
						activeSearchOptions = searchOptions,
						onClose = { viewModel.search.clear() },
						onNavigate = { lineIndexToFocus = it }
					)
				} else if (!isTextMode && activeTab != null) {
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
							// Set by the elements screen, which cannot switch to Text Mode or
							// scroll the list itself.
							LaunchedEffect(pendingJumpOffset) {
								val offset = pendingJumpOffset
								if (offset != null) {
									val line = docState.session.lineFromPosition(offset)
									jumpToLine(lineIndexFor(line))
									viewModel.consumeJumpRequest()
								}
							}
							LaunchedEffect(docState.documentUri) {
								if (docState.initialScrollIndex > 0) {
									lineIndexToFocus = docState.initialScrollIndex
								}
							}
							var previousTextMode by remember { mutableStateOf<Boolean?>(null) }
							LaunchedEffect(isTextMode) {
								val previous = previousTextMode
								previousTextMode = isTextMode
								if (isTextMode) {
									val line = docState.session.lineFromPosition(ttsPosition)
									val index = lineIndexFor(line)
									listState.scrollToItem(index)
									lineIndexToFocus = index
								} else if (shouldSyncPositionFromList(previous, isTextMode)) {
									viewModel.savePosition(docState.session, docState.documentUri, listState.firstVisibleItemIndex)
									viewModel.refreshSegmentPreview()
								}
							}
							LaunchedEffect(ttsPosition) {
								if (isTextMode) {
									val line = docState.session.lineFromPosition(ttsPosition)
									val index = lineIndexFor(line)
									listState.scrollToItem(index)
									lineIndexToFocus = index
								}
							}
							LaunchedEffect(docState.documentUri) {
								snapshotFlow { listState.firstVisibleItemIndex }
									.scrollsWorthSaving()
									.collect { index -> viewModel.savePosition(docState.session, docState.documentUri, index) }
							}
							if (!isTextMode) {
								// An audio-only book's buffer is one blank line per narration file, so a
								// share of its characters says nothing about how far in the reader is,
								// and its recording is no better: a plain zip of narration files gives
								// every clip the same placeholder duration, so a share of the running
								// time is just as made up. It goes without a reading rather than with
								// a wrong one.
								val session = docState.session
								val progressPercent = remember(session, docState.isAudioOnly, ttsPosition) {
									if (docState.isAudioOnly) null else session.getStatusInfo(ttsPosition).percentage
								}
								ReadAloudPane(
									segmentText = currentSegmentText,
									textStyle = readability.textStyle,
									progressPercent = progressPercent,
									sleepTimerRemaining = sleepTimerRemaining,
									onCancelSleepTimer = { viewModel.sleepTimer.cancel() }
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
										viewModel.search.clear()
									}
								)
							}
							DocumentTextDialogs(
								docState = docState,
								viewModel = viewModel,
								isTextMode = isTextMode,
								listState = listState,
								goToInitialMode = goToInitialMode,
								onGoToLine = jumpToLine,
								onFocusLine = { lineIndexToFocus = it }
							)
						}
						DocumentToolDialogs(
							docState = docState,
							viewModel = viewModel,
							onExportFormatChosen = { format, fileName ->
								selectedExportFormat = format
								exportDocumentLauncher.launch(fileName)
							}
						)
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
				val savedPath = viewModel.configManager.getAppString(LAST_FILE_MANAGER_DIRECTORY_KEY, "")
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
						viewModel.configManager.setAppString(LAST_FILE_MANAGER_DIRECTORY_KEY, dir.absolutePath)
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
				val savedPath = viewModel.configManager.getAppString(LAST_FILE_MANAGER_DIRECTORY_KEY, "")
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
						viewModel.configManager.setAppString(LAST_FILE_MANAGER_DIRECTORY_KEY, dir.absolutePath)
						viewModel.configManager.flush()
					}
				},
				onFileSelected = { file ->
					showFileManagerForImport = false
					val uri = Uri.fromFile(file)
					scope.launch(Dispatchers.IO) {
						// TRANSLATORS: Toast confirming a .paperback settings file was imported successfully, or the failure message if not
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
		PermissionsGate(viewModel)
	}
}
