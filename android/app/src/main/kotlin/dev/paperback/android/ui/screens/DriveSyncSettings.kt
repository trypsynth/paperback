package dev.paperback.android.ui.screens

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.selection.selectable
import androidx.compose.foundation.selection.selectableGroup
import androidx.compose.foundation.selection.toggleable
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.RadioButton
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import dev.paperback.android.t
import dev.paperback.android.ui.state.DriveStatus
import dev.paperback.android.ui.state.DriveSync

@Composable
internal fun DriveSyncSettings(drive: DriveSync) {
	val enabled by drive.enabled.collectAsStateWithLifecycle()
	val connected by drive.connected.collectAsStateWithLifecycle()
	val mobileData by drive.mobileData.collectAsStateWithLifecycle()
	val status by drive.status.collectAsStateWithLifecycle()
	val account by drive.account.collectAsStateWithLifecycle()
	var askToConnect by rememberSaveable { mutableStateOf(false) }
	Text(
		t("Google Drive synchronization"), style = MaterialTheme.typography.titleMedium,
		modifier = Modifier.semantics {
			heading()
		}
	)
	Row(
		modifier = Modifier
			.fillMaxWidth()
			.toggleable(enabled, role = Role.Switch, onValueChange = { value ->
				drive.setEnabled(value)
				askToConnect = value && !connected
			})
			.padding(vertical = 8.dp),
		verticalAlignment = Alignment.CenterVertically,
		horizontalArrangement = Arrangement.SpaceBetween
	) {
		Text(t("Sync book data"), modifier = Modifier.weight(1f))
		Switch(checked = enabled, onCheckedChange = null)
	}
	if (enabled) {
		if (!connected) {
			TextButton(onClick = drive::connect) { Text(t("Connect Google account")) }
		}
		if (account.isNotEmpty()) Text(account)
		Text(
			when (status) {
				DriveStatus.Disconnected -> t("Google Drive is disconnected.")
				DriveStatus.NeedsConnection -> t("Connect to Google Drive to sync.")
				DriveStatus.Syncing -> t("Synchronizing book data…")
				DriveStatus.Synced -> t("Book data synchronized.")
				DriveStatus.WaitingForFiles -> t("Book data synchronized. Book files are waiting for a connection.")
				DriveStatus.Unavailable -> t("Google Drive is unavailable in this build of Paperback.")
				DriveStatus.Failed -> t("Google Drive could not synchronize. You can retry.")
			}
		)
		if (connected && status == DriveStatus.Failed) {
			TextButton(onClick = drive::connect) { Text(t("Retry synchronization")) }
		}
		Text(t("Connection for book files"), modifier = Modifier.semantics { heading() })
		Column(modifier = Modifier.selectableGroup()) {
			DriveConnectionChoice(t("Wi-Fi only"), !mobileData) { drive.setMobileData(false) }
			DriveConnectionChoice(t("Wi-Fi and mobile data"), mobileData) { drive.setMobileData(true) }
		}
		Text(t("Reading positions, bookmarks and notes sync on any internet connection."))
		TextButton(onClick = { drive.showLibrary(true) }) { Text(t("Cloud books")) }
	}
	if (askToConnect) {
		AlertDialog(
			onDismissRequest = { askToConnect = false },
			title = { Text(t("Connect your Google account now?")) },
			confirmButton = {
				TextButton(onClick = {
					askToConnect = false
					drive.connect()
				}) { Text(t("Yes")) }
			},
			dismissButton = { TextButton(onClick = { askToConnect = false }) { Text(t("No")) } }
		)
	}
}

@Composable
private fun DriveConnectionChoice(label: String, selected: Boolean, onClick: () -> Unit) {
	Row(
		modifier = Modifier
			.fillMaxWidth()
			.selectable(selected, role = Role.RadioButton, onClick = onClick)
			.padding(vertical = 8.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		RadioButton(selected = selected, onClick = null)
		Text(label, modifier = Modifier.padding(start = 8.dp))
	}
}
