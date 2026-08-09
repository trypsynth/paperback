package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Article
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import dev.paperback.android.t
import uniffi.paperback.DocumentStatsFfi

@Composable
fun WordCountDialog(
	stats: DocumentStatsFfi,
	onDismiss: () -> Unit
) {
	// TRANSLATORS: Sentence announced to screen readers with the document's word count; {} is replaced with the number
	val announcement = t("This document contains {} words.").replace("{}", "${stats.wordCount}")

	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Word Count" },
		icon = { Icon(Icons.AutoMirrored.Filled.Article, contentDescription = null) },
		// TRANSLATORS: Title of the dialog showing the current document's word count
		title = { Text(t("Word Count")) },
		text = {
			Column(
				horizontalAlignment = Alignment.CenterHorizontally,
				modifier = Modifier
					.fillMaxWidth()
					.clearAndSetSemantics { contentDescription = announcement }
			) {
				Text(
					text = "${stats.wordCount}",
					style = MaterialTheme.typography.displaySmall
				)
				Text(
					// TRANSLATORS: Unit label shown under the large word-count number
					text = t("words"),
					style = MaterialTheme.typography.bodyMedium,
					color = MaterialTheme.colorScheme.onSurfaceVariant
				)
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				// TRANSLATORS: Button to close the Word Count dialog
				Text(t("OK"))
			}
		}
	)
}
