package dev.paperback.android.ui.state

import uniffi.paperback.DocumentSession
import uniffi.paperback.TocEntry

data class DocumentTabState(
	val session: DocumentSession,
	val title: String,
	val author: String,
	val fileName: String,
	val lineCount: Long,
	val toc: List<TocEntry>,
	val documentUri: String,
	val docKey: String,
	val initialScrollIndex: Int = 0,
	val savedPosition: Long = 0L,
	/** True when read-aloud should play this document's recorded narration instead of TTS. */
	val hasAudio: Boolean = false,
	/** True when the text spine is only there to anchor audio, so there is nothing to navigate
	 * but the recording itself (a zip that is a bundle of narration files, say). */
	val isAudioOnly: Boolean = false
)

data class RecentDocumentItem(
	val uri: String,
	val displayName: String,
	val isOpen: Boolean,
	val isMissing: Boolean = false
)

sealed class MainScreenUiState {
	object Idle : MainScreenUiState()

	object Loading : MainScreenUiState()

	data class Success(
		val tabs: List<DocumentTabState>,
		val activeTabIndex: Int,
		val recentDocuments: List<RecentDocumentItem> = emptyList()
	) : MainScreenUiState() {
		val activeTab: DocumentTabState? get() = tabs.getOrNull(activeTabIndex)
	}

	data class Error(
		val message: String
	) : MainScreenUiState()
}

/**
 * The tab the reader is looking at, for the callers that hold the state in its sealed form:
 * null until a document is open, and for every state but [MainScreenUiState.Success].
 */
val MainScreenUiState.activeTab: DocumentTabState?
	get() = (this as? MainScreenUiState.Success)?.activeTab

/**
 * The index in the list of the line numbered [line]. Line numbers count from one and list indices
 * from zero, and a document position before the first line reports line zero, which would
 * otherwise index off the front of the list.
 */
fun lineIndexFor(line: Long): Int = (line - 1).toInt().coerceAtLeast(0)
