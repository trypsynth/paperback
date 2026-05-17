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

sealed class MainScreenUiState {
	object Idle : MainScreenUiState()
	object Loading : MainScreenUiState()
	data class Success(
		val session: DocumentSession,
		val title: String,
		val author: String,
		val lineCount: Long,
		val toc: List<TocEntry>,
		val documentUri: String,
		val initialScrollIndex: Int = 0,
	) : MainScreenUiState()
	data class Error(val message: String) : MainScreenUiState()
}

class MainScreenViewModel(application: Application) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	private val config = ConfigManagerFfi().also {
		it.initialize(context.filesDir.absolutePath + "/config.toml")
	}

	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState

	init {
		val lastUri = config.getRecentDocuments().firstOrNull()
		if (lastUri != null) {
			viewModelScope.launch {
				val savedPosition = config.getDocumentPosition(lastUri)
				loadDocument(Uri.parse(lastUri), savedPosition)
			}
		}
	}

	fun openDocument(uri: Uri) {
		val uriString = uri.toString()
		viewModelScope.launch(Dispatchers.IO) {
			try {
				context.contentResolver.takePersistableUriPermission(
					uri, Intent.FLAG_GRANT_READ_URI_PERMISSION
				)
			} catch (_: SecurityException) {}
			config.addRecentDocument(uriString)
			config.setDocumentPosition(uriString, 0L)
			loadDocument(uri, 0L)
		}
	}

	fun savePosition(session: DocumentSession, documentUri: String, scrollIndex: Int) {
		val position = session.positionFromLine((scrollIndex + 1).toLong())
		config.setDocumentPosition(documentUri, position)
	}

	override fun onCleared() {
		config.flush()
	}

	private suspend fun loadDocument(uri: Uri, savedPosition: Long) = withContext(Dispatchers.IO) {
		_uiState.value = MainScreenUiState.Loading
		try {
			val inputStream = context.contentResolver.openInputStream(uri)
				?: run {
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
			val tempFile = File(context.cacheDir, "doc_${java.util.UUID.randomUUID()}.$ext")
			FileOutputStream(tempFile).use { inputStream.copyTo(it) }
			inputStream.close()
			val session = DocumentSession.newFfi(tempFile.absolutePath, "", "")
			val initialScrollIndex = if (savedPosition > 0L) {
				(session.lineFromPosition(savedPosition) - 1L).toInt().coerceAtLeast(0)
			} else {
				0
			}
			_uiState.value = MainScreenUiState.Success(
				session = session,
				title = session.title(),
				author = session.author(),
				lineCount = session.lineCount(),
				toc = session.getToc(),
				documentUri = uri.toString(),
				initialScrollIndex = initialScrollIndex,
			)
		} catch (e: Exception) {
			_uiState.value = MainScreenUiState.Error(e.message ?: "Unknown error")
		}
	}
}
