package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Article
import androidx.compose.material.icons.filled.Code
import androidx.compose.material.icons.filled.Description
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import uniffi.paperback.ExportFormat

@Composable
fun ExportDocumentDialog(
	supportedFormats: List<ExportFormat>,
	onFormatSelected: (ExportFormat) -> Unit,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Export Document" },
		// TRANSLATORS: Title of the dialog for exporting the current document to another file format
		title = { Text(t("Export Document")) },
		text = {
			Column {
				Text(
					// TRANSLATORS: Instruction text in the Export Document dialog, above the list of export format options
					t("Select a format to export the current document:"),
					modifier = Modifier.padding(bottom = 8.dp)
				)
				supportedFormats.forEachIndexed { index, format ->
					val (label, icon) = when (format) {
						// TRANSLATORS: Export format option to save the document as a plain text file
						ExportFormat.TEXT -> t("Plain Text (.txt)") to Icons.Filled.Description
						// TRANSLATORS: Export format option to save the document as an HTML file
						ExportFormat.HTML -> t("HTML (.html)") to Icons.Filled.Code
						// TRANSLATORS: Export format option to save the document as a Markdown file
						ExportFormat.MARKDOWN -> t("Markdown (.md)") to Icons.AutoMirrored.Filled.Article
					}
					ListItem(
						headlineContent = { Text(label) },
						leadingContent = { Icon(icon, contentDescription = null) },
						modifier = Modifier.clickable(role = Role.Button) { onFormatSelected(format) }
					)
					if (index < supportedFormats.lastIndex) {
						HorizontalDivider()
					}
				}
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				// TRANSLATORS: Button to close the Export Document dialog without exporting
				Text(t("Cancel"))
			}
		}
	)
}
