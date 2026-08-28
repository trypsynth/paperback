import SwiftUI
import Combine
import UIKit

@MainActor
@Observable
final class AppViewModel {
	// MARK: - Tabs
	var tabs: [DocumentTab] = []
	// Stops TTS whenever the active document changes so a paused/playing utterance from the
	// previous book can never bleed into the next one (its buffer stays scheduled on the audio
	// node across pause() until something clears it — see TtsManager.pause()).
	var activeTabId: UUID? = nil {
		didSet {
			guard activeTabId != oldValue else { return }
			reading.ttsManager.stop()
		}
	}

	var activeTab: DocumentTab? {
		guard let id = activeTabId else { return nil }
		return tabs.first { $0.id == id }
	}

	var activeSession: DocumentSession? { activeTab?.session }

	// MARK: - Navigation
	let navigation = NavigationRouter()

	// MARK: - Reading
	let reading = ReadingController()

	// MARK: - Settings
	var restorePreviousDocuments = true {
		didSet { configManager.setAppBool(key: "restore_previous_documents", value: restorePreviousDocuments) }
	}
	var swipeUpMovesForward = true {
		didSet { configManager.setAppBool(key: "swipe_up_moves_forward", value: swipeUpMovesForward) }
	}

	// MARK: - Recents
	var recentDocuments: [RecentDocument] = []

	// MARK: - Config
	let configManager = ConfigManagerFfi()
	@ObservationIgnored private var cancellables = Set<AnyCancellable>()

	init() {
		setPdfiumLibraryPath(path: Bundle.main.bundlePath + "/Frameworks/libpdfium.framework")

		let configPath = configFilePath()
		_ = configManager.initialize(configPath: configPath)
		restorePreviousDocuments = configManager.getAppBool(key: "restore_previous_documents", defaultValue: true)
		swipeUpMovesForward = configManager.getAppBool(key: "swipe_up_moves_forward", defaultValue: true)

		reading.context = self

		let ttsManager = reading.ttsManager
		let savedRate = configManager.getAppString(key: "tts_speech_rate", defaultValue: "")
		if let r = Float(savedRate) { ttsManager.speechRate = r }

		let savedPitch = configManager.getAppString(key: "tts_pitch", defaultValue: "")
		if let p = Float(savedPitch) { ttsManager.pitch = p }

		let savedVoice = configManager.getAppString(key: "tts_voice_identifier", defaultValue: "")
		if !savedVoice.isEmpty { ttsManager.selectedVoiceIdentifier = savedVoice }

		loadRecentsFromConfig()
		reading.start()
		ttsManager.onSpeechRateChanged = { [weak self] rate in
			self?.configManager.setAppString(key: "tts_speech_rate", value: "\(rate)")
		}
		ttsManager.onPitchChanged = { [weak self] pitch in
			self?.configManager.setAppString(key: "tts_pitch", value: "\(pitch)")
		}
		ttsManager.onVoiceChanged = { [weak self] identifier in
			self?.configManager.setAppString(key: "tts_voice_identifier", value: identifier ?? "")
		}

		if restorePreviousDocuments {
			for path in configManager.getOpenedDocuments() {
				tryRestoreDocument(path: path)
			}
		}
		NotificationCenter.default.publisher(for: UIApplication.didEnterBackgroundNotification)
			.sink { [weak self] _ in
				self?.configManager.flush()
			}
			.store(in: &cancellables)
		NotificationCenter.default.publisher(for: .pbMagicTap)
			.sink { [weak self] _ in
				Task { @MainActor [weak self] in
					self?.reading.togglePlayPause()
				}
			}
			.store(in: &cancellables)
		reading.updateNowPlaying()
	}

	// MARK: - Document management

	var debugMessage: String? = nil

