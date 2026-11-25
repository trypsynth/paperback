package dev.paperback.android.ui.dialogs

import android.app.Activity
import android.text.format.Formatter
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.IntentSenderRequest
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import dev.paperback.android.t
import dev.paperback.android.ui.state.DriveNotice
import dev.paperback.android.ui.state.MainScreenViewModel
import uniffi.paperback.DriveBookItem
import uniffi.paperback.ResumeChoice
import java.util.Locale

@Composable
internal fun DriveSyncDialogs(viewModel: MainScreenViewModel) {
	val drive = viewModel.driveSync
	val pending by drive.authorizationPrompt.collectAsStateWithLifecycle()
	val upload by drive.upload.collectAsStateWithLifecycle()
	val notice by drive.notice.collectAsStateWithLifecycle()
	val libraryOpen by drive.libraryOpen.collectAsStateWithLifecycle()
	val books by drive.books.collectAsStateWithLifecycle()
	val context = LocalContext.current
	val authorize = rememberLauncherForActivityResult(ActivityResultContracts.StartIntentSenderForResult()) { result ->
		drive.finishAuthorization(result.data.takeIf { result.resultCode == Activity.RESULT_OK })
	}
	LaunchedEffect(pending) {
		val request = pending ?: return@LaunchedEffect
		drive.takeAuthorizationPrompt()
		authorize.launch(IntentSenderRequest.Builder(request.intentSender).build())
	}
	notice?.let { current ->
		val needsConnection = current == DriveNotice.NeedsConnection
		AlertDialog(
			onDismissRequest = drive::dismissNotice,
			title = {
				Text(
					when (current) {
						DriveNotice.NeedsConnection -> t("No Google account is connected.")
						DriveNotice.AccountPending -> t("The Google Drive account is still being checked. Try again shortly.")
						DriveNotice.FileUnavailable -> t("The book file is not available on this device yet.")
					}
				)
			},
			confirmButton = {
				TextButton(onClick = {
					drive.dismissNotice()
					if (needsConnection) drive.connect()
				}) { Text(if (needsConnection) t("Connect Google account") else t("Close")) }
			},
			dismissButton = {
				if (needsConnection) TextButton(onClick = drive::dismissNotice) { Text(t("Close")) }
			}
		)
	}
	upload?.let { selected ->
		AlertDialog(
			onDismissRequest = { drive.finishUpload(false) },
			title = { Text(t("Upload book file to Google Drive")) },
			text = {
				Text(
					t(
						"Upload the book file to Google Drive (%s)?",
						Formatter
							.formatFileSize(context, selected.bytes.toLong())
					)
				)
			},
			confirmButton = { TextButton(onClick = { drive.finishUpload(true) }) { Text(t("Upload")) } },
			dismissButton = { TextButton(onClick = { drive.finishUpload(false) }) { Text(t("Cancel")) } }
		)
	}
	if (libraryOpen && notice == null) {
		DriveLibraryDialog(viewModel, books)
	}
}

@Composable
private fun DriveLibraryDialog(viewModel: MainScreenViewModel, books: List<DriveBookItem>) {
	val context = LocalContext.current
	var selection by remember { mutableStateOf<DriveBookItem?>(null) }
	var removeCloud by remember { mutableStateOf(false) }
	val drive = viewModel.driveSync
	val book = selection?.let { selected -> books.firstOrNull { it.key == selected.key } }
	if (removeCloud && book != null) {
		AlertDialog(
			onDismissRequest = { removeCloud = false },
			title = { Text(t("Remove book file from Google Drive?")) },
			text = { Text(book.title) },
			confirmButton = {
				TextButton(onClick = {
					drive.removeFile(book.key, cloud = true)
					removeCloud = false
				}) { Text(t("Remove")) }
			},
			dismissButton = { TextButton(onClick = { removeCloud = false }) { Text(t("Cancel")) } }
		)
	} else if (book != null) {
		AlertDialog(
			onDismissRequest = { selection = null },
			title = { Text(book.title) },
			text = {
				Column {
					if (book.choices.isNotEmpty()) {
						Text(t("Choose a place to resume"))
						for (choice in book.choices) {
							TextButton(onClick = {
								drive.showLibrary(false)
								viewModel.resumeDriveBook(book.key, choice.position)
							}) { Text(choiceLabel(choice)) }
						}
					} else {
						TextButton(onClick = {
							drive.showLibrary(false)
							viewModel.resumeDriveBook(book.key)
						}) { Text(t("Resume")) }
					}
					if (book.offlinePath?.let(drive::canRemoveOfflineCopy) == true) {
						TextButton(onClick = { drive.removeFile(book.key, cloud = false) }) { Text(t("Remove offline copy")) }
					}
					if (book.cloudBytes != null) {
						TextButton(onClick = { removeCloud = true }) { Text(t("Remove cloud file")) }
					}
				}
			},
			confirmButton = { TextButton(onClick = { selection = null }) { Text(t("Back")) } }
		)
	} else {
		val bytes = books.sumOf { it.cloudBytes?.toLong() ?: 0 }
		AlertDialog(
			onDismissRequest = { drive.showLibrary(false) },
			title = { Text(t("Cloud books")) },
			text = {
				Column {
					Text(t("Book files on Google Drive: %s", Formatter.formatFileSize(context, bytes)))
					LazyColumn(modifier = Modifier.heightIn(max = 360.dp)) {
						items(books, key = { it.key.id }) { entry ->
							TextButton(onClick = { selection = entry }) {
								val status = when {
									entry.localLocator != null || entry.offlinePath != null -> t("Available offline")
									entry.cloudBytes != null -> t("On Google Drive")
									else -> t("Book data only")
								}
								Text(t("%s, %s", entry.title, status))
							}
						}
					}
				}
			},
			confirmButton = { TextButton(onClick = { drive.showLibrary(false) }) { Text(t("Close")) } }
		)
	}
}

private fun choiceLabel(choice: ResumeChoice): String {
	val ms = choice.position.audioMs?.toLong()
	val position = if (ms == null) {
		t("Character %s", (choice.position.text + 1).toString())
	} else {
		val seconds = ms / 1000
		String.format(Locale.ROOT, "%d:%02d:%02d", seconds / 3600, seconds / 60 % 60, seconds % 60)
	}
	return t("%s, %s", choice.deviceName, position)
}
