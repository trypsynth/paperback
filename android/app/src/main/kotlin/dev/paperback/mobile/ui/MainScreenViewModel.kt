package dev.paperback.mobile.ui

import android.app.Application
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import android.webkit.MimeTypeMap
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import dev.paperback.mobile.tts.TtsManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DocumentSession
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi
import uniffi.paperback.TocEntry
import java.io.File
import java.io.FileOutputStream
import java.util.UUID

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

class MainScreenViewModel(
	application: Application
) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	private val config = ConfigManagerFfi()
	val configManager: ConfigManagerFfi get() = config

	val ttsManager = TtsManager(application, config)
	private val _currentSegmentType = MutableStateFlow(SegmentTypeFfi.PARAGRAPH)
	val currentSegmentType: StateFlow<SegmentTypeFfi> = _currentSegmentType

	private val _ttsPosition = MutableStateFlow(0L)
	val ttsPosition: StateFlow<Long> = _ttsPosition

	private val _currentSegmentText = MutableStateFlow("")
	val currentSegmentText: StateFlow<String> = _currentSegmentText

	private val _sleepTimerRemaining = MutableStateFlow<Int?>(null)
	val sleepTimerRemaining: StateFlow<Int?> = _sleepTimerRemaining

	private val _sleepTimerExpired = MutableSharedFlow<Unit>(extraBufferCapacity = 1)
	val sleepTimerExpired: SharedFlow<Unit> = _sleepTimerExpired

	private var sleepTimerJob: Job? = null

	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState

	private val currentTabs = mutableListOf<DocumentTabState>()
	private var currentActiveIndex = -1
	private var recentDocumentsList = emptyList<RecentDocumentItem>()

	private val _supportedMimeTypes = MutableStateFlow<Array<String>>(arrayOf("*/*"))
	val supportedMimeTypes: StateFlow<Array<String>> = _supportedMimeTypes

	init {
		ttsManager.onUtteranceCompleted = {
			playNextContinuousSegment()
		}
		ttsManager.onPlayCommand = { resumeTts() }
		ttsManager.onPauseCommand = { pauseTts() }
		ttsManager.onNextCommand = { playNextSegment() }
		ttsManager.onPrevCommand = { playPrevSegment() }
		viewModelScope.launch(Dispatchers.IO) {
			config.initialize(context.filesDir.absolutePath + "/config.toml")
			withContext(Dispatchers.Main) {
				ttsManager.loadConfigAndInit()
			}
			_supportedMimeTypes.value = buildSupportedMimeTypes()
			val restorePrevious = config.getAppBool("restore_previous_documents", true)
			val openedUris = if (restorePrevious) config.getOpenedDocuments() else emptyList()
			val activeDocKey = config.getAppString("active_document", "")
			if (openedUris.isNotEmpty()) {
				val restoredTabs = mutableListOf<DocumentTabState>()
				for (uriString in openedUris) {
					val tab = prepareDocumentTabIO(Uri.parse(uriString))
					if (tab != null) {
						restoredTabs.add(tab)
					}
				}
				val initialRecents = getRecentDocumentsListIO()
				withContext(Dispatchers.Main) {
					currentTabs.addAll(restoredTabs)
					recentDocumentsList = initialRecents
					if (currentTabs.isNotEmpty()) {
						val matchingIndex = currentTabs.indexOfFirst { it.docKey == activeDocKey }
						currentActiveIndex = if (matchingIndex != -1) matchingIndex else 0
					} else {
						currentActiveIndex = -1
					}
					_uiState.value = MainScreenUiState.Success(
						tabs = currentTabs.toList(),
						activeTabIndex = currentActiveIndex,
						recentDocuments = recentDocumentsList
					)
					currentTabs.getOrNull(currentActiveIndex)?.let {
						_ttsPosition.value = it.savedPosition
						refreshSegmentPreview()
					}
				}
			} else {
				val initialRecents = getRecentDocumentsListIO()
				withContext(Dispatchers.Main) {
					recentDocumentsList = initialRecents
					_uiState.value = MainScreenUiState.Success(
						tabs = currentTabs.toList(),
						activeTabIndex = currentActiveIndex,
						recentDocuments = recentDocumentsList
					)
				}
			}
		}
	}

	private fun buildSupportedMimeTypes(): Array<String> {
		val extensions = config.getSupportedExtensions()
		val mimeMap = MimeTypeMap.getSingleton()
		val mimes = mutableSetOf<String>()
		for (ext in extensions) {
			val mime: String? = mimeMap.getMimeTypeFromExtension(ext)
			if (mime != null) {
				mimes.add(mime)
			}
			when (ext.lowercase()) {
				"epub" -> mimes.add("application/epub+zip")
				"fb2" -> mimes.add("application/x-fictionbook+xml")
				"md" -> mimes.add("text/markdown")
				"chm" -> mimes.add("application/vnd.ms-htmlhelp")
				"opf" -> mimes.add("application/oebps-package+xml")
				"fodp" -> mimes.add("application/vnd.oasis.opendocument.presentation")
				"fodt" -> mimes.add("application/vnd.oasis.opendocument.text")
				"zip" -> mimes.add("application/zip")
				"rtf" -> mimes.add("application/rtf")
				"pdf" -> mimes.add("application/pdf")
				"txt" -> mimes.add("text/plain")
				"xml" -> {
					mimes.add("application/xml")
					mimes.add("text/xml")
				}
				"html" -> mimes.add("text/html")
				"doc" -> mimes.add("application/msword")
				"docx" -> mimes.add("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
				"docm" -> mimes.add("application/vnd.ms-word.document.macroEnabled.12")
				"odt" -> mimes.add("application/vnd.oasis.opendocument.text")
				"odp" -> mimes.add("application/vnd.oasis.opendocument.presentation")
				"pptx" -> mimes.add("application/vnd.openxmlformats-officedocument.presentationml.presentation")
				"mobi" -> mimes.add("application/x-mobipocket-ebook")
			}
		}
		return if (mimes.isEmpty()) arrayOf("*/*") else mimes.toTypedArray()
	}

	private suspend fun updateRecentDocuments() {
		val updatedList = getRecentDocumentsListIO()
		withContext(Dispatchers.Main) {
			recentDocumentsList = updatedList
		}
	}

	private suspend fun getRecentDocumentsListIO(): List<RecentDocumentItem> =
		withContext(Dispatchers.IO) {
			val recents = config.getRecentDocuments()
			val opened = config.getOpenedDocuments().toSet()
			recents.map { uriString ->
				val uri = Uri.parse(uriString)
				var displayName = uri.lastPathSegment ?: uriString
				var isMissing = false
				try {
					context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
						if (cursor.moveToFirst()) {
							val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
							if (nameIndex != -1) displayName = cursor.getString(nameIndex)
						} else {
							isMissing = true
						}
					} ?: run { isMissing = true }
					if (!isMissing) {
						context.contentResolver.openAssetFileDescriptor(uri, "r")?.close()
					}
				} catch (e: Exception) {
					isMissing = true
				}
				RecentDocumentItem(uriString, displayName, opened.contains(uriString), isMissing)
			}
		}

	fun removeRecentDocument(uriString: String) {
		viewModelScope.launch(Dispatchers.IO) {
			config.removeDocumentHistory(uriString)
			config.flush()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
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
			loadDocument(uri, true)
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
					if (currentActiveIndex != -1) {
						val activeKey = currentTabs[currentActiveIndex].docKey
						viewModelScope.launch(Dispatchers.IO) {
							config.setAppString("active_document", activeKey)
						}
					}
					_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
				}
			}
		}
	}

	fun setActiveTab(index: Int) {
		if (index in currentTabs.indices && index != currentActiveIndex) {
			currentActiveIndex = index
			viewModelScope.launch(Dispatchers.IO) {
				config.setAppString("active_document", currentTabs[index].docKey)
				config.flush()
			}
			_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
			_ttsPosition.value = currentTabs[index].savedPosition
			refreshSegmentPreview()
		}
	}

	fun savePosition(
		session: DocumentSession,
		documentUri: String,
		scrollIndex: Int
	) {
		val position = session.positionFromLine((scrollIndex + 1).toLong())
		_ttsPosition.value = position
		viewModelScope.launch(Dispatchers.IO) {
			config.setDocumentPosition(documentUri, position)
			config.flush()
		}
	}

	override fun onCleared() {
		super.onCleared()
		ttsManager.shutdown()
		Thread {
			try {
				config.flush()
			} catch (_: Exception) {
			}
		}.start()
	}

	private suspend fun prepareDocumentTabIO(uri: Uri): DocumentTabState? =
		withContext(Dispatchers.IO) {
			try {
				val inputStream = context.contentResolver.openInputStream(uri) ?: return@withContext null
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
				config.associateUriWithLocalFile(uri.toString(), tempFile.absolutePath)
				val docKey = config.getDocKey(uri.toString())
				val savedPosition = config.getDocumentPosition(uri.toString())
				val session = DocumentSession.newFfi(tempFile.absolutePath, "", "")
				val initialScrollIndex = if (savedPosition > 0L) {
					(session.lineFromPosition(savedPosition) - 1L).toInt().coerceAtLeast(0)
				} else {
					0
				}
				DocumentTabState(
					session = session,
					title = session.title().ifBlank { displayName },
					author = session.author(),
					fileName = displayName,
					lineCount = session.lineCount(),
					toc = session.getToc(),
					documentUri = uri.toString(),
					docKey = docKey,
					initialScrollIndex = initialScrollIndex,
					savedPosition = savedPosition
				)
			} catch (e: Exception) {
				null
			}
		}

	private suspend fun loadDocument(
		uri: Uri,
		makeActive: Boolean
	) = withContext(Dispatchers.IO) {
		if (currentTabs.isEmpty()) {
			_uiState.value = MainScreenUiState.Loading
		}
		val tabState = prepareDocumentTabIO(uri)
		if (tabState == null) {
			withContext(Dispatchers.Main) {
				_uiState.value = MainScreenUiState.Error("Failed to open file")
				if (currentTabs.isNotEmpty()) {
					_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
				}
			}
			return@withContext
		}
		val recentDocsUpdated = getRecentDocumentsListIO()
		val activeDocKey = config.getAppString("active_document", "")
		withContext(Dispatchers.Main) {
			recentDocumentsList = recentDocsUpdated
			val existingIndex = currentTabs.indexOfFirst { it.docKey == tabState.docKey }
			if (existingIndex != -1) {
				val oldTab = currentTabs[existingIndex]
				if (oldTab.documentUri != uri.toString()) {
					viewModelScope.launch(Dispatchers.IO) {
						config.removeOpenedDocument(oldTab.documentUri)
						config.addOpenedDocument(uri.toString())
						config.flush()
					}
					currentTabs[existingIndex] = tabState
				}
				if (makeActive) {
					currentActiveIndex = existingIndex
					viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", tabState.docKey) }
				} else if (activeDocKey == tabState.docKey && !makeActive) {
					currentActiveIndex = existingIndex
				}
			} else {
				currentTabs.add(tabState)
				if (makeActive) {
					currentActiveIndex = currentTabs.size - 1
					viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", tabState.docKey) }
				} else if (activeDocKey == tabState.docKey) {
					currentActiveIndex = currentTabs.size - 1
					viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", tabState.docKey) }
				} else if (currentActiveIndex == -1) {
					currentActiveIndex = 0
				}
			}
			_uiState.value =
				MainScreenUiState.Success(tabs = currentTabs.toList(), activeTabIndex = currentActiveIndex, recentDocumentsList)
			if (makeActive) {
				_ttsPosition.value = tabState.savedPosition
				refreshSegmentPreview()
			}
		}
	}

	fun setSegmentType(type: SegmentTypeFfi) {
		_currentSegmentType.value = type
	}

	fun togglePlayPause() {
		if (ttsManager.isSpeaking.value) {
			pauseTts()
		} else {
			speakCurrentSegment()
		}
	}

	private fun saveTtsPositionToConfig(pos: Long) {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val docUri = state.activeTab?.documentUri ?: return
		viewModelScope.launch(Dispatchers.IO) {
			config.setDocumentPosition(docUri, pos)
			config.flush()
		}
	}

	fun refreshSegmentPreview() {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		_currentSegmentText.value = if (segment.text.isNotBlank()) {
			segment.text
		} else {
			tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT).text
		}
	}

	private fun speakCurrentSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				ttsManager.speak(segment.text)
			} else {
				playNextSegment()
			}
		}
	}

	fun playNextSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, _currentSegmentType.value, SegmentDirectionFfi.NEXT)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				ttsManager.speak(segment.text)
			}
		}
	}

	fun playNextContinuousSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment =
				tab.session.getTextSegment(
					_ttsPosition.value,
					SegmentTypeFfi.PARAGRAPH,
					SegmentDirectionFfi.NEXT
				)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				ttsManager.speak(segment.text)
			}
		}
	}

	fun playPrevSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, _currentSegmentType.value, SegmentDirectionFfi.PREVIOUS)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				ttsManager.speak(segment.text)
			}
		}
	}

	fun pauseTts() {
		ttsManager.stop()
	}

	fun setSleepTimer(minutes: Int) {
		sleepTimerJob?.cancel()
		sleepTimerJob = viewModelScope.launch {
			var remaining = minutes * 60
			_sleepTimerRemaining.value = remaining
			while (remaining > 0) {
				delay(1000)
				remaining--
				_sleepTimerRemaining.value = remaining
			}
			_sleepTimerRemaining.value = null
			pauseTts()
			_sleepTimerExpired.emit(Unit)
		}
	}

	fun cancelSleepTimer() {
		sleepTimerJob?.cancel()
		sleepTimerJob = null
		_sleepTimerRemaining.value = null
	}

	fun navigateByType(type: SegmentTypeFfi, direction: SegmentDirectionFfi) {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, type, direction)
		if (segment.text.isNotBlank()) {
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = segment.text
			saveTtsPositionToConfig(segment.startPos)
			ttsManager.speak(segment.text)
		}
	}

	fun resumeTts() {
		speakCurrentSegment()
	}

	fun updateTtsPosition(pos: Long) {
		_ttsPosition.value = pos
	}

	fun seekToPercent(percent: Int) {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		val pos = tab.session.positionFromPercentFfi(percent)
		_ttsPosition.value = pos
		saveTtsPositionToConfig(pos)
	}
}
