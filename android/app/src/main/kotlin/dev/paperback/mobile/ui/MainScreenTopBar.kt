package dev.paperback.mobile.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.filled.Settings
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
import androidx.compose.ui.semantics.Role
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.t

@OptIn(ExperimentalFoundationApi::class, ExperimentalMaterial3Api::class)
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
	onSleepTimerOpen: () -> Unit,
	onElementsOpen: () -> Unit,
	onExportSettings: () -> Unit,
	onImportSettings: () -> Unit
) {
	var moreOptionsExpanded by remember { mutableStateOf(false) }
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.windowInsetsPadding(WindowInsets.statusBars)
			.padding(horizontal = 16.dp, vertical = 8.dp)
			.semantics { isTraversalGroup = true }
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
			modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
			horizontalArrangement = Arrangement.SpaceBetween,
			verticalAlignment = Alignment.Top
		) {
			Column(horizontalAlignment = Alignment.Start) {
				var openBookMenuExpanded by remember { mutableStateOf(false) }
				Box {
					Surface(
						shape = ButtonDefaults.shape,
						color = MaterialTheme.colorScheme.primary,
						contentColor = MaterialTheme.colorScheme.onPrimary,
						modifier = Modifier
							.combinedClickable(
								role = Role.Button,
								onClick = onOpenBook,
								onLongClick = { openBookMenuExpanded = true },
								onLongClickLabel = "show import and export options"
							)
							.semantics {
								traversalIndex = 1f
								customActions = listOf(
									CustomAccessibilityAction(t("Import Document Data")) {
										onImportSettings()
										true
									},
									CustomAccessibilityAction(t("Export Document Data")) {
										onExportSettings()
										true
									}
								)
							}
					) {
						Row(
							modifier = Modifier.padding(horizontal = 24.dp, vertical = 10.dp),
							horizontalArrangement = Arrangement.Center,
							verticalAlignment = Alignment.CenterVertically
						) {
							Text(t("Open Book"), fontWeight = FontWeight.Medium)
						}
					}
					DropdownMenu(
						expanded = openBookMenuExpanded,
						onDismissRequest = { openBookMenuExpanded = false }
					) {
						DropdownMenuItem(
							text = { Text(t("Import Document Data")) },
							onClick = {
								openBookMenuExpanded = false
								onImportSettings()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Export Document Data")) },
							onClick = {
								openBookMenuExpanded = false
								onExportSettings()
							}
						)
					}
				}
			}
			if (state is MainScreenUiState.Success && state.tabs.isNotEmpty()) {
				Box {
					IconButton(
						onClick = { moreOptionsExpanded = true },
						modifier = Modifier.semantics {
							traversalIndex = 2f
							this.onClick(label = "show all options in a menu") {
								moreOptionsExpanded = true
								true
							}
							customActions = mutableListOf<CustomAccessibilityAction>().apply {
								if (state.activeTab != null) {
									add(
										CustomAccessibilityAction(t("Table of Contents")) {
											onTocOpen()
											true
										}
									)
								}
								add(
									CustomAccessibilityAction(t("Elements List")) {
										onElementsOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Find")) {
										onFindOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Go To")) {
										onGoToOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Recent Documents")) {
										onRecentsOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Word Count")) {
										onWordCountOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Document Information")) {
										onDocumentInfoOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(if (isTextMode) t("Show Document") else t("Show Text")) {
										onToggleTextMode()
										true
									}
								)
								if (isTextMode) {
									add(
										CustomAccessibilityAction(if (isSpeaking) t("Pause Read Aloud") else t("Read Aloud")) {
											onTogglePlayPause()
											true
										}
									)
								}
								add(
									CustomAccessibilityAction(t("Sleep Timer")) {
										onSleepTimerOpen()
										true
									}
								)
								add(
									CustomAccessibilityAction(t("Settings")) {
										onSettingsOpen()
										true
									}
								)
							}
						}
					) {
						Icon(Icons.Filled.MoreVert, contentDescription = t("More Options"))
					}
					DropdownMenu(
						expanded = moreOptionsExpanded,
						onDismissRequest = { moreOptionsExpanded = false }
					) {
						if (state.activeTab != null) {
							DropdownMenuItem(
								text = { Text(t("Table of Contents")) },
								onClick = {
									moreOptionsExpanded = false
									onTocOpen()
								}
							)
						}
						DropdownMenuItem(
							text = { Text(t("Elements List")) },
							onClick = {
								moreOptionsExpanded = false
								onElementsOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Find")) },
							onClick = {
								moreOptionsExpanded = false
								onFindOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Go To")) },
							onClick = {
								moreOptionsExpanded = false
								onGoToOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Recent Documents")) },
							onClick = {
								moreOptionsExpanded = false
								onRecentsOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Word Count")) },
							onClick = {
								moreOptionsExpanded = false
								onWordCountOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Document Information")) },
							onClick = {
								moreOptionsExpanded = false
								onDocumentInfoOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(if (isTextMode) t("Show Document") else t("Show Text")) },
							onClick = {
								onToggleTextMode()
								moreOptionsExpanded = false
							}
						)
						if (isTextMode) {
							DropdownMenuItem(
								text = { Text(if (isSpeaking) t("Pause Read Aloud") else t("Read Aloud")) },
								onClick = {
									onTogglePlayPause()
									moreOptionsExpanded = false
								}
							)
						}
						DropdownMenuItem(
							text = { Text(t("Sleep Timer")) },
							onClick = {
								moreOptionsExpanded = false
								onSleepTimerOpen()
							}
						)
						DropdownMenuItem(
							text = { Text(t("Settings")) },
							onClick = {
								moreOptionsExpanded = false
								onSettingsOpen()
							}
						)
					}
				}
			} else {
				Box {
					IconButton(
						onClick = { onSettingsOpen() },
						modifier = Modifier.semantics {
							traversalIndex = -1f
						}
					) {
						Icon(Icons.Filled.Settings, contentDescription = "Settings")
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
