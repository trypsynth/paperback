package dev.paperback.mobile.ui

import android.content.Intent
import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
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
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	var expandedTocIndices by remember { mutableStateOf(setOf<Int>()) }
	val context = LocalContext.current

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
									customActions = listOf(
										CustomAccessibilityAction("Recent Documents") {
											recentsDialogOpen = true
											true
										},
										CustomAccessibilityAction("Go To") {
											goToDialogOpen = true
											true
										},
										CustomAccessibilityAction("Word Count") {
											wordCountDialogOpen = true
											true
										},
										CustomAccessibilityAction("Document Information") {
											documentInfoDialogOpen = true
											true
										}
									)
								}
							) {
								Icon(Icons.Filled.MoreVert, contentDescription = "More Options")
							}
							DropdownMenu(
								expanded = moreOptionsExpanded,
								onDismissRequest = { moreOptionsExpanded = false }
							) {
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

						// Track the scroll position
						LaunchedEffect(docState.documentUri) {
							snapshotFlow { listState.firstVisibleItemIndex }
								.distinctUntilChanged()
								.debounce(500)
								.collect { index -> viewModel.savePosition(docState.session, docState.documentUri, index) }
						}

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
									val focusRequester = remember { FocusRequester() }
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
										textModifier = textModifier.focusRequester(focusRequester).focusable()
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
											kotlinx.coroutines.delay(700)
											try {
												focusRequester.requestFocus()
											} catch (e: Exception) {
											}
											kotlinx.coroutines.delay(1500)
											isTemporaryFocusTarget = false
											if (lineIndexToFocus == index) {
												lineIndexToFocus = null
											}
										}
									}
								}
							}
						}

						if (tocSheetOpen && docState != null) {
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
									scope.launch {
										listState.scrollToItem(indexToScroll)
										lineIndexToFocus = indexToScroll
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
