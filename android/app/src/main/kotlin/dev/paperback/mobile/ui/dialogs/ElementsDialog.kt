package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.PrimaryTabRow
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
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
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
					val visibleHeadings = remember(items, expandedHeadingIndices) {
						val result = mutableListOf<Pair<Int, uniffi.paperback.HeadingTreeItemFfi>>()
						var skipLevelGreaterThan = Int.MAX_VALUE
						for (index in items.indices) {
							val item = items[index]
							val level = levels[index]
							if (level > skipLevelGreaterThan) {
								continue
							} else {
								skipLevelGreaterThan = Int.MAX_VALUE
							}
							result.add(index to item)
							val hasChildren = index + 1 < items.size && levels[index + 1] > level
							if (hasChildren && !expandedHeadingIndices.contains(index)) {
								skipLevelGreaterThan = level
							}
						}
						result
					}

					LazyColumn(modifier = Modifier.fillMaxSize()) {
						items(visibleHeadings.size) { i ->
							val (originalIndex, item) = visibleHeadings[i]
							val level = levels[originalIndex]
							val hasChildren = originalIndex + 1 < items.size && levels[originalIndex + 1] > level
							val isExpanded = expandedHeadingIndices.contains(originalIndex)
							val paddingLeft = (16 + (level * 16)).dp

							Row(
								modifier = Modifier
									.fillMaxWidth()
									.clickable(onClickLabel = "go to heading") {
										onNavigate(item.offset)
										onDismiss()
									}
									.semantics(mergeDescendants = true) {
										if (hasChildren) {
											stateDescription = if (isExpanded) t("Expanded") else t("Collapsed")
											customActions = listOf(
												CustomAccessibilityAction(
													label = if (isExpanded) t("Collapse") else t("Expand"),
													action = {
														expandedHeadingIndices = if (isExpanded) {
															expandedHeadingIndices - originalIndex
														} else {
															expandedHeadingIndices + originalIndex
														}
														true
													}
												)
											)
										}
									}
									.padding(start = paddingLeft, top = 8.dp, bottom = 8.dp, end = 16.dp),
								verticalAlignment = Alignment.CenterVertically
							) {
								if (hasChildren) {
									Box(
										modifier = Modifier
											.size(36.dp)
											.pointerInput(Unit) {
												detectTapGestures(onTap = {
													expandedHeadingIndices = if (isExpanded) {
														expandedHeadingIndices - originalIndex
													} else {
														expandedHeadingIndices + originalIndex
													}
												})
											},
										contentAlignment = Alignment.Center
									) {
										Text(
											text = if (isExpanded) "▼" else "▶",
											style = MaterialTheme.typography.bodyMedium,
											modifier = Modifier.clearAndSetSemantics { }
										)
									}
								} else {
									Spacer(modifier = Modifier.width(36.dp))
								}
								Text(
									text = "${item.text.ifBlank { t("Untitled") }}, Level ${level + 1}",
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
										}
										.padding(16.dp)
								) {
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

private fun calculateDepth(items: List<uniffi.paperback.HeadingTreeItemFfi>, parentIndex: Int): Int {
	var depth = 0
	var currentIndex = parentIndex
	while (currentIndex >= 0 && currentIndex < items.size) {
		depth++
		currentIndex = items[currentIndex].parentIndex
	}
	return depth
}
