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
	onImportSettings: () -> Unit,
	onHelpOpen: () -> Unit
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
									// TRANSLATORS: Accessibility action to import a document's saved settings and bookmarks from a .paperback file
									CustomAccessibilityAction(t("Import Document Data")) {
										onImportSettings()
										true
									},
									// TRANSLATORS: Accessibility action to export a document's saved settings and bookmarks to a .paperback file
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
							// TRANSLATORS: Button label to open a document; long-pressing the button reveals import/export options
							Text(t("Open Book"), fontWeight = FontWeight.Medium)
						}
					}
					DropdownMenu(
						expanded = openBookMenuExpanded,
						onDismissRequest = { openBookMenuExpanded = false }
					) {
						DropdownMenuItem(
							// TRANSLATORS: Menu item to import a document's saved settings and bookmarks from a .paperback file
							text = { Text(t("Import Document Data")) },
							onClick = {
								openBookMenuExpanded = false
								onImportSettings()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to export a document's saved settings and bookmarks to a .paperback file
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
										// TRANSLATORS: Accessibility action to open the table of contents for the current document
										CustomAccessibilityAction(t("Table of Contents")) {
											onTocOpen()
											true
										}
									)
								}
								add(
									// TRANSLATORS: Accessibility action to open the list of headings and links in the current document
									CustomAccessibilityAction(t("Elements List")) {
										onElementsOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to open the find/search bar
									CustomAccessibilityAction(t("Find")) {
										onFindOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to open the go-to dialog, for jumping to a page, line, or percentage
									CustomAccessibilityAction(t("Go To")) {
										onGoToOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to open the list of recently opened documents
									CustomAccessibilityAction(t("Recent Documents")) {
										onRecentsOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to show word/character/line count statistics for the current document
									CustomAccessibilityAction(t("Word Count")) {
										onWordCountOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to show metadata (title, author, etc.) about the current document
									CustomAccessibilityAction(t("Document Information")) {
										onDocumentInfoOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action toggling between the read-aloud view and the plain text view; label names the view that tapping it switches to
									CustomAccessibilityAction(if (isTextMode) t("Show Document") else t("Show Text")) {
										onToggleTextMode()
										true
									}
								)
								if (isTextMode) {
									add(
										// TRANSLATORS: Accessibility action toggling text-to-speech playback; label names the action that tapping it performs
										CustomAccessibilityAction(if (isSpeaking) t("Pause Read Aloud") else t("Read Aloud")) {
											onTogglePlayPause()
											true
										}
									)
								}
								add(
									// TRANSLATORS: Accessibility action to open the sleep timer dialog
									CustomAccessibilityAction(t("Sleep Timer")) {
										onSleepTimerOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to open the app's settings
									CustomAccessibilityAction(t("Settings")) {
										onSettingsOpen()
										true
									}
								)
								add(
									// TRANSLATORS: Accessibility action to open the in-app help document
									CustomAccessibilityAction(t("Help")) {
										onHelpOpen()
										true
									}
								)
							}
						}
					) {
						// TRANSLATORS: Accessibility label for the overflow icon button that opens the options menu
						Icon(Icons.Filled.MoreVert, contentDescription = t("More Options"))
					}
					DropdownMenu(
						expanded = moreOptionsExpanded,
						onDismissRequest = { moreOptionsExpanded = false }
					) {
						if (state.activeTab != null) {
							DropdownMenuItem(
								// TRANSLATORS: Menu item to open the table of contents for the current document
								text = { Text(t("Table of Contents")) },
								onClick = {
									moreOptionsExpanded = false
									onTocOpen()
								}
							)
						}
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the list of headings and links in the current document
							text = { Text(t("Elements List")) },
							onClick = {
								moreOptionsExpanded = false
								onElementsOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the find/search bar
							text = { Text(t("Find")) },
							onClick = {
								moreOptionsExpanded = false
								onFindOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the go-to dialog, for jumping to a page, line, or percentage
							text = { Text(t("Go To")) },
							onClick = {
								moreOptionsExpanded = false
								onGoToOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the list of recently opened documents
							text = { Text(t("Recent Documents")) },
							onClick = {
								moreOptionsExpanded = false
								onRecentsOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to show word/character/line count statistics for the current document
							text = { Text(t("Word Count")) },
							onClick = {
								moreOptionsExpanded = false
								onWordCountOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to show metadata (title, author, etc.) about the current document
							text = { Text(t("Document Information")) },
							onClick = {
								moreOptionsExpanded = false
								onDocumentInfoOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item toggling between the read-aloud view and the plain text view; label names the view that tapping it switches to
							text = { Text(if (isTextMode) t("Show Document") else t("Show Text")) },
							onClick = {
								onToggleTextMode()
								moreOptionsExpanded = false
							}
						)
						if (isTextMode) {
							DropdownMenuItem(
								// TRANSLATORS: Menu item toggling text-to-speech playback; label names the action that tapping it performs
								text = { Text(if (isSpeaking) t("Pause Read Aloud") else t("Read Aloud")) },
								onClick = {
									onTogglePlayPause()
									moreOptionsExpanded = false
								}
							)
						}
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the sleep timer dialog
							text = { Text(t("Sleep Timer")) },
							onClick = {
								moreOptionsExpanded = false
								onSleepTimerOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the app's settings
							text = { Text(t("Settings")) },
							onClick = {
								moreOptionsExpanded = false
								onSettingsOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the in-app help document
							text = { Text(t("Help")) },
							onClick = {
								moreOptionsExpanded = false
								onHelpOpen()
							}
						)
					}
				}
			} else {
				var emptyMenuExpanded by remember { mutableStateOf(false) }
				Box {
					IconButton(
						onClick = { emptyMenuExpanded = true },
						modifier = Modifier.semantics {
							traversalIndex = 2f
							this.onClick(label = "show all options in a menu") {
								emptyMenuExpanded = true
								true
							}
							customActions = listOf(
								// TRANSLATORS: Accessibility action to open the app's settings
								CustomAccessibilityAction(t("Settings")) {
									onSettingsOpen()
									true
								},
								// TRANSLATORS: Accessibility action to open the in-app help document
								CustomAccessibilityAction(t("Help")) {
									onHelpOpen()
									true
								}
							)
						}
					) {
						// TRANSLATORS: Accessibility label for the overflow icon button that opens the options menu
						Icon(Icons.Filled.MoreVert, contentDescription = t("More Options"))
					}
					DropdownMenu(
						expanded = emptyMenuExpanded,
						onDismissRequest = { emptyMenuExpanded = false }
					) {
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the app's settings
							text = { Text(t("Settings")) },
							onClick = {
								emptyMenuExpanded = false
								onSettingsOpen()
							}
						)
						DropdownMenuItem(
							// TRANSLATORS: Menu item to open the in-app help document
							text = { Text(t("Help")) },
							onClick = {
								emptyMenuExpanded = false
								onHelpOpen()
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
				modifier = Modifier.fillMaxWidth().semantics {
					isTraversalGroup = true
					traversalIndex = 3f
				}
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
