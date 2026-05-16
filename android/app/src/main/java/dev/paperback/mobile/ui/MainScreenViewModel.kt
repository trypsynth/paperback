package dev.paperback.mobile.ui

import android.content.Context
import android.net.Uri
import android.provider.OpenableColumns
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
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
		val toc: List<TocEntry>
	) : MainScreenUiState()
	data class Error(val message: String) : MainScreenUiState()
}

class MainScreenViewModel : ViewModel() {
	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState

	fun openDocument(context: Context, uri: Uri) {
		_uiState.value = MainScreenUiState.Loading
		viewModelScope.launch(Dispatchers.IO) {
			try {
				val inputStream = context.contentResolver.openInputStream(uri)
				if (inputStream == null) {
					_uiState.value = MainScreenUiState.Error("Failed to open file")
					return@launch
				}

				var displayName = ""
				context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
					if (cursor.moveToFirst()) {
						val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
						if (nameIndex != -1) {
							displayName = cursor.getString(nameIndex)
						}
					}
				}

				val ext = displayName.substringAfterLast('.', "epub").lowercase()

				val tempFile = File(context.cacheDir, "temp_doc.$ext")
				val outputStream = FileOutputStream(tempFile)
				inputStream.copyTo(outputStream)
				inputStream.close()
				outputStream.close()

				val session = DocumentSession.newFfi(tempFile.absolutePath, "", "")
				
				_uiState.value = MainScreenUiState.Success(
					session = session,
					title = session.title(),
					author = session.author(),
					lineCount = session.lineCount(),
					toc = session.getToc()
				)
			} catch (e: Exception) {
				_uiState.value = MainScreenUiState.Error(e.message ?: "Unknown error")
			}
		}
	}
}
