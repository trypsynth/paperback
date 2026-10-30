import SwiftUI
import Combine
import UIKit

@MainActor
@Observable
final class AppViewModel {
	var tabs: [DocumentTab] = []
	// TTS stops on a tab switch because a paused buffer stays scheduled on the audio node and would bleed into the next book.
	var activeTabId: UUID? = nil {
		didSet {
			guard activeTabId != oldValue else { return }
			reading.ttsManager.stop()
			speech.load(bookAt: activeTab?.url.path(percentEncoded: false))
			if let tab = activeTab { reading.loadSegment(for: tab) }
		}
	}

	var activeTab: DocumentTab? {
		guard let id = activeTabId else { return nil }
		return tabs.first { $0.id == id }
	}

	var activeSession: DocumentSession? { activeTab?.session }

	let navigation = NavigationRouter()

	let reading = ReadingController()

	let speech = SpeechSettings()

	var restorePreviousDocuments = true {
		didSet { configManager.setAppBool(key: "restore_previous_documents", value: restorePreviousDocuments) }
	}
	var swipeUpMovesForward = true {
		didSet { configManager.setAppBool(key: "swipe_up_moves_forward", value: swipeUpMovesForward) }
	}

	// Spacing and alignment share desktop's keys and values; text size doesn't, since desktop stores points and this scales Dynamic Type.
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
	/// Off by default because the swipe actions on the play button that replace these are not discoverable on their own.
	var hidePrevNextButtons: Bool = false {
		didSet { configManager.setAppBool(key: "hide_prev_next_buttons", value: hidePrevNextButtons) }
	}
	/// 0 system, 1 light, 2 dark: readers often want the page dark while the rest of the phone stays light.
	var appearanceChoice: Int = 0 {
		didSet { configManager.setAppInt(key: "appearance", value: Int32(appearanceChoice)) }
	}
	/// The system's Increase Contrast turns this on too.
	var highContrastText: Bool = false {
		didSet { configManager.setAppBool(key: "high_contrast_text", value: highContrastText) }
	}

	var recentDocuments: [RecentDocument] = []

	let configManager = ConfigManagerFfi()
	@ObservationIgnored private var cancellables = Set<AnyCancellable>()

