package dev.paperback.mobile.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.BottomAppBar
import androidx.compose.material3.Button
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.isTraversalGroup
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import androidx.compose.runtime.rememberCoroutineScope
import uniffi.paperback.SearchOptionsFfi
import dev.paperback.mobile.t

@Composable
fun SearchBottomBar(
	docState: DocumentTabState,
	listState: LazyListState,
	activeSearchQuery: String,
	activeSearchOptions: SearchOptionsFfi,
	onClose: () -> Unit,
	onNavigate: (Int) -> Unit
) {
	val scope = rememberCoroutineScope()
	BottomAppBar(
		modifier = Modifier.semantics { isTraversalGroup = true },
		contentPadding = PaddingValues(horizontal = 16.dp)
	) {
		Row(
			modifier = Modifier.fillMaxWidth(),
			horizontalArrangement = Arrangement.SpaceBetween,
			verticalAlignment = Alignment.CenterVertically
		) {
			IconButton(
				onClick = onClose,
				modifier = Modifier.semantics { contentDescription = t("Close Search") }
			) {
				Text("X", fontWeight = FontWeight.Bold, style = MaterialTheme.typography.titleLarge)
			}
			Row {
				Button(
					onClick = {
						val currentIdx = listState.firstVisibleItemIndex
						val pos = docState.session.positionFromLine((currentIdx + 1).toLong())
						val res = docState.session.searchFfi(activeSearchQuery, pos, activeSearchOptions.copy(forward = false))
						if (res.found) {
							val targetIndex = (docState.session.lineFromPosition(res.position) - 1).toInt().coerceAtLeast(0)
							scope.launch {
								listState.scrollToItem(targetIndex)
								onNavigate(targetIndex)
							}
						}
					}
				) {
					Text(t("Find Previous"))
				}
				Spacer(modifier = Modifier.width(8.dp))
				Button(
					onClick = {
						val currentIdx = listState.firstVisibleItemIndex
						val nextLine = (currentIdx + 2).toLong().coerceAtMost(docState.lineCount)
						val pos = docState.session.positionFromLine(nextLine)
						val res = docState.session.searchFfi(activeSearchQuery, pos, activeSearchOptions.copy(forward = true))
						if (res.found) {
							val targetIndex = (docState.session.lineFromPosition(res.position) - 1).toInt().coerceAtLeast(0)
							scope.launch {
								listState.scrollToItem(targetIndex)
								onNavigate(targetIndex)
							}
						}
					}
				) {
					Text(t("Find Next"))
				}
			}
		}
	}
}
