package dev.paperback.mobile.ui

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.view.accessibility.AccessibilityManager
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.NavKey
import dev.paperback.mobile.ui.dialogs.*
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.launch
import androidx.compose.foundation.lazy.items as lazyItems
import dev.paperback.mobile.t

@OptIn(ExperimentalMaterial3Api::class, kotlinx.coroutines.FlowPreview::class)
@Composable
fun MainScreen(
	onItemClick: (NavKey) -> Unit = {},
	modifier: Modifier = Modifier,
	viewModel: MainScreenViewModel = viewModel()
) {
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	val listStates = remember { mutableStateMapOf<String, LazyListState>() }
	val tocSheetOpen by viewModel.showTocDialog.collectAsStateWithLifecycle()
	var recentsDialogOpen by remember { mutableStateOf(false) }
	val wordCountDialogOpen by viewModel.showWordCountDialog.collectAsStateWithLifecycle()
	val documentInfoDialogOpen by viewModel.showDocumentInfoDialog.collectAsStateWithLifecycle()
	val goToDialogOpen by viewModel.showGoToDialog.collectAsStateWithLifecycle()
	val goToInitialMode by viewModel.goToInitialMode.collectAsStateWithLifecycle()
	val findDialogOpen by viewModel.showFindDialog.collectAsStateWithLifecycle()
	val optionsDialogOpen by viewModel.showSettingsDialog.collectAsStateWithLifecycle()
	val sleepTimerDialogOpen by viewModel.showSleepTimerDialog.collectAsStateWithLifecycle()
	var isScreenDimmed by remember { mutableStateOf(false) }
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	var restorePreviousDocuments by remember {
		mutableStateOf(viewModel.configManager.getAppBool("restore_previous_documents", true))
	}
	var useInAppFileBrowser by remember {
		mutableStateOf(viewModel.configManager.getAppBool("use_in_app_file_browser", false))
	}
	var swipeUpMovesForward by remember {
		mutableStateOf(viewModel.configManager.getAppBool("swipe_up_moves_forward", true))
	}
	val activeSearchQuery by viewModel.activeSearchQuery.collectAsStateWithLifecycle()
	val activeSearchOptions by viewModel.activeSearchOptions.collectAsStateWithLifecycle()
	var expandedTocIndices by remember { mutableStateOf(setOf<Int>()) }
	var activeTocIndex by remember { mutableStateOf<Int?>(null) }
	var isTextMode by remember { mutableStateOf(false) }

	LaunchedEffect(tocSheetOpen) {
		if (tocSheetOpen) {
			val stateValue = viewModel.uiState.value
			if (stateValue is MainScreenUiState.Success) {
				val tab = stateValue.activeTab
				if (tab != null) {
					val toc = tab.toc
					if (toc.isNotEmpty()) {
						var activeIndex = 0
						var bestDistance = Long.MAX_VALUE
						val currentPos = viewModel.ttsPosition.value

						for (i in toc.indices) {
							if (toc[i].position <= currentPos) {
								val distance = currentPos - toc[i].position
								if (distance < bestDistance) {
									bestDistance = distance
									activeIndex = i
								}
							}
						}
						activeTocIndex = activeIndex

						val toExpand = mutableSetOf<Int>()
						var currentLevel = toc[activeIndex].level
						for (i in activeIndex - 1 downTo 0) {
							if (toc[i].level < currentLevel) {
								toExpand.add(i)
								currentLevel = toc[i].level
								if (currentLevel == 0) break
							}
						}
						expandedTocIndices = expandedTocIndices + toExpand
					}
				}
			}
		} else {
			activeTocIndex = null
		}
	}

	LaunchedEffect(Unit) {
		viewModel.performSearchEvent.collect { forward ->
			if (activeSearchQuery != null && activeSearchOptions != null) {
				val state = viewModel.uiState.value
				if (state is MainScreenUiState.Success) {
					val tab = state.activeTab
					if (tab != null) {
						val searchPos = if (isTextMode) {
							val listState = listStates[tab.documentUri]
							if (listState != null) {
								val nextLine = (listState.firstVisibleItemIndex + if (forward) 2 else 1).toLong()
								tab.session.positionFromLine(nextLine)
							} else {
								viewModel.ttsPosition.value
							}
						} else {
							val currentPos = viewModel.ttsPosition.value
							if (forward) currentPos + 1L else currentPos
						}

						val res = tab.session.searchFfi(activeSearchQuery!!, searchPos, activeSearchOptions!!.copy(forward = forward))
						if (res.found) {
							if (isTextMode) {
								val line = tab.session.lineFromPosition(res.position)
								val indexToScroll = (line - 1).toInt().coerceAtLeast(0)
								val listState = listStates[tab.documentUri]
								listState?.scrollToItem(indexToScroll)
							} else {
								viewModel.updateTtsPosition(res.position)
								viewModel.refreshSegmentPreview()
								if (viewModel.ttsManager.isSpeaking.value) {
									viewModel.resumeTts()
								}
							}
						}
					}
				}
			}
		}
	}
	val isSpeaking by viewModel.ttsManager.isSpeaking.collectAsStateWithLifecycle()
	val currentSegmentType by viewModel.currentSegmentType.collectAsStateWithLifecycle()
	val ttsPosition by viewModel.ttsPosition.collectAsStateWithLifecycle()
	val currentSpeechRate by viewModel.ttsManager.currentSpeechRate.collectAsStateWithLifecycle()
	val currentPitch by viewModel.ttsManager.currentPitch.collectAsStateWithLifecycle()
	val currentSegmentText by viewModel.currentSegmentText.collectAsStateWithLifecycle()
	val availableVoices by viewModel.ttsManager.availableVoices.collectAsStateWithLifecycle()
	val currentVoice by viewModel.ttsManager.currentVoice.collectAsStateWithLifecycle()
	val currentEngineName by viewModel.ttsManager.currentEngineName.collectAsStateWithLifecycle()
	var ttsConfigDialogOpen by remember { mutableStateOf(false) }
	val sleepTimerRemaining by viewModel.sleepTimerRemaining.collectAsStateWithLifecycle()
	val showElementsDialog by viewModel.showElementsDialog.collectAsStateWithLifecycle()
	val currentHeadings by viewModel.currentHeadings.collectAsStateWithLifecycle()
	val currentLinks by viewModel.currentLinks.collectAsStateWithLifecycle()
	val passwordPromptUri by viewModel.passwordPromptUri.collectAsStateWithLifecycle()
	val importPromptPath by viewModel.importPromptPath.collectAsStateWithLifecycle()

	val view = androidx.compose.ui.platform.LocalView.current
	LaunchedEffect(Unit) {
		viewModel.accessibilityAnnouncement.collect { message ->
			@Suppress("DEPRECATION")
			view.announceForAccessibility(message)
		}
	}

	LaunchedEffect(Unit) {
		viewModel.sleepTimerExpired.collect {
			isScreenDimmed = true
		}
	}
	val context = LocalContext.current
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
	val activity = context as? android.app.Activity
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

	val filePickerLauncher = androidx.activity.compose.rememberLauncherForActivityResult(
		contract = androidx.activity.result.contract.ActivityResultContracts.OpenDocument(),
		onResult = { uri -> uri?.let { viewModel.openDocument(it) } }
	)

	var showFileManager by remember { mutableStateOf(false) }
	var showFileManagerForImport by remember { mutableStateOf(false) }

	val importSettingsLauncher = androidx.activity.compose.rememberLauncherForActivityResult(
		contract = androidx.activity.result.contract.ActivityResultContracts.OpenDocument(),
		onResult = { uri ->
			if (uri != null) {
				scope.launch(kotlinx.coroutines.Dispatchers.IO) {
					val success = viewModel.importSettingsFromUri(context, uri)
					val message = if (success) t("Settings imported") else t("Failed to import settings")
					kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.Main) {
						android.widget.Toast.makeText(context, message, android.widget.Toast.LENGTH_SHORT).show()
					}
				}
			}
		}
	)

	val exportSettingsLauncher = androidx.activity.compose.rememberLauncherForActivityResult(
		contract = androidx.activity.result.contract.ActivityResultContracts.CreateDocument("*/*"),
		onResult = { uri ->
			if (uri != null) {
				scope.launch(kotlinx.coroutines.Dispatchers.IO) {
					val success = viewModel.exportSettingsToUri(context, uri)
					val message = if (success) t("Settings exported") else t("Failed to export settings")
					kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.Main) {
						android.widget.Toast.makeText(context, message, android.widget.Toast.LENGTH_SHORT).show()
					}
				}
			}
		}
	)

	Box(modifier = Modifier.fillMaxSize()) {
	Scaffold(
		topBar = {
			MainScreenTopBar(
				state = state,
				isTextMode = isTextMode,
				isSpeaking = isSpeaking,
				onOpenBook = {
					if (useInAppFileBrowser) {
						if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R && !android.os.Environment.isExternalStorageManager()) {
							viewModel.setShowPermissionRationale(true)
						} else {
							showFileManager = true
						}
					} else {
						filePickerLauncher.launch(supportedMimeTypes)
					}
				},
				onTocOpen = { viewModel.openTocDialog() },
				onTabSelect = { viewModel.setActiveTab(it) },
				onTabClose = { viewModel.closeTab(it) },
				onToggleTextMode = { isTextMode = !isTextMode },
				onTogglePlayPause = { viewModel.togglePlayPause() },
				onRecentsOpen = { recentsDialogOpen = true },
				onGoToOpen = { viewModel.openGoToDialog() },
				onFindOpen = { viewModel.openFindDialog() },
				onWordCountOpen = { viewModel.openWordCountDialog() },
				onDocumentInfoOpen = { viewModel.openDocumentInfoDialog() },
				onSettingsOpen = { viewModel.openSettingsDialog() },
				onSleepTimerOpen = { viewModel.openSleepTimerDialog() },
				onElementsOpen = { viewModel.openElementsDialog() },
				onExportSettings = {
					val activeDocUri = (state as? MainScreenUiState.Success)?.activeTab?.documentUri
					if (activeDocUri != null) {
						if (activeDocUri.startsWith("content://")) {
							exportSettingsLauncher.launch("document.paperback")
						} else {
							if (viewModel.exportCurrentSettings()) {
								android.widget.Toast.makeText(context, t("Settings exported"), android.widget.Toast.LENGTH_SHORT).show()
							} else {
								android.widget.Toast.makeText(context, t("Failed to export settings"), android.widget.Toast.LENGTH_SHORT).show()
							}
						}
					}
				},
				onImportSettings = {
					if (useInAppFileBrowser) {
						if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R && !android.os.Environment.isExternalStorageManager()) {
							viewModel.setShowPermissionRationale(true)
						} else {
							showFileManagerForImport = true
						}
					} else {
						importSettingsLauncher.launch(arrayOf("*/*"))
					}
				}
			)
		},
		bottomBar = {
			val searchDocState = if (activeSearchQuery != null && activeSearchOptions != null && !isTouchExplorationEnabled) {
				(state as? MainScreenUiState.Success)?.activeTab
			} else null
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
				val supportedSegmentTypes = remember(activeTab.session) {
					activeTab.session.getSupportedSegmentTypesFfi()
				}

				LaunchedEffect(supportedSegmentTypes) {
					if (!supportedSegmentTypes.contains(currentSegmentType)) {
						viewModel.setSegmentType(uniffi.paperback.SegmentTypeFfi.PARAGRAPH)
					}
				}

				TtsBottomBar(
					isSpeaking = isSpeaking,
					onPlayPause = { viewModel.togglePlayPause() },
					onPrev = { viewModel.playPrevSegment(speak = isSpeaking, announce = !isSpeaking) },
					onNext = { viewModel.playNextSegment(speak = isSpeaking, announce = !isSpeaking) },
					onPrevButton = { viewModel.playPrevSegment(speak = isSpeaking) },
					onNextButton = { viewModel.playNextSegment(speak = isSpeaking) },
					currentSegmentType = currentSegmentType,
					supportedSegmentTypes = supportedSegmentTypes,
					onSegmentTypeChange = { viewModel.setSegmentType(it) },
					swipeUpMovesForward = swipeUpMovesForward
				)
			}
		}
	) { padding ->
		Column(modifier = modifier.fillMaxSize().padding(padding)) {
			when (state) {
				MainScreenUiState.Idle -> {
					Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
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
						Column(
							modifier = Modifier.fillMaxSize().padding(16.dp),
							horizontalAlignment = Alignment.CenterHorizontally,
							verticalArrangement = Arrangement.Center
						) {
							Text(
								"No document open.",
								style = MaterialTheme.typography.titleLarge,
								modifier = Modifier.padding(bottom = 24.dp)
							)
							if (successState.recentDocuments.isNotEmpty()) {
								Text(
									"Recently Opened",
									style = MaterialTheme.typography.titleMedium,
									modifier = Modifier.padding(bottom = 8.dp)
								)
								LazyColumn(
									modifier = Modifier.weight(1f).fillMaxWidth(),
									contentPadding = PaddingValues(vertical = 8.dp)
								) {
									lazyItems(successState.recentDocuments.take(5)) { recentDoc ->
										RecentDocumentItemRow(
											item = recentDoc,
											showClosedStatus = false,
											onOpen = { viewModel.openDocument(Uri.parse(recentDoc.uri)) },
											onRemove = { viewModel.removeRecentDocument(recentDoc.uri) }
										)
									}
								}
								TextButton(
									onClick = { recentsDialogOpen = true },
									modifier = Modifier.padding(top = 8.dp)
								) {
									Text(t("Recent Documents"))
								}
							}
						}
					} else {
						val listState = listStates.getOrPut(docState.documentUri) {
							LazyListState(firstVisibleItemIndex = docState.initialScrollIndex)
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
							Column(
								modifier = Modifier.fillMaxSize().padding(16.dp),
								horizontalAlignment = Alignment.CenterHorizontally,
								verticalArrangement = Arrangement.Center
							) {
								Text(
									text = currentSegmentText,
									style = MaterialTheme.typography.bodyLarge,
									modifier = Modifier.padding(16.dp).semantics {
										val actions = mutableListOf<CustomAccessibilityAction>()
										if (activeSearchQuery != null && activeSearchOptions != null) {
											actions.add(CustomAccessibilityAction(t("Find Next")) {
												viewModel.triggerFindNext()
												true
											})
											actions.add(CustomAccessibilityAction(t("Find Previous")) {
												viewModel.triggerFindPrevious()
												true
											})
											actions.add(CustomAccessibilityAction(t("Close Search")) {
												viewModel.clearSearch()
												true
											})
										}
										if (actions.isNotEmpty()) {
											customActions = actions
										}
									}
								)
								val remaining = sleepTimerRemaining
								if (remaining != null) {
									val min = remaining / 60
									val sec = remaining % 60
									Text(
										"Sleep timer: %d:%02d".format(min, sec),
										style = MaterialTheme.typography.labelMedium,
										color = MaterialTheme.colorScheme.onSurfaceVariant,
										modifier = Modifier.semantics {
											customActions = listOf(
												CustomAccessibilityAction(t("Cancel sleep timer")) {
													viewModel.cancelSleepTimer()
													true
												}
											)
										}
									)
								}
							}
						} else {
							DocumentTextView(
								docState = docState,
								listState = listState,
								lineIndexToFocus = lineIndexToFocus,
								onLineIndexChange = { lineIndexToFocus = it },
								activeSearchQuery = activeSearchQuery,
								activeSearchOptions = activeSearchOptions,
								onCloseSearch = {
									viewModel.clearSearch()
								}
							)
						}
						if (tocSheetOpen) {
							TocDialog(
								toc = docState.toc,
								expandedTocIndices = expandedTocIndices,
								activeTocIndex = activeTocIndex,
								onToggleExpand = { originalIndex ->
									expandedTocIndices = if (expandedTocIndices.contains(originalIndex)) {
										expandedTocIndices - originalIndex
									} else {
										expandedTocIndices + originalIndex
									}
								},
								onItemClick = { item ->
									viewModel.updateTtsPosition(item.position)
									val line = docState.session.lineFromPosition(item.position)
									val indexToScroll = (line - 1).toInt().coerceAtLeast(0)
									scope.launch {
										viewModel.closeTocDialog()
										listState.scrollToItem(indexToScroll)
										lineIndexToFocus = indexToScroll
									}
								},
								onDismiss = { viewModel.closeTocDialog() }
							)
						}
						if (goToDialogOpen) {
							GoToDialog(
								docState = docState,
								onDismiss = { viewModel.closeGoToDialog() },
								initialMode = goToInitialMode,
								onGoTo = { indexToScroll ->
									viewModel.savePosition(docState.session, docState.documentUri, indexToScroll)
									viewModel.refreshSegmentPreview()
									isTextMode = true
									scope.launch {
										listState.scrollToItem(indexToScroll)
										lineIndexToFocus = indexToScroll
									}
								}
							)
						}
						if (findDialogOpen) {
							FindDialog(
								configManager = viewModel.configManager,
								initialQuery = activeSearchQuery ?: "",
								onDismiss = { viewModel.closeFindDialog() },
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
											viewModel.updateTtsPosition(res.position)
											viewModel.refreshSegmentPreview()
											if (wasSpeaking) {
												viewModel.resumeTts()
											}
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
									val indexToScroll = (line - 1).toInt().coerceAtLeast(0)
									viewModel.savePosition(docState.session, docState.documentUri, indexToScroll)
									viewModel.refreshSegmentPreview()
									isTextMode = true
									scope.launch {
										listState.scrollToItem(indexToScroll)
										lineIndexToFocus = indexToScroll
									}
								},
								onDismiss = { viewModel.closeElementsDialog() }
							)
						}
					}
					if (recentsDialogOpen) {
						AllDocumentsDialog(
							recentDocuments = successState.recentDocuments,
							onDismiss = { recentsDialogOpen = false },
							onOpenDocument = { uri -> viewModel.openDocument(uri) },
							onRemoveDocument = { uri -> viewModel.removeRecentDocument(uri) }
						)
					}
					if (wordCountDialogOpen && docState != null) {
						val stats = remember(docState.session) { docState.session.getStatsFfi() }
						WordCountDialog(
							stats = stats,
							onDismiss = { viewModel.closeWordCountDialog() }
						)
					}
					if (documentInfoDialogOpen && docState != null) {
						val stats = remember(docState.session) { docState.session.getStatsFfi() }
						DocumentInfoDialog(
							docState = docState,
							stats = stats,
							onDismiss = { viewModel.closeDocumentInfoDialog() }
						)
					}
					if (optionsDialogOpen) {
						SettingsDialog(
							initialRestorePreviousDocuments = restorePreviousDocuments,
							initialUseInAppFileBrowser = useInAppFileBrowser,
							initialSwipeUpMovesForward = swipeUpMovesForward,
							engines = viewModel.ttsManager.getAvailableEngines(),
							currentEngine = currentEngineName ?: viewModel.ttsManager.getDefaultEngine(),
							voices = availableVoices,
							currentVoice = currentVoice,
							currentRate = currentSpeechRate,
							currentPitch = currentPitch,
							onEngineSelected = { viewModel.ttsManager.setEngine(it) },
							onVoiceSelected = { viewModel.ttsManager.setVoice(it) },
							onRateChanged = { viewModel.ttsManager.setSpeechRate(it) },
							onPitchChanged = { viewModel.ttsManager.setPitch(it) },
							onPlaySample = {
								viewModel.ttsManager.speak("This is a sample of the selected speech engine.", isSample = true)
							},
							onSaveOptions = { restore, useInApp, swipeUpFwd ->
								restorePreviousDocuments = restore
								useInAppFileBrowser = useInApp
								swipeUpMovesForward = swipeUpFwd
								viewModel.configManager.setAppBool("restore_previous_documents", restore)
								viewModel.configManager.setAppBool("use_in_app_file_browser", useInApp)
								viewModel.configManager.setAppBool("swipe_up_moves_forward", swipeUpFwd)
								viewModel.configManager.flush()
								viewModel.closeSettingsDialog()
							},
							onDismiss = { viewModel.closeSettingsDialog() }
						)
					}
					if (sleepTimerDialogOpen) {
						SleepTimerDialog(
							remainingSeconds = sleepTimerRemaining,
							onSetTimer = { viewModel.setSleepTimer(it) },
							onCancelTimer = { viewModel.cancelSleepTimer() },
							onDismiss = { viewModel.closeSleepTimerDialog() }
						)
					}
				}
				is MainScreenUiState.Error -> {
					Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
						Text("Error loading document: ${(state as MainScreenUiState.Error).message}")
					}
				}
			}
		}
	}
	if (isScreenDimmed) {
		Box(
			modifier = Modifier
				.fillMaxSize()
				.background(Color.Black)
				.pointerInput(Unit) { detectTapGestures { isScreenDimmed = false } }
				.semantics { contentDescription = t("Screen dimmed by sleep timer. Tap to wake.") }
		)
	}
	if (passwordPromptUri != null) {
		PasswordDialog(
			onConfirm = { viewModel.submitPassword(it) },
			onDismiss = { viewModel.cancelPasswordPrompt() }
		)
	}

	if (importPromptPath != null) {
		AlertDialog(
			onDismissRequest = { viewModel.cancelImportSettings() },
			modifier = Modifier.semantics { paneTitle = "Import document data" },
			title = { Text(t("Import document data")) },
			text = { Text(t("A .paperback file was found for this document. Would you like to import it?")) },
			confirmButton = {
				TextButton(onClick = { viewModel.confirmImportSettings() }) {
					Text(t("Import"))
				}
			},
			dismissButton = {
				TextButton(onClick = { viewModel.cancelImportSettings() }) {
					Text(t("Cancel"))
				}
			}
		)
	}

	val lifecycleOwner = LocalLifecycleOwner.current
	DisposableEffect(lifecycleOwner) {
		val observer = LifecycleEventObserver { _, event ->
			if (event == Lifecycle.Event.ON_RESUME &&
				android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R &&
				android.os.Environment.isExternalStorageManager()
			) {
				if (!useInAppFileBrowser) {
					useInAppFileBrowser = true
					viewModel.configManager.setAppBool("use_in_app_file_browser", true)
					viewModel.configManager.flush()
				}
			}
		}
		lifecycleOwner.lifecycle.addObserver(observer)
		onDispose { lifecycleOwner.lifecycle.removeObserver(observer) }
	}

	val showPermissionRationale by viewModel.showPermissionRationale.collectAsStateWithLifecycle()
	LaunchedEffect(Unit) {
		if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
			if (!android.os.Environment.isExternalStorageManager()) {
				viewModel.setShowPermissionRationale(true)
			}
		}
	}
	if (showPermissionRationale) {
		PermissionRationaleDialog(
			onGrantClick = {
				viewModel.setShowPermissionRationale(false)
				val intent = Intent(android.provider.Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION)
				intent.data = Uri.parse("package:${context.packageName}")
				context.startActivity(intent)
			},
			onDismiss = {
				viewModel.setShowPermissionRationale(false)
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
				android.os.Environment.getExternalStorageDirectory().absolutePath
			}
		}
		FileManagerDialog(
			supportedExtensions = extensions.toList(),
			initialDirectory = java.io.File(initialDirPath),
			onDirectoryChanged = { dir ->
				scope.launch(kotlinx.coroutines.Dispatchers.IO) {
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
				android.os.Environment.getExternalStorageDirectory().absolutePath
			}
		}
		FileManagerDialog(
			supportedExtensions = extensions,
			initialDirectory = java.io.File(initialDirPath),
			onDirectoryChanged = { dir ->
				scope.launch(kotlinx.coroutines.Dispatchers.IO) {
					viewModel.configManager.setAppString("last_file_manager_directory", dir.absolutePath)
					viewModel.configManager.flush()
				}
			},
			onFileSelected = { file ->
				showFileManagerForImport = false
				val uri = Uri.fromFile(file)
				scope.launch(kotlinx.coroutines.Dispatchers.IO) {
					if (viewModel.importSettingsFromUri(context, uri)) {
						launch(kotlinx.coroutines.Dispatchers.Main) {
							android.widget.Toast.makeText(context, t("Settings imported"), android.widget.Toast.LENGTH_SHORT).show()
						}
					} else {
						launch(kotlinx.coroutines.Dispatchers.Main) {
							android.widget.Toast.makeText(context, t("Failed to import settings"), android.widget.Toast.LENGTH_SHORT).show()
						}
					}
				}
			},
			onDismiss = { showFileManagerForImport = false }
		)
	}
	} // end outer Box
}
