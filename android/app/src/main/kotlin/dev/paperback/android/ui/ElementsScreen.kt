package dev.paperback.android.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.TreeExpandChevron
import dev.paperback.android.ui.dialogs.applyTreeExpandSemantics
import dev.paperback.android.ui.dialogs.flattenVisibleTreeIndices
import dev.paperback.android.ui.dialogs.hasTreeChildren

/**
 * The document's headings and links, as a real destination rather than a dialog, so the lists get
 * the whole window and the system back gesture leaves them.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ElementsScreen(
	viewModel: MainScreenViewModel = viewModel(),
	onDismiss: () -> Unit
) {
	val headings by viewModel.currentHeadings.collectAsStateWithLifecycle()
	val links by viewModel.currentLinks.collectAsStateWithLifecycle()
	var selectedTabIndex by remember { mutableIntStateOf(0) }
	var expandedHeadingIndices by remember { mutableStateOf(setOf<Int>()) }
	// The lists are only needed while this screen is up, and they can be large.
	DisposableEffect(Unit) {
		onDispose { viewModel.clearElements() }
	}

	// TRANSLATORS: Tab labels in the Elements screen for browsing a document's headings vs. its links
	val tabs = listOf(t("Headings"), t("Links"))

	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = "Elements" },
		color = MaterialTheme.colorScheme.background
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = {
					// TRANSLATORS: Title of the screen listing the document's headings and links
					Text(t("Elements"))
				},
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						// TRANSLATORS: Accessibility label for the back button that leaves the settings screen
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				}
			)
			PrimaryTabRow(selectedTabIndex = selectedTabIndex) {
				tabs.forEachIndexed { index, title ->
					Tab(
						selected = selectedTabIndex == index,
						onClick = { selectedTabIndex = index },
						text = { Text(title) }
					)
				}
			}

			if (selectedTabIndex == 0) {
				val items = headings?.items ?: emptyList()
				val levels = remember(items) { items.map { calculateDepth(items, it.parentIndex) } }
				val levelAt = remember(levels) { { index: Int -> levels[index] } }
				val visibleHeadingIndices = remember(items, expandedHeadingIndices) {
					flattenVisibleTreeIndices(items.size, levelAt, expandedHeadingIndices)
				}

				LazyColumn(modifier = Modifier.fillMaxSize()) {
					items(visibleHeadingIndices.size) { i ->
						val originalIndex = visibleHeadingIndices[i]
						val item = items[originalIndex]
						val level = levelAt(originalIndex)
						val hasChildren = hasTreeChildren(items.size, levelAt, originalIndex)
						val isExpanded = expandedHeadingIndices.contains(originalIndex)
						val paddingLeft = (16 + (level * 16)).dp
						// TRANSLATORS: Fallback label for a heading in the Elements screen when the document gave it no text
						val headingLabel = item.text.ifBlank { t("Untitled") }
						val toggleExpanded = {
							expandedHeadingIndices = if (isExpanded) {
								expandedHeadingIndices - originalIndex
							} else {
								expandedHeadingIndices + originalIndex
							}
						}

						Row(
							modifier = Modifier
								.fillMaxWidth()
								.clickable(onClickLabel = "go to heading") {
									viewModel.requestJumpToOffset(item.offset)
									onDismiss()
								}.semantics(mergeDescendants = true) {
									// The row's indentation shows the level on screen, so the
									// number itself is spoken rather than drawn.
									contentDescription = "$headingLabel, Level ${level + 1}"
									applyTreeExpandSemantics(hasChildren, isExpanded, toggleExpanded)
								}.padding(start = paddingLeft, top = 8.dp, bottom = 8.dp, end = 16.dp),
							verticalAlignment = Alignment.CenterVertically
						) {
							TreeExpandChevron(hasChildren, isExpanded, toggleExpanded)
							Text(
								text = headingLabel,
								modifier = Modifier.weight(1f).padding(start = 8.dp)
							)
						}
					}
				}
			} else {
				LazyColumn(modifier = Modifier.fillMaxSize()) {
					links?.items?.let { items ->
						items(items) { item ->
							Row(
								modifier = Modifier
									.fillMaxWidth()
									.clickable(onClickLabel = "go to link") {
										viewModel.requestJumpToOffset(item.offset)
										onDismiss()
									}.padding(16.dp)
							) {
								// TRANSLATORS: Fallback label for a link in the Elements screen when it has no visible text
								Text(text = item.text.ifBlank { t("Untitled Link") })
							}
						}
					}
				}
			}
		}
	}
}

private fun calculateDepth(
	items: List<uniffi.paperback.HeadingTreeItemFfi>,
	parentIndex: Int
): Int {
	var depth = 0
	var currentIndex = parentIndex
	while (currentIndex >= 0 && currentIndex < items.size) {
		depth++
		currentIndex = items[currentIndex].parentIndex
	}
	return depth
}
