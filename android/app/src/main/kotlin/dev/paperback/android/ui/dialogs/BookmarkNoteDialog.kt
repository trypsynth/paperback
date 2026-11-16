package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import dev.paperback.android.ui.state.BookmarkNoteTarget
import dev.paperback.android.ui.state.Bookmarks

@Composable
fun BookmarkNoteDialog(bookmarks: Bookmarks, target: BookmarkNoteTarget) {
	var text by remember(target) { mutableStateOf(target.existing?.note.orEmpty()) }
	val focusRequester = remember { FocusRequester() }
	LaunchedEffect(target) { runCatching { focusRequester.requestFocus() } }
	val hadNote = !target.existing?.note.isNullOrEmpty()
	// TRANSLATORS: Title of the dialog for adding or editing a bookmark note
	val title = t("Bookmark Note")
	AlertDialog(
		onDismissRequest = { bookmarks.closeNote(target) },
		modifier = Modifier.semantics { paneTitle = title },
		title = { Text(title) },
		text = {
			Column {
				if (target.passage.isNotEmpty()) {
					Text(
						target.passage,
						style = MaterialTheme.typography.bodySmall,
						color = MaterialTheme.colorScheme.onSurfaceVariant,
						maxLines = 4,
						overflow = TextOverflow.Ellipsis,
						modifier = Modifier.padding(bottom = 12.dp)
					)
				}
				OutlinedTextField(
					value = text,
					onValueChange = { text = it },
					// TRANSLATORS: Prompt label in the bookmark note dialog asking the user to type their note
					label = { Text(t("Enter bookmark note:")) },
					minLines = 3,
					maxLines = 10,
					modifier = Modifier.fillMaxWidth().focusRequester(focusRequester)
				)
				if (hadNote) {
					TextButton(
						onClick = {
							bookmarks.saveNote("", target)
							bookmarks.closeNote(target)
						},
						modifier = Modifier.padding(top = 4.dp)
					) {
						// TRANSLATORS: Button that clears a bookmark's note but keeps the bookmark
						Text(t("Delete Note"), color = MaterialTheme.colorScheme.error)
					}
				}
			}
		},
		confirmButton = {
			TextButton(onClick = {
				bookmarks.saveNote(text, target)
				bookmarks.closeNote(target)
			}) {
				// TRANSLATORS: Button that saves what was typed and closes the editor, such as a speech rule or a bookmark's note
				Text(t("Save"))
			}
		},
		dismissButton = {
			TextButton(onClick = { bookmarks.closeNote(target) }) { Text(t("Cancel")) }
		}
	)
}

/** Asked only for a bookmark with a note, since a plain one is put back with one tap. */
@Composable
fun BookmarkRemoveDialog(note: String, onConfirm: () -> Unit, onDismiss: () -> Unit) {
	// TRANSLATORS: Title of the alert confirming removal of a bookmark that has a note
	val title = t("Remove this bookmark and its note?")
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = title },
		title = { Text(title) },
		text = { Text(note) },
		confirmButton = {
			// TRANSLATORS: Button confirming removal of a bookmark and its note
			TextButton(onClick = onConfirm) { Text(t("Remove")) }
		},
		dismissButton = { TextButton(onClick = onDismiss) { Text(t("Cancel")) } }
	)
}
