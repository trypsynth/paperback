package dev.paperback.mobile.ui.dialogs

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import uniffi.paperback.DocumentStatsFfi
import dev.paperback.mobile.t

@Composable
fun WordCountDialog(
	stats: DocumentStatsFfi,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Word Count" },
		title = { Text(t("Word Count")) },
		text = {
			Text(
				"This document contains ${stats.wordCount} words.",
				style = MaterialTheme.typography.bodyLarge
			)
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				Text(t("OK"))
			}
		}
	)
}