	init() {
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
		reading.stopAfterFindMatch = configManager.getAppBool(key: "tts_stop_after_find_match", defaultValue: true)
		reading.onStopAfterFindMatchChanged = { [weak self] stop in
			self?.configManager.setAppBool(key: "tts_stop_after_find_match", value: stop)
		}

		let narration = RecordedNarration(config: configManager)
		narration.onClipChanged = { [weak self] position in
			// While it plays, the recording is the authority on where the reader is.
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
		speech.start(config: configManager, tts: ttsManager)
		speech.onRestart = { [weak self] in self?.reading.playCurrentSegment() }

		let savedPause = configManager.getAppString(key: "tts_paragraph_pause_ms", defaultValue: "")
		if let ms = Int(savedPause) {
			ttsManager.paragraphPauseMs = min(max(ms, paragraphPauseRangeMs.lowerBound), paragraphPauseRangeMs.upperBound)
		}

		loadRecentsFromConfig()
		reading.start()
		ttsManager.onParagraphPauseChanged = { [weak self] ms in
			self?.configManager.setAppString(key: "tts_paragraph_pause_ms", value: "\(ms)")
		}

		reading.isTextMode = configManager.getAppBool(key: "text_mode", defaultValue: false)
		reading.onTextModeChanged = { [weak self] isTextMode in
			self?.configManager.setAppBool(key: "text_mode", value: isTextMode)
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

	/// Laid out the first time a document is shown in text mode, and kept while its tab is open.
	@ObservationIgnored private var bookModels: [UUID: BookModel] = [:]

	func bookModel(for tab: DocumentTab) -> BookModel? {
		if let model = bookModels[tab.id] { return model }
		guard let session = tab.session else { return nil }
		let model = BookModel(session: session, layout: ReadingLayout(session: session))
		bookModels[tab.id] = model
		return model
	}

	/// False when the document couldn't be opened, including when it asks for a password.
	@discardableResult
	func openDocument(url: URL, password: String? = nil, track: Bool = true) -> Bool {
		if let existing = tabs.first(where: { $0.url == url }) {
			activeTabId = existing.id
			return true
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
			if let password { configManager.setDocumentPassword(path: path, password: password) }
			reading.updateNowPlaying()
			return true
		} catch {
			if scopeStarted { url.stopAccessingSecurityScopedResource() }
			// The core starts a missing or wrong password's error with this marker, so the reader is asked for one rather than shown a failure.
			if "\(error)".contains("[password_required]") {
				navigation.passwordPromptUrl = url
				return false
			}
			debugMessage = "Error opening '\(url.lastPathComponent)':\n\(error)\n\nPath: \(path)"
			return false
		}
	}

	func openHelpDocument() {
		let preferred = Bundle.main.preferredLocalizations.first ?? "en"
		let lang = preferred.split(separator: "-").first.map(String.init) ?? preferred
		// Probing the bundle rather than a hardcoded language list, which drifted out of sync with the readme files that exist.
		let localizedURL = Bundle.main.url(forResource: "readme-\(lang)", withExtension: "html", subdirectory: "Readmes")
		let fallbackURL = Bundle.main.url(forResource: "readme", withExtension: "html", subdirectory: "Readmes")
		guard let url = localizedURL ?? fallbackURL else {
			// TRANSLATORS: Shown when the bundled Help document fails to load
			debugMessage = t("Failed to load document.")
			return
		}
		openDocument(url: url, track: false)
	}

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

	var supportedExportFormats: [ExportFormat] {
		activeSession?.getSupportedExportFormatsFfi() ?? []
	}

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

	/// A copy because the original may sit behind a security-scoped URL the share sheet cannot read once this returns.
	func copyActiveDocumentForSharing() -> URL? {
		guard let tab = activeTab else { return nil }
		let tempURL = FileManager.default.temporaryDirectory.appendingPathComponent(tab.url.lastPathComponent)
		let scopeStarted = tab.url.startAccessingSecurityScopedResource()
		defer { if scopeStarted { tab.url.stopAccessingSecurityScopedResource() } }
		try? FileManager.default.removeItem(at: tempURL)
		do {
			try FileManager.default.copyItem(at: tab.url, to: tempURL)
		} catch {
			return nil
		}
		return tempURL
	}

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
		bookModels[tab.id] = nil
		let path = tab.url.path(percentEncoded: false)
		if tab.session != nil {
			let position = tabs.first { $0.id == tab.id }?.currentPosition ?? tab.currentPosition
			configManager.setDocumentPosition(path: path, position: position)
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
	}

	private func loadRecentsFromConfig() {
		let paths = configManager.getRecentDocuments()
		let openPaths = Set(tabs.map { $0.url.path(percentEncoded: false) })
		recentDocuments = paths.map { path in
			// Files picked from outside the app's container aren't readable by bare path once the picker's grant ends, so the bookmark is needed.
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

	func locateRecentDocument(_ oldURL: URL, at newURL: URL) {
		configManager.renameDocumentPath(
			oldPath: oldURL.path(percentEncoded: false),
			newPath: newURL.path(percentEncoded: false)
		)
		loadRecentsFromConfig()
	}

	/// Open tabs are left alone: a document being read is not one the reader is finished with.
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

	func persistPosition(_ position: Int64) {
		updateTabPosition(position)
	}
}

enum NavUnit: Hashable {
	case segment(SegmentTypeFfi)
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

func formatDuration(_ ms: Int64) -> String {
	let totalSeconds = (max(ms, 0) + 500) / 1000
	let hours = totalSeconds / 3600
	let minutes = (totalSeconds % 3600) / 60
	let seconds = totalSeconds % 60
	return hours > 0
		? String(format: "%d:%02d:%02d", hours, minutes, seconds)
		: String(format: "%d:%02d", minutes, seconds)
}

/// Matches the presets desktop shows in its Options dialog.
let audioSeekAmountsSeconds = [5, 10, 30, 60, 120]

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

/// A whole phrase per unit, because many languages inflect "previous"/"next" to agree with the noun (#1033).
func previousUnitLabel(_ type: SegmentTypeFfi) -> String {
	switch type {
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by paragraph
	case .paragraph: return t("Previous Paragraph")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by line
	case .line: return t("Previous Line")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by heading
	case .heading: return t("Previous Heading")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by link
	case .link: return t("Previous Link")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by section
	case .section: return t("Previous Section")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by page
	case .page: return t("Previous Page")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by list
	case .list: return t("Previous List")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by list item
	case .listItem: return t("Previous List Item")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by table
	case .table: return t("Previous Table")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by separator
	case .separator: return t("Previous Separator")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by image
	case .image: return t("Previous Image")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by figure
	case .figure: return t("Previous Figure")
	// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by formula
	case .formula: return t("Previous Formula")
	}
}

/// A whole phrase per unit, because many languages inflect "previous"/"next" to agree with the noun (#1033).
func nextUnitLabel(_ type: SegmentTypeFfi) -> String {
	switch type {
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by paragraph
	case .paragraph: return t("Next Paragraph")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by line
	case .line: return t("Next Line")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by heading
	case .heading: return t("Next Heading")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by link
	case .link: return t("Next Link")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by section
	case .section: return t("Next Section")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by page
	case .page: return t("Next Page")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by list
	case .list: return t("Next List")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by list item
	case .listItem: return t("Next List Item")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by table
	case .table: return t("Next Table")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by separator
	case .separator: return t("Next Separator")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by image
	case .image: return t("Next Image")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by figure
	case .figure: return t("Next Figure")
	// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by formula
	case .formula: return t("Next Formula")
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
