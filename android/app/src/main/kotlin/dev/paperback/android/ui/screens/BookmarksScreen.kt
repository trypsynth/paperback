package dev.paperback.android.ui.screens

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.WindowInsetsSides
import androidx.compose.foundation.layout.asPaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.only
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
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
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.BookmarkRemoveDialog
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.formatDuration
import uniffi.paperback.BookmarkFilterType
import uniffi.paperback.BookmarkRowFfi

/** A real destination like Elements, so the list gets the whole window and the system back gesture leaves it. */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun BookmarksScreen(viewModel: MainScreenViewModel = viewModel(), onDismiss: () -> Unit) {
	val bookmarks = viewModel.bookmarks
	val revision by bookmarks.revision.collectAsStateWithLifecycle()
	var filterIndex by remember { mutableIntStateOf(0) }
	val filter = FILTERS[filterIndex]
	val list = remember(revision, filter) { bookmarks.rows(filter) }
	val hasAny = remember(revision) { bookmarks.hasBookmarks() }
	var pendingDelete by remember { mutableStateOf<BookmarkRowFfi?>(null) }

	// TRANSLATORS: Title of the screen listing the document's bookmarks and notes
	val title = t("Bookmarks")
	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = title },
		color = MaterialTheme.colorScheme.background
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = { Text(title) },
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				}
			)
			if (!hasAny) {
				EmptyBookmarks()
				return@Column
			}
			PrimaryTabRow(selectedTabIndex = filterIndex) {
				filterLabels().forEachIndexed { index, label ->
					Tab(selected = filterIndex == index, onClick = { filterIndex = index }, text = { Text(label) })
				}
			}
			val rows = list?.rows.orEmpty()
			if (rows.isEmpty()) {
				Box(modifier = Modifier.fillMaxSize().padding(24.dp), contentAlignment = Alignment.Center) {
					// TRANSLATORS: Announced when there are no bookmarks/notes at all to navigate to
					Text(if (filter == BookmarkFilterType.NOTES_ONLY) t("No notes.") else t("No bookmarks."))
				}
				return@Column
			}
			val closest = list?.closestIndex ?: -1
			val listState = rememberLazyListState(initialFirstVisibleItemIndex = closest.coerceAtLeast(0))
			val closestFocus = remember { FocusRequester() }
			LaunchedEffect(filter) {
				// So TalkBack starts on the bookmark nearest the reader rather than the top of a long list.
				if (closest >= 0) runCatching { closestFocus.requestFocus() }
			}
			// Otherwise the navigation bar half covers the last row; the top bar pads itself.
			LazyColumn(
				state = listState,
				modifier = Modifier.fillMaxSize(),
				contentPadding = WindowInsets.safeDrawing.only(WindowInsetsSides.Bottom).asPaddingValues()
			) {
				items(rows.size) { index ->
					val row = rows[index]
					BookmarkRow(
						row = row,
						modifier = if (index == closest) Modifier.focusRequester(closestFocus) else Modifier,
						onOpen = {
							bookmarks.goTo(row)
							onDismiss()
						},
						onEditNote = { bookmarks.openNoteFor(row) },
						onDelete = { if (row.note.isEmpty()) bookmarks.delete(row) else pendingDelete = row }
					)
				}
			}
		}
	}
	pendingDelete?.let { row ->
		BookmarkRemoveDialog(
			note = row.note,
			onConfirm = {
				pendingDelete = null
				bookmarks.delete(row)
			},
			onDismiss = { pendingDelete = null }
		)
	}
}

private val FILTERS = listOf(BookmarkFilterType.ALL, BookmarkFilterType.BOOKMARKS_ONLY, BookmarkFilterType.NOTES_ONLY)

private fun filterLabels() = listOf(
	// TRANSLATORS: Bookmarks screen filter showing bookmarks and notes together
	t("All"),
	// TRANSLATORS: Bookmarks screen filter showing only bookmarks without a note
	t("Bookmarks"),
	// TRANSLATORS: Bookmarks screen filter showing only bookmarks with a note
	t("Notes")
)

@OptIn(ExperimentalFoundationApi::class)
@Composable
private fun BookmarkRow(
	row: BookmarkRowFfi,
	modifier: Modifier,
	onOpen: () -> Unit,
	onEditNote: () -> Unit,
	onDelete: () -> Unit
) {
	var menuExpanded by remember { mutableStateOf(false) }
	val time = row.audioMs?.let { formatDuration(it) }
	// TRANSLATORS: Fallback label for a bookmark in the Bookmarks screen when it marks no text
	val main = (if (row.note.isEmpty()) row.snippet else row.note).ifBlank { t("Bookmark") }
	val secondary = if (row.note.isEmpty()) "" else row.snippet
	val noteAction = noteActionLabel(row.note.isNotEmpty())
	// TRANSLATORS: Menu item / TalkBack action deleting a bookmark from the Bookmarks screen
	val deleteLabel = t("Delete")
	Box {
		Row(
			modifier = modifier
				.fillMaxWidth()
				.combinedClickable(
					// TRANSLATORS: TalkBack action label for a bookmark in the Bookmarks screen, read as "double tap to go to bookmark"
					onClickLabel = t("go to bookmark"),
					onClick = onOpen,
					onLongClick = { menuExpanded = true }
				).semantics(mergeDescendants = true) {
					contentDescription = listOfNotNull(main, secondary.ifEmpty { null }, time).joinToString(", ")
					customActions = listOf(
						CustomAccessibilityAction(noteAction) {
							onEditNote()
							true
						},
						CustomAccessibilityAction(deleteLabel) {
							onDelete()
							true
						}
					)
				}.padding(horizontal = 16.dp, vertical = 12.dp),
			verticalAlignment = Alignment.CenterVertically
		) {
			Column(modifier = Modifier.weight(1f)) {
				Text(main, maxLines = 3, overflow = TextOverflow.Ellipsis)
				if (secondary.isNotEmpty()) {
					Text(
						secondary,
						style = MaterialTheme.typography.bodySmall,
						color = MaterialTheme.colorScheme.onSurfaceVariant,
						maxLines = 2,
						overflow = TextOverflow.Ellipsis
					)
				}
			}
			if (time != null) {
				Text(
					time,
					style = MaterialTheme.typography.labelMedium,
					color = MaterialTheme.colorScheme.onSurfaceVariant,
					modifier = Modifier.padding(start = 12.dp)
				)
			}
		}
		DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
			DropdownMenuItem(text = { Text(noteAction) }, onClick = {
				menuExpanded = false
				onEditNote()
			})
			DropdownMenuItem(text = { Text(deleteLabel) }, onClick = {
				menuExpanded = false
				onDelete()
			})
		}
	}
}

@Composable
private fun EmptyBookmarks() {
	Column(
		modifier = Modifier.fillMaxSize().padding(24.dp),
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = androidx.compose.foundation.layout.Arrangement.Center
	) {
		// TRANSLATORS: Title shown when the current document has no bookmarks or notes
		Text(t("No Bookmarks"), style = MaterialTheme.typography.titleMedium)
		// TRANSLATORS: Description shown under the "No Bookmarks" title
		Text(
			t("Tap the bookmark button while reading to mark your place."),
			style = MaterialTheme.typography.bodyMedium,
			color = MaterialTheme.colorScheme.onSurfaceVariant,
			modifier = Modifier.padding(top = 8.dp)
		)
	}
}

// TRANSLATORS: Menu item / accessibility action to write a note on the current bookmark, or to change the one it has
fun noteActionLabel(hasNote: Boolean): String = if (hasNote) t("Edit Note...") else t("Add Note...")
