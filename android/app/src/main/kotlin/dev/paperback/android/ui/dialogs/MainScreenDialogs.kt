package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.DocumentInfoDialog
import dev.paperback.android.ui.dialogs.ExportDocumentDialog
import dev.paperback.android.ui.dialogs.FindDialog
import dev.paperback.android.ui.dialogs.GoToDialog
import dev.paperback.android.ui.dialogs.PasswordDialog
import dev.paperback.android.ui.dialogs.SleepTimerDialog
import dev.paperback.android.ui.dialogs.WordCountDialog
import dev.paperback.android.ui.state.DocumentTabState
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.NavUnit
import kotlinx.coroutines.launch
import uniffi.paperback.ExportFormat

/**
 * The dialogs that only report on, or set a timer over, whatever document is open: they need the
 * open document and nothing else from the reading view, so they live here rather than in the
 * middle of `MainScreen`'s layout. Each reads its own open flag, so the caller just places this
 * once and lets the view model decide what is showing.
 */
@Composable
fun DocumentToolDialogs(
	docState: DocumentTabState?,
	viewModel: MainScreenViewModel,
	onExportFormatChosen: (ExportFormat, String) -> Unit
) {
	val wordCountOpen by viewModel.wordCountDialog.isOpen.collectAsStateWithLifecycle()
	val documentInfoOpen by viewModel.documentInfoDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerOpen by viewModel.sleepTimerDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerRemaining by viewModel.sleepTimer.remaining.collectAsStateWithLifecycle()
	val exportOpen by viewModel.exportDocumentDialog.isOpen.collectAsStateWithLifecycle()

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
			onSetTimer = { viewModel.sleepTimer.start(it) },
			onCancelTimer = { viewModel.sleepTimer.cancel() },
			onDismiss = { viewModel.sleepTimerDialog.close() }
		)
	}
	if (exportOpen) {
		if (docState != null) {
			ExportDocumentDialog(
				supportedFormats = docState.session.getSupportedExportFormatsFfi(),
				onFormatSelected = { format ->
					viewModel.exportDocumentDialog.close()
					onExportFormatChosen(format, "${docState.fileName.substringBeforeLast(".")}.${format.fileExtension()}")
				},
				onDismiss = { viewModel.exportDocumentDialog.close() }
			)
		} else {
			// Nothing to export: the last tab closed while the dialog was open. Closing from an
			// effect rather than straight from composition keeps the state change out of the
			// composition that is reading it.
			LaunchedEffect(Unit) { viewModel.exportDocumentDialog.close() }
		}
	}
}

/** The extension an exported document is offered under, matching the format it is rendered in. */
internal fun ExportFormat.fileExtension(): String =
	when (this) {
		ExportFormat.TEXT -> "txt"
		ExportFormat.HTML -> "html"
		ExportFormat.MARKDOWN -> "md"
	}

/**
 * The dialogs that act on the open document's text. Unlike the tool dialogs these need the
 * reading view itself: where a search starts, and where landing on a match puts the reader, both
 * depend on which mode is showing and where the list is scrolled to.
 */
@Composable
fun DocumentTextDialogs(
	docState: DocumentTabState,
	viewModel: MainScreenViewModel,
	isTextMode: Boolean,
	listState: LazyListState,
	goToInitialMode: String,
	onGoToLine: (Int) -> Unit,
	onFocusLine: (Int) -> Unit
) {
	val goToOpen by viewModel.showGoToDialog.collectAsStateWithLifecycle()
	val findOpen by viewModel.findDialog.isOpen.collectAsStateWithLifecycle()
	val activeQuery by viewModel.search.query.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	if (goToOpen) {
		GoToDialog(
			docState = docState,
			onDismiss = { viewModel.closeGoToDialog() },
			initialMode = goToInitialMode,
			onGoTo = onGoToLine,
			onSeekPercent = { viewModel.seekAudioToPercent(it) }
		)
	}
	if (findOpen) {
		FindDialog(
			configManager = viewModel.configManager,
			initialQuery = activeQuery ?: "",
			onDismiss = { viewModel.findDialog.close() },
			onSearch = { query, options ->
				val wasSpeaking = viewModel.ttsManager.isSpeaking.value
				if (wasSpeaking) {
					viewModel.pauseTts()
				}
				// Searching again for what is already running would just re-find the match the
				// reader is sitting on, so that case starts one step past it.
				val isSameQuery = viewModel.search.isSameAs(query, options)
				viewModel.search.start(query, options)
				if (!isTextMode) {
					viewModel.setNavUnit(NavUnit.Find)
				}
				val searchPos = if (isTextMode) {
					val nextLineOffset = if (isSameQuery) 2 else 1
					docState.session.positionFromLine((listState.firstVisibleItemIndex + nextLineOffset).toLong())
				} else {
					val currentPos = viewModel.ttsPosition.value
					if (isSameQuery) currentPos + 1L else currentPos
				}
				val res = docState.session.searchFfi(query, searchPos, options)
				if (!res.found) return@FindDialog
				if (isTextMode) {
					val targetLine = docState.session.lineFromPosition(res.position)
					val targetIndex = (targetLine - 1).toInt().coerceAtLeast(0)
					scope.launch {
						listState.scrollToItem(targetIndex)
						onFocusLine(targetIndex)
					}
				} else {
					viewModel.jumpToFoundPosition(res.position, resume = wasSpeaking)
				}
			}
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
