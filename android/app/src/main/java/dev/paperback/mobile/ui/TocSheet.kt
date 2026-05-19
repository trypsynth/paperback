package dev.paperback.mobile.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
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
	val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)
	val scope = rememberCoroutineScope()
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
	
	ModalBottomSheet(
		onDismissRequest = onDismiss,
		sheetState = sheetState,
		dragHandle = null,
		modifier = Modifier.semantics { paneTitle = "Table of Contents" }
	) {
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
						.clickable(onClickLabel = "go to chapter") {
							scope.launch {
								sheetState.hide()
								onItemClick(item)
							}
						}.semantics(mergeDescendants = true) {
							if (hasChildren) {
								stateDescription = if (isExpanded) "Expanded" else "Collapsed"
								customActions = listOf(
									CustomAccessibilityAction(
										label = if (isExpanded) "Collapse" else "Expand",
										action = {
											onToggleExpand(originalIndex)
											true
										}
									)
								)
							}
						}.padding(start = paddingLeft, end = 16.dp, top = 8.dp, bottom = 8.dp),
					verticalAlignment = Alignment.CenterVertically
				) {
					if (hasChildren) {
						Box(
							modifier = Modifier
								.size(36.dp)
								.pointerInput(Unit) {
									detectTapGestures(onTap = { onToggleExpand(originalIndex) })
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
						text = "${item.title}, Level ${item.level + 1}",
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier.weight(1f).padding(start = 8.dp)
					)
				}
			}
		}
	}
}