	func openDocument(url: URL, password: String? = nil, track: Bool = true) {
		if let existing = tabs.first(where: { $0.url == url }) {
			activeTabId = existing.id
			return
		}
		let scopeStarted = url.startAccessingSecurityScopedResource()
		let path = url.path(percentEncoded: false)
		let pass = password ?? configManager.getDocumentPassword(path: path)
		do {
			let session = try DocumentSession.newFfi(
				filePath: path,
				password: pass,
				forcedExtension: "",
				renderTablesInline: false
			)
			let title = session.title().isEmpty
				? url.deletingPathExtension().lastPathComponent
				: session.title()
			let savedPos = configManager.getDocumentPosition(path: path)
			var tab = DocumentTab(title: title, url: url, session: session)
			tab.currentPosition = savedPos
			tab.securityScopeURL = scopeStarted ? url : nil
			tabs.append(tab)
			activeTabId = tab.id
			if track {
				configManager.addRecentDocument(path: path)
				configManager.addOpenedDocument(path: path)
				loadRecentsFromConfig()
				saveBookmark(for: url, path: path)
			}
			reading.loadSegment(for: tab)
			reading.updateNowPlaying()
		} catch {
			if scopeStarted { url.stopAccessingSecurityScopedResource() }
			debugMessage = "Error opening '\(url.lastPathComponent)':\n\(error)\n\nPath: \(path)"
		}
	}

	func openHelpDocument() {
		let preferred = Bundle.main.preferredLocalizations.first ?? "en"
		let lang = preferred.split(separator: "-").first.map(String.init) ?? preferred
		// Try the localized doc first, falling back to English rather than checking a
		// hardcoded language list against a hand-maintained set of resource names — that
		// list drifts out of sync with which readme-<lang>.html files actually exist.
		let localizedURL = Bundle.main.url(forResource: "readme-\(lang)", withExtension: "html", subdirectory: "Readmes")
		let fallbackURL = Bundle.main.url(forResource: "readme", withExtension: "html", subdirectory: "Readmes")
		guard let url = localizedURL ?? fallbackURL else {
			// TRANSLATORS: Shown when the bundled Help document fails to load
			debugMessage = t("Failed to load document.")
			return
		}
		openDocument(url: url, track: false)
	}

	// MARK: - Document data import/export

	// Writes the active document's bookmarks/position to a temporary .paperback
	// file and returns its URL, ready to hand to a file mover/exporter. Returns
	// nil if there's no active document or the write failed.
	func exportActiveDocumentSettings() -> URL? {
		guard let tab = activeTab else { return nil }
		let path = tab.url.path(percentEncoded: false)
		let name = tab.url.deletingPathExtension().lastPathComponent
		let tempURL = FileManager.default.temporaryDirectory
			.appendingPathComponent(name)
			.appendingPathExtension("paperback")
		try? FileManager.default.removeItem(at: tempURL)
		configManager.exportDocumentSettings(docPath: path, exportPath: tempURL.path(percentEncoded: false))
		return FileManager.default.fileExists(atPath: tempURL.path) ? tempURL : nil
	}

	// Applies a .paperback file's bookmarks/position to the active document.
	@discardableResult
	func importActiveDocumentSettings(from url: URL) -> Bool {
		guard let tab = activeTab else { return false }
		let scopeStarted = url.startAccessingSecurityScopedResource()
		defer { if scopeStarted { url.stopAccessingSecurityScopedResource() } }
		guard FileManager.default.fileExists(atPath: url.path(percentEncoded: false)) else { return false }
		let path = tab.url.path(percentEncoded: false)
		configManager.importSettingsFromFile(docPath: path, importPath: url.path(percentEncoded: false))
		let savedPos = configManager.getDocumentPosition(path: path)
		if let idx = tabs.firstIndex(where: { $0.id == tab.id }) {
			tabs[idx].currentPosition = savedPos
		}
		if activeTabId == tab.id {
			reading.goToPosition(savedPos)
		}
		return true
	}

	func closeTab(_ tab: DocumentTab) {
		let path = tab.url.path(percentEncoded: false)
		if tab.session != nil {
			configManager.setDocumentPosition(path: path, position: tab.currentPosition)
		}
		configManager.removeOpenedDocument(path: path)
		tab.securityScopeURL?.stopAccessingSecurityScopedResource()
		tabs.removeAll { $0.id == tab.id }
		if activeTabId == tab.id {
			activeTabId = tabs.last?.id
		}
	}

