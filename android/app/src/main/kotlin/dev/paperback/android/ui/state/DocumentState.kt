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
	val hasAudio: Boolean = false,
	/** The text spine only anchors the audio, e.g. a zip of narration files. */
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

	data class Error(val message: String) : MainScreenUiState()
}

val MainScreenUiState.activeTab: DocumentTabState?
	get() = (this as? MainScreenUiState.Success)?.activeTab

/** A position before the first line reports line zero, which would otherwise index off the front of the list. */
fun lineIndexFor(line: Long): Int = (line - 1).toInt().coerceAtLeast(0)
