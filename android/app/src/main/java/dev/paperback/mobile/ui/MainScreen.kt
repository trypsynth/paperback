package dev.paperback.mobile.ui

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.view.accessibility.AccessibilityManager
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.relocation.BringIntoViewRequester
import androidx.compose.foundation.relocation.bringIntoViewRequester
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.onClick
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.semantics.traversalIndex
import androidx.compose.ui.text.LinkAnnotation
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.TextLinkStyles
import androidx.compose.ui.text.buildAnnotatedString
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.text.withLink
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.NavKey
import dev.paperback.mobile.ui.dialogs.*
import kotlinx.coroutines.flow.debounce
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.launch
import uniffi.paperback.LinkActionFfi
import uniffi.paperback.MarkerTypeFfi
import androidx.compose.foundation.lazy.items as lazyItems

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
	var tocSheetOpen by remember { mutableStateOf(false) }
	var recentsDialogOpen by remember { mutableStateOf(false) }
	var moreOptionsExpanded by remember { mutableStateOf(false) }
	var wordCountDialogOpen by remember { mutableStateOf(false) }
	var documentInfoDialogOpen by remember { mutableStateOf(false) }
	var goToDialogOpen by remember { mutableStateOf(false) }
	var findDialogOpen by remember { mutableStateOf(false) }
	var optionsDialogOpen by remember { mutableStateOf(false) }
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }

	var restorePreviousDocuments by remember {
		mutableStateOf(viewModel.configManager.getAppBool("restore_previous_documents", true))
	}

	var activeSearchQuery by remember { mutableStateOf<String?>(null) }
	var activeSearchOptions by remember { mutableStateOf<uniffi.paperback.SearchOptionsFfi?>(null) }
	var expandedTocIndices by remember { mutableStateOf(setOf<Int>()) }

	var isTextMode by remember { mutableStateOf(false) }
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

	val launcher = rememberLauncherForActivityResult(contract = ActivityResultContracts.OpenDocument()) { uri: Uri? ->
		if (uri != null) {
			viewModel.openDocument(uri)
		}
	}

	Scaffold(
		topBar = {
			Column(
				modifier = Modifier
					.fillMaxWidth()
					.windowInsetsPadding(WindowInsets.statusBars)
					.padding(horizontal = 16.dp, vertical = 8.dp)
			) {
				val titleText = if (state is MainScreenUiState.Success) {
					val successState = state as MainScreenUiState.Success
					successState.activeTab?.title ?: "Paperback"
				} else {
					"Paperback"
				}
				Text(
					text = titleText,
					style = MaterialTheme.typography.headlineSmall,
					fontWeight = FontWeight.Bold,
					modifier = Modifier.padding(bottom = 16.dp).semantics {
						heading()
						traversalIndex = 0f
					}
				)

				Row(
					modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp).semantics { isTraversalGroup = true },
					horizontalArrangement = Arrangement.SpaceBetween,
					verticalAlignment = Alignment.Top
				) {
					Column(horizontalAlignment = Alignment.Start, modifier = Modifier.semantics { isTraversalGroup = true }) {
						if (state is MainScreenUiState.Success && (state as MainScreenUiState.Success).activeTab != null) {
							Button(onClick = { tocSheetOpen = true }, modifier = Modifier.semantics { traversalIndex = 1f }) {
								Text("Table of Contents")
							}
							Spacer(modifier = Modifier.height(8.dp))
						}
						Button(
							onClick = { launcher.launch(supportedMimeTypes) },
							modifier = Modifier.semantics {
								traversalIndex =
									2f
							}
						) {
							Text("Open Book")
						}
					}

					if (state is MainScreenUiState.Success && (state as MainScreenUiState.Success).tabs.isNotEmpty()) {
						Box(modifier = Modifier.semantics { isTraversalGroup = true }) {
							IconButton(
								onClick = { moreOptionsExpanded = true },
								modifier = Modifier.semantics {
									traversalIndex = 3f
									this.onClick(label = "show all options in a menu") {
										moreOptionsExpanded = true
										true
									}
									customActions = mutableListOf<CustomAccessibilityAction>().apply {
										add(
											CustomAccessibilityAction(if (isTextMode) "Read Aloud" else "Show Text") {
												isTextMode = !isTextMode
												true
											}
										)
										add(
											CustomAccessibilityAction("Recent Documents") {
												recentsDialogOpen = true
												true
											}
										)
										add(
											CustomAccessibilityAction("Go To") {
												goToDialogOpen = true
												true
											}
										)
										add(
											CustomAccessibilityAction("Find") {
												findDialogOpen = true
												true
											}
										)
										add(
											CustomAccessibilityAction("Word Count") {
												wordCountDialogOpen = true
												true
											}
										)
										add(
											CustomAccessibilityAction("Document Information") {
												documentInfoDialogOpen = true
												true
											}
										)
										add(
											CustomAccessibilityAction("Settings") {
												optionsDialogOpen = true
												true
											}
										)
									}
								}
							) {
								Icon(Icons.Filled.MoreVert, contentDescription = "More Options")
							}
							DropdownMenu(
								expanded = moreOptionsExpanded,
								onDismissRequest = { moreOptionsExpanded = false }
							) {
								DropdownMenuItem(
									text = { Text(if (isTextMode) "Show Document" else "Show Text") },
									onClick = {
										isTextMode = !isTextMode
										moreOptionsExpanded = false
									}
								)
								if (isTextMode) {
									DropdownMenuItem(
										text = { Text(if (isSpeaking) "Pause Read Aloud" else "Read Aloud") },
										onClick = {
											viewModel.togglePlayPause()
											moreOptionsExpanded = false
										}
									)
								}
								DropdownMenuItem(
									text = { Text("Recent Documents") },
									onClick = {
										moreOptionsExpanded = false
										recentsDialogOpen = true
									}
								)
								DropdownMenuItem(
									text = { Text("Go To") },
									onClick = {
										moreOptionsExpanded = false
										goToDialogOpen = true
									}
								)
								DropdownMenuItem(
									text = { Text("Find") },
									onClick = {
										moreOptionsExpanded = false
										findDialogOpen = true
									}
								)
								DropdownMenuItem(
									text = { Text("Word Count") },
									onClick = {
										moreOptionsExpanded = false
										wordCountDialogOpen = true
									}
								)
								DropdownMenuItem(
									text = { Text("Document Information") },
									onClick = {
										moreOptionsExpanded = false
										documentInfoDialogOpen = true
									}
								)
								DropdownMenuItem(
									text = { Text("Settings") },
									onClick = {
										moreOptionsExpanded = false
										optionsDialogOpen = true
									}
								)
							}
						}
					}
				}
				if (state is MainScreenUiState.Success) {
					val successState = state as MainScreenUiState.Success
					if (successState.tabs.isNotEmpty()) {
						PrimaryScrollableTabRow(
							selectedTabIndex = successState.activeTabIndex,
							edgePadding = 8.dp,
							modifier = Modifier.fillMaxWidth()
						) {
							successState.tabs.forEachIndexed { index, tab ->
								Tab(
									selected = successState.activeTabIndex == index,
									onClick = { viewModel.setActiveTab(index) },
									modifier = Modifier.semantics {
										customActions = listOf(
											CustomAccessibilityAction(
												label = "Close ${tab.title}",
												action = {
													viewModel.closeTab(index)
													true
												}
											)
										)
									},
									text = {
										Row(verticalAlignment = Alignment.CenterVertically) {
											Text(tab.title, maxLines = 1, modifier = Modifier.widthIn(max = 150.dp))
											Spacer(modifier = Modifier.width(4.dp))
											IconButton(
												onClick = { viewModel.closeTab(index) },
												modifier = Modifier.size(24.dp).clearAndSetSemantics { }
											) {
												Text("X", fontWeight = FontWeight.Bold)
											}
										}
									}
								)
							}
						}
					}
				}
			}
		},
		bottomBar = {
			if (activeSearchQuery != null && activeSearchOptions != null && !isTouchExplorationEnabled) {
				BottomAppBar(
					modifier = Modifier.semantics { isTraversalGroup = true },
					contentPadding = PaddingValues(horizontal = 16.dp)
				) {
					Row(
						modifier = Modifier.fillMaxWidth(),
						horizontalArrangement = Arrangement.SpaceBetween,
						verticalAlignment = Alignment.CenterVertically
					) {
						IconButton(
							onClick = {
								activeSearchQuery = null
								activeSearchOptions = null
							},
							modifier = Modifier.semantics { contentDescription = "Close Search" }
						) {
							Text("X", fontWeight = FontWeight.Bold, style = MaterialTheme.typography.titleLarge)
						}

						Row {
							Button(
								onClick = {
									val docState = (state as? MainScreenUiState.Success)?.activeTab
									val currentListState = docState?.let { listStates[it.documentUri] }
									if (docState != null && currentListState != null) {
										val currentIdx = currentListState.firstVisibleItemIndex
										val pos = docState.session.positionFromLine((currentIdx + 1).toLong())
										val res = docState.session.searchFfi(activeSearchQuery!!, pos, activeSearchOptions!!.copy(forward = false))
										if (res.found) {
											val targetLine = docState.session.lineFromPosition(res.position)
											val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
											scope.launch {
												currentListState.scrollToItem(targetIndex)
												lineIndexToFocus = targetIndex
											}
										}
									}
								}
							) {
								Text("Find Previous")
							}
							Spacer(modifier = Modifier.width(8.dp))
							Button(
								onClick = {
									val docState = (state as? MainScreenUiState.Success)?.activeTab
									val currentListState = docState?.let { listStates[it.documentUri] }
									if (docState != null && currentListState != null) {
										val currentIdx = currentListState.firstVisibleItemIndex
										val nextLine = (currentIdx + 2).toLong().coerceAtMost(docState.lineCount)
										val pos = docState.session.positionFromLine(nextLine)
										val res = docState.session.searchFfi(activeSearchQuery!!, pos, activeSearchOptions!!.copy(forward = true))
										if (res.found) {
											val targetLine = docState.session.lineFromPosition(res.position)
											val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
											scope.launch {
												currentListState.scrollToItem(targetIndex)
												lineIndexToFocus = targetIndex
											}
										}
									}
								}
							) {
								Text("Find Next")
							}
						}
					}
				}
			} else if (!isTextMode &&
				state is MainScreenUiState.Success &&
				(state as MainScreenUiState.Success).activeTab != null
			) {
				TtsBottomBar(
					isSpeaking = isSpeaking,
					onPlayPause = { viewModel.togglePlayPause() },
					onPrev = { viewModel.playPrevSegment() },
					onNext = { viewModel.playNextSegment() },
					currentSegmentType = currentSegmentType,
					onSegmentTypeChange = { viewModel.setSegmentType(it) }
				)
			}
		}
	) { padding ->
		Column(modifier = modifier.fillMaxSize().padding(padding)) {
			when (state) {
				MainScreenUiState.Idle -> {
					Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
						Text("No document open. Please open a book.")
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
									Text("Recent Documents")
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
								modifier = Modifier.fillMaxSize().padding(16.dp).semantics { isTraversalGroup = true },
								horizontalAlignment = Alignment.CenterHorizontally,
								verticalArrangement = Arrangement.Center
							) {
								val stats = remember(ttsPosition) { docState.session.getStatusInfoFfi(ttsPosition) }
								var isDragging by remember { mutableStateOf(false) }
								var sliderValue by remember { mutableFloatStateOf(stats.percentage.toFloat()) }
								LaunchedEffect(ttsPosition) {
									if (!isDragging) sliderValue = stats.percentage.toFloat()
								}
								Slider(
									value = sliderValue,
									onValueChange = { isDragging = true; sliderValue = it },
									onValueChangeFinished = {
										isDragging = false
										viewModel.seekToPercent(sliderValue.toInt())
									},
									valueRange = 0f..100f,
									modifier = Modifier.fillMaxWidth().semantics {
										stateDescription = "${stats.percentage}%"
									}
								)
								Text("${stats.percentage}%", style = MaterialTheme.typography.labelMedium)
								Spacer(modifier = Modifier.height(24.dp))
								Text(
									text = currentSegmentText,
									style = MaterialTheme.typography.bodyLarge,
									modifier = Modifier.padding(16.dp)
								)
							}
						} else {
							LazyColumn(
								state = listState,
								modifier = Modifier.fillMaxSize().semantics { isTraversalGroup = true },
								contentPadding = PaddingValues(start = 16.dp, end = 16.dp, top = 4.dp, bottom = 16.dp)
							) {
								items(
									count = docState.lineCount.toInt(),
									key = { it }
								) { index ->
									val lineNum = (index + 1).toLong()
									val pos = docState.session.positionFromLine(lineNum)
									val lineText = docState.session.getLineText(pos).trimEnd()
									val markers = docState.session.getLineMarkers(lineNum)

									if (lineText.isNotBlank()) {
										val bringIntoViewRequester = remember { BringIntoViewRequester() }
										var isTemporaryFocusTarget by remember { mutableStateOf(lineIndexToFocus == index) }
										LaunchedEffect(lineIndexToFocus) {
											if (lineIndexToFocus == index) {
												isTemporaryFocusTarget = true
											}
										}

										var textModifier = Modifier.padding(vertical = 4.dp).semantics(mergeDescendants = true) {}
										var isHeading = false
										var headingLevel = 0

										val annotatedString = buildAnnotatedString {
											var currentIdx = 0
											val sortedMarkers = markers.sortedBy { it.position }

											sortedMarkers.forEach { marker ->
												when (marker.mtype) {
													MarkerTypeFfi.HEADING1 -> {
														isHeading = true
														headingLevel = 1
													}
													MarkerTypeFfi.HEADING2 -> {
														isHeading = true
														headingLevel = 2
													}
													MarkerTypeFfi.HEADING3 -> {
														isHeading = true
														headingLevel = 3
													}
													MarkerTypeFfi.HEADING4 -> {
														isHeading = true
														headingLevel = 4
													}
													MarkerTypeFfi.HEADING5 -> {
														isHeading = true
														headingLevel = 5
													}
													MarkerTypeFfi.HEADING6 -> {
														isHeading = true
														headingLevel = 6
													}
													MarkerTypeFfi.LINK -> {
														val markerStartInLine = (marker.position - pos).toInt().coerceAtLeast(0)
														val markerTextLength = marker.text.length

														if (markerStartInLine > currentIdx) {
															append(lineText.substring(currentIdx, markerStartInLine.coerceAtMost(lineText.length)))
															currentIdx = markerStartInLine
														}

														if (currentIdx < lineText.length) {
															val linkEnd = (currentIdx + markerTextLength).coerceAtMost(lineText.length)
															val linkText = lineText.substring(currentIdx, linkEnd)

															val linkAnnotation = LinkAnnotation.Clickable(
																tag = marker.position.toString(),
																styles = TextLinkStyles(
																	style = SpanStyle(
																		color = MaterialTheme.colorScheme.primary,
																		textDecoration = TextDecoration.Underline
																	)
																)
															) {
																val result = docState.session.activateLinkFfi(marker.position)
																if (result.found) {
																	when (result.action) {
																		LinkActionFfi.EXTERNAL -> {
																			val intent = Intent(Intent.ACTION_VIEW, Uri.parse(result.url))
																			context.startActivity(intent)
																		}
																		LinkActionFfi.INTERNAL -> {
																			val targetLine = docState.session.lineFromPosition(result.offset)
																			val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
																			scope.launch {
																				listState.scrollToItem(targetIndex)
																				lineIndexToFocus = targetIndex
																			}
																		}
																		else -> {}
																	}
																}
															}

															withLink(linkAnnotation) {
																append(linkText)
															}
															currentIdx = linkEnd
														}
													}
													else -> {}
												}
											}

											if (currentIdx < lineText.length) {
												append(lineText.substring(currentIdx))
											}
										}

										if (isHeading) {
											textModifier = textModifier.semantics {
												heading()
												if (headingLevel > 0) {
													stateDescription = "Heading $headingLevel"
												}
											}
										}

										if (isTemporaryFocusTarget) {
											textModifier = textModifier.bringIntoViewRequester(bringIntoViewRequester).focusable()
										}

										val currentOptions = activeSearchOptions
										val currentQuery = activeSearchQuery
										if (currentQuery != null && currentOptions != null) {
											textModifier = textModifier.semantics {
												customActions = listOf(
													CustomAccessibilityAction("Find Next") {
														val nextLine = (index + 2).toLong().coerceAtMost(docState.lineCount)
														val pos = docState.session.positionFromLine(nextLine)
														val res = docState.session.searchFfi(currentQuery, pos, currentOptions.copy(forward = true))
														if (res.found) {
															val targetLine = docState.session.lineFromPosition(res.position)
															val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
															scope.launch {
																listState.scrollToItem(targetIndex)
																lineIndexToFocus = targetIndex
															}
														}
														true
													},
													CustomAccessibilityAction("Find Previous") {
														val pos = docState.session.positionFromLine((index + 1).toLong())
														val res = docState.session.searchFfi(currentQuery, pos, currentOptions.copy(forward = false))
														if (res.found) {
															val targetLine = docState.session.lineFromPosition(res.position)
															val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
															scope.launch {
																listState.scrollToItem(targetIndex)
																lineIndexToFocus = targetIndex
															}
														}
														true
													},
													CustomAccessibilityAction("Close Search") {
														activeSearchQuery = null
														activeSearchOptions = null
														true
													}
												)
											}
										}

										val textStyle = if (isHeading) {
											MaterialTheme.typography.bodyLarge.copy(fontWeight = FontWeight.Bold)
										} else {
											MaterialTheme.typography.bodyLarge
										}

										Text(
											text = annotatedString,
											style = textStyle,
											modifier = textModifier
										)

										if (isTemporaryFocusTarget) {
											LaunchedEffect(Unit) {
												try {
													bringIntoViewRequester.bringIntoView()
												} catch (e: Exception) {
												}
												isTemporaryFocusTarget = false
												if (lineIndexToFocus == index) {
													lineIndexToFocus = null
												}
											}
										}
									}
								}
							}
						}

						if (tocSheetOpen) {
							TocDialog(
								toc = docState.toc,
								expandedTocIndices = expandedTocIndices,
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
										tocSheetOpen = false
										listState.scrollToItem(indexToScroll)
										lineIndexToFocus = indexToScroll
									}
								},
								onDismiss = { tocSheetOpen = false }
							)
						}

						if (goToDialogOpen) {
							GoToDialog(
								docState = docState,
								onDismiss = { goToDialogOpen = false },
								onGoTo = { indexToScroll ->
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
								onDismiss = { findDialogOpen = false },
								onSearch = { query, options ->
									viewModel.pauseTts()
									isTextMode = true
									activeSearchQuery = query
									activeSearchOptions = options
									val currentPos = docState.session.positionFromLine((listState.firstVisibleItemIndex + 1).toLong())
									val res = docState.session.searchFfi(query, currentPos, options)
									if (res.found) {
										val targetLine = docState.session.lineFromPosition(res.position)
										val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
										scope.launch {
											listState.scrollToItem(targetIndex)
											lineIndexToFocus = targetIndex
										}
									}
								}
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
							onDismiss = { wordCountDialogOpen = false }
						)
					}

					if (documentInfoDialogOpen && docState != null) {
						val stats = remember(docState.session) { docState.session.getStatsFfi() }
						DocumentInfoDialog(
							docState = docState,
							stats = stats,
							onDismiss = { documentInfoDialogOpen = false }
						)
					}

					if (optionsDialogOpen) {
						SettingsDialog(
							initialRestorePreviousDocuments = restorePreviousDocuments,
							onSaveOptions = { checked ->
								restorePreviousDocuments = checked
								viewModel.configManager.setAppBool("restore_previous_documents", checked)
								viewModel.configManager.flush()
								optionsDialogOpen = false
							},
							onOpenTtsConfig = {
								optionsDialogOpen = false
								ttsConfigDialogOpen = true
							},
							onDismiss = { optionsDialogOpen = false }
						)
					}

					if (ttsConfigDialogOpen) {
						TtsConfigDialog(
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
							onDismiss = { ttsConfigDialogOpen = false }
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
}
