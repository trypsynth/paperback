package dev.paperback.mobile.ui

import android.app.Application
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DocumentSession
import uniffi.paperback.TocEntry
import java.io.File
import java.io.FileOutputStream
import java.util.UUID

data class DocumentTabState(
	val session: DocumentSession,
	val title: String,
	val author: String,
	val lineCount: Long,
	val toc: List<TocEntry>,
	val documentUri: String,
	val initialScrollIndex: Int = 0
)

data class RecentDocumentItem(
	val uri: String,
	val displayName: String,
	val isOpen: Boolean
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

class MainScreenViewModel(
	application: Application
) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	private val config = ConfigManagerFfi().also {
		it.initialize(context.filesDir.absolutePath + "/config.toml")
	}

	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState

	private val currentTabs = mutableListOf<DocumentTabState>()
	private var currentActiveIndex = -1
	private var recentDocumentsList = emptyList<RecentDocumentItem>()

	init {
		viewModelScope.launch {
			val openedUris = config.getOpenedDocuments()
			if (openedUris.isNotEmpty()) {
				openedUris.forEach { uriString ->
					val savedPosition = config.getDocumentPosition(uriString)
					loadDocument(Uri.parse(uriString), savedPosition)
				}
			}
			updateRecentDocuments()
			
			if (currentTabs.isEmpty()) {
				_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
			}
		}
	}

	private fun updateRecentDocuments() {
		val recents = config.getRecentDocuments()
		val opened = config.getOpenedDocuments().toSet()
		recentDocumentsList = recents.map { uriString ->
			val uri = Uri.parse(uriString)
			var displayName = uri.lastPathSegment ?: uriString
			try {
				context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
					if (cursor.moveToFirst()) {
						val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
						if (nameIndex != -1) displayName = cursor.getString(nameIndex)
					}
				}
			} catch (e: Exception) {
				// Ignore
			}
			RecentDocumentItem(uriString, displayName, opened.contains(uriString))
		}
	}

	fun removeRecentDocument(uriString: String) {
		viewModelScope.launch(Dispatchers.IO) {
			config.removeDocumentHistory(uriString)
			config.flush()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				if (currentTabs.isEmpty()) {
					_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
				} else {
					_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
				}
			}
		}
	}

	fun openDocument(uri: Uri) {
		val uriString = uri.toString()
		viewModelScope.launch(Dispatchers.IO) {
			try {
				context.contentResolver.takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
			} catch (_: SecurityException) {
			}
			config.addRecentDocument(uriString)
			config.addOpenedDocument(uriString)
			config.flush()
			val savedPosition = config.getDocumentPosition(uriString)
			loadDocument(uri, savedPosition)
		}
	}

	fun closeTab(index: Int) {
		if (index in currentTabs.indices) {
			val closedTab = currentTabs.removeAt(index)
			viewModelScope.launch(Dispatchers.IO) {
				config.removeOpenedDocument(closedTab.documentUri)
				config.setDocumentOpened(closedTab.documentUri, false)
				config.flush()
				updateRecentDocuments()
				withContext(Dispatchers.Main) {
					currentActiveIndex = if (currentTabs.isEmpty()) -1 else currentActiveIndex.coerceIn(0, currentTabs.size - 1)
					_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
				}
			}
		}
	}

	fun setActiveTab(index: Int) {
		if (index in currentTabs.indices && index != currentActiveIndex) {
			currentActiveIndex = index
			_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
		}
	}

	fun savePosition(
		session: DocumentSession,
		documentUri: String,
		scrollIndex: Int
	) {
		val position = session.positionFromLine((scrollIndex + 1).toLong())
		viewModelScope.launch(Dispatchers.IO) {
			config.setDocumentPosition(documentUri, position)
			config.flush()
		}
	}

	override fun onCleared() {
		config.flush()
	}

	private suspend fun loadDocument(
		uri: Uri,
		savedPosition: Long
	) = withContext(Dispatchers.IO) {
		if (currentTabs.isEmpty()) {
			_uiState.value = MainScreenUiState.Loading
		}
		try {
			val inputStream = context.contentResolver.openInputStream(uri) ?: run {
				_uiState.value = MainScreenUiState.Error("Failed to open file")
				return@withContext
			}
			
			var displayName = ""
			context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
				if (cursor.moveToFirst()) {
					val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
					if (nameIndex != -1) displayName = cursor.getString(nameIndex)
				}
			}
			
			val ext = displayName.substringAfterLast('.', "epub").lowercase()
			val tempFile = File(context.cacheDir, "doc_${UUID.randomUUID()}.$ext")
			FileOutputStream(tempFile).use { inputStream.copyTo(it) }
			inputStream.close()

			val session = DocumentSession.newFfi(tempFile.absolutePath, "", "")
			val initialScrollIndex = if (savedPosition > 0L) {
				(session.lineFromPosition(savedPosition) - 1L).toInt().coerceAtLeast(0)
			} else {
				0
			}

			val tabState = DocumentTabState(
				session = session,
				title = session.title().ifBlank { displayName },
				author = session.author(),
				lineCount = session.lineCount(),
				toc = session.getToc(),
				documentUri = uri.toString(),
				initialScrollIndex = initialScrollIndex
			)

			// Check if already open
			val existingIndex = currentTabs.indexOfFirst { it.documentUri == tabState.documentUri }
			if (existingIndex != -1) {
				currentActiveIndex = existingIndex
			} else {
				currentTabs.add(tabState)
				currentActiveIndex = currentTabs.size - 1
			}

			updateRecentDocuments()
			_uiState.value =
				MainScreenUiState.Success(tabs = currentTabs.toList(), activeTabIndex = currentActiveIndex, recentDocumentsList)
		} catch (e: Exception) {
			_uiState.value = MainScreenUiState.Error(e.message ?: "Unknown error")
			if (currentTabs.isNotEmpty()) {
				_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
			}
		}
	}
}
