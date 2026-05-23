package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp

private val presets = listOf(5, 10, 15, 30, 45, 60)

@Composable
fun SleepTimerDialog(
	remainingSeconds: Int?,
	onSetTimer: (Int) -> Unit,
	onCancelTimer: () -> Unit,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Sleep Timer" },
		title = { Text("Sleep Timer") },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
				if (remainingSeconds != null) {
					val min = remainingSeconds / 60
					val sec = remainingSeconds % 60
					Text(
						"Active: %d:%02d remaining".format(min, sec),
						style = MaterialTheme.typography.bodyLarge,
						modifier = Modifier.padding(bottom = 8.dp)
					)
					OutlinedButton(
						onClick = { onCancelTimer(); onDismiss() },
						modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp)
					) {
						Text("Cancel Timer")
					}
					Text("Change to:", style = MaterialTheme.typography.labelMedium)
					Spacer(modifier = Modifier.height(8.dp))
				}
				presets.chunked(3).forEach { row ->
					Row(
						modifier = Modifier.fillMaxWidth(),
						horizontalArrangement = Arrangement.spacedBy(8.dp)
					) {
						row.forEach { minutes ->
							OutlinedButton(
								onClick = { onSetTimer(minutes); onDismiss() },
								modifier = Modifier.weight(1f)
							) {
								Text("${minutes}m")
							}
						}
					}
					Spacer(modifier = Modifier.height(8.dp))
				}
			}
		},
		confirmButton = {},
		dismissButton = {
			TextButton(onClick = onDismiss) { Text("Close") }
		}
	)
}
