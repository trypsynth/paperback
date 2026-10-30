package dev.paperback.android.ui.state

import android.app.Application
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Environment
import android.provider.OpenableColumns
import android.widget.Toast
import androidx.core.net.toUri
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import dev.paperback.android.assetLocaleTags
import dev.paperback.android.bestLocaleMatch
import dev.paperback.android.saveSoon
import dev.paperback.android.t
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
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.util.Locale

private const val ACTIVE_DOCUMENT_KEY = "active_document"

class MainScreenViewModel(application: Application) : AndroidViewModel(application) {
	private val context get() = getApplication<Application>()

	// Initialized here, not in a coroutine: reads before `initialize` return defaults, and ReaderSettings is built later in this constructor.
	private val config = ConfigManagerFfi().apply {
		initialize(application.filesDir.absolutePath + "/config.toml")
	}
	val configManager: ConfigManagerFfi get() = config

	val ttsManager = TtsManager(application, config)

	private val documentCache = DocumentCache(application.cacheDir)
	private val settingsTransfer = SettingsTransfer(config, application.cacheDir)

	private val _uiState = MutableStateFlow<MainScreenUiState>(MainScreenUiState.Idle)
	val uiState: StateFlow<MainScreenUiState> = _uiState.asStateFlow()

	private val currentTabs = mutableListOf<DocumentTabState>()
	private var currentActiveIndex = -1
	private var recentDocumentsList = emptyList<RecentDocumentItem>()

	// Lets updateTtsMetadata tell a real tab switch from closing a background tab, which leaves the active one unchanged.
	private var lastMetadataDocumentUri: String? = null

	private fun emitTabsState() {
		_uiState.value = MainScreenUiState.Success(currentTabs.toList(), currentActiveIndex, recentDocumentsList)
	}

	private fun persistActiveDocument(docKey: String) {
		viewModelScope.launch(Dispatchers.IO) {
			config.setAppString(ACTIVE_DOCUMENT_KEY, docKey)
			config.saveSoon()
		}
	}

	private val _supportedMimeTypes = MutableStateFlow<Array<String>>(arrayOf("*/*"))
	val supportedMimeTypes: StateFlow<Array<String>> = _supportedMimeTypes.asStateFlow()

	val settings = ReaderSettings(config)

	val search = DocumentSearch()

	private val _accessibilityAnnouncement = MutableSharedFlow<String>(extraBufferCapacity = 1)
	val accessibilityAnnouncement: SharedFlow<String> = _accessibilityAnnouncement.asSharedFlow()

	val reading = ReadingController(
		application,
		config,
		ttsManager,
		settings,
		search,
		viewModelScope,
		activeTab = { uiState.value.activeTab },
		announcer = ::announceForAccessibility
	)

	val navigation = NavigationRouter()

	/** Downloads go where file managers can reach them, falling back to private storage when shared storage is missing. */
	val links = LinkOpener(
		viewModelScope,
		folder = { context.getExternalFilesDir(Environment.DIRECTORY_DOWNLOADS) ?: File(context.filesDir, "downloads") },
		onSaved = { openDocument(Uri.fromFile(it)) }
	)

	val toc = TocState()

