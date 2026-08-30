package dev.paperback.android.ui

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.onClick
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.traversalIndex
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.paperback.android.t

/**
 * A single menu entry, rendered as both a visible [DropdownMenuItem] and a TalkBack
 * [CustomAccessibilityAction] from one definition, so the two can't drift out of sync.
 */
private data class MenuAction(
	val label: String,
	val onClick: () -> Unit
)

private fun List<MenuAction>.toCustomActions(): List<CustomAccessibilityAction> =
	map { action ->
		CustomAccessibilityAction(action.label) {
			action.onClick()
			true
		}
	}

@Composable
private fun MenuActionItems(
	actions: List<MenuAction>,
	onItemClick: () -> Unit
) {
	actions.forEach { action ->
		DropdownMenuItem(
			text = { Text(action.label) },
			onClick = {
				onItemClick()
				action.onClick()
			}
		)
	}
}

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
	onExportDocumentOpen: () -> Unit,
	onExportSettings: () -> Unit,
	onImportSettings: () -> Unit,
	onHelpOpen: () -> Unit
) {
	var moreOptionsExpanded by remember { mutableStateOf(false) }
	val bookMenuActions = listOf(
		MenuAction(
			// TRANSLATORS: Menu item / accessibility action to import a document's saved settings and bookmarks from a .paperback file
			t("Import Document Data"),
			onImportSettings
		),
		MenuAction(
			// TRANSLATORS: Menu item / accessibility action to export a document's saved settings and bookmarks to a .paperback file
			t("Export Document Data"),
			onExportSettings
		)
	)
	// TRANSLATORS: Accessibility label for the overflow icon button that opens the options menu
	val moreOptionsLabel = t("More Options")

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
							).semantics {
								traversalIndex = 1f
								customActions = bookMenuActions.toCustomActions()
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
						MenuActionItems(bookMenuActions) { openBookMenuExpanded = false }
					}
				}
			}
			if (state is MainScreenUiState.Success && state.tabs.isNotEmpty()) {
				// An audio-only book (a zip of nothing but narration files) has no real text
				// spine, so switching to Text Mode, listing elements, or counting words doesn't
				// make sense for it.
				val isAudioOnly = state.activeTab?.isAudioOnly == true
				val menuActions = buildList {
					if (!isAudioOnly) {
						add(
							MenuAction(
								// TRANSLATORS: Menu item / accessibility action toggling between the read-aloud view and the plain text view; label names the mode that tapping it switches TO
								if (isTextMode) t("Switch to TTS Mode") else t("Switch to Text Mode"),
								onToggleTextMode
							)
						)
					}
					if (isTextMode && !isAudioOnly) {
						add(
							MenuAction(
								// TRANSLATORS: Menu item / accessibility action toggling text-to-speech playback; label names the action that tapping it performs
								if (isSpeaking) t("Pause Read Aloud") else t("Read Aloud"),
								onTogglePlayPause
							)
						)
					}
					if (state.activeTab != null) {
						add(
							MenuAction(
								// TRANSLATORS: Menu item / accessibility action to open the table of contents for the current document
								t("Table of Contents"),
								onTocOpen
							)
						)
						// TRANSLATORS: Menu item / accessibility action to open the export document dialog
						add(MenuAction(t("Export As"), onExportDocumentOpen))
					}
					if (!isAudioOnly) {
						add(
							MenuAction(
								// TRANSLATORS: Menu item / accessibility action to open the list of headings and links in the current document
								t("Elements List"),
								onElementsOpen
							)
						)
					}
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the find/search bar
							t("Find"),
							onFindOpen
						)
					)
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the go-to dialog, for jumping to a page, line, or percentage
							t("Go To"),
							onGoToOpen
						)
					)
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the list of recently opened documents
							t("Recent Documents"),
							onRecentsOpen
						)
					)
					if (!isAudioOnly) {
						add(
							MenuAction(
								// TRANSLATORS: Menu item / accessibility action to show word/character/line count statistics for the current document
								t("Word Count"),
								onWordCountOpen
							)
						)
					}
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to show metadata (title, author, etc.) about the current document
							t("Document Information"),
							onDocumentInfoOpen
						)
					)
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the sleep timer dialog
							t("Sleep Timer"),
							onSleepTimerOpen
						)
					)
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the in-app help document
							t("Help"),
							onHelpOpen
						)
					)
					add(
						MenuAction(
							// TRANSLATORS: Menu item / accessibility action to open the app's settings
							t("Settings"),
							onSettingsOpen
						)
					)
				}
				Box {
					IconButton(
						onClick = { moreOptionsExpanded = true },
						modifier = Modifier.semantics {
							traversalIndex = 2f
							this.onClick(label = "show all options in a menu") {
								moreOptionsExpanded = true
								true
							}
							customActions = menuActions.toCustomActions()
						}
					) {
						Icon(Icons.Filled.MoreVert, contentDescription = moreOptionsLabel)
					}
					DropdownMenu(
						expanded = moreOptionsExpanded,
						onDismissRequest = { moreOptionsExpanded = false }
					) {
						MenuActionItems(menuActions) { moreOptionsExpanded = false }
					}
				}
			} else {
				var emptyMenuExpanded by remember { mutableStateOf(false) }
				val emptyMenuActions = listOf(
					// TRANSLATORS: Menu item / accessibility action to open the in-app help document, shown in the top bar menu when no document is open
					MenuAction(t("Help"), onHelpOpen),
					// TRANSLATORS: Menu item / accessibility action to open the app's settings, shown in the top bar menu when no document is open
					MenuAction(t("Settings"), onSettingsOpen)
				)
				Box {
					IconButton(
						onClick = { emptyMenuExpanded = true },
						modifier = Modifier.semantics {
							traversalIndex = 2f
							this.onClick(label = "show all options in a menu") {
								emptyMenuExpanded = true
								true
							}
							customActions = emptyMenuActions.toCustomActions()
						}
					) {
						Icon(Icons.Filled.MoreVert, contentDescription = moreOptionsLabel)
					}
					DropdownMenu(
						expanded = emptyMenuExpanded,
						onDismissRequest = { emptyMenuExpanded = false }
					) {
						MenuActionItems(emptyMenuActions) { emptyMenuExpanded = false }
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
									Icon(
										Icons.Filled.Close,
										contentDescription = null,
										modifier = Modifier.size(16.dp)
									)
								}
							}
						}
					)
				}
			}
		}
	}
}
