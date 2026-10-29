package dev.paperback.android.ui.screens

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.view.accessibility.AccessibilityManager
import android.widget.Toast
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.*
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
import dev.paperback.android.ui.dialogs.BrowseForFileDialog
import dev.paperback.android.ui.dialogs.DocumentPromptDialogs
import dev.paperback.android.ui.dialogs.DocumentTextDialogs
import dev.paperback.android.ui.dialogs.DocumentToolDialogs
import dev.paperback.android.ui.dialogs.PermissionRationaleDialog
import dev.paperback.android.ui.state.MainScreenUiState
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.NavUnit
import dev.paperback.android.ui.state.OnScreenRequest
import dev.paperback.android.ui.state.activeTab
import dev.paperback.android.ui.state.lineIndexFor
import dev.paperback.android.ui.state.scrollsWorthSaving
import dev.paperback.android.ui.state.shouldSyncPositionFromList
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ExportFormat

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
	modifier: Modifier = Modifier,
	onItemClick: (NavKey) -> Unit = {},
	viewModel: MainScreenViewModel = viewModel()
) {
	val context = LocalContext.current
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val pendingJumpOffset by viewModel.navigation.pendingJumpOffset.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	val listStates = remember { mutableStateMapOf<String, LazyListState>() }
	var selectedExportFormat by remember { mutableStateOf<ExportFormat?>(null) }
	val goToInitialMode by viewModel.navigation.goToInitialMode.collectAsStateWithLifecycle()
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	val settings = viewModel.settings
	val restorePreviousDocuments by settings.restorePreviousDocuments.state.collectAsStateWithLifecycle()
	val useInAppFileBrowser by settings.useInAppFileBrowser.state.collectAsStateWithLifecycle()
	val swipeUpMovesForward by settings.swipeUpMovesForward.state.collectAsStateWithLifecycle()
	val hidePrevNextButtons by settings.hidePrevNextButtons.state.collectAsStateWithLifecycle()
	val speechRatePercent by viewModel.ttsManager.currentSpeechRate.collectAsStateWithLifecycle()
	val ttsEngine by viewModel.ttsManager.currentEngineName.collectAsStateWithLifecycle()
	val activeSearchQuery by viewModel.search.query.collectAsStateWithLifecycle()
	val activeSearchOptions by viewModel.search.options.collectAsStateWithLifecycle()
	val isTextMode by settings.textMode.state.collectAsStateWithLifecycle()

	// An audio-only tab hides the Text Mode toggle, so staying in Text Mode would strand the reader on a blank view with no way back.
	LaunchedEffect(state.activeTab?.documentUri) {
		if (state.activeTab?.isAudioOnly == true) {
			settings.textMode.set(false)
		}
	}

	LaunchedEffect(Unit) {
		viewModel.search.stepRequests.collect { forward ->
			if (!isTextMode) {
				// Without a search, F3 would otherwise step whatever unit is selected instead of doing nothing.
				if (activeSearchQuery != null && activeSearchOptions != null) {
					val speaking = viewModel.ttsManager.isSpeaking.value
					if (forward) {
						viewModel.reading.playNextSegment(speak = speaking, announce = !speaking)
					} else {
						viewModel.reading.playPrevSegment(speak = speaking, announce = !speaking)
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
				viewModel.reading.ttsPosition.value
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
	val currentNavUnit by viewModel.reading.currentNavUnit.collectAsStateWithLifecycle()
	val ttsPosition by viewModel.reading.ttsPosition.collectAsStateWithLifecycle()
	val currentSegmentText by viewModel.reading.currentSegmentText.collectAsStateWithLifecycle()
	val textScalePercent by settings.textScalePercent.state.collectAsStateWithLifecycle()
	val lineSpacing by settings.lineSpacing.state.collectAsStateWithLifecycle()
	val paragraphSpacing by settings.paragraphSpacing.state.collectAsStateWithLifecycle()
	val textAlignment by settings.textAlignment.state.collectAsStateWithLifecycle()
	val highContrastText by settings.highContrastText.state.collectAsStateWithLifecycle()
	val readability = rememberReadabilityStyle(
		textScalePercent,
		lineSpacing,
		paragraphSpacing,
		textAlignment,
		highContrastText
	)
	var ttsConfigDialogOpen by remember { mutableStateOf(false) }
	val sleepTimerRemaining by viewModel.reading.sleepTimer.remaining.collectAsStateWithLifecycle()

	val view = LocalView.current
	LaunchedEffect(Unit) {
		viewModel.accessibilityAnnouncement.collect { message ->
			@Suppress("DEPRECATION")
			view.announceForAccessibility(message)
		}
	}

	LaunchedEffect(Unit) {
		viewModel.reading.sleepTimer.expired.collect {
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
		contract = ActivityResultContracts.OpenMultipleDocuments(),
		onResult = { uris -> if (uris.isNotEmpty()) viewModel.openDocuments(uris) }
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
				scope.reportResult(
					context = context,
					// TRANSLATORS: Toast confirming a .paperback settings file was imported successfully, or the failure message if not
					succeeded = t("Settings imported"),
					failed = t("Failed to import settings")
				) { viewModel.importSettingsFromUri(context, uri) }
			}
		}
	)

	val exportSettingsLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.CreateDocument("*/*"),
		onResult = { uri ->
			if (uri != null) {
				scope.reportResult(
					context = context,
					// TRANSLATORS: Toast confirming the document's settings were exported to a .paperback file, or the failure message if not
					succeeded = t("Settings exported"),
					failed = t("Failed to export settings")
				) { viewModel.exportSettingsToUri(context, uri) }
			}
		}
	)

	val exportDocumentLauncher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.CreateDocument("*/*"),
		onResult = { uri ->
			if (uri != null) {
				selectedExportFormat?.let { format ->
					scope.reportResult(
						context = context,
						// TRANSLATORS: Toast confirming the document was exported successfully, or the failure message if not
						succeeded = t("Document exported"),
						failed = t("Failed to export document")
					) { viewModel.exportDocumentToUri(context, uri, format) }
				}
			}
		}
	)

	// Shared by the menu and the keyboard shortcuts so both pick the same picker for the setting and the document's origin.
	val openBook: () -> Unit = {
		if (useInAppFileBrowser) {
			if (needsAllFilesAccessPermission()) {
				viewModel.navigation.permissionRationaleDialog.open()
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
				viewModel.navigation.permissionRationaleDialog.open()
			} else {
				showFileManagerForImport = true
			}
		} else {
			importSettingsLauncher.launch(arrayOf("*/*"))
		}
	}

	OnScreenRequest(viewModel.navigation.openBookRequest, openBook)
	OnScreenRequest(viewModel.navigation.exportSettingsRequest, exportSettings)
	OnScreenRequest(viewModel.navigation.importSettingsRequest, importSettings)

	Box(modifier = Modifier.fillMaxSize()) {
		Scaffold(
			// safeDrawing keeps text clear of a landscape cutout; applied only here so the bars' own padding isn't counted twice.
			contentWindowInsets = WindowInsets.safeDrawing,
			topBar = {
				MainScreenTopBar(
					state = state,
					isTextMode = isTextMode,
					isSpeaking = isSpeaking,
					onOpenBook = openBook,
					onTocOpen = { viewModel.navigation.tocRequest.request() },
					onTabSelect = { viewModel.setActiveTab(it) },
					onTabClose = { viewModel.closeTab(it) },
					onToggleTextMode = { settings.textMode.set(!isTextMode) },
					onTogglePlayPause = { viewModel.reading.togglePlayPause() },
					onRecentsOpen = { onItemClick(AllDocumentsRoute) },
					onGoToOpen = { viewModel.openGoToDialog() },
					onFindOpen = { viewModel.navigation.findDialog.open() },
					onWordCountOpen = { viewModel.openWordCountDialog() },
					onDocumentInfoOpen = { viewModel.navigation.documentInfoDialog.open() },
					onSettingsOpen = { onItemClick(SettingsRoute) },
					onSleepTimerOpen = { viewModel.navigation.sleepTimerDialog.open() },
					onElementsOpen = { viewModel.openElements() },
					onExportDocumentOpen = { viewModel.openExportDocumentDialog() },
					onExportSettings = exportSettings,
					onImportSettings = importSettings,
					onOpenFromUrl = { viewModel.links.prompt() },
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
					val audioSpeed by viewModel.reading.audioSpeed.collectAsStateWithLifecycle()
					val baseNavUnits = remember(activeTab.session) { viewModel.reading.navUnitsFor(activeTab) }
					val navUnits = if (activeSearchQuery != null) baseNavUnits + NavUnit.Find else baseNavUnits
					LaunchedEffect(navUnits) {
						viewModel.reading.ensureNavUnitSupported(navUnits)
					}
					TtsBottomBar(
						isSpeaking = isSpeaking,
						onPlayPause = { viewModel.reading.togglePlayPause() },
						onPrev = { viewModel.reading.playPrevSegment(speak = isSpeaking, announce = !isSpeaking) },
						onNext = { viewModel.reading.playNextSegment(speak = isSpeaking, announce = !isSpeaking) },
						onPrevButton = { viewModel.reading.playPrevSegment(speak = isSpeaking) },
						onNextButton = { viewModel.reading.playNextSegment(speak = isSpeaking) },
						currentUnit = currentNavUnit,
						navUnits = navUnits,
						onNavUnitChange = { viewModel.reading.setNavUnit(it) },
						speechRatePercent = speechRatePercent.takeUnless { ttsEngine == null },
						onSpeechRateChange = { viewModel.ttsManager.setSpeechRate(it) },
						audioSpeed = audioSpeed.takeIf { activeTab.hasAudio },
						onAudioSpeedChange = { viewModel.reading.setAudioSpeed(it) },
						swipeUpMovesForward = swipeUpMovesForward,
						hidePrevNextButtons = hidePrevNextButtons
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
							val jumpToLine: (Int) -> Unit = { indexToScroll ->
								viewModel.reading.savePosition(docState.session, docState.documentUri, indexToScroll)
								viewModel.reading.refreshSegmentPreview()
								settings.textMode.set(true)
								scope.launch {
									listState.scrollToItem(indexToScroll)
									lineIndexToFocus = indexToScroll
								}
							}
							// Set by the elements screen, which cannot switch to Text Mode or scroll the list itself.
							LaunchedEffect(pendingJumpOffset) {
								val offset = pendingJumpOffset
								if (offset != null) {
									val line = docState.session.lineFromPosition(offset)
									jumpToLine(lineIndexFor(line))
									viewModel.navigation.consumeJumpRequest()
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
									viewModel.reading.savePosition(docState.session, docState.documentUri, listState.firstVisibleItemIndex)
									viewModel.reading.refreshSegmentPreview()
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
									.collect { index -> viewModel.reading.savePosition(docState.session, docState.documentUri, index) }
							}
							if (!isTextMode) {
								// Neither an audio-only book's text (a blank line per file) nor its placeholder clip durations give a true share, so it shows none rather than a wrong one.
								val session = docState.session
								val progressPercent = remember(session, docState.isAudioOnly, ttsPosition) {
									if (docState.isAudioOnly) null else session.getStatusInfo(ttsPosition).percentage
								}
								ReadAloudPane(
									segmentText = currentSegmentText,
									readability = readability,
									progressPercent = progressPercent,
									sleepTimerRemaining = sleepTimerRemaining,
									onCancelSleepTimer = { viewModel.reading.sleepTimer.cancel() }
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
		val showPermissionRationale by viewModel.navigation.permissionRationaleDialog.isOpen.collectAsStateWithLifecycle()
		if (showPermissionRationale) {
			PermissionRationaleDialog(
				onGrantClick = {
					viewModel.navigation.permissionRationaleDialog.close()
					openAllFilesAccessSettings(context)
				},
				onDismiss = {
					viewModel.navigation.permissionRationaleDialog.close()
				}
			)
		}
		if (showFileManager) {
			val extensions = remember(viewModel.configManager) { viewModel.configManager.getSupportedExtensions() }
			BrowseForFileDialog(
				configManager = viewModel.configManager,
				supportedExtensions = extensions.toList(),
				scope = scope,
				onFileSelected = { file ->
					showFileManager = false
					viewModel.openDocument(Uri.fromFile(file))
				},
				onDismiss = { showFileManager = false }
			)
		}
		if (showFileManagerForImport) {
			BrowseForFileDialog(
				configManager = viewModel.configManager,
				supportedExtensions = listOf("paperback"),
				scope = scope,
				onFileSelected = { file ->
					showFileManagerForImport = false
					scope.reportResult(
						context = context,
						// TRANSLATORS: Toast confirming a .paperback settings file was imported successfully, or the failure message if not
						succeeded = t("Settings imported"),
						failed = t("Failed to import settings")
					) { viewModel.importSettingsFromUri(context, Uri.fromFile(file)) }
				},
				onDismiss = { showFileManagerForImport = false }
			)
		}
		PermissionsGate(viewModel)
	}
}

/** The result is a file the reader can't see from here, so it has to be said. */
private fun CoroutineScope.reportResult(context: Context, succeeded: String, failed: String, work: () -> Boolean) {
	launch(Dispatchers.IO) {
		val message = if (work()) succeeded else failed
		withContext(Dispatchers.Main) {
			Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
		}
	}
}
