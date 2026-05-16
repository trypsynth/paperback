package dev.paperback.mobile.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import uniffi.paperback.TocEntry

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TocSheet(
	toc: List<TocEntry>,
	expandedTocIndices: Set<Int>,
	onToggleExpand: (Int) -> Unit,
	onItemClick: (TocEntry) -> Unit,
	onDismiss: () -> Unit
) {
	val visibleToc = remember(toc, expandedTocIndices) {
		val result = mutableListOf<Pair<Int, TocEntry>>()
		var skipLevelGreaterThan = Int.MAX_VALUE

		for ((index, entry) in toc.withIndex()) {
			if (entry.level > skipLevelGreaterThan) {
				continue
			} else {
				skipLevelGreaterThan = Int.MAX_VALUE
			}

			result.add(index to entry)

			val hasChildren = index + 1 < toc.size && toc[index + 1].level > entry.level
			if (hasChildren && !expandedTocIndices.contains(index)) {
				skipLevelGreaterThan = entry.level
			}
		}
		result
	}

	ModalBottomSheet(onDismissRequest = onDismiss) {
		LazyColumn(contentPadding = PaddingValues(bottom = 32.dp)) {
			item {
				Text(
					text = "Table of Contents",
					style = MaterialTheme.typography.titleLarge,
					modifier = Modifier.padding(16.dp)
				)
			}
			items(visibleToc.size) { i ->
				val (originalIndex, item) = visibleToc[i]
				val hasChildren = originalIndex + 1 < toc.size && toc[originalIndex + 1].level > item.level
				val isExpanded = expandedTocIndices.contains(originalIndex)
				val paddingLeft = (16 + (item.level * 16)).dp

				Row(
					modifier = Modifier
						.fillMaxWidth()
						.clickable {
							if (hasChildren) {
								onToggleExpand(originalIndex)
							} else {
								onItemClick(item)
							}
						}
						.semantics {
							if (hasChildren) {
								stateDescription = if (isExpanded) "Expanded" else "Collapsed"
							}
						}
						.padding(start = paddingLeft, end = 16.dp, top = 12.dp, bottom = 12.dp),
					verticalAlignment = Alignment.CenterVertically
				) {
					if (hasChildren) {
						Text(
							text = if (isExpanded) "▼ " else "▶ ",
							style = MaterialTheme.typography.bodyLarge,
							modifier = Modifier.clearAndSetSemantics { }
						)
					} else {
						Spacer(modifier = Modifier.width(24.dp))
					}
					Text(
						text = item.title,
						style = MaterialTheme.typography.bodyLarge
					)
				}
			}
		}
	}
}
