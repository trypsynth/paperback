import SwiftUI
import Combine
import UIKit

@MainActor
@Observable
final class AppViewModel {
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

	let navigation = NavigationRouter()

	let reading = ReadingController()

	var restorePreviousDocuments = true {
		didSet { configManager.setAppBool(key: "restore_previous_documents", value: restorePreviousDocuments) }
	}
	var swipeUpMovesForward = true {
		didSet { configManager.setAppBool(key: "swipe_up_moves_forward", value: swipeUpMovesForward) }
	}

	// Spacing and alignment share the desktop's config keys and value meanings (spacing
	// 0/1/2, alignment 0 leading, 1 center, 2 trailing) so a document reads the same way on
	// both. Text size does not: the desktop stores an absolute point size, while this scales
	// whatever size Dynamic Type is already asking for.
	var textScalePercent: Int = 100 {
		didSet { configManager.setAppInt(key: "text_scale_percent", value: Int32(textScalePercent)) }
	}
	var lineSpacingChoice: Int = 0 {
		didSet { configManager.setAppInt(key: "line_spacing", value: Int32(lineSpacingChoice)) }
	}
	var paragraphSpacingChoice: Int = 0 {
		didSet { configManager.setAppInt(key: "paragraph_spacing", value: Int32(paragraphSpacingChoice)) }
	}
	var textAlignmentChoice: Int = 0 {
		didSet { configManager.setAppInt(key: "text_alignment", value: Int32(textAlignmentChoice)) }
	}
	/// 0 follows the system, 1 forces light, 2 forces dark. Readers often want the page dark
	/// while the rest of the phone stays light, which following the system alone cannot do.
	/// Drops the previous and next buttons from the reading bar's screen reader order. They
	/// duplicate the swipe up and down actions already on the play button, so hiding them makes
	/// the bar three stops instead of five. Off by default: the swipe is not discoverable on its
	/// own, so nobody should lose the buttons without having chosen to.
	var hidePrevNextButtons: Bool = false {
		didSet { configManager.setAppBool(key: "hide_prev_next_buttons", value: hidePrevNextButtons) }
	}
	var appearanceChoice: Int = 0 {
		didSet { configManager.setAppInt(key: "appearance", value: Int32(appearanceChoice)) }
	}
	/// Pure black on white, or white on black, for document text. The system's own Increase
	/// Contrast setting turns this on too, so a reader who has already asked for it everywhere
	/// does not have to ask again here.
	var highContrastText: Bool = false {
		didSet { configManager.setAppBool(key: "high_contrast_text", value: highContrastText) }
	}

	var recentDocuments: [RecentDocument] = []

	let configManager = ConfigManagerFfi()
	@ObservationIgnored private var cancellables = Set<AnyCancellable>()

