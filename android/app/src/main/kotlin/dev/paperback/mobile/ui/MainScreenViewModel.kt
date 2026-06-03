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
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DocumentSession
import uniffi.paperback.HeadingTreeFfi
import uniffi.paperback.LinkListFfi
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi
import java.io.File
import java.io.FileOutputStream
import java.util.UUID

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

	private val _showElementsDialog = MutableStateFlow(false)
	val showElementsDialog: StateFlow<Boolean> = _showElementsDialog

	private val _showFindDialog = MutableStateFlow(false)
	val showFindDialog: StateFlow<Boolean> = _showFindDialog

	private val _showSettingsDialog = MutableStateFlow(false)
	val showSettingsDialog: StateFlow<Boolean> = _showSettingsDialog

	private val _showTocDialog = MutableStateFlow(false)
	val showTocDialog: StateFlow<Boolean> = _showTocDialog

	private val _showGoToDialog = MutableStateFlow(false)
	val showGoToDialog: StateFlow<Boolean> = _showGoToDialog

	private val _goToInitialMode = MutableStateFlow("Line")
	val goToInitialMode: StateFlow<String> = _goToInitialMode

	private val _showWordCountDialog = MutableStateFlow(false)
	val showWordCountDialog: StateFlow<Boolean> = _showWordCountDialog

	private val _showDocumentInfoDialog = MutableStateFlow(false)
	val showDocumentInfoDialog: StateFlow<Boolean> = _showDocumentInfoDialog

	private val _activeSearchQuery = MutableStateFlow<String?>(null)
	val activeSearchQuery: StateFlow<String?> = _activeSearchQuery

	private val _activeSearchOptions = MutableStateFlow<uniffi.paperback.SearchOptionsFfi?>(null)
	val activeSearchOptions: StateFlow<uniffi.paperback.SearchOptionsFfi?> = _activeSearchOptions

	private val _performSearchEvent = MutableSharedFlow<Boolean>(extraBufferCapacity = 1)
	val performSearchEvent: SharedFlow<Boolean> = _performSearchEvent

	fun startSearch(query: String, options: uniffi.paperback.SearchOptionsFfi) {
		_activeSearchQuery.value = query
		_activeSearchOptions.value = options
	}

	fun clearSearch() {
		_activeSearchQuery.value = null
		_activeSearchOptions.value = null
	}

	fun triggerFindNext() {
		_performSearchEvent.tryEmit(true)
	}

	fun triggerFindPrevious() {
		_performSearchEvent.tryEmit(false)
	}

	private val _showSleepTimerDialog = MutableStateFlow(false)
	val showSleepTimerDialog: StateFlow<Boolean> = _showSleepTimerDialog

	private val _currentHeadings = MutableStateFlow<HeadingTreeFfi?>(null)
	val currentHeadings: StateFlow<HeadingTreeFfi?> = _currentHeadings

	private val _currentLinks = MutableStateFlow<LinkListFfi?>(null)
	val currentLinks: StateFlow<LinkListFfi?> = _currentLinks

	private val _passwordPromptUri = MutableStateFlow<Uri?>(null)
	val passwordPromptUri = _passwordPromptUri.asStateFlow()

	private val _showPermissionRationale = MutableStateFlow(false)
	val showPermissionRationale = _showPermissionRationale.asStateFlow()

	private val _importPromptPath = MutableStateFlow<String?>(null)
	val importPromptPath: StateFlow<String?> = _importPromptPath

	fun confirmImportSettings() {
		val path = _importPromptPath.value ?: return
		config.importDocumentSettings(path)

		val state = uiState.value as? MainScreenUiState.Success
		val tab = state?.activeTab
		if (tab != null) {
			val savedPosition = config.getDocumentPosition(tab.documentUri)
			updateTtsPosition(savedPosition)
			refreshSegmentPreview()
		}
		_importPromptPath.value = null
	}

	fun cancelImportSettings() {
		_importPromptPath.value = null
	}

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
					val tab = prepareDocumentTabIO(Uri.parse(uriString), isRestore = true)
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
						updateTtsMetadata()
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

				if (uri.scheme == "content") {
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
				} else {
					val file = File(uri.path ?: uriString)
					displayName = file.name
					isMissing = !file.exists()
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
					if (currentActiveIndex != -1) {
						_ttsPosition.value = currentTabs[currentActiveIndex].savedPosition
						updateTtsMetadata()
						refreshSegmentPreview()
					} else {
						_ttsPosition.value = 0
						_currentSegmentText.value = ""
						updateTtsMetadata()
					}
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
			updateTtsMetadata()
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

	private suspend fun prepareDocumentTabIO(uri: Uri, providedPassword: String? = null, isRestore: Boolean = false): DocumentTabState? =
		withContext(Dispatchers.IO) {
			try {
				val uriString = uri.toString()
				val isContentUri = uri.scheme == "content"
				val absolutePath: String
				val displayName: String

				if (isContentUri) {
					val inputStream = context.contentResolver.openInputStream(uri) ?: return@withContext null
					var name = ""
					context.contentResolver.query(uri, null, null, null, null)?.use { cursor ->
						if (cursor.moveToFirst()) {
							val nameIndex = cursor.getColumnIndex(OpenableColumns.DISPLAY_NAME)
							if (nameIndex != -1) name = cursor.getString(nameIndex)
						}
					}
					displayName = name
					val ext = displayName.substringAfterLast('.', "epub").lowercase()
					val tempDir = File(context.cacheDir, UUID.randomUUID().toString())
					tempDir.mkdirs()
					val tempFile = File(tempDir, displayName.ifBlank { "document.$ext" })
					FileOutputStream(tempFile).use { inputStream.copyTo(it) }
					inputStream.close()
					absolutePath = tempFile.absolutePath
					config.associateUriWithLocalFile(uriString, absolutePath)
				} else {
					absolutePath = uri.path ?: uriString
					val file = File(absolutePath)
					displayName = file.name
					config.associateUriWithLocalFile(uriString, absolutePath)
				}

				val file = File(absolutePath)
				val nameWithoutExtension = file.nameWithoutExtension
				val paperbackPath = File(file.parentFile, "$nameWithoutExtension.paperback").absolutePath

				if (!isRestore && File(paperbackPath).exists()) {
					_importPromptPath.value = absolutePath
				}

				val docKey = config.getDocKey(uriString)
				val savedPosition = config.getDocumentPosition(uriString)
				val password = providedPassword ?: config.getDocumentPassword(uriString)
				val session = DocumentSession.newFfi(absolutePath, password, "")
				if (providedPassword != null) {
					config.setDocumentPassword(uriString, providedPassword)
					config.flush()
				}
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
					documentUri = uriString,
					docKey = docKey,
					initialScrollIndex = initialScrollIndex,
					savedPosition = savedPosition
				)
			} catch (e: Exception) {
				val msg = e.message ?: ""
				if (msg.contains("[password_required]")) {
					withContext(Dispatchers.Main) {
						_passwordPromptUri.value = uri
					}
					return@withContext null
				}
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
				if (uri.scheme == "file" && android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R && !android.os.Environment.isExternalStorageManager()) {
					setShowPermissionRationale(true)
				} else {
					_uiState.value = MainScreenUiState.Error("Failed to open file")
				}
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
				updateTtsMetadata()
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

	fun playNextSegment(speak: Boolean = true, announce: Boolean = false) {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, _currentSegmentType.value, SegmentDirectionFfi.NEXT)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				if (speak) {
					ttsManager.speak(segment.text)
				} else if (announce) {
					announceNavigationCue(segment.text)
				}
			}
		}
	}

	private fun announceNavigationCue(text: String) {
		val cue = text.trim().split("\\s+".toRegex()).take(5).joinToString(" ")
		_accessibilityAnnouncement.tryEmit(cue)
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

	fun playPrevSegment(speak: Boolean = true, announce: Boolean = false) {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, _currentSegmentType.value, SegmentDirectionFfi.PREVIOUS)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				if (speak) {
					ttsManager.speak(segment.text)
				} else if (announce) {
					announceNavigationCue(segment.text)
				}
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

	fun exportCurrentSettings(): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		if (docUri.startsWith("content://")) return false
		val absolutePath = Uri.parse(docUri).path ?: docUri
		val file = File(absolutePath)
		val nameWithoutExtension = file.nameWithoutExtension
		val paperbackPath = File(file.parentFile, "$nameWithoutExtension.paperback").absolutePath
		return try {
			config.exportDocumentSettings(absolutePath, paperbackPath)
			true
		} catch (e: Exception) {
			false
		}
	}

	fun exportSettingsToUri(context: android.content.Context, destUri: android.net.Uri): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		val absolutePath = if (docUri.startsWith("content://")) {
			docUri
		} else {
			Uri.parse(docUri).path ?: docUri
		}

		val tempFile = java.io.File(context.cacheDir, "temp_export.paperback")
		return try {
			config.exportDocumentSettings(absolutePath, tempFile.absolutePath)
			context.contentResolver.openOutputStream(destUri)?.use { out ->
				tempFile.inputStream().use { input ->
					input.copyTo(out)
				}
			}
			true
		} catch (e: Exception) {
			false
		} finally {
			if (tempFile.exists()) tempFile.delete()
		}
	}

	fun importSettingsFromUri(context: android.content.Context, sourceUri: android.net.Uri): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		val absolutePath = if (docUri.startsWith("content://")) {
			docUri
		} else {
			Uri.parse(docUri).path ?: docUri
		}

		val tempFile = java.io.File(context.cacheDir, "temp_import.paperback")
		return try {
			context.contentResolver.openInputStream(sourceUri)?.use { input ->
				tempFile.outputStream().use { out ->
					input.copyTo(out)
				}
			}
			config.importSettingsFromFile(absolutePath, tempFile.absolutePath)

			val savedPosition = config.getDocumentPosition(docUri)
			if (savedPosition > 0L) {
				updateTtsPosition(savedPosition)
			}
			true
		} catch (e: Exception) {
			false
		} finally {
			if (tempFile.exists()) tempFile.delete()
		}
	}

	private fun updateTtsMetadata() {
		if (currentActiveIndex in currentTabs.indices) {
			val tab = currentTabs[currentActiveIndex]
			ttsManager.currentDocumentTitle = tab.title.ifBlank { tab.fileName }
			ttsManager.currentDocumentAuthor = tab.author.ifBlank { "Unknown Author" }
		} else {
			ttsManager.currentDocumentTitle = "Paperback"
			ttsManager.currentDocumentAuthor = "Unknown"
		}
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

	fun openElementsDialog() {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		viewModelScope.launch(Dispatchers.IO) {
			val pos = _ttsPosition.value
			val headings = tab.session.getHeadingTreeFfi(pos)
			val links = tab.session.getLinkListFfi(pos)
			withContext(Dispatchers.Main) {
				_currentHeadings.value = headings
				_currentLinks.value = links
				_showElementsDialog.value = true
			}
		}
	}

	fun closeElementsDialog() {
		_showElementsDialog.value = false
		_currentHeadings.value = null
		_currentLinks.value = null
	}

	fun openFindDialog() {
		_showFindDialog.value = true
	}

	fun closeFindDialog() {
		_showFindDialog.value = false
	}

	fun openSettingsDialog() {
		_showSettingsDialog.value = true
	}

	fun closeSettingsDialog() {
		_showSettingsDialog.value = false
	}

	fun openTocDialog() {
		_showTocDialog.value = true
	}

	fun closeTocDialog() {
		_showTocDialog.value = false
	}

	private val _accessibilityAnnouncement = MutableSharedFlow<String>(extraBufferCapacity = 1)
	val accessibilityAnnouncement: SharedFlow<String> = _accessibilityAnnouncement

	fun announceForAccessibility(message: String) {
		_accessibilityAnnouncement.tryEmit(message)
	}

	fun openGoToDialog(initialMode: String = "Line") {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab
			if (tab != null && initialMode == "Page" && tab.session.pageCountFfi() == 0) {
				announceForAccessibility("This document does not contain pages.")
				return
			}
		}
		_goToInitialMode.value = initialMode
		_showGoToDialog.value = true
	}

	fun closeGoToDialog() {
		_showGoToDialog.value = false
	}

	fun openWordCountDialog() {
		_showWordCountDialog.value = true
	}

	fun closeWordCountDialog() {
		_showWordCountDialog.value = false
	}

	fun openDocumentInfoDialog() {
		_showDocumentInfoDialog.value = true
	}

	fun closeDocumentInfoDialog() {
		_showDocumentInfoDialog.value = false
	}

	fun openSleepTimerDialog() {
		_showSleepTimerDialog.value = true
	}

	fun closeSleepTimerDialog() {
		_showSleepTimerDialog.value = false
	}

	fun submitPassword(password: String) {
		val uri = _passwordPromptUri.value ?: return
		_passwordPromptUri.value = null
		viewModelScope.launch(Dispatchers.IO) {
			if (currentTabs.isEmpty()) {
				_uiState.value = MainScreenUiState.Loading
			}
			val tabState = prepareDocumentTabIO(uri, password)
			if (tabState == null) {
				withContext(Dispatchers.Main) {
					_uiState.value = MainScreenUiState.Error("Failed to open file or incorrect password")
					if (currentTabs.isNotEmpty()) {
						_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
					}
				}
				return@launch
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
					currentActiveIndex = existingIndex
					viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", tabState.docKey) }
				} else {
					currentTabs.add(tabState)
					currentActiveIndex = currentTabs.size - 1
					viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", tabState.docKey) }
				}
				_uiState.value = MainScreenUiState.Success(tabs = currentTabs.toList(), activeTabIndex = currentActiveIndex, recentDocumentsList)
				_ttsPosition.value = tabState.savedPosition
				refreshSegmentPreview()
			}
		}
	}

	fun cancelPasswordPrompt() {
		val uriStr = _passwordPromptUri.value?.toString()
		_passwordPromptUri.value = null

		viewModelScope.launch(Dispatchers.IO) {
			if (uriStr != null) {
				config.removeOpenedDocument(uriStr)
				config.setDocumentOpened(uriStr, false)
				config.flush()
				updateRecentDocuments()
			}
			withContext(Dispatchers.Main) {
				_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
			}
		}
	}

	fun setShowPermissionRationale(show: Boolean) {
		_showPermissionRationale.value = show
	}
}
