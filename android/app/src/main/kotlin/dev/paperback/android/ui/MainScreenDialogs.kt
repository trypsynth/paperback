package dev.paperback.android.ui

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.DocumentInfoDialog
import dev.paperback.android.ui.dialogs.PasswordDialog
import dev.paperback.android.ui.dialogs.SleepTimerDialog
import dev.paperback.android.ui.dialogs.WordCountDialog

/**
 * The dialogs that only report on, or set a timer over, whatever document is open: they need the
 * open document and nothing else from the reading view, so they live here rather than in the
 * middle of `MainScreen`'s layout. Each reads its own open flag, so the caller just places this
 * once and lets the view model decide what is showing.
 */
@Composable
fun DocumentToolDialogs(
	docState: DocumentTabState?,
	viewModel: MainScreenViewModel
) {
	val wordCountOpen by viewModel.wordCountDialog.isOpen.collectAsStateWithLifecycle()
	val documentInfoOpen by viewModel.documentInfoDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerOpen by viewModel.sleepTimerDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerRemaining by viewModel.sleepTimerRemaining.collectAsStateWithLifecycle()

	if (wordCountOpen && docState != null) {
		val stats = remember(docState.session) { docState.session.getStatsFfi() }
		WordCountDialog(
			stats = stats,
			onDismiss = { viewModel.wordCountDialog.close() }
		)
	}
	if (documentInfoOpen && docState != null) {
		val stats = remember(docState.session) { docState.session.getStatsFfi() }
		DocumentInfoDialog(
			docState = docState,
			stats = stats,
			onDismiss = { viewModel.documentInfoDialog.close() }
		)
	}
	if (sleepTimerOpen) {
		SleepTimerDialog(
			remainingSeconds = sleepTimerRemaining,
			onSetTimer = { viewModel.setSleepTimer(it) },
			onCancelTimer = { viewModel.cancelSleepTimer() },
			onDismiss = { viewModel.sleepTimerDialog.close() }
		)
	}
}

/**
 * The two prompts opening a document can raise on its own: a password for an encrypted file, and
 * the offer to import a `.paperback` sidecar found next to it. Both are driven entirely by the
 * view model, so unlike the tool dialogs above they need nothing from the caller at all.
 */
@Composable
fun DocumentPromptDialogs(viewModel: MainScreenViewModel) {
	val passwordPromptUri by viewModel.passwordPromptUri.collectAsStateWithLifecycle()
	val importPromptPath by viewModel.importPromptPath.collectAsStateWithLifecycle()

	if (passwordPromptUri != null) {
		PasswordDialog(
			onConfirm = { viewModel.submitPassword(it) },
			onDismiss = { viewModel.cancelPasswordPrompt() }
		)
	}
	if (importPromptPath != null) {
		AlertDialog(
			onDismissRequest = { viewModel.cancelImportSettings() },
			modifier = Modifier.semantics { paneTitle = t("Import document data") },
			// TRANSLATORS: Title of the dialog offering to import a document's saved settings/bookmarks found alongside it
			title = { Text(t("Import document data")) },
			// TRANSLATORS: Body text of the dialog offering to import a found .paperback settings file for the current document
			text = { Text(t("A .paperback file was found for this document. Would you like to import it?")) },
			confirmButton = {
				TextButton(onClick = { viewModel.confirmImportSettings() }) {
					// TRANSLATORS: Confirm button to proceed with importing the found document settings
					Text(t("Import"))
				}
			},
			dismissButton = {
				TextButton(onClick = { viewModel.cancelImportSettings() }) {
					// TRANSLATORS: Button to decline importing the found document settings
					Text(t("Cancel"))
				}
			}
		)
	}
}
