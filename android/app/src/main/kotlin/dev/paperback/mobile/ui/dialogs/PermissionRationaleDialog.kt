package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.t

@Composable
fun PermissionRationaleDialog(
	onGrantClick: () -> Unit,
	onDismiss: () -> Unit
) {
	AlertDialog(
		onDismissRequest = onDismiss,
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
					// TRANSLATORS: Heading introducing the bulleted list of reasons the permission is needed
					text = t("Why we need this:"),
					style = MaterialTheme.typography.bodyMedium,
					fontWeight = FontWeight.Bold,
					modifier = Modifier.padding(bottom = 4.dp)
				)
				Text(
					// TRANSLATORS: First bullet point reason for requesting the "All Files Access" permission
					text = t("• To provide a fast, fully screen-reader accessible file manager inside the app."),
					style = MaterialTheme.typography.bodyMedium
				)
				Text(
					// TRANSLATORS: Second bullet point reason for requesting the "All Files Access" permission
					text = t("• To load large files instantly without needing to copy them into the app's cache."),
					style = MaterialTheme.typography.bodyMedium
				)
				Text(
					// TRANSLATORS: Third bullet point reason for requesting the "All Files Access" permission
					text = t("• To display the exact local file paths of your documents."),
					style = MaterialTheme.typography.bodyMedium,
					modifier = Modifier.padding(bottom = 8.dp)
				)
				Text(
					// TRANSLATORS: Closing note explaining the fallback if the user denies the permission
					text = t("If you deny this permission, you can still use the Android System File Picker to open your books by turning off the custom file browser setting."),
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
