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
	initialUseInAppFileBrowser: Boolean,
	onSaveOptions: (Boolean, Boolean) -> Unit,
	onOpenTtsConfig: () -> Unit,
	onDismiss: () -> Unit
) {
	var restorePreviousDocuments by remember { mutableStateOf(initialRestorePreviousDocuments) }
	var useInAppFileBrowser by remember { mutableStateOf(initialUseInAppFileBrowser) }

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
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = useInAppFileBrowser,
							onValueChange = { useInAppFileBrowser = it },
							role = Role.Switch
						).padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					Text("Use in-app file browser (requires All Files permission)")
					Switch(
						checked = useInAppFileBrowser,
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
			TextButton(onClick = { onSaveOptions(restorePreviousDocuments, useInAppFileBrowser) }) {
				Text("Save")
			}
		}
	)
}
