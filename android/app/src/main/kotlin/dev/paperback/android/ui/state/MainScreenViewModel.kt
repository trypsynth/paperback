package dev.paperback.android.ui.state

import android.app.Application
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.provider.OpenableColumns
import android.widget.Toast
import androidx.core.net.toUri
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import dev.paperback.android.assetLocaleTags
import dev.paperback.android.bestLocaleMatch
import dev.paperback.android.t
import dev.paperback.android.tts.Narrator
import dev.paperback.android.tts.RecordedNarration
import dev.paperback.android.tts.RecordedNarrator
import dev.paperback.android.tts.SpokenNarrator
import dev.paperback.android.tts.TtsManager
import dev.paperback.android.ui.dialogs.GO_TO_LINE
import dev.paperback.android.ui.dialogs.GO_TO_PAGE
import dev.paperback.android.ui.dialogs.GO_TO_PERCENTAGE
import dev.paperback.android.ui.screens.needsAllFilesAccessPermission
import kotlinx.coroutines.Dispatchers
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
import uniffi.paperback.ExportFormat
import uniffi.paperback.HeadingTreeFfi
import uniffi.paperback.LinkListFfi
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi
import uniffi.paperback.TextSegmentFfi
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.util.Locale

private const val AUDIO_SEEK_AMOUNT_KEY = "audio_seek_amount_seconds"
private const val ACTIVE_DOCUMENT_KEY = "active_document"
private const val DEFAULT_AUDIO_SEEK_SECONDS = 10

