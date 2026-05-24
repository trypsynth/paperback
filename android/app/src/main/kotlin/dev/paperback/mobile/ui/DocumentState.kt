package dev.paperback.mobile.ui

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
	val savedPosition: Long = 0L
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
