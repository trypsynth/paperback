package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsDialog(
	initialRestorePreviousDocuments: Boolean,
	onSaveOptions: (Boolean) -> Unit,
	onOpenTtsConfig: () -> Unit,
	onDismiss: () -> Unit
) {
	var restorePreviousDocuments by remember { mutableStateOf(initialRestorePreviousDocuments) }

	AlertDialog(
		modifier = Modifier.semantics { paneTitle = "Settings" },
		onDismissRequest = onDismiss,
		title = { Text("Settings") },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = restorePreviousDocuments,
							onValueChange = { restorePreviousDocuments = it },
							role = Role.Switch
						).padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					Text("Restore last open book")
					Switch(
						checked = restorePreviousDocuments,
						onCheckedChange = null
					)
				}
				Spacer(modifier = Modifier.height(16.dp))
				Button(onClick = onOpenTtsConfig, modifier = Modifier.fillMaxWidth()) {
					Text("TTS Settings")
				}
			}
		},
		dismissButton = {
			TextButton(onClick = onDismiss) {
				Text("Cancel")
			}
		},
		confirmButton = {
			TextButton(onClick = { onSaveOptions(restorePreviousDocuments) }) {
				Text("Save")
			}
		}
	)
}