	func setActiveTab(_ tab: DocumentTab) {
		activeTabId = tab.id
		if let t = activeTab {
			reading.loadSegment(for: t)
		}
	}

	// MARK: - Recents

	private func loadRecentsFromConfig() {
		let paths = configManager.getRecentDocuments()
		let openPaths = Set(tabs.map { $0.url.path(percentEncoded: false) })
		recentDocuments = paths.map { path in
			// Resolve the persisted security-scoped bookmark rather than constructing a plain
			// path URL: files picked from outside the app's own container (the common case)
			// aren't readable via a bare path once the picker's access grant has ended, which
			// otherwise shows every such entry as missing and fails to open with a parse error.
			let resolved = resolvedURL(forPath: path)
			let url = resolved ?? URL(fileURLWithPath: path)
			let title = url.deletingPathExtension().lastPathComponent
			return RecentDocument(
				title: title,
				url: url,
				isMissing: resolved == nil,
				isOpen: openPaths.contains(path)
			)
		}
	}

	func addRecentDocument(url: URL, title: String) {
		configManager.addRecentDocument(path: url.path(percentEncoded: false))
		loadRecentsFromConfig()
	}

	// Points a recent-document entry at a new file location, e.g. after the
	// original was moved or renamed outside the app.
	func locateRecentDocument(_ oldURL: URL, at newURL: URL) {
		configManager.renameDocumentPath(
			oldPath: oldURL.path(percentEncoded: false),
			newPath: newURL.path(percentEncoded: false)
		)
		loadRecentsFromConfig()
	}

	func removeRecentDocument(url: URL) {
		configManager.removeDocumentHistory(path: url.path(percentEncoded: false))
		recentDocuments.removeAll { $0.url == url }
	}

	// MARK: - Private helpers

	private func tryRestoreDocument(path: String) {
		guard let url = resolvedURL(forPath: path) else { return }
		openDocument(url: url)
	}

	// Resolves a stored path back to a usable URL: prefers the persisted security-scoped
	// bookmark (needed for files outside the app's own container), falling back to a plain
	// path URL for files the app can read directly. Returns nil if neither resolves.
	private func resolvedURL(forPath path: String) -> URL? {
		if let data = UserDefaults.standard.data(forKey: bookmarkKey(path)) {
			var isStale = false
			if let url = try? URL(resolvingBookmarkData: data, bookmarkDataIsStale: &isStale) {
				return url
			}
		}
		return FileManager.default.fileExists(atPath: path) ? URL(fileURLWithPath: path) : nil
	}

	private func saveBookmark(for url: URL, path: String) {
		guard let data = try? url.bookmarkData(options: .minimalBookmark, includingResourceValuesForKeys: nil, relativeTo: nil) else { return }
		UserDefaults.standard.set(data, forKey: bookmarkKey(path))
	}

	private func bookmarkKey(_ path: String) -> String {
		"pb_bm_\(path)"
	}

	private func updateTabPosition(_ position: Int64) {
		guard let id = activeTabId,
		      let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
		tabs[idx].currentPosition = position
		let path = tabs[idx].url.path(percentEncoded: false)
		configManager.setDocumentPosition(path: path, position: position)
	}
}

// MARK: - ReadingContext

extension AppViewModel: ReadingContext {
	var activeTitle: String? { activeTab?.title }

	var activeLineScrollIndex: Int {
		get { activeTab?.lineScrollIndex ?? 0 }
		set {
			guard let id = activeTabId,
			      let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
			tabs[idx].lineScrollIndex = newValue
		}
	}

	func persistPosition(_ position: Int64) {
		updateTabPosition(position)
	}
}

// MARK: - Supporting types

enum SegmentType: String, CaseIterable {
	case paragraph = "Paragraph"
	case line = "Line"
	case heading = "Heading"
	case section = "Section"
}

enum GoToMode {
	case line, page, percent
}

struct SearchOptions {
	var matchCase: Bool = false
	var wholeWord: Bool = false
	var regex: Bool = false
}

private func configFilePath() -> String {
	let support = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
	let dir = support.appendingPathComponent("dev.paperback.mobile", isDirectory: true)
	try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
	return dir.appendingPathComponent("config.toml").path
}
