package dev.paperback.android.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.TreeExpandChevron
import dev.paperback.android.ui.dialogs.applyTreeExpandSemantics
import dev.paperback.android.ui.dialogs.flattenVisibleTreeIndices
import dev.paperback.android.ui.dialogs.hasTreeChildren

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TocScreen(
	viewModel: MainScreenViewModel = viewModel(),
	onDismiss: () -> Unit
) {
	val state by viewModel.uiState.collectAsStateWithLifecycle()
	val tocState by viewModel.tocState.collectAsStateWithLifecycle()
	val expandedTocIndices = tocState.expandedIndices
	val activeTocIndex = tocState.activeIndex
	val toc = (state as? MainScreenUiState.Success)?.activeTab?.toc.orEmpty()
	val listState = rememberLazyListState()
	val focusRequester = remember { FocusRequester() }
	val levelAt = remember(toc) { { index: Int -> toc[index].level } }
	val visibleTocIndices = remember(toc, expandedTocIndices) {
		flattenVisibleTreeIndices(toc.size, levelAt, expandedTocIndices)
	}

	LaunchedEffect(Unit) { viewModel.prepareToc() }

	LaunchedEffect(activeTocIndex) {
		val active = activeTocIndex ?: return@LaunchedEffect
		val visibleIndex = visibleTocIndices.indexOf(active)
		if (visibleIndex != -1) {
			listState.scrollToItem(visibleIndex)
			try {
				focusRequester.requestFocus()
			} catch (e: Exception) {
				// Ignore if not attached
			}
		}
	}

	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = t("Table of Contents") },
		color = MaterialTheme.colorScheme.surface
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = {
					// TRANSLATORS: Title heading at the top of the Table of Contents screen
					Text(t("Table of Contents"))
				},
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						// TRANSLATORS: Accessibility label for the back button that leaves the settings screen
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				}
			)
			LazyColumn(
				state = listState,
				modifier = Modifier.fillMaxSize(),
				contentPadding = PaddingValues(bottom = 32.dp)
			) {
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
								viewModel.updateTtsPosition(item.position)
								onDismiss()
							}.semantics(mergeDescendants = true) {
								// The row's indentation shows the level on screen, so the number
								// itself is spoken rather than drawn.
								contentDescription = "${item.title}, Level ${item.level + 1}"
								if (isActive) {
									selected = true
								}
								applyTreeExpandSemantics(hasChildren, isExpanded) {
									viewModel.toggleTocExpanded(originalIndex)
								}
							}.padding(start = paddingLeft, end = 16.dp, top = 8.dp, bottom = 8.dp),
						verticalAlignment = Alignment.CenterVertically
					) {
						TreeExpandChevron(hasChildren, isExpanded) { viewModel.toggleTocExpanded(originalIndex) }
						Text(
							text = item.title,
							style = if (isActive) {
								MaterialTheme.typography.titleMedium
							} else {
								MaterialTheme.typography.bodyLarge
							},
							color = if (isActive) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurface,
							modifier = Modifier.weight(1f).padding(start = 8.dp)
						)
					}
				}
			}
		}
	}
}