	fun prepareToc() {
		val tab = uiState.value.activeTab
		toc.pointAt(tab?.toc.orEmpty(), reading.ttsPosition.value)
	}

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
			reading.updateTtsPosition(savedPosition)
			reading.refreshSegmentPreview()
		}
		_importPromptPath.value = null
	}

	fun cancelImportSettings() {
		_importPromptPath.value = null
	}

	init {
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
					// A VIEW intent can open its tab while this restore is still parsing, so don't duplicate it or pull the reader off it.
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
						reading.setPosition(it.savedPosition)
						updateTtsMetadata()
						reading.refreshSegmentPreview()
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

	private suspend fun getRecentDocumentsListIO(): List<RecentDocumentItem> = withContext(Dispatchers.IO) {
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

	/** Open tabs are left alone: a document being read is not one the reader is finished with. */
	fun clearRecentDocuments() {
		viewModelScope.launch(Dispatchers.IO) {
			config.clearRecentDocuments()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				emitTabsState()
			}
		}
	}

	fun removeRecentDocument(uriString: String) {
		viewModelScope.launch(Dispatchers.IO) {
			config.removeDocumentHistory(uriString)
			config.saveSoon()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				emitTabsState()
			}
		}
	}

	fun locateRecentDocument(oldUriString: String, newUri: Uri) {
		val newUriString = newUri.toString()
		viewModelScope.launch(Dispatchers.IO) {
			try {
				context.contentResolver.takePersistableUriPermission(newUri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
			} catch (_: SecurityException) {
			}
			config.renameDocumentPath(oldUriString, newUriString)
			config.saveSoon()
			updateRecentDocuments()
			withContext(Dispatchers.Main) {
				emitTabsState()
			}
		}
	}

	fun openDocument(uri: Uri, track: Boolean = true) = openDocuments(listOf(uri), track)

	// One after another in a single job: a job per document would race to add their tabs and write the config.
	fun openDocuments(uris: List<Uri>, track: Boolean = true) {
		viewModelScope.launch(Dispatchers.IO) {
			for (uri in uris) {
				val uriString = uri.toString()
				try {
					context.contentResolver.takePersistableUriPermission(uri, Intent.FLAG_GRANT_READ_URI_PERMISSION)
				} catch (_: SecurityException) {
				}
				if (track) {
					config.addRecentDocument(uriString)
					config.addOpenedDocument(uriString)
					config.saveSoon()
				}
				loadDocument(uri, true)
			}
		}
	}

	fun closeTab(index: Int) {
		if (index in currentTabs.indices) {
			val closedTab = currentTabs.removeAt(index)
			viewModelScope.launch(Dispatchers.IO) {
				config.removeOpenedDocument(closedTab.documentUri)
				config.setDocumentOpened(closedTab.documentUri, false)
				config.saveSoon()
				documentCache.directoryFor(closedTab.documentUri).deleteRecursively()
				updateRecentDocuments()
				withContext(Dispatchers.Main) {
					currentActiveIndex = if (currentTabs.isEmpty()) -1 else currentActiveIndex.coerceIn(0, currentTabs.size - 1)
					if (currentActiveIndex != -1) {
						persistActiveDocument(currentTabs[currentActiveIndex].docKey)
					}
					emitTabsState()
					if (currentActiveIndex != -1) {
						reading.setPosition(currentTabs[currentActiveIndex].savedPosition)
						updateTtsMetadata()
						reading.refreshSegmentPreview()
					} else {
						reading.setPosition(0)
						reading.clearSegment()
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
			reading.setPosition(currentTabs[index].savedPosition)
			updateTtsMetadata()
			reading.refreshSegmentPreview()
		}
	}

	override fun onCleared() {
		reading.shutdown()
		Thread {
			try {
				config.saveSoon()
			} catch (_: Exception) {
			}
		}.start()
	}

	private suspend fun prepareDocumentTabIO(
		uri: Uri,
		providedPassword: String? = null,
		isRestore: Boolean = false
	): DocumentTabState? = withContext(Dispatchers.IO) {
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
				config.saveSoon()
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

	private suspend fun loadDocument(uri: Uri, makeActive: Boolean) = withContext(Dispatchers.IO) {
		if (currentTabs.isEmpty()) {
			_uiState.value = MainScreenUiState.Loading
		}
		val tabState = prepareDocumentTabIO(uri)
		if (tabState == null) {
			withContext(Dispatchers.Main) {
				if (uri.scheme == "file" && needsAllFilesAccessPermission()) {
					navigation.permissionRationaleDialog.open()
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
						config.saveSoon()
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
				reading.setPosition(tabState.savedPosition)
				updateTtsMetadata()
				reading.refreshSegmentPreview()
			}
		}
	}

	fun exportCurrentSettings(): Boolean {
		val tab = uiState.value.activeTab ?: return false
		return settingsTransfer.exportToSidecar(tab.documentUri)
	}

	fun exportSettingsToUri(context: Context, destUri: Uri): Boolean {
		val tab = uiState.value.activeTab ?: return false
		return settingsTransfer.exportTo(context, tab.documentUri, destUri)
	}

	fun exportDocumentToUri(context: Context, destUri: Uri, format: ExportFormat): Boolean {
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

	fun importSettingsFromUri(context: Context, sourceUri: Uri): Boolean {
		val tab = uiState.value.activeTab ?: return false
		if (!settingsTransfer.importFrom(context, tab.documentUri, sourceUri)) return false
		val savedPosition = config.getDocumentPosition(tab.documentUri)
		if (savedPosition > 0L) {
			reading.updateTtsPosition(savedPosition)
		}
		return true
	}

	private fun updateTtsMetadata() {
		// A search belongs to its document, but closing a background tab also lands here and must not wipe the active tab's search.
		val activeDocumentUri = currentTabs.getOrNull(currentActiveIndex)?.documentUri
		if (activeDocumentUri != lastMetadataDocumentUri) {
			lastMetadataDocumentUri = activeDocumentUri
			search.clear()
		}
		reading.follow(currentTabs.getOrNull(currentActiveIndex))
	}

	fun openExportDocumentDialog() {
		// The menu hides the entry, but Ctrl+E would otherwise arm a dialog that appears over the next document opened.
		if (uiState.value.activeTab == null) return
		navigation.exportDocumentDialog.open()
	}

	fun openWordCountDialog() {
		// The menu hides the entry, but Ctrl+W would otherwise open a dialog full of zeroes.
		if (uiState.value.activeTab?.isAudioOnly == true) return
		navigation.wordCountDialog.open()
	}

	fun openElements() {
		val tab = uiState.value.activeTab ?: return
		// The menu hides the entry, but F7 would otherwise open two empty lists for an audio-only book.
		if (tab.isAudioOnly) return
		viewModelScope.launch(Dispatchers.IO) {
			val pos = reading.ttsPosition.value
			val headings = tab.session.getHeadingTreeFfi(pos)
			val links = tab.session.getLinkListFfi(pos)
			withContext(Dispatchers.Main) {
				_currentHeadings.value = headings
				_currentLinks.value = links
				// Only once the lists are in hand, so the screen never appears empty and then fills in.
				navigation.elementsRequest.request()
			}
		}
	}

	fun clearElements() {
		_currentHeadings.value = null
		_currentLinks.value = null
	}

	fun announceForAccessibility(message: String) {
		_accessibilityAnnouncement.tryEmit(message)
	}

	fun openGoToDialog(initialMode: String = GO_TO_LINE) {
		val tab = uiState.value.activeTab
		var mode = initialMode
		// Line and page mean nothing when the text is one blank line per audio file.
		if (tab != null && tab.isAudioOnly) {
			mode = GO_TO_PERCENTAGE
		} else if (tab != null && mode == GO_TO_PAGE && tab.session.pageCountFfi() == 0) {
			// TRANSLATORS: Announced when the Go To Page shortcut is used on a document with no page numbers
			announceForAccessibility(t("This document does not contain pages."))
			return
		}
		navigation.openGoTo(mode)
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
							config.saveSoon()
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
				reading.setPosition(tabState.savedPosition)
				reading.refreshSegmentPreview()
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
				config.saveSoon()
				updateRecentDocuments()
			}
			withContext(Dispatchers.Main) {
				emitTabsState()
			}
		}
	}

	fun openHelpDocument() {
		// Readmes are named for the po files (readme-zh_CN.html), so asking for the bare device language (readme-zh.html) only ever found English.
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
}
