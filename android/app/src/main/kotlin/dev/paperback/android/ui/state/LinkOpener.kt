package dev.paperback.android.ui.state

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.LinkDownload
import uniffi.paperback.LinkDownloadException
import java.io.File

/** Where Open from URL is: asking for the link, waiting on the server, asking about what it sends, or downloading. */
sealed interface LinkStage {
	object Entering : LinkStage

	object Connecting : LinkStage

	data class Confirming(val download: LinkDownload, val warning: String) : LinkStage

	data class Downloading(val download: LinkDownload, val name: String, val fraction: Float?) : LinkStage
}

/** Open from URL, run by the core off the main thread, saving into `folder` and handing the file to `onSaved`. */
class LinkOpener(
	private val scope: CoroutineScope,
	private val folder: () -> File,
	private val onSaved: (File) -> Unit
) {
	private val _stage = MutableStateFlow<LinkStage?>(null)
	val stage: StateFlow<LinkStage?> = _stage.asStateFlow()

	private val _failure = MutableStateFlow<String?>(null)
	val failure: StateFlow<String?> = _failure.asStateFlow()

	private var job: Job? = null

	fun prompt() {
		if (_stage.value == null) _stage.value = LinkStage.Entering
	}

	fun open(link: String) {
		if (_stage.value != LinkStage.Entering) return
		_stage.value = LinkStage.Connecting
		job = scope.launch {
			try {
				val download = withContext(Dispatchers.IO) { LinkDownload(link) }
				// Closed while the server was being asked: what it sent is dropped.
				if (_stage.value != LinkStage.Connecting) return@launch
				val warning = download.info().warning
				if (warning.isEmpty()) save(download) else _stage.value = LinkStage.Confirming(download, warning)
			} catch (error: LinkDownloadException) {
				fail(error)
			}
		}
	}

	fun confirm() {
		val confirming = _stage.value as? LinkStage.Confirming ?: return
		job = scope.launch { save(confirming.download) }
	}

	/** Back to the link, after declining a warning. */
	fun decline() {
		if (_stage.value is LinkStage.Confirming) _stage.value = LinkStage.Entering
	}

	fun close() {
		(_stage.value as? LinkStage.Downloading)?.download?.cancel()
		_stage.value = null
		job = null
	}

	fun dismissFailure() {
		_failure.value = null
	}

	private suspend fun save(download: LinkDownload) {
		val name = download.info().fileName
		_stage.value = LinkStage.Downloading(download, name, null)
		val watcher = scope.launch {
			while (isActive) {
				val progress = download.progress()
				val fraction = if (progress.total > 0) (progress.done.toFloat() / progress.total).coerceIn(0f, 1f) else null
				if (_stage.value is LinkStage.Downloading) _stage.value = LinkStage.Downloading(download, name, fraction)
				delay(250)
			}
		}
		try {
			val path = withContext(Dispatchers.IO) { download.save(folder().absolutePath) }
			_stage.value = null
			onSaved(File(path))
		} catch (_: LinkDownloadException.Cancelled) {
		} catch (error: LinkDownloadException) {
			fail(error)
		} finally {
			watcher.cancel()
		}
	}

	private fun fail(error: LinkDownloadException) {
		if (_stage.value == null) return
		_stage.value = LinkStage.Entering
		_failure.value = (error as? LinkDownloadException.Failed)?.message ?: error.toString()
	}
}