	init() {
		setPdfiumLibraryPath(path: Bundle.main.bundlePath + "/Frameworks/libpdfium.framework")

		let configPath = configFilePath()
		_ = configManager.initialize(configPath: configPath)
		restorePreviousDocuments = configManager.getAppBool(key: "restore_previous_documents", defaultValue: true)
		swipeUpMovesForward = configManager.getAppBool(key: "swipe_up_moves_forward", defaultValue: true)
		textScalePercent = Int(configManager.getAppInt(key: "text_scale_percent", defaultValue: 100))
		lineSpacingChoice = Int(configManager.getAppInt(key: "line_spacing", defaultValue: 0))
		paragraphSpacingChoice = Int(configManager.getAppInt(key: "paragraph_spacing", defaultValue: 0))
		textAlignmentChoice = Int(configManager.getAppInt(key: "text_alignment", defaultValue: 0))
		hidePrevNextButtons = configManager.getAppBool(key: "hide_prev_next_buttons", defaultValue: false)
		appearanceChoice = Int(configManager.getAppInt(key: "appearance", defaultValue: 0))
		highContrastText = configManager.getAppBool(key: "high_contrast_text", defaultValue: false)

		reading.context = self

		let narration = RecordedNarration(config: configManager)
		narration.onClipChanged = { [weak self] position in
			// The recording is the authority on where the reader is while it plays, so the
			// caret and the displayed text follow it rather than the other way round.
			guard let self else { return }
			self.reading.ttsPosition = position
			self.reading.refreshSegmentForAudio()
			self.updateTabPosition(position)
		}
		narration.onPlayingChanged = { [weak self] _ in
			self?.reading.updateNowPlaying()
		}
		narration.onSeekLanded = { [weak self] elapsedMs in
			self?.reading.announceAudioSeekLanded(elapsedMs)
		}
		reading.narration = narration

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

	/// The export formats the open document can be rendered as, which the core decides per
	/// document rather than per app.
	var supportedExportFormats: [ExportFormat] {
		activeSession?.getSupportedExportFormatsFfi() ?? []
	}

	/// Renders the open document in `format` to a temporary file named after the book, ready to
	/// hand to the file mover. Nil when there is nothing open or the write failed.
	func exportActiveDocument(as format: ExportFormat) -> URL? {
		guard let tab = activeTab, let session = tab.session else { return nil }
		let name = tab.url.deletingPathExtension().lastPathComponent
		let tempURL = FileManager.default.temporaryDirectory
			.appendingPathComponent(name)
			.appendingPathExtension(format.fileExtension)
		try? FileManager.default.removeItem(at: tempURL)
		do {
			try session.renderExportFfi(format: format).write(to: tempURL, atomically: true, encoding: .utf8)
		} catch {
			return nil
		}
		return tempURL
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

	/// Empties the recent documents list. Open tabs are left alone: a document being read is
	/// not a document the reader is finished with.
	func clearRecentDocuments() {
		configManager.clearRecentDocuments()
		recentDocuments.removeAll()
	}

	func removeRecentDocument(url: URL) {
		configManager.removeDocumentHistory(path: url.path(percentEncoded: false))
		recentDocuments.removeAll { $0.url == url }
	}

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

// What the previous/next controls move by. Ordinarily a structural unit; while a Find query is
// active, Find joins the list and steps between that query's matches instead.
enum NavUnit: Hashable {
	case segment(SegmentTypeFfi)
	/// An amount of elapsed recording to skip, for a book with its own narration. Stepping by
	/// paragraph means nothing in a bundle of audio files with no real text to walk.
	case time(seconds: Int)
	case find

	var name: String {
		switch self {
		case .segment(let type): return segmentTypeName(type)
		case .time(let seconds): return seekAmountName(seconds)
		// TRANSLATORS: Name of the "Find" navigation unit, which moves between search matches
		case .find: return t("Find")
		}
	}
}

/// Renders a duration the way the reading bar reports one: minutes and seconds, with hours only
/// once there are any.
func formatDuration(_ ms: Int64) -> String {
	let totalSeconds = (max(ms, 0) + 500) / 1000
	let hours = totalSeconds / 3600
	let minutes = (totalSeconds % 3600) / 60
	let seconds = totalSeconds % 60
	return hours > 0
		? String(format: "%d:%02d:%02d", hours, minutes, seconds)
		: String(format: "%d:%02d", minutes, seconds)
}

/// The seek amounts a recorded book offers, matching the presets desktop shows in its Options.
let audioSeekAmountsSeconds = [5, 10, 30, 60, 120]

/// Matches the labels desktop shows for the same presets in its Options dialog.
func seekAmountName(_ seconds: Int) -> String {
	switch seconds {
	// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
	case 5: return t("5 seconds")
	// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
	case 10: return t("10 seconds")
	// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
	case 30: return t("30 seconds")
	// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
	case 60: return t("1 minute")
	// TRANSLATORS: Audio seek amount, shown as a navigation unit in the read-aloud bar
	case 120: return t("2 minutes")
	default: return "\(seconds)"
	}
}

func segmentTypeName(_ type: SegmentTypeFfi) -> String {
	switch type {
	// TRANSLATORS: Name of the "paragraph" reading/navigation unit
	case .paragraph: return t("Paragraph")
	// TRANSLATORS: Name of the "line" reading/navigation unit
	case .line: return t("Line")
	// TRANSLATORS: Name of the "heading" reading/navigation unit
	case .heading: return t("Heading")
	// TRANSLATORS: Name of the "link" reading/navigation unit
	case .link: return t("Link")
	// TRANSLATORS: Name of the "section" reading/navigation unit
	case .section: return t("Section")
	// TRANSLATORS: Name of the "page" reading/navigation unit
	case .page: return t("Page")
	// TRANSLATORS: Name of the "list" reading/navigation unit
	case .list: return t("List")
	// TRANSLATORS: Name of the "list item" reading/navigation unit
	case .listItem: return t("List Item")
	// TRANSLATORS: Name of the "table" reading/navigation unit
	case .table: return t("Table")
	// TRANSLATORS: Name of the "separator" reading/navigation unit
	case .separator: return t("Separator")
	// TRANSLATORS: Name of the "image" reading/navigation unit
	case .image: return t("Image")
	// TRANSLATORS: Name of the "figure" reading/navigation unit
	case .figure: return t("Figure")
	// TRANSLATORS: Name of the "formula" reading/navigation unit
	case .formula: return t("Formula")
	}
}

enum GoToMode {
	case line, page, percent
}

struct SearchOptions: Equatable {
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
