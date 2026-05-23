package dev.paperback.mobile.ui.dialogs

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
import dev.paperback.mobile.ui.DocumentTabState
import uniffi.paperback.DocumentStatsFfi

@Composable
fun DocumentInfoDialog(
	docState: DocumentTabState,
	stats: DocumentStatsFfi,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Document Information" },
		title = { Text("Document Information") },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
				if (docState.title.isNotBlank()) {
					Text(
						"Title: ${docState.title}",
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier.padding(vertical = 4.dp)
					)
				}
				if (docState.author.isNotBlank()) {
					Text(
						"Author: ${docState.author}",
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier.padding(vertical = 4.dp)
					)
				}
				if (docState.fileName.isNotBlank()) {
					Text(
						"File: ${docState.fileName}",
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier.padding(vertical = 4.dp)
					)
				}
				Text(
					"Words: ${stats.wordCount}",
					style = MaterialTheme.typography.bodyLarge,
					modifier = Modifier.padding(vertical = 4.dp)
				)
				Text(
					"Lines: ${stats.lineCount}",
					style = MaterialTheme.typography.bodyLarge,
					modifier = Modifier.padding(vertical = 4.dp)
				)
				Text(
					"Characters: ${stats.charCount}",
					style = MaterialTheme.typography.bodyLarge,
					modifier = Modifier.padding(vertical = 4.dp)
				)
				Text(
					"Characters (excluding spaces): ${stats.charCountNoWhitespace}",
					style = MaterialTheme.typography.bodyLarge,
					modifier = Modifier.padding(vertical = 4.dp)
				)
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				Text("OK")
			}
		}
	)
}
