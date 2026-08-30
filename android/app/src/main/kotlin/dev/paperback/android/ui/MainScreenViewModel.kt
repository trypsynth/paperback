package dev.paperback.android.ui

import android.app.Application
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import android.webkit.MimeTypeMap
import android.widget.Toast
import androidx.core.net.toUri
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import dev.paperback.android.assetLocaleTags
import dev.paperback.android.bestLocaleMatch
import dev.paperback.android.t
import dev.paperback.android.tts.DaisyAudioPlayer
import dev.paperback.android.tts.TtsManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.DocumentSession
import uniffi.paperback.HeadingTreeFfi
import uniffi.paperback.LinkListFfi
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi
import uniffi.paperback.TextSegmentFfi
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.security.MessageDigest
import java.util.Locale

private const val AUDIO_SEEK_AMOUNT_KEY = "audio_seek_amount_seconds"
private const val DEFAULT_AUDIO_SEEK_SECONDS = 10

class MainScreenViewModel(
	application: Application
) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	private val config = ConfigManagerFfi()
	val configManager: ConfigManagerFfi get() = config

	val ttsManager = TtsManager(application, config)

	// Narrates DAISY audiobooks' recorded audio in place of synthesized TTS (see
	// DocumentTabState.hasAudio). A single instance re-attached to whichever tab is active.
	private val daisyAudioPlayer = DaisyAudioPlayer(application)

	// The document URI daisyAudioPlayer is currently attached to.
	private var daisyAttachedDocumentUri: String? = null

	// Whether playback controls should route to daisyAudioPlayer instead of ttsManager.
	// Centralized so every dispatch site checks the exact same condition rather than each
	// re-deriving "the active tab, if any, has audio" on its own.
	private val activeTabHasAudio: Boolean
		get() = (uiState.value as? MainScreenUiState.Success)?.activeTab?.hasAudio == true

	private val _currentNavUnit = MutableStateFlow<NavUnit>(NavUnit.Segment(SegmentTypeFfi.PARAGRAPH))
	val currentNavUnit: StateFlow<NavUnit> = _currentNavUnit.asStateFlow()

	// The source whose position was last announced after an audio seek, so a seek that stays in
	// the same file doesn't repeat its name every time.
	private var lastAnnouncedAudioSource: Int? = null

	private val _ttsPosition = MutableStateFlow(0L)
	val ttsPosition: StateFlow<Long> = _ttsPosition.asStateFlow()

	private val _currentSegmentText = MutableStateFlow("")
	val currentSegmentText: StateFlow<String> = _currentSegmentText.asStateFlow()

	private val _sleepTimerRemaining = MutableStateFlow<Int?>(null)
	val sleepTimerRemaining: StateFlow<Int?> = _sleepTimerRemaining.asStateFlow()

	private val _sleepTimerExpired = MutableSharedFlow<Unit>(extraBufferCapacity = 1)
	val sleepTimerExpired: SharedFlow<Unit> = _sleepTimerExpired.asSharedFlow()

	private var sleepTimerJob: Job? = null

	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState.asStateFlow()

	private val currentTabs = mutableListOf<DocumentTabState>()
	private var currentActiveIndex = -1
	private var recentDocumentsList = emptyList<RecentDocumentItem>()

	// The active tab's document URI as of the last `updateTtsMetadata` call, so that function can
	// tell a real tab switch (clear the active search) apart from a no-op call, e.g. closing a
	// background tab that leaves the active one unchanged.
	private var lastMetadataDocumentUri: String? = null

	private fun emitTabsState() {
		_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
	}

	private fun persistActiveDocument(docKey: String) {
		viewModelScope.launch(Dispatchers.IO) { config.setAppString("active_document", docKey) }
	}

	private val _supportedMimeTypes = MutableStateFlow<Array<String>>(arrayOf("*/*"))
	val supportedMimeTypes: StateFlow<Array<String>> = _supportedMimeTypes.asStateFlow()

	private val elementsDialogState = DialogState()
	val showElementsDialog: StateFlow<Boolean> = elementsDialogState.isOpen

	val findDialog = DialogState()

	val settingsRequest = ScreenRequest()

	private val _restorePreviousDocuments = MutableStateFlow(config.getAppBool("restore_previous_documents", true))
	val restorePreviousDocuments: StateFlow<Boolean> = _restorePreviousDocuments.asStateFlow()

	private val _useInAppFileBrowser = MutableStateFlow(config.getAppBool("use_in_app_file_browser", false))
	val useInAppFileBrowser: StateFlow<Boolean> = _useInAppFileBrowser.asStateFlow()

	private val _swipeUpMovesForward = MutableStateFlow(config.getAppBool("swipe_up_moves_forward", true))
	val swipeUpMovesForward: StateFlow<Boolean> = _swipeUpMovesForward.asStateFlow()

	// Spacing and alignment share the desktop's config keys and value meanings (spacing 0/1/2,
	// alignment 0 leading, 1 center, 2 trailing, 3 justify) so a document reads the same way on
	// every platform. Text size does not: the desktop stores an absolute point size, while this
	// scales whatever size the system font setting is already asking for.
	private val _textScalePercent = MutableStateFlow(config.getAppInt("text_scale_percent", 100))
	val textScalePercent: StateFlow<Int> = _textScalePercent.asStateFlow()

	private val _lineSpacing = MutableStateFlow(config.getAppInt("line_spacing", 0))
	val lineSpacing: StateFlow<Int> = _lineSpacing.asStateFlow()

	private val _paragraphSpacing = MutableStateFlow(config.getAppInt("paragraph_spacing", 0))
	val paragraphSpacing: StateFlow<Int> = _paragraphSpacing.asStateFlow()

	private val _textAlignment = MutableStateFlow(config.getAppInt("text_alignment", 0))
	val textAlignment: StateFlow<Int> = _textAlignment.asStateFlow()

	val tocRequest = ScreenRequest()

	private val _tocState = MutableStateFlow(TocUiState())
	val tocState: StateFlow<TocUiState> = _tocState.asStateFlow()

	fun toggleTocExpanded(index: Int) {
		val expanded = _tocState.value.expandedIndices
		_tocState.value = _tocState.value.copy(
			expandedIndices = if (expanded.contains(index)) expanded - index else expanded + index
		)
	}

	/**
	 * Points the table of contents at wherever the reader currently is: the nearest entry at or
	 * before the reading position becomes the active one, and its ancestors are expanded so it is
	 * actually on screen when the list opens.
	 */
	fun prepareToc() {
		val toc = (uiState.value as? MainScreenUiState.Success)?.activeTab?.toc.orEmpty()
		if (toc.isEmpty()) {
			_tocState.value = _tocState.value.copy(activeIndex = null)
			return
		}
		var activeIndex = 0
		var bestDistance = Long.MAX_VALUE
		val currentPos = _ttsPosition.value
		for (i in toc.indices) {
			if (toc[i].position <= currentPos) {
				val distance = currentPos - toc[i].position
				if (distance < bestDistance) {
					bestDistance = distance
					activeIndex = i
				}
			}
		}
		val toExpand = mutableSetOf<Int>()
		var currentLevel = toc[activeIndex].level
		for (i in activeIndex - 1 downTo 0) {
			if (toc[i].level < currentLevel) {
				toExpand.add(i)
				currentLevel = toc[i].level
				if (currentLevel == 0) break
			}
		}
		_tocState.value = TocUiState(
			expandedIndices = _tocState.value.expandedIndices + toExpand,
			activeIndex = activeIndex
		)
	}

	private val goToDialogState = DialogState()
	val showGoToDialog: StateFlow<Boolean> = goToDialogState.isOpen

	private val _goToInitialMode = MutableStateFlow("Line")
	val goToInitialMode: StateFlow<String> = _goToInitialMode.asStateFlow()

	val wordCountDialog = DialogState()

	val documentInfoDialog = DialogState()

	private val _activeSearchQuery = MutableStateFlow<String?>(null)
	val activeSearchQuery: StateFlow<String?> = _activeSearchQuery.asStateFlow()

	private val _activeSearchOptions = MutableStateFlow<uniffi.paperback.SearchOptionsFfi?>(null)
	val activeSearchOptions: StateFlow<uniffi.paperback.SearchOptionsFfi?> = _activeSearchOptions.asStateFlow()

	private val _performSearchEvent = MutableSharedFlow<Boolean>(extraBufferCapacity = 1)
	val performSearchEvent: SharedFlow<Boolean> = _performSearchEvent.asSharedFlow()

	fun startSearch(
		query: String,
		options: uniffi.paperback.SearchOptionsFfi
	) {
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

	val sleepTimerDialog = DialogState()

	private val _currentHeadings = MutableStateFlow<HeadingTreeFfi?>(null)
	val currentHeadings: StateFlow<HeadingTreeFfi?> = _currentHeadings.asStateFlow()

	private val _currentLinks = MutableStateFlow<LinkListFfi?>(null)
	val currentLinks: StateFlow<LinkListFfi?> = _currentLinks.asStateFlow()

	private val _passwordPromptUri = MutableStateFlow<Uri?>(null)
	val passwordPromptUri = _passwordPromptUri.asStateFlow()

	val permissionRationaleDialog = DialogState()

	private val _importPromptPath = MutableStateFlow<String?>(null)
	val importPromptPath: StateFlow<String?> = _importPromptPath.asStateFlow()

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
		// Continuous-reading auto-advance: once an utterance finishes, keep moving forward
		// paragraph by paragraph on its own. That's the right behavior for ordinary reading, but
		// wrong while browsing Find matches — landing on a match should just speak its context
		// and then wait, not silently keep auto-advancing past it before the next button press.
		ttsManager.onSegmentTransition = {
			if (_currentNavUnit.value !is NavUnit.Find) {
				transitionToNextContinuousSegment()
			}
		}
		ttsManager.onUtteranceCompleted = {
			if (_currentNavUnit.value !is NavUnit.Find) {
				playNextContinuousSegment()
			}
		}
		ttsManager.onPlayCommand = { resumeTts() }
		ttsManager.onPauseCommand = { pauseTts() }
		ttsManager.onNextCommand = { playNextSegment() }
		ttsManager.onPrevCommand = { playPrevSegment() }
		daisyAudioPlayer.onPlaybackStateChanged = { isPlaying ->
			ttsManager.setExternalPlaybackState(isPlaying)
			if (!isPlaying) persistDaisyAudioPosition()
		}
		daisyAudioPlayer.onRelativeSeekLanded = { elapsedMs ->
			announceAudioSeek(elapsedMs)
		}
		daisyAudioPlayer.onClipChanged = { position ->
			// Same reasoning as the TTS auto-advance callbacks above: natural playback tracking
			// would otherwise keep dragging the tracked position forward (mid-clip, off the exact
			// match) while browsing Find, racing with the next Find Previous/Next press. This
			// doesn't extend to persistDaisyAudioPosition() below: unlike desktop's on-close save,
			// Android can kill this process with no lifecycle callback at all, so the raw audio
			// time still needs saving on every clip change (not just pause/stop) regardless of
			// nav unit, or a kill mid-Find would resume from before the jump on relaunch.
			if (_currentNavUnit.value !is NavUnit.Find) {
				_ttsPosition.value = position
				refreshSegmentPreview()
				saveTtsPositionToConfig(position)
			}
			persistDaisyAudioPosition()
		}
		viewModelScope.launch(Dispatchers.IO) {
			config.initialize(context.filesDir.absolutePath + "/config.toml")
			purgeLegacyDocumentCache()
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
					val tab = prepareDocumentTabIO(uriString.toUri(), isRestore = true)
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
					emitTabsState()
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
					emitTabsState()
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
				val uri = uriString.toUri()
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
					} catch (_: Exception) {
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
				emitTabsState()
			}
		}
	}

	fun locateRecentDocument(
		oldUriString: String,
		newUri: Uri
	) {
		val newUriString = newUri.toString()
		viewModelScope.launch(Dispatchers.IO) {
			try {
				context.contentResolver.takePersistableUriPermission(newUri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
			} catch (_: SecurityException) {
			}
			config.renameDocumentPath(oldUriString, newUriString)
			config.flush()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				emitTabsState()
			}
		}
	}

	fun openDocument(
		uri: Uri,
		track: Boolean = true
	) {
		val uriString = uri.toString()
		viewModelScope.launch(Dispatchers.IO) {
			try {
				context.contentResolver.takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
			} catch (_: SecurityException) {
			}
			if (track) {
				config.addRecentDocument(uriString)
				config.addOpenedDocument(uriString)
				config.flush()
			}
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
				documentCacheDir(closedTab.documentUri).deleteRecursively()
				updateRecentDocuments()
				withContext(Dispatchers.Main) {
					currentActiveIndex = if (currentTabs.isEmpty()) -1 else currentActiveIndex.coerceIn(0, currentTabs.size - 1)
					if (currentActiveIndex != -1) {
						persistActiveDocument(currentTabs[currentActiveIndex].docKey)
					}
					emitTabsState()
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
			emitTabsState()
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
		if (daisyAttachedDocumentUri != documentUri && ttsManager.isPaused.value) {
			ttsManager.stop()
		}
	}

	override fun onCleared() {
		super.onCleared()
		detachDaisyAudio()
		daisyAudioPlayer.shutdown()
		ttsManager.shutdown()
		Thread {
			try {
				config.flush()
			} catch (_: Exception) {
			}
		}.start()
	}

	/**
	 * Extraction directory for [uriString], stable across opens. Naming it with a fresh UUID
	 * meant every pick -- and every tab restore at launch -- left behind another full copy of
	 * the document, with nothing that ever deleted it.
	 */
	private fun documentCacheDir(uriString: String): File {
		val digest = MessageDigest.getInstance("SHA-256").digest(uriString.toByteArray())
		val name = digest.take(16).joinToString("") { "%02x".format(it) }
		return File(context.cacheDir, "$DOCUMENT_CACHE_DIR/$name")
	}

	/** Clears extraction directories left by builds that named them with a raw UUID. */
	private fun purgeLegacyDocumentCache() {
		try {
			context.cacheDir.listFiles()?.forEach {
				if (it.isDirectory && UUID_DIR_REGEX.matches(it.name)) {
					it.deleteRecursively()
				}
			}
		} catch (_: Exception) {
		}
	}

	private suspend fun prepareDocumentTabIO(
		uri: Uri,
		providedPassword: String? = null,
		isRestore: Boolean = false
	): DocumentTabState? =
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
					val tempDir = documentCacheDir(uriString)
					tempDir.deleteRecursively()
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
				val session = DocumentSession.newFfi(absolutePath, password, "", false)
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
					savedPosition = savedPosition,
					hasAudio = session.hasAudioFfi(),
					isAudioOnly = session.isAudioOnlyFfi()
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
				if (uri.scheme == "file" && needsAllFilesAccessPermission()) {
					permissionRationaleDialog.open()
				} else {
					_uiState.value = MainScreenUiState.Error("Failed to open file")
				}
				if (currentTabs.isNotEmpty()) {
					emitTabsState()
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
					persistActiveDocument(tabState.docKey)
				} else if (activeDocKey == tabState.docKey && !makeActive) {
					currentActiveIndex = existingIndex
				}
			} else {
				currentTabs.add(tabState)
				if (makeActive) {
					currentActiveIndex = currentTabs.size - 1
					persistActiveDocument(tabState.docKey)
				} else if (activeDocKey == tabState.docKey) {
					currentActiveIndex = currentTabs.size - 1
					persistActiveDocument(tabState.docKey)
				} else if (currentActiveIndex == -1) {
					currentActiveIndex = 0
				}
			}
			emitTabsState()
			if (makeActive) {
				_ttsPosition.value = tabState.savedPosition
				updateTtsMetadata()
				refreshSegmentPreview()
			}
		}
	}

	fun setNavUnit(unit: NavUnit) {
		_currentNavUnit.value = unit
		// Seek amounts are a global preference rather than per-document, and share desktop's
		// setting so the two agree about what "forward" means.
		if (unit is NavUnit.Time) {
			viewModelScope.launch(Dispatchers.IO) {
				config.setAppInt(AUDIO_SEEK_AMOUNT_KEY, unit.seconds)
				config.flush()
			}
		}
	}

	/** The navigation units `tab` can offer. A document whose text spine is only there to anchor
	 * audio has nothing to step through but the recording itself, so it gets seek amounts plus
	 * Section (each underlying audio file is its own section); a DAISY book with real prose gets
	 * seek amounts and every supported segment type, seek amounts first. */
	fun navUnitsFor(tab: DocumentTabState): List<NavUnit> {
		val segments = tab.session.getSupportedSegmentTypesFfi().map { NavUnit.Segment(it) }
		if (!tab.hasAudio) return segments
		val times = AUDIO_SEEK_AMOUNTS_SECONDS.map { NavUnit.Time(it) }
		if (tab.isAudioOnly) {
			return times + segments.filter { it.type == SegmentTypeFfi.SECTION }
		}
		return times + segments
	}

	/** Falls back to a unit the newly active document actually supports, preferring the saved
	 * seek amount where seek amounts are on offer. */
	fun ensureNavUnitSupported(units: List<NavUnit>) {
		if (units.isEmpty() || units.contains(_currentNavUnit.value)) return
		val savedSeconds = config.getAppInt(AUDIO_SEEK_AMOUNT_KEY, DEFAULT_AUDIO_SEEK_SECONDS)
		_currentNavUnit.value = units.firstOrNull { it == NavUnit.Time(savedSeconds) } ?: units.first()
	}

	private fun navSegmentType(): SegmentTypeFfi =
		(_currentNavUnit.value as? NavUnit.Segment)?.type ?: SegmentTypeFfi.PARAGRAPH

	/** Handles previous/next for a document being navigated by elapsed time rather than by text
	 * unit. False when that isn't what's happening, leaving the ordinary text path to run. */
	private fun seekAudioByNavUnit(forward: Boolean): Boolean {
		val unit = _currentNavUnit.value
		if (unit !is NavUnit.Time || !activeTabHasAudio) return false
		val deltaMs = unit.seconds * 1000L
		daisyAudioPlayer.seekRelativeMs(if (forward) deltaMs else -deltaMs)
		return true
	}

	/** Handles previous/next for a document being navigated by Find match instead of by text
	 * unit or elapsed time. False when that isn't what's happening, leaving the ordinary text
	 * path to run. Always returns true once "Find" is the active unit, even with no query or no
	 * more matches, since there is nothing else for prev/next to fall back to in that case. */
	private fun navigateByFind(
		forward: Boolean,
		speak: Boolean,
		announce: Boolean
	): Boolean {
		if (_currentNavUnit.value !is NavUnit.Find) return false
		val query = _activeSearchQuery.value
		val options = _activeSearchOptions.value
		val state = uiState.value as? MainScreenUiState.Success ?: return true
		val tab = state.activeTab ?: return true
		if (query == null || options == null) return true
		// Forward search is inclusive of the start position, so searching from the current
		// match's own start would just re-find it; nudge past it first. Backward search is
		// already exclusive of the start position, so it needs no such adjustment.
		val searchPos = if (forward) _ttsPosition.value + 1L else _ttsPosition.value
		val res = tab.session.searchFfi(query, searchPos, options.copy(forward = forward))
		if (!res.found) {
			// TRANSLATORS: Announced when stepping to the next/previous Find match runs off the end of the document
			_accessibilityAnnouncement.tryEmit(t("No more matches."))
			return true
		}
		_ttsPosition.value = res.position
		val text = displayTextFor(tab, tab.session.getTextSegment(res.position, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT))
		_currentSegmentText.value = text
		saveTtsPositionToConfig(res.position)
		if (tab.hasAudio) {
			daisyAudioPlayer.seekToPosition(res.position)
			if (speak) {
				daisyAudioPlayer.play()
			} else if (announce) {
				announceNavigationCue(text)
			}
		} else if (speak) {
			ttsManager.stop()
			ttsManager.speak(text)
		} else if (announce) {
			if (ttsManager.isPaused.value) {
				ttsManager.stop()
			}
			announceNavigationCue(text)
		}
		return true
	}

	/** Speaks where an audio seek landed. An audiobook that is a bundle of narration files has
	 * no meaningful document-wide elapsed time (its clips carry placeholder durations), so its
	 * position reads as an offset into the file now playing, named whenever the file changes. */
	private fun announceAudioSeek(elapsedMs: Long) {
		val tab = (uiState.value as? MainScreenUiState.Success)?.activeTab ?: return
		val cursor = tab.session.audioCursorAtElapsedFfi(elapsedMs)
		if (!cursor.found) return
		val clip = tab.session.audioClipFfi(cursor.clipIndex)
		if (!clip.found) return
		val time = formatDuration(if (tab.isAudioOnly) cursor.seekMs else elapsedMs)
		val fileChanged = lastAnnouncedAudioSource != clip.source
		lastAnnouncedAudioSource = clip.source
		val sectionTitle = tab.toc
			.lastOrNull { it.position <= clip.start }
			?.title
			.orEmpty()
		_accessibilityAnnouncement.tryEmit(
			if (fileChanged && sectionTitle.isNotBlank()) "$sectionTitle, $time" else time
		)
	}

	fun togglePlayPause() {
		if (activeTabHasAudio) {
			daisyAudioPlayer.toggle()
			return
		}
		if (ttsManager.isSpeaking.value) {
			pauseTts()
		} else if (ttsManager.isPaused.value) {
			resumeTts()
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
		val current = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		_currentSegmentText.value = displayTextFor(tab, current).ifBlank {
			displayTextFor(tab, tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT))
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
				ttsManager.stop()
				ttsManager.speak(segment.text)
				precacheNextContinuousSegment()
			} else {
				playNextSegment()
			}
		}
	}

	/** Seeks daisyAudioPlayer to `segment`'s start, then either resumes playback there or just
	 * announces `announceText`. */
	private fun navigateDaisyAudioToSegment(
		segment: TextSegmentFfi,
		announceText: String,
		speak: Boolean,
		announce: Boolean
	) {
		daisyAudioPlayer.seekToPosition(segment.startPos)
		if (speak) {
			daisyAudioPlayer.play()
		} else if (announce) {
			announceNavigationCue(announceText)
		}
	}

	/** A segment's own text, falling back to its enclosing section's TOC title when blank — the
	 * case for a plain-audio DAISY section, whose buffer content is just a placeholder space. */
	private fun displayTextFor(tab: DocumentTabState, segment: TextSegmentFfi): String =
		segment.text.ifBlank {
			tab.toc.lastOrNull { it.position <= segment.startPos }?.title.orEmpty()
		}

	/** Jumps straight to `pos` (a freshly found Find match) and, if `resume` says the reader was
	 * already going, speaks/plays from exactly there. Deliberately does not go through
	 * `updateTtsPosition`/`speakCurrentSegment`, which re-derive the *enclosing paragraph* of a
	 * position and snap to its start — fine for ordinary navigation, but it would silently move
	 * a Find jump off the match it just found and back to that paragraph's beginning. */
	fun jumpToFoundPosition(
		pos: Long,
		resume: Boolean
	) {
		val tab = (uiState.value as? MainScreenUiState.Success)?.activeTab ?: return
		_ttsPosition.value = pos
		val segment = tab.session.getTextSegment(pos, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val text = displayTextFor(tab, segment)
		_currentSegmentText.value = text
		saveTtsPositionToConfig(pos)
		if (tab.hasAudio) {
			daisyAudioPlayer.seekToPosition(pos)
			if (resume) {
				daisyAudioPlayer.play()
			}
			return
		}
		if (resume) {
			ttsManager.stop()
			ttsManager.speak(text)
		} else if (ttsManager.isPaused.value) {
			// Was paused mid-utterance elsewhere; clear that stale state so a later resume
			// doesn't play the old paragraph's audio instead of the new position.
			ttsManager.stop()
		}
	}

	fun playNextSegment(
		speak: Boolean = true,
		announce: Boolean = false
	) {
		if (navigateByFind(forward = true, speak = speak, announce = announce)) return
		if (seekAudioByNavUnit(forward = true)) return
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.NEXT)
			if (segment.found) {
				val text = displayTextFor(tab, segment)
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = text
				saveTtsPositionToConfig(segment.startPos)
				if (tab.hasAudio) {
					navigateDaisyAudioToSegment(segment, text, speak, announce)
					return
				}
				if (speak) {
					ttsManager.speak(text)
					precacheNextContinuousSegment()
				} else {
					if (ttsManager.isPaused.value) {
						ttsManager.stop()
					}
					if (announce) {
						announceNavigationCue(text)
					}
				}
			}
		}
	}

	private fun announceNavigationCue(text: String) {
		val cue = text
			.trim()
			.split(WHITESPACE_REGEX)
			.take(5)
			.joinToString(" ")
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
				precacheNextContinuousSegment()
			}
		}
	}

	fun transitionToNextContinuousSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
			if (segment.text.isNotBlank()) {
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = segment.text
				saveTtsPositionToConfig(segment.startPos)
				precacheNextContinuousSegment()
			}
		}
	}

	fun precacheNextContinuousSegment() {
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
			if (segment.text.isNotBlank()) {
				ttsManager.precache(segment.text)
			}
		}
	}

	fun playPrevSegment(
		speak: Boolean = true,
		announce: Boolean = false
	) {
		if (navigateByFind(forward = false, speak = speak, announce = announce)) return
		if (seekAudioByNavUnit(forward = false)) return
		val state = uiState.value
		if (state is MainScreenUiState.Success) {
			val tab = state.activeTab ?: return
			val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.PREVIOUS)
			if (segment.found) {
				val text = displayTextFor(tab, segment)
				_ttsPosition.value = segment.startPos
				_currentSegmentText.value = text
				saveTtsPositionToConfig(segment.startPos)
				if (tab.hasAudio) {
					navigateDaisyAudioToSegment(segment, text, speak, announce)
					return
				}
				if (speak) {
					ttsManager.speak(text)
					precacheNextContinuousSegment()
				} else {
					if (ttsManager.isPaused.value) {
						ttsManager.stop()
					}
					if (announce) {
						announceNavigationCue(text)
					}
				}
			}
		}
	}

	fun pauseTts() {
		if (activeTabHasAudio) {
			daisyAudioPlayer.pause()
			return
		}
		ttsManager.pause()
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

	fun navigateByType(
		type: SegmentTypeFfi,
		direction: SegmentDirectionFfi
	) {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, type, direction)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateDaisyAudioToSegment(segment, text, speak = true, announce = false)
				return
			}
			ttsManager.stop()
			ttsManager.speak(text)
			precacheNextContinuousSegment()
		}
	}

	fun resumeTts() {
		if (activeTabHasAudio) {
			daisyAudioPlayer.play()
			return
		}
		if (ttsManager.isPaused.value) {
			ttsManager.resume()
		} else {
			speakCurrentSegment()
		}
	}

	fun exportCurrentSettings(): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		if (docUri.startsWith("content://")) return false
		val absolutePath = docUri.toUri().path ?: docUri
		val file = File(absolutePath)
		val nameWithoutExtension = file.nameWithoutExtension
		val paperbackPath = File(file.parentFile, "$nameWithoutExtension.paperback").absolutePath
		return try {
			config.exportDocumentSettings(absolutePath, paperbackPath)
			true
		} catch (_: Exception) {
			false
		}
	}

	fun exportSettingsToUri(
		context: Context,
		destUri: Uri
	): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		val absolutePath = if (docUri.startsWith("content://")) {
			docUri
		} else {
			docUri.toUri().path ?: docUri
		}

		val tempFile = File(context.cacheDir, "temp_export.paperback")
		return try {
			config.exportDocumentSettings(absolutePath, tempFile.absolutePath)
			context.contentResolver.openOutputStream(destUri)?.use { out ->
				tempFile.inputStream().use { input ->
					input.copyTo(out)
				}
			}
			true
		} catch (_: Exception) {
			false
		} finally {
			if (tempFile.exists()) tempFile.delete()
		}
	}

	fun exportDocumentToUri(
		context: Context,
		destUri: Uri,
		format: uniffi.paperback.ExportFormat
	): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false

		return try {
			val content = tab.session.renderExportFfi(format)
			context.contentResolver.openOutputStream(destUri)?.use { out ->
				out.write(content.toByteArray(Charsets.UTF_8))
			}
			true
		} catch (_: Exception) {
			false
		}
	}

	fun importSettingsFromUri(
		context: Context,
		sourceUri: Uri
	): Boolean {
		val state = uiState.value as? MainScreenUiState.Success ?: return false
		val tab = state.activeTab ?: return false
		val docUri = tab.documentUri
		val absolutePath = if (docUri.startsWith("content://")) {
			docUri
		} else {
			docUri.toUri().path ?: docUri
		}

		val tempFile = File(context.cacheDir, "temp_import.paperback")
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
		} catch (_: Exception) {
			false
		} finally {
			if (tempFile.exists()) tempFile.delete()
		}
	}

	private fun updateTtsMetadata() {
		// A search's matches belong to the document it ran against; carrying it over to whatever
		// tab becomes active next (including the Find nav unit it puts on the slider) is never
		// correct, and is actively wrong for an audio-only tab, which has no real searchable text.
		// This function also runs when closing a tab that wasn't the active one, though, which
		// doesn't change what's active at all — guard on the active document's identity actually
		// changing so that case doesn't wipe an in-progress search on the tab still being read.
		val activeDocumentUri = currentTabs.getOrNull(currentActiveIndex)?.documentUri
		if (activeDocumentUri != lastMetadataDocumentUri) {
			lastMetadataDocumentUri = activeDocumentUri
			clearSearch()
		}
		if (currentActiveIndex in currentTabs.indices) {
			val tab = currentTabs[currentActiveIndex]
			ttsManager.currentDocumentTitle = tab.title.ifBlank { tab.fileName }
			ttsManager.currentDocumentAuthor = tab.author.ifBlank { "Unknown Author" }
			attachDaisyAudioForActiveTab(tab)
		} else {
			ttsManager.currentDocumentTitle = "Paperback"
			ttsManager.currentDocumentAuthor = "Unknown"
			detachDaisyAudio()
		}
	}

	/** Switches daisyAudioPlayer to narrate `tab`, resuming from its saved audio position (or
	 * saved text position, absent that). No-op if `tab` has no audio or is already attached. */
	private fun attachDaisyAudioForActiveTab(tab: DocumentTabState) {
		if (daisyAttachedDocumentUri == tab.documentUri) return
		detachDaisyAudio()
		if (!tab.hasAudio) return
		daisyAudioPlayer.attach(tab.session, tab.docKey)
		daisyAttachedDocumentUri = tab.documentUri
		val savedAudioMs = config.getDocumentAudioTimeFfi(tab.documentUri)
		if (savedAudioMs >= 0) {
			daisyAudioPlayer.seekToMs(savedAudioMs)
		} else {
			daisyAudioPlayer.seekToPosition(tab.savedPosition)
		}
	}

	/** Persists wherever daisyAudioPlayer currently is (if it's attached to anything) and
	 * detaches it, ahead of switching to a different document or the app going away. */
	private fun detachDaisyAudio() {
		persistDaisyAudioPosition()
		daisyAudioPlayer.detach()
		daisyAttachedDocumentUri = null
		lastAnnouncedAudioSource = null
	}

	private fun persistDaisyAudioPosition() {
		val uri = daisyAttachedDocumentUri ?: return
		val ms = daisyAudioPlayer.resumePointMs() ?: return
		viewModelScope.launch(Dispatchers.IO) {
			config.setDocumentAudioTimeFfi(uri, ms)
			config.flush()
		}
	}

	fun updateTtsPosition(pos: Long) {
		_ttsPosition.value = pos
		refreshSegmentPreview()
		saveTtsPositionToConfig(pos)
		if (activeTabHasAudio) {
			daisyAudioPlayer.seekToPosition(pos)
			return
		}
		if (ttsManager.isSpeaking.value) {
			speakCurrentSegment()
		} else if (ttsManager.isPaused.value) {
			ttsManager.stop()
		}
	}

	fun seekToPercent(percent: Int) {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		val pos = tab.session.positionFromPercent(percent)
		updateTtsPosition(pos)
	}

	fun openWordCountDialog() {
		// Nothing to count in an audio-only book. The menu hides the entry for one; this keeps
		// the Ctrl+W shortcut from opening a dialog full of zeroes anyway.
		val state = uiState.value as? MainScreenUiState.Success ?: return
		if (state.activeTab?.isAudioOnly == true) return
		wordCountDialog.open()
	}

	fun openElementsDialog() {
		val state = uiState.value as? MainScreenUiState.Success ?: return
		val tab = state.activeTab ?: return
		// An audio-only book has no text spine, so both tabs of the dialog would come up empty.
		// The menu hides the entry for one; this keeps the F7 shortcut from opening it anyway.
		if (tab.isAudioOnly) return
		viewModelScope.launch(Dispatchers.IO) {
			val pos = _ttsPosition.value
			val headings = tab.session.getHeadingTreeFfi(pos)
			val links = tab.session.getLinkListFfi(pos)
			withContext(Dispatchers.Main) {
				_currentHeadings.value = headings
				_currentLinks.value = links
				elementsDialogState.open()
			}
		}
	}

	fun closeElementsDialog() {
		elementsDialogState.close()
		_currentHeadings.value = null
		_currentLinks.value = null
	}

	fun setRestorePreviousDocuments(value: Boolean) {
		_restorePreviousDocuments.value = value
		config.setAppBool("restore_previous_documents", value)
		config.flush()
	}

	fun setUseInAppFileBrowser(value: Boolean) {
		_useInAppFileBrowser.value = value
		config.setAppBool("use_in_app_file_browser", value)
		config.flush()
	}

	fun setSwipeUpMovesForward(value: Boolean) {
		_swipeUpMovesForward.value = value
		config.setAppBool("swipe_up_moves_forward", value)
		config.flush()
	}

	fun setTextScalePercent(value: Int) {
		val clamped = value.coerceIn(MIN_TEXT_SCALE_PERCENT, MAX_TEXT_SCALE_PERCENT)
		_textScalePercent.value = clamped
		config.setAppInt("text_scale_percent", clamped)
		config.flush()
	}

	fun setLineSpacing(value: Int) {
		_lineSpacing.value = value
		config.setAppInt("line_spacing", value)
		config.flush()
	}

	fun setParagraphSpacing(value: Int) {
		_paragraphSpacing.value = value
		config.setAppInt("paragraph_spacing", value)
		config.flush()
	}

	fun setTextAlignment(value: Int) {
		_textAlignment.value = value
		config.setAppInt("text_alignment", value)
		config.flush()
	}

	private val _accessibilityAnnouncement = MutableSharedFlow<String>(extraBufferCapacity = 1)
	val accessibilityAnnouncement: SharedFlow<String> = _accessibilityAnnouncement.asSharedFlow()

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
		goToDialogState.open()
	}

	fun closeGoToDialog() {
		goToDialogState.close()
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
						emitTabsState()
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
					persistActiveDocument(tabState.docKey)
				} else {
					currentTabs.add(tabState)
					currentActiveIndex = currentTabs.size - 1
					persistActiveDocument(tabState.docKey)
				}
				emitTabsState()
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
				emitTabsState()
			}
		}
	}

	fun openHelpDocument() {
		// Matched against the readmes actually shipped rather than a hardcoded language list,
		// which drifts out of sync, and by the same rule as the string catalogue: the files are
		// named for the po files ("readme-zh_CN.html"), not for the bare language code a device
		// reports, so asking for readme-zh.html only ever found the English fallback.
		val lang = bestLocaleMatch(assetLocaleTags(context, "readmes", "readme-", ".html"), Locale.getDefault())
		viewModelScope.launch(Dispatchers.IO) {
			try {
				val assetStream = try {
					context.assets.open("readmes/readme-$lang.html")
				} catch (_: IOException) {
					context.assets.open("readmes/readme.html")
				}
				val tempFile = File(context.cacheDir, "readme-$lang.html")
				assetStream.use { input ->
					FileOutputStream(tempFile).use { output ->
						input.copyTo(output)
					}
				}
				withContext(Dispatchers.Main) {
					openDocument(Uri.fromFile(tempFile), track = false)
				}
			} catch (_: Exception) {
				withContext(Dispatchers.Main) {
					// TRANSLATORS: Toast shown when the bundled Help document fails to load
					Toast.makeText(context, t("Failed to load document."), Toast.LENGTH_LONG).show()
				}
			}
		}
	}

	companion object {
		/** Bounds of the readability text size multiplier, shared with the settings slider. */
		const val MIN_TEXT_SCALE_PERCENT = 70
		const val MAX_TEXT_SCALE_PERCENT = 300
		const val TEXT_SCALE_PERCENT_STEP = 10

		private val WHITESPACE_REGEX = "\\s+".toRegex()
		private const val DOCUMENT_CACHE_DIR = "documents"
		private val UUID_DIR_REGEX = Regex("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
	}
}
