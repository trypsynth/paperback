package dev.paperback.mobile.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.onClick
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.traversalIndex
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreenTopBar(
	state: MainScreenUiState,
	isTextMode: Boolean,
	isSpeaking: Boolean,
	onOpenBook: () -> Unit,
	onTocOpen: () -> Unit,
	onTabSelect: (Int) -> Unit,
	onTabClose: (Int) -> Unit,
	onToggleTextMode: () -> Unit,
	onTogglePlayPause: () -> Unit,
	onRecentsOpen: () -> Unit,
	onGoToOpen: () -> Unit,
	onFindOpen: () -> Unit,
	onWordCountOpen: () -> Unit,
	onDocumentInfoOpen: () -> Unit,
	onSettingsOpen: () -> Unit,
	onSleepTimerOpen: () -> Unit
) {
	var moreOptionsExpanded by remember { mutableStateOf(false) }

	Column(
		modifier = Modifier
			.fillMaxWidth()
			.windowInsetsPadding(WindowInsets.statusBars)
			.padding(horizontal = 16.dp, vertical = 8.dp)
	) {
		val titleText = if (state is MainScreenUiState.Success) {
			state.activeTab?.title ?: "Paperback"
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
				if (state is MainScreenUiState.Success && state.activeTab != null) {
					Button(onClick = onTocOpen, modifier = Modifier.semantics { traversalIndex = 1f }) {
						Text("Table of Contents")
					}
					Spacer(modifier = Modifier.height(8.dp))
				}
				Button(
					onClick = onOpenBook,
					modifier = Modifier.semantics { traversalIndex = 2f }
				) {
					Text("Open Book")
				}
			}

			if (state is MainScreenUiState.Success && state.tabs.isNotEmpty()) {
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
										onToggleTextMode()
										true
									}
								)
								add(
									CustomAccessibilityAction("Recent Documents") {
										onRecentsOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Go To") {
										onGoToOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Find") {
										onFindOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Word Count") {
										onWordCountOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Document Information") {
										onDocumentInfoOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Settings") {
										onSettingsOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction("Sleep Timer") {
										onSleepTimerOpen()
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
								onToggleTextMode()
								moreOptionsExpanded = false
							}
						)
						if (isTextMode) {
							DropdownMenuItem(
								text = { Text(if (isSpeaking) "Pause Read Aloud" else "Read Aloud") },
								onClick = {
									onTogglePlayPause()
									moreOptionsExpanded = false
								}
							)
						}
						DropdownMenuItem(
							text = { Text("Recent Documents") },
							onClick = {
								moreOptionsExpanded = false
								onRecentsOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Go To") },
							onClick = {
								moreOptionsExpanded = false
								onGoToOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Find") },
							onClick = {
								moreOptionsExpanded = false
								onFindOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Word Count") },
							onClick = {
								moreOptionsExpanded = false
								onWordCountOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Document Information") },
							onClick = {
								moreOptionsExpanded = false
								onDocumentInfoOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Settings") },
							onClick = {
								moreOptionsExpanded = false
								onSettingsOpen()
							}
						)
						DropdownMenuItem(
							text = { Text("Sleep Timer") },
							onClick = {
								moreOptionsExpanded = false
								onSleepTimerOpen()
							}
						)
					}
				}
			}
		}

		if (state is MainScreenUiState.Success && state.tabs.isNotEmpty()) {
			PrimaryScrollableTabRow(
				selectedTabIndex = state.activeTabIndex,
				edgePadding = 8.dp,
				modifier = Modifier.fillMaxWidth()
			) {
				state.tabs.forEachIndexed { index, tab ->
					Tab(
						selected = state.activeTabIndex == index,
						onClick = { onTabSelect(index) },
						modifier = Modifier.semantics {
							customActions = listOf(
								CustomAccessibilityAction(
									label = "Close ${tab.title}",
									action = {
										onTabClose(index)
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
									onClick = { onTabClose(index) },
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