class MainScreenViewModel(
	application: Application
) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	// Loaded here rather than from the coroutine below, because every `ConfigManagerFfi` read
	// before `initialize` returns the caller's default: `ReaderSettings` is built further down
	// this same constructor, so a later load would leave it publishing defaults for the whole
	// session and the reader's text size would come up wrong on every launch.
	private val config = ConfigManagerFfi().apply {
		initialize(application.filesDir.absolutePath + "/config.toml")
	}
	val configManager: ConfigManagerFfi get() = config

	val ttsManager = TtsManager(application, config)

	private val documentCache = DocumentCache(application.cacheDir)
	private val settingsTransfer = SettingsTransfer(config, application.cacheDir)

	// Narrates DAISY audiobooks' recorded audio in place of synthesized TTS (see
	// DocumentTabState.hasAudio), re-attached to whichever tab is active.
	private val narration = RecordedNarration(application, config, viewModelScope)

	private val recordedNarrator = RecordedNarrator(narration)
	private val spokenNarrator = SpokenNarrator(ttsManager) { speakCurrentSegment() }

	/**
	 * Whatever is reading the active document aloud: its own recording for a DAISY audiobook, the
	 * synthesizer for everything else. Worked out per call rather than held, so switching tabs
	 * needs no bookkeeping of its own.
	 */
	private val narrator: Narrator
		get() = if (uiState.value.activeTab?.hasAudio == true) recordedNarrator else spokenNarrator

	private val _currentNavUnit = MutableStateFlow<NavUnit>(NavUnit.Segment(SegmentTypeFfi.PARAGRAPH))
	val currentNavUnit: StateFlow<NavUnit> = _currentNavUnit.asStateFlow()

	// The source whose position was last announced after an audio seek, so a seek that stays in
	// the same file doesn't repeat its name every time.
	private var lastAnnouncedAudioSource: Int? = null

	private val _ttsPosition = MutableStateFlow(0L)
	val ttsPosition: StateFlow<Long> = _ttsPosition.asStateFlow()

	private val _currentSegmentText = MutableStateFlow("")
	val currentSegmentText: StateFlow<String> = _currentSegmentText.asStateFlow()

	val sleepTimer = SleepTimer(viewModelScope) { pauseTts() }

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
		viewModelScope.launch(Dispatchers.IO) {
			config.setAppString(ACTIVE_DOCUMENT_KEY, docKey)
			config.flush()
		}
	}

	private val _supportedMimeTypes = MutableStateFlow<Array<String>>(arrayOf("*/*"))
	val supportedMimeTypes: StateFlow<Array<String>> = _supportedMimeTypes.asStateFlow()

	val settings = ReaderSettings(config)

	// Every screen and dialog the reading UI can put on top of itself, in one place: a
	// ScreenRequest for the ones somewhere else has to carry out (a navigation destination for
	// MainNavigation, a file picker for MainScreen), a DialogState for the rest. The ones whose
	// state is private are opened through a function on this class that has work to do first
	// (loading the element lists, choosing the Go To mode), so nothing outside can open them
	// straight into an empty or stale state.
	val settingsRequest = ScreenRequest()
	val tocRequest = ScreenRequest()
	val allDocumentsRequest = ScreenRequest()
	val openBookRequest = ScreenRequest()
	val exportSettingsRequest = ScreenRequest()
	val importSettingsRequest = ScreenRequest()

	val findDialog = DialogState()
	val exportDocumentDialog = DialogState()
	val wordCountDialog = DialogState()
	val documentInfoDialog = DialogState()
	val sleepTimerDialog = DialogState()
	val permissionRationaleDialog = DialogState()

	val elementsRequest = ScreenRequest()

	/**
	 * A document offset the reader has asked to jump to from another screen.
	 *
	 * Getting there means switching to Text Mode and scrolling the list, both of which belong to
	 * MainScreen, so a screen that is not MainScreen leaves the offset here and MainScreen acts
	 * on it when it comes back to the front.
	 */
	private val _pendingJumpOffset = MutableStateFlow<Long?>(null)
	val pendingJumpOffset: StateFlow<Long?> = _pendingJumpOffset.asStateFlow()

	private val goToDialogState = DialogState()
	val showGoToDialog: StateFlow<Boolean> = goToDialogState.isOpen

	private val _goToInitialMode = MutableStateFlow("Line")
	val goToInitialMode: StateFlow<String> = _goToInitialMode.asStateFlow()

	val toc = TocState()

	/** Points the table of contents at wherever the reader currently is, for the screen that is
	 * about to show it. */
	fun prepareToc() {
		val tab = uiState.value.activeTab
		toc.pointAt(tab?.toc.orEmpty(), _ttsPosition.value)
	}

	val search = DocumentSearch()

	private val _currentHeadings = MutableStateFlow<HeadingTreeFfi?>(null)
	val currentHeadings: StateFlow<HeadingTreeFfi?> = _currentHeadings.asStateFlow()

	private val _currentLinks = MutableStateFlow<LinkListFfi?>(null)
	val currentLinks: StateFlow<LinkListFfi?> = _currentLinks.asStateFlow()

	private val _passwordPromptUri = MutableStateFlow<Uri?>(null)
	val passwordPromptUri = _passwordPromptUri.asStateFlow()

	private val _importPromptPath = MutableStateFlow<String?>(null)
	val importPromptPath: StateFlow<String?> = _importPromptPath.asStateFlow()

	fun confirmImportSettings() {
		val path = _importPromptPath.value ?: return
		config.importDocumentSettings(path)
		val tab = uiState.value.activeTab
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
		narration.onPlayingChanged = { isPlaying -> ttsManager.setExternalPlaybackState(isPlaying) }
		narration.onSeekLanded = { elapsedMs -> announceAudioSeek(elapsedMs) }
		narration.onDetached = { lastAnnouncedAudioSource = null }
		narration.onClipChanged = { position ->
			// Same reasoning as the TTS auto-advance callbacks above: natural playback tracking
			// would otherwise keep dragging the tracked position forward (mid-clip, off the exact
			// match) while browsing Find, racing with the next Find Previous/Next press. This
			// doesn't extend to saving the audio position, which RecordedNarration does on every
			// clip change whatever the nav unit: unlike desktop's on-close save, Android can kill
			// this process with no lifecycle callback at all, so a kill mid-Find would otherwise
			// resume from before the jump on relaunch.
			if (_currentNavUnit.value !is NavUnit.Find) {
				_ttsPosition.value = position
				refreshSegmentPreview()
				saveTtsPositionToConfig(position)
			}
		}
		viewModelScope.launch(Dispatchers.IO) {
			documentCache.purgeLegacy()
			withContext(Dispatchers.Main) {
				ttsManager.loadConfigAndInit()
			}
			_supportedMimeTypes.value = mimeTypesFor(config.getSupportedExtensions())
			val restorePrevious = settings.restorePreviousDocuments.state.value
			val openedUris = if (restorePrevious) config.getOpenedDocuments() else emptyList()
			val activeDocKey = config.getAppString(ACTIVE_DOCUMENT_KEY, "")
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
					// A VIEW intent opens its document as soon as the screen composes, which is while
					// this restore is still parsing the saved set. That tab is therefore already here,
					// and adding the saved set wholesale would list its document a second time and
					// then pull the reader off it onto whichever document was last active.
					val openedBeforeRestore = currentTabs.isNotEmpty()
					for (restored in restoredTabs) {
						if (currentTabs.none { it.docKey == restored.docKey }) {
							currentTabs.add(restored)
						}
					}
					recentDocumentsList = initialRecents
					if (currentTabs.isEmpty()) {
						currentActiveIndex = -1
					} else if (!openedBeforeRestore) {
						val matchingIndex = currentTabs.indexOfFirst { it.docKey == activeDocKey }
						currentActiveIndex = if (matchingIndex != -1) matchingIndex else 0
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
				documentCache.directoryFor(closedTab.documentUri).deleteRecursively()
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
			persistActiveDocument(currentTabs[index].docKey)
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
		if (!narration.isNarrating(documentUri) && ttsManager.isPaused.value) {
			ttsManager.stop()
		}
	}

	override fun onCleared() {
		super.onCleared()
		narration.detach()
		narration.shutdown()
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
					val tempDir = documentCache.directoryFor(uriString)
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

				if (!isRestore && File(sidecarPathFor(absolutePath)).exists()) {
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
		val activeDocKey = config.getAppString(ACTIVE_DOCUMENT_KEY, "")
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
		if (unit !is NavUnit.Time || uiState.value.activeTab?.hasAudio != true) return false
		val deltaMs = unit.seconds * 1000L
		narration.seekRelativeMs(if (forward) deltaMs else -deltaMs)
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
		val query = search.query.value
		val options = search.options.value
		val tab = uiState.value.activeTab ?: return true
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
		val segment = tab.session.getTextSegment(res.position, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val text = displayTextFor(tab, segment)
		_currentSegmentText.value = text
		saveTtsPositionToConfig(res.position)
		if (tab.hasAudio) {
			narration.seekToPosition(res.position)
			if (speak) {
				narration.play()
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
		val tab = uiState.value.activeTab ?: return
		val cursor = tab.session.audioCursorAtElapsedFfi(elapsedMs)
		if (!cursor.found) return
		val clip = tab.session.audioClipFfi(cursor.clipIndex)
		if (!clip.found) return
		val time = formatDuration(if (tab.isAudioOnly) cursor.seekMs else elapsedMs)
		val fileChanged = lastAnnouncedAudioSource != clip.source
		lastAnnouncedAudioSource = clip.source
		val sectionTitle = sectionTitleAt(tab.toc, clip.start)
		_accessibilityAnnouncement.tryEmit(
			if (fileChanged && sectionTitle.isNotBlank()) "$sectionTitle, $time" else time
		)
	}

	fun togglePlayPause() {
		if (narrator.isPlaying()) narrator.pause() else narrator.play()
	}

	private fun saveTtsPositionToConfig(pos: Long) {
		val docUri = uiState.value.activeTab?.documentUri ?: return
		viewModelScope.launch(Dispatchers.IO) {
			config.setDocumentPosition(docUri, pos)
			config.flush()
		}
	}

	fun refreshSegmentPreview() {
		val tab = uiState.value.activeTab ?: return
		// An audio-only book's buffer is one placeholder space per file with no newlines
		// anywhere, so asking for the paragraph enclosing a position collapses to the whole
		// buffer and reports it as starting at 0. Deriving the label from that would pin it to
		// the first file's name for the life of the book, however far playback had moved; the
		// section (that is, the file) holding the current position is the only label there is.
		if (tab.isAudioOnly) {
			_currentSegmentText.value = sectionTitleAt(tab.toc, _ttsPosition.value)
			return
		}
		val current = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		_currentSegmentText.value = displayTextFor(tab, current).ifBlank {
			val next = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
			displayTextFor(tab, next)
		}
	}

	private fun speakCurrentSegment() {
		val tab = uiState.value.activeTab ?: return
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

	/** Seeks the recorded narration to `segment`'s start, resumes playback there when `speak` says
	 * the reader was already going, and announces `announceText` where that is the only sign
	 * anything moved. */
	private fun navigateNarrationToSegment(
		segment: TextSegmentFfi,
		announceText: String,
		speak: Boolean,
		announce: Boolean
	) {
		narration.seekToPosition(segment.startPos)
		if (speak) {
			narration.play()
		}
		// Stepping by section moves between whole narration files, so name where the jump
		// landed the way a time seek names the file it crossed into: unconditionally, because
		// with playback resuming there is otherwise no cue at all that anything moved, and in
		// full rather than announceNavigationCue's five-word prefix, since here the name is the
		// whole message rather than the opening of a paragraph being previewed.
		val bySection = (_currentNavUnit.value as? NavUnit.Segment)?.type == SegmentTypeFfi.SECTION
		if (bySection && announceText.isNotBlank()) {
			_accessibilityAnnouncement.tryEmit(announceText)
		} else if (!speak && announce) {
			announceNavigationCue(announceText)
		}
	}

	/** A segment's own text, falling back to its enclosing section's TOC title when blank — the
	 * case for a plain-audio DAISY section, whose buffer content is just a placeholder space. */
	private fun displayTextFor(
		tab: DocumentTabState,
		segment: TextSegmentFfi
	): String {
		if (segment.text.isNotBlank()) return segment.text
		return sectionTitleAt(tab.toc, segment.startPos)
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
		val tab = uiState.value.activeTab ?: return
		_ttsPosition.value = pos
		val segment = tab.session.getTextSegment(pos, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val text = displayTextFor(tab, segment)
		_currentSegmentText.value = text
		saveTtsPositionToConfig(pos)
		if (tab.hasAudio) {
			narration.seekToPosition(pos)
			if (resume) {
				narration.play()
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
		val tab = uiState.value.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.NEXT)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak, announce)
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

	private fun announceNavigationCue(text: String) {
		val cue = text
			.trim()
			.split(WHITESPACE_REGEX)
			.take(5)
			.joinToString(" ")
		_accessibilityAnnouncement.tryEmit(cue)
	}

	fun playNextContinuousSegment() {
		val tab = uiState.value.activeTab ?: return
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

	fun transitionToNextContinuousSegment() {
		val tab = uiState.value.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
		if (segment.text.isNotBlank()) {
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = segment.text
			saveTtsPositionToConfig(segment.startPos)
			precacheNextContinuousSegment()
		}
	}

	fun precacheNextContinuousSegment() {
		val tab = uiState.value.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.NEXT)
		if (segment.text.isNotBlank()) {
			ttsManager.precache(segment.text)
		}
	}

	fun playPrevSegment(
		speak: Boolean = true,
		announce: Boolean = false
	) {
		if (navigateByFind(forward = false, speak = speak, announce = announce)) return
		if (seekAudioByNavUnit(forward = false)) return
		val tab = uiState.value.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, navSegmentType(), SegmentDirectionFfi.PREVIOUS)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak, announce)
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

	fun pauseTts() {
		narrator.pause()
	}

	fun navigateByType(
		type: SegmentTypeFfi,
		direction: SegmentDirectionFfi
	) {
		val tab = uiState.value.activeTab ?: return
		val segment = tab.session.getTextSegment(_ttsPosition.value, type, direction)
		if (segment.found) {
			val text = displayTextFor(tab, segment)
			_ttsPosition.value = segment.startPos
			_currentSegmentText.value = text
			saveTtsPositionToConfig(segment.startPos)
			if (tab.hasAudio) {
				navigateNarrationToSegment(segment, text, speak = true, announce = false)
				return
			}
			ttsManager.stop()
			ttsManager.speak(text)
			precacheNextContinuousSegment()
		}
	}

	fun resumeTts() {
		narrator.play()
	}

	fun exportCurrentSettings(): Boolean {
		val tab = uiState.value.activeTab ?: return false
		return settingsTransfer.exportToSidecar(tab.documentUri)
	}

	fun exportSettingsToUri(
		context: Context,
		destUri: Uri
	): Boolean {
		val tab = uiState.value.activeTab ?: return false
		return settingsTransfer.exportTo(context, tab.documentUri, destUri)
	}

	fun exportDocumentToUri(
		context: Context,
		destUri: Uri,
		format: ExportFormat
	): Boolean {
		val tab = uiState.value.activeTab ?: return false
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
		val tab = uiState.value.activeTab ?: return false
		if (!settingsTransfer.importFrom(context, tab.documentUri, sourceUri)) return false
		val savedPosition = config.getDocumentPosition(tab.documentUri)
		if (savedPosition > 0L) {
			updateTtsPosition(savedPosition)
		}
		return true
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
			search.clear()
		}
		if (currentActiveIndex in currentTabs.indices) {
			val tab = currentTabs[currentActiveIndex]
			ttsManager.currentDocumentTitle = tab.title.ifBlank { tab.fileName }
			ttsManager.currentDocumentAuthor = tab.author.ifBlank { "Unknown Author" }
			narration.attach(tab)
		} else {
			ttsManager.currentDocumentTitle = "Paperback"
			ttsManager.currentDocumentAuthor = "Unknown"
			narration.detach()
		}
	}

	fun updateTtsPosition(pos: Long) {
		_ttsPosition.value = pos
		refreshSegmentPreview()
		saveTtsPositionToConfig(pos)
		narrator.moveTo(pos)
	}

	/**
	 * Seeks the recording to `percent` of its running time, reporting whether it applied.
	 *
	 * False for a document with no audio, and for one whose file lengths are not all known, so
	 * the caller maps the percentage through the text instead, as everything did before. A
	 * percentage through the text of an audiobook counts blank lines, one per file, so it treats
	 * a two minute file and an hour long one as equal shares of the book.
	 *
	 * The reading position follows on its own: the player reports the clip it lands in.
	 */
	fun seekAudioToPercent(percent: Int): Boolean {
		val tab = uiState.value.activeTab ?: return false
		val targetMs = tab.session.audioElapsedForPercentFfi(percent)
		if (targetMs < 0) return false
		return narration.seekToMs(targetMs)
	}

	fun openExportDocumentDialog() {
		// There is nothing to export with no document open. The menu hides the entry then; this
		// keeps the Ctrl+E shortcut from arming a dialog that would appear over whatever document
		// is opened next.
		if (uiState.value.activeTab == null) return
		exportDocumentDialog.open()
	}

	fun openWordCountDialog() {
		// Nothing to count in an audio-only book. The menu hides the entry for one; this keeps
		// the Ctrl+W shortcut from opening a dialog full of zeroes anyway.
		if (uiState.value.activeTab?.isAudioOnly == true) return
		wordCountDialog.open()
	}

	fun openElements() {
		val tab = uiState.value.activeTab ?: return
		// An audio-only book has no text spine, so both tabs would come up empty. The menu hides
		// the entry for one; this keeps the F7 shortcut from opening it anyway.
		if (tab.isAudioOnly) return
		viewModelScope.launch(Dispatchers.IO) {
			val pos = _ttsPosition.value
			val headings = tab.session.getHeadingTreeFfi(pos)
			val links = tab.session.getLinkListFfi(pos)
			withContext(Dispatchers.Main) {
				_currentHeadings.value = headings
				_currentLinks.value = links
				// Requested once the lists are in hand, so the screen never appears empty and
				// then fills in.
				elementsRequest.request()
			}
		}
	}

	/** Drops the heading and link lists once the elements screen is gone. */
	fun clearElements() {
		_currentHeadings.value = null
		_currentLinks.value = null
	}

	fun requestJumpToOffset(offset: Long) {
		_pendingJumpOffset.value = offset
	}

	fun consumeJumpRequest() {
		_pendingJumpOffset.value = null
	}

	private val _accessibilityAnnouncement = MutableSharedFlow<String>(extraBufferCapacity = 1)
	val accessibilityAnnouncement: SharedFlow<String> = _accessibilityAnnouncement.asSharedFlow()

	fun announceForAccessibility(message: String) {
		_accessibilityAnnouncement.tryEmit(message)
	}

	fun openGoToDialog(initialMode: String = GO_TO_LINE) {
		val tab = uiState.value.activeTab
		var mode = initialMode
		// Line and page mean nothing in a book whose text is one blank line per audio file,
		// so the shortcuts for them land on the one mode it does have.
		if (tab != null && tab.isAudioOnly) {
			mode = GO_TO_PERCENTAGE
		} else if (tab != null && mode == GO_TO_PAGE && tab.session.pageCountFfi() == 0) {
			// TRANSLATORS: Announced when the Go To Page shortcut is used on a document with no page numbers
			announceForAccessibility(t("This document does not contain pages."))
			return
		}
		_goToInitialMode.value = mode
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
			val activeDocKey = config.getAppString(ACTIVE_DOCUMENT_KEY, "")
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
		private val WHITESPACE_REGEX = "\\s+".toRegex()
	}
}
