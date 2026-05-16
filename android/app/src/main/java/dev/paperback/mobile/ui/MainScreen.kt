package dev.paperback.mobile.ui

import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.focusable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.text.selection.SelectionContainer
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation3.runtime.NavKey
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
	onItemClick: (NavKey) -> Unit = {},
	modifier: Modifier = Modifier,
	viewModel: MainScreenViewModel = viewModel()
) {
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val context = LocalContext.current
	val scope = rememberCoroutineScope()
	val listState = rememberLazyListState()
	var tocSheetOpen by remember { mutableStateOf(false) }
	var lineIndexToFocus by remember { mutableStateOf<Int?>(null) }
	var expandedTocIndices by remember { mutableStateOf(setOf<Int>()) }
	val launcher = rememberLauncherForActivityResult(
		contract = ActivityResultContracts.GetContent()
	) { uri: Uri? ->
		if (uri != null) {
			viewModel.openDocument(context, uri)
		}
	}
	Scaffold(
		topBar = {
			TopAppBar(
				title = { Text("Paperback") },
				navigationIcon = {
					if (state is MainScreenUiState.Success) {
						TextButton(onClick = { tocSheetOpen = true }) {
							Text("TOC")
						}
					}
				},
				actions = {
					Button(onClick = { launcher.launch("*/*") }) {
						Text("Open Book")
					}
				}
			)
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
					val docState = state as MainScreenUiState.Success
					SelectionContainer {
						LazyColumn(
							state = listState,
							modifier = Modifier.fillMaxSize(),
							contentPadding = PaddingValues(16.dp)
						) {
							items(docState.lineCount.toInt()) { index ->
								val pos = docState.session.positionFromLine((index + 1).toLong())
								val lineText = docState.session.getLineText(pos)
								if (lineText.isNotBlank()) {
									val focusRequester = remember { FocusRequester() }
									var isTemporaryFocusTarget by remember { mutableStateOf(lineIndexToFocus == index) }
									LaunchedEffect(lineIndexToFocus) {
										if (lineIndexToFocus == index) {
											isTemporaryFocusTarget = true
										}
									}
									val textModifier = if (isTemporaryFocusTarget) {
										Modifier.focusRequester(focusRequester).focusable()
									} else {
										Modifier
									}
									Text(
										text = lineText.trimEnd(),
										style = MaterialTheme.typography.bodyLarge,
										modifier = textModifier.padding(vertical = 4.dp)
									)
									if (isTemporaryFocusTarget) {
										LaunchedEffect(Unit) {
											kotlinx.coroutines.delay(700)
											try {
												focusRequester.requestFocus()
											} catch (e: Exception) {}
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
				is MainScreenUiState.Error -> {
					Box(modifier = Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
						Text("Error loading document: ${(state as MainScreenUiState.Error).message}")
					}
				}
			}
		}
	}
}
