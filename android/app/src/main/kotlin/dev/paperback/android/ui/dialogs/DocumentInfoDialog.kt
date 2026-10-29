package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ListItem
import androidx.compose.material3.ListItemDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import dev.paperback.android.t
import dev.paperback.android.ui.state.DocumentTabState
import dev.paperback.android.ui.state.formatDuration
import uniffi.paperback.DocumentStatsFfi
import java.text.NumberFormat

@Composable
private fun InfoRow(label: String, value: String) {
	ListItem(
		overlineContent = { Text(label) },
		headlineContent = { Text(value) },
		colors = ListItemDefaults.colors(containerColor = Color.Transparent)
	)
}

@Composable
fun DocumentInfoDialog(
	docState: DocumentTabState,
	stats: DocumentStatsFfi,
	progressPercent: Int?,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = t("Document Info") },
		// TRANSLATORS: Title of the dialog showing the current document's title, author, file name, and word/line/character counts
		title = { Text(t("Document Info")) },
		text = {
			val numbers = NumberFormat.getIntegerInstance()
			Column(modifier = Modifier.fillMaxWidth()) {
				if (docState.title.isNotBlank()) {
					// TRANSLATORS: Label for the document's title in the Document Information dialog
					InfoRow(t("Title"), docState.title)
				}
				if (docState.author.isNotBlank()) {
					// TRANSLATORS: Label for the document's author in the Document Information dialog
					InfoRow(t("Author"), docState.author)
				}
				// TRANSLATORS: Label for the document's file name in the Document Information dialog
				InfoRow(t("File Name"), docState.fileName)
				// Text mode has no reading bar to show this on, so it is here for either mode.
				if (progressPercent != null) {
					// TRANSLATORS: Label in the Document Information dialog for how far through the document the reader has got
					InfoRow(t("Progress"), t("{}% through", progressPercent.toString()))
				}
				// TRANSLATORS: Label for the document's word count in the Document Information dialog
				InfoRow(t("Words"), numbers.format(stats.wordCount))
				// TRANSLATORS: Label for the document's line count in the Document Information dialog
				InfoRow(t("Lines"), numbers.format(stats.lineCount))
				// TRANSLATORS: Label for the document's character count in the Document Information dialog
				InfoRow(t("Characters"), numbers.format(stats.charCount))
				InfoRow(
					// TRANSLATORS: Label for the document's character count excluding whitespace in the Document Information dialog
					t("Characters (excluding spaces)"),
					numbers.format(stats.charCountNoWhitespace)
				)
				if (stats.audioFileCount > 0) {
					// TRANSLATORS: Label for the number of audio files in the document in the Document Information dialog
					InfoRow(t("Number of files"), numbers.format(stats.audioFileCount))
					// Negative when any file's length is unknown, which would make any total hours too long.
					if (stats.audioTotalDurationMs >= 0) {
						// TRANSLATORS: Label for the total playback duration across all of the document's audio files in the Document Information dialog
						InfoRow(t("Total duration"), formatDuration(stats.audioTotalDurationMs))
						// TRANSLATORS: Label for the average playback duration per audio file in the document in the Document Information dialog
						InfoRow(t("Average duration"), formatDuration(stats.audioTotalDurationMs / stats.audioFileCount))
					}
				}
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				// TRANSLATORS: Button to close the Document Information dialog
				Text(t("OK"))
			}
		}
	)
}
