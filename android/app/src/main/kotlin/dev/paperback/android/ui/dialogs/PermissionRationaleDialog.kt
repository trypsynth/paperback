package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Accessibility
import androidx.compose.material.icons.filled.Folder
import androidx.compose.material.icons.filled.FolderOpen
import androidx.compose.material.icons.filled.Speed
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.paperback.android.t

@Composable
private fun ReasonRow(
	icon: ImageVector,
	text: String
) {
	Row(modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp)) {
		Icon(
			icon,
			contentDescription = null,
			tint = MaterialTheme.colorScheme.primary,
			modifier = Modifier.size(20.dp).padding(top = 2.dp, end = 12.dp)
		)
		Text(text, style = MaterialTheme.typography.bodyMedium)
	}
}

@Composable
fun PermissionRationaleDialog(
	onGrantClick: () -> Unit,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "All Files Access Required" },
		icon = { Icon(Icons.Filled.FolderOpen, contentDescription = null) },
		// TRANSLATORS: Title of the dialog explaining why the app wants the "All Files Access" permission
		title = { Text(text = t("All Files Access Required")) },
		text = {
			Column {
				Text(
					// TRANSLATORS: Intro sentence explaining what the "All Files Access" permission is used for
					text = t("Paperback requires the 'All Files Access' permission to enable the custom in-app file browser."),
					style = MaterialTheme.typography.bodyMedium,
					modifier = Modifier.padding(bottom = 8.dp)
				)
				Text(
					// TRANSLATORS: Heading introducing the list of reasons the permission is needed
					text = t("Why we need this:"),
					style = MaterialTheme.typography.bodyMedium,
					fontWeight = FontWeight.Bold,
					modifier = Modifier.padding(bottom = 4.dp)
				)
				ReasonRow(
					Icons.Filled.Accessibility,
					// TRANSLATORS: First reason for requesting the "All Files Access" permission
					t("To provide a fast, fully screen-reader accessible file manager inside the app.")
				)
				ReasonRow(
					Icons.Filled.Speed,
					// TRANSLATORS: Second reason for requesting the "All Files Access" permission
					t("To load large files instantly without needing to copy them into the app's cache.")
				)
				ReasonRow(
					Icons.Filled.Folder,
					// TRANSLATORS: Third reason for requesting the "All Files Access" permission
					t("To display the exact local file paths of your documents.")
				)
				Spacer(modifier = Modifier.height(8.dp))
				Text(
					// TRANSLATORS: Closing note explaining the fallback if the user denies the permission
					text = t(
						"If you deny this permission, you can still use the Android System File Picker to open your books by turning off the custom file browser setting."
					),
					style = MaterialTheme.typography.bodyMedium,
					fontWeight = FontWeight.Bold
				)
			}
		},
		confirmButton = {
			Button(onClick = onGrantClick) {
				// TRANSLATORS: Button label to grant the requested permission
				Text(t("Grant"))
			}
		},
		dismissButton = {
			TextButton(onClick = onDismiss) {
				// TRANSLATORS: Button label to dismiss the permission rationale dialog without granting it
				Text(t("Not Now"))
			}
		}
	)
}
