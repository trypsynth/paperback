package dev.paperback.mobile.ui

import android.content.Intent
import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.clickable
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
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
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	var expandedTocIndices by remember { mutableStateOf(setOf<Int>()) }
	val context = LocalContext.current

	val launcher = rememberLauncherForActivityResult(contract = ActivityResultContracts.OpenDocument()) { uri: Uri? ->
		if (uri != null) {
			viewModel.openDocument(uri)
		}
	}

	Scaffold(
		topBar = {
			Column {
				TopAppBar(
					title = { Text("Paperback") },
					navigationIcon = {
						if (state is MainScreenUiState.Success && (state as MainScreenUiState.Success).activeTab != null) {
							TextButton(onClick = { tocSheetOpen = true }) {
								Text("Table of Contents")
							}
						}
					},
					actions = {
						if (state is MainScreenUiState.Success && (state as MainScreenUiState.Success).tabs.isNotEmpty()) {
							TextButton(onClick = { recentsDialogOpen = true }) {
								Text("Recent Books")
							}
						}
						Button(onClick = { launcher.launch(arrayOf("*/*")) }) {
							Text("Open Book")
						}
					}
				)
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
											onOpen = { viewModel.openDocument(Uri.parse(recentDoc.uri)) },
											onRemove = { viewModel.removeRecentDocument(recentDoc.uri) }
										)
									}
								}
								TextButton(
									onClick = { recentsDialogOpen = true },
									modifier = Modifier.padding(top = 8.dp)
								) {
									Text("Recent Books")
								}
							}
						}
					} else {
						val listState = listStates.getOrPut(docState.documentUri) {
							LazyListState(firstVisibleItemIndex = docState.initialScrollIndex)
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
							contentPadding = PaddingValues(16.dp)
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

						if (tocSheetOpen) {
							TocSheet(
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
					}
					
					if (recentsDialogOpen) {
						AlertDialog(
							onDismissRequest = { recentsDialogOpen = false },
							title = { Text("Recent Documents") },
							text = {
								LazyColumn(
									modifier = Modifier.fillMaxWidth()
								) {
									lazyItems(successState.recentDocuments) { recentDoc ->
										RecentDocumentItemRow(
											item = recentDoc,
											onOpen = {
												recentsDialogOpen = false
												viewModel.openDocument(Uri.parse(recentDoc.uri))
											},
											onRemove = { viewModel.removeRecentDocument(recentDoc.uri) }
										)
									}
								}
							},
							confirmButton = {
								TextButton(onClick = { recentsDialogOpen = false }) {
									Text("Close")
								}
							}
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

@Composable
fun RecentDocumentItemRow(
	item: RecentDocumentItem,
	onOpen: () -> Unit,
	onRemove: () -> Unit
) {
	Row(
		modifier = Modifier
			.fillMaxWidth()
			.clickable(
				onClickLabel = "open",
				onClick = onOpen
			).semantics {
				customActions = listOf(
					CustomAccessibilityAction("Remove") {
						onRemove()
						true
					}
				)
				stateDescription = if (item.isOpen) "Open" else "Closed"
			}.padding(vertical = 12.dp, horizontal = 8.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Column(modifier = Modifier.weight(1f)) {
			Text(text = item.displayName, style = MaterialTheme.typography.bodyLarge)
			Text(
				text = if (item.isOpen) "Currently Open" else "Closed",
				style = MaterialTheme.typography.bodySmall,
				color = MaterialTheme.colorScheme.onSurfaceVariant
			)
		}
		TextButton(
			onClick = onRemove,
			modifier = Modifier.clearAndSetSemantics { }
		) {
			Text("Remove")
		}
	}
}
