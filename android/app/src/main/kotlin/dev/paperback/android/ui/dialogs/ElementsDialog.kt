package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.PrimaryTabRow
import androidx.compose.material3.Surface
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import dev.paperback.android.t
import uniffi.paperback.HeadingTreeFfi
import uniffi.paperback.LinkListFfi

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ElementsDialog(
	headings: HeadingTreeFfi?,
	links: LinkListFfi?,
	onNavigate: (Long) -> Unit,
	onDismiss: () -> Unit
) {
	Dialog(
		onDismissRequest = onDismiss,
		properties = DialogProperties(usePlatformDefaultWidth = false)
	) {
		Surface(
			modifier = Modifier.fillMaxSize(),
			color = MaterialTheme.colorScheme.background
		) {
			var selectedTabIndex by remember { mutableIntStateOf(0) }
			// TRANSLATORS: Tab labels in the Elements dialog for browsing a document's headings vs. its links
			val tabs = listOf(t("Headings"), t("Links"))

			Column(modifier = Modifier.fillMaxSize()) {
				PrimaryTabRow(selectedTabIndex = selectedTabIndex) {
					tabs.forEachIndexed { index, title ->
						Tab(
							selected = selectedTabIndex == index,
							onClick = { selectedTabIndex = index },
							text = { Text(title) }
						)
					}
				}

				var expandedHeadingIndices by remember { mutableStateOf(setOf<Int>()) }

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
							// TRANSLATORS: Fallback label for a heading in the Elements dialog when the document gave it no text
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
										onNavigate(item.offset)
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
											onNavigate(item.offset)
											onDismiss()
										}.padding(16.dp)
								) {
									// TRANSLATORS: Fallback label for a link in the Elements dialog when it has no visible text
									Text(text = item.text.ifBlank { t("Untitled Link") })
								}
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
