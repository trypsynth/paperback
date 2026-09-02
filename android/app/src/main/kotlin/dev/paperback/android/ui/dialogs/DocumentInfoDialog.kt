package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Article
import androidx.compose.material.icons.automirrored.filled.InsertDriveFile
import androidx.compose.material.icons.automirrored.filled.List
import androidx.compose.material.icons.filled.Folder
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.SpaceBar
import androidx.compose.material.icons.filled.TextFields
import androidx.compose.material.icons.filled.Title
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.ListItemDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import dev.paperback.android.t
import dev.paperback.android.ui.DocumentTabState
import uniffi.paperback.DocumentStatsFfi

@Composable
private fun InfoRow(
	icon: ImageVector,
	label: String,
	value: String
) {
	ListItem(
		leadingContent = { Icon(icon, contentDescription = null) },
		overlineContent = { Text(label) },
		headlineContent = { Text(value) },
		colors = ListItemDefaults.colors(containerColor = Color.Transparent)
	)
}

@Composable
fun DocumentInfoDialog(
	docState: DocumentTabState,
	stats: DocumentStatsFfi,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = t("Document Information") },
		// TRANSLATORS: Title of the dialog showing the current document's title, author, path, and word/line/character counts
		title = { Text(t("Document Information")) },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
				if (docState.title.isNotBlank()) {
					// TRANSLATORS: Label for the document's title in the Document Information dialog
					InfoRow(Icons.Filled.Title, t("Title"), docState.title)
				}
				if (docState.author.isNotBlank()) {
					// TRANSLATORS: Label for the document's author in the Document Information dialog
					InfoRow(Icons.Filled.Person, t("Author"), docState.author)
				}
				if (docState.documentUri.isNotBlank()) {
					if (docState.documentUri.startsWith("content://")) {
						// TRANSLATORS: Label for the document's file name in the Document Information dialog
						InfoRow(Icons.AutoMirrored.Filled.InsertDriveFile, t("File Name"), docState.fileName)
					} else {
						// TRANSLATORS: Label for the document's file path in the Document Information dialog
						InfoRow(Icons.Filled.Folder, t("Path"), docState.documentUri.removePrefix("file://"))
					}
				}
				// TRANSLATORS: Label for the document's word count in the Document Information dialog
				InfoRow(Icons.AutoMirrored.Filled.Article, t("Words"), "${stats.wordCount}")
				// TRANSLATORS: Label for the document's line count in the Document Information dialog
				InfoRow(Icons.AutoMirrored.Filled.List, t("Lines"), "${stats.lineCount}")
				// TRANSLATORS: Label for the document's character count in the Document Information dialog
				InfoRow(Icons.Filled.TextFields, t("Characters"), "${stats.charCount}")
				InfoRow(
					Icons.Filled.SpaceBar,
					// TRANSLATORS: Label for the document's character count excluding whitespace in the Document Information dialog
					t("Characters (excluding spaces)"),
					"${stats.charCountNoWhitespace}"
				)
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
