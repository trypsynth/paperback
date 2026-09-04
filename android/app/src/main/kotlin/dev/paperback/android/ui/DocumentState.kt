package dev.paperback.android.ui

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

/**
 * The table of contents screen's own state: which rows are expanded, and which row the reader is
 * currently inside. One object rather than two flows so a recomposition can never pair a fresh
 * active row with a stale expansion set and scroll to the wrong place.
 */
data class TocUiState(
	val expandedIndices: Set<Int> = emptySet(),
	val activeIndex: Int? = null
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
