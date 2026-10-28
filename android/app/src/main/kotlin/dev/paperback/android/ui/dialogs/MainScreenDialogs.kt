package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
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
import dev.paperback.android.ui.state.lineIndexFor
import kotlinx.coroutines.launch
import uniffi.paperback.ExportFormat

@Composable
fun DocumentToolDialogs(
	docState: DocumentTabState?,
	viewModel: MainScreenViewModel,
	onExportFormatChosen: (ExportFormat, String) -> Unit
) {
	val wordCountOpen by viewModel.navigation.wordCountDialog.isOpen.collectAsStateWithLifecycle()
	val documentInfoOpen by viewModel.navigation.documentInfoDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerOpen by viewModel.navigation.sleepTimerDialog.isOpen.collectAsStateWithLifecycle()
	val sleepTimerRemaining by viewModel.reading.sleepTimer.remaining.collectAsStateWithLifecycle()
	val exportOpen by viewModel.navigation.exportDocumentDialog.isOpen.collectAsStateWithLifecycle()

	if (wordCountOpen && docState != null) {
		val stats = remember(docState.session) { docState.session.getStatsFfi() }
		WordCountDialog(
			stats = stats,
			onDismiss = { viewModel.navigation.wordCountDialog.close() }
		)
	}
	if (documentInfoOpen && docState != null) {
		val stats = remember(docState.session) { docState.session.getStatsFfi() }
		val progressPercent = remember(docState.session) { viewModel.reading.progressPercent() }
		DocumentInfoDialog(
			docState = docState,
			stats = stats,
			progressPercent = progressPercent,
			onDismiss = { viewModel.navigation.documentInfoDialog.close() }
		)
	}
	if (sleepTimerOpen) {
		SleepTimerDialog(
			remainingSeconds = sleepTimerRemaining,
			onSetTimer = { viewModel.reading.sleepTimer.start(it) },
			onCancelTimer = { viewModel.reading.sleepTimer.cancel() },
			onDismiss = { viewModel.navigation.sleepTimerDialog.close() }
		)
	}
	if (exportOpen) {
		if (docState != null) {
			ExportDocumentDialog(
				supportedFormats = docState.session.getSupportedExportFormatsFfi(),
				onFormatSelected = { format ->
					viewModel.navigation.exportDocumentDialog.close()
					onExportFormatChosen(format, "${docState.fileName.substringBeforeLast(".")}.${format.fileExtension()}")
				},
				onDismiss = { viewModel.navigation.exportDocumentDialog.close() }
			)
		} else {
			// The last tab closed while the dialog was open; closing from an effect keeps the state change out of the composition reading it.
			LaunchedEffect(Unit) { viewModel.navigation.exportDocumentDialog.close() }
		}
	}
}

internal fun ExportFormat.fileExtension(): String = when (this) {
	ExportFormat.TEXT -> "txt"
	ExportFormat.HTML -> "html"
	ExportFormat.MARKDOWN -> "md"
}

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
	val goToOpen by viewModel.navigation.showGoToDialog.collectAsStateWithLifecycle()
	val findOpen by viewModel.navigation.findDialog.isOpen.collectAsStateWithLifecycle()
	val activeQuery by viewModel.search.query.collectAsStateWithLifecycle()
	val scope = rememberCoroutineScope()
	if (goToOpen) {
		GoToDialog(
			docState = docState,
			onDismiss = { viewModel.navigation.closeGoToDialog() },
			initialMode = goToInitialMode,
			onGoTo = onGoToLine,
			onSeekPercent = { viewModel.reading.seekAudioToPercent(it) }
		)
	}
	// Focus stays in the Find dialog while it is open, so the last match's line only takes screen reader focus once it closes.
	var lastMatchLine by remember { mutableStateOf<Int?>(null) }
	if (findOpen) {
		FindDialog(
			configManager = viewModel.configManager,
			initialQuery = activeQuery ?: "",
			onDismiss = {
				viewModel.navigation.findDialog.close()
				lastMatchLine?.let(onFocusLine)
				lastMatchLine = null
			},
			onSearch = { query, options ->
				val wasSpeaking = viewModel.ttsManager.isSpeaking.value
				if (wasSpeaking) {
					viewModel.reading.pauseTts()
				}
				// Searching again for the running query would re-find the match the reader is on, so start one step past it.
				val isSameQuery = viewModel.search.isSameAs(query, options)
				viewModel.search.start(query, options)
				if (!isTextMode) {
					viewModel.reading.setNavUnit(NavUnit.Find)
				}
				val searchPos = if (isTextMode) {
					val nextLineOffset = if (isSameQuery) 2 else 1
					docState.session.positionFromLine((listState.firstVisibleItemIndex + nextLineOffset).toLong())
				} else {
					val currentPos = viewModel.reading.ttsPosition.value
					if (isSameQuery) currentPos + 1L else currentPos
				}
				val res = docState.session.searchFfi(query, searchPos, options)
				if (!res.found) {
					// TRANSLATORS: Announced when stepping to the next/previous Find match runs off the end of the document
					// TRANSLATORS: Announced when a Find query matches nothing anywhere in the document
					viewModel.announceForAccessibility(if (isSameQuery) t("No more matches.") else t("No matches."))
					return@FindDialog
				}
				if (isTextMode) {
					val targetLine = docState.session.lineFromPosition(res.position)
					val targetIndex = lineIndexFor(targetLine)
					lastMatchLine = targetIndex
					scope.launch { listState.scrollToItem(targetIndex) }
				} else {
					viewModel.reading.jumpToFoundPosition(res.position, resume = wasSpeaking)
				}
				if (isTextMode || !wasSpeaking) {
					viewModel.reading.announceFoundMatch(res.position)
				}
			}
		)
	}
}

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
