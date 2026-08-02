package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import uniffi.paperback.ExportFormatFfi

@Composable
fun ExportDocumentDialog(
	supportedFormats: List<ExportFormatFfi>,
	onFormatSelected: (ExportFormatFfi) -> Unit,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Export Document" },
		title = { Text(t("Export Document")) },
		text = {
			Column {
				Text(
					t("Select a format to export the current document:"),
					modifier = Modifier.padding(bottom = 16.dp)
				)
				supportedFormats.forEach { format ->
					val label = when (format) {
						ExportFormatFfi.TEXT -> t("Plain Text (.txt)")
						ExportFormatFfi.HTML -> t("HTML (.html)")
						ExportFormatFfi.MARKDOWN -> t("Markdown (.md)")
					}
					Text(
						text = label,
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier
							.fillMaxWidth()
							.clickable { onFormatSelected(format) }
							.padding(vertical = 12.dp)
					)
				}
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				Text(t("Cancel"))
			}
		}
	)
}
