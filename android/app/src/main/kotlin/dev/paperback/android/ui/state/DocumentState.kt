package dev.paperback.android.ui.state

import uniffi.paperback.DocumentSession
import uniffi.paperback.PageLayout
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
	val isAudioOnly: Boolean = false,
	/** The copy that is parsed, which a content URI's document is read from again when its page layout changes. */
	val localPath: String = "",
	val hasPageLayout: Boolean = false,
	/** Null follows the app's page layout. */
	val ownPageLayout: PageLayout? = null
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

/** Offsets shift when a document is read again with another layout, so the place is carried over as the same share of the text. */
fun positionAfterReread(position: Long, oldLength: Long, newLength: Long): Long {
	if (oldLength <= 0L) return 0L
	return (position.toDouble() * newLength / oldLength).toLong().coerceIn(0L, newLength.coerceAtLeast(0L))
}
