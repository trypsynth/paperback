package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import kotlinx.coroutines.launch
import uniffi.paperback.TocEntry

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TocDialog(
	toc: List<TocEntry>,
	expandedTocIndices: Set<Int>,
	activeTocIndex: Int?,
	onToggleExpand: (Int) -> Unit,
	onItemClick: (TocEntry) -> Unit,
	onDismiss: () -> Unit
) {
	val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)
	val scope = rememberCoroutineScope()
	val listState = rememberLazyListState()
	val focusRequester = remember { FocusRequester() }
	val levelAt = remember(toc) { { index: Int -> toc[index].level } }
	val visibleTocIndices = remember(toc, expandedTocIndices) {
		flattenVisibleTreeIndices(toc.size, levelAt, expandedTocIndices)
	}

	LaunchedEffect(activeTocIndex) {
		if (activeTocIndex != null) {
			val visibleIndex = visibleTocIndices.indexOf(activeTocIndex)
			if (visibleIndex != -1) {
				listState.scrollToItem(visibleIndex + 1)
				try {
					focusRequester.requestFocus()
				} catch (e: Exception) {
					// Ignore if not attached
				}
			}
		}
	}

	ModalBottomSheet(
		onDismissRequest = onDismiss,
		sheetState = sheetState,
		dragHandle = null,
		modifier = Modifier.semantics { paneTitle = "Table of Contents" }
	) {
		LazyColumn(state = listState, contentPadding = PaddingValues(bottom = 32.dp)) {
			item {
				Text(
					// TRANSLATORS: Title heading at the top of the Table of Contents sheet
					text = t("Table of Contents"),
					style = MaterialTheme.typography.titleLarge,
					modifier = Modifier.padding(16.dp)
				)
			}
			items(visibleTocIndices.size) { i ->
				val originalIndex = visibleTocIndices[i]
				val item = toc[originalIndex]
				val hasChildren = hasTreeChildren(toc.size, levelAt, originalIndex)
				val isExpanded = expandedTocIndices.contains(originalIndex)
				val isActive = originalIndex == activeTocIndex
				val paddingLeft = (16 + (item.level * 16)).dp
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.then(if (isActive) Modifier.focusRequester(focusRequester) else Modifier)
						.clickable(onClickLabel = "go to chapter") {
							scope.launch {
								sheetState.hide()
								onItemClick(item)
							}
						}.semantics(mergeDescendants = true) {
							if (isActive) {
								selected = true
							}
							applyTreeExpandSemantics(hasChildren, isExpanded) { onToggleExpand(originalIndex) }
						}.padding(start = paddingLeft, end = 16.dp, top = 8.dp, bottom = 8.dp),
					verticalAlignment = Alignment.CenterVertically
				) {
					TreeExpandChevron(hasChildren, isExpanded) { onToggleExpand(originalIndex) }
					Text(
						text = "${item.title}, Level ${item.level + 1}",
						style = if (isActive) MaterialTheme.typography.titleMedium else MaterialTheme.typography.bodyLarge,
						color = if (isActive) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurface,
						modifier = Modifier.weight(1f).padding(start = 8.dp)
					)
				}
			}
		}
	}
}
