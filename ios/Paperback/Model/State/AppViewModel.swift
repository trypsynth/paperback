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
			configManager.setAppString(key: "active_document", value: activeTab?.url.path(percentEncoded: false) ?? "")
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
	var pageLayout: PageLayout = .automatic {
		didSet {
			guard pageLayout != oldValue else { return }
			configManager.setPageLayout(layout: pageLayout)
			readAgain(tabs.filter { tab in
				let path = tab.url.path(percentEncoded: false)
				return configManager.pageLayoutAppliesTo(path: path) && configManager.getDocumentPageLayout(path: path) == nil
			})
		}
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
		pageLayout = configManager.getPageLayout()

		reading.context = self
		reading.onRequestBookmarkNote = { [weak self] target in
			self?.navigation.bookmarkNote = target
		}
		reading.onRequestBookmarkRemoval = { [weak self] bookmark in
			self?.navigation.bookmarkRemoval = bookmark
		}
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
			restoreDocuments()
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
		do {
			let session = try Self.openSession(path: path, password: password ?? configManager.getDocumentPassword(path: path), pageLayout: configManager.pageLayoutFor(path: path))
			activeTabId = addTab(url: url, session: session, scopeStarted: scopeStarted)
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
			reportOpenFailure(error, url: url)
			return false
		}
	}

	nonisolated private static func openSession(path: String, password: String, pageLayout: PageLayout) throws -> DocumentSession {
		try DocumentSession.newFfi(filePath: path, password: password, forcedExtension: "", renderTablesInline: false, pageLayout: pageLayout)
	}

	/// Nil when the active document isn't laid out as pages, and so has no page layout to choose.
	var activeDocumentPageLayout: PageLayout?? {
		_ = pageLayoutRevision
		guard let path = activeDocPath, configManager.pageLayoutAppliesTo(path: path) else { return nil }
		return .some(configManager.getDocumentPageLayout(path: path))
	}

	/// Nil gives the active document back to the page layout in Settings; it is read again only if that changes how it reads, as on desktop.
	func setActiveDocumentPageLayout(_ layout: PageLayout?) {
		guard let tab = activeTab else { return }
		let path = tab.url.path(percentEncoded: false)
		guard configManager.pageLayoutAppliesTo(path: path) else { return }
		let before = configManager.pageLayoutFor(path: path)
		configManager.setDocumentPageLayout(path: path, layout: layout)
		configManager.flush()
		if configManager.pageLayoutFor(path: path) != before { readAgain([tab]) }
		pageLayoutRevision += 1
	}

	/// The document's own layout lives only in the core, so the menu showing it is told of a change through this.
	private(set) var pageLayoutRevision = 0

	/// Kept so a test can wait for the documents a page layout change reads again.
	@ObservationIgnored private(set) var readAgainTask: Task<Void, Never>?

	/// Parsed off the main thread, as at launch, since a long PDF takes a while; each waits for the one before, and is dropped if the layout changed again while it parsed.
	private func readAgain(_ readTabs: [DocumentTab]) {
		let requests = readTabs.map { tab in
			let path = tab.url.path(percentEncoded: false)
			return ReadAgainRequest(id: tab.id, path: path, password: configManager.getDocumentPassword(path: path), pageLayout: configManager.pageLayoutFor(path: path))
		}
		guard !requests.isEmpty else { return }
		let previous = readAgainTask
		readAgainTask = Task {
			await previous?.value
			let sessions = await Task.detached {
				requests.map { request in try? Self.openSession(path: request.path, password: request.password, pageLayout: request.pageLayout) }
			}.value
			for (request, session) in zip(requests, sessions) {
				guard let session, configManager.pageLayoutFor(path: request.path) == request.pageLayout else { continue }
				replaceSession(ofTab: request.id, with: session)
			}
		}
	}

	private func replaceSession(ofTab id: UUID, with session: DocumentSession) {
		guard let index = tabs.firstIndex(where: { $0.id == id }), let old = tabs[index].session else { return }
		let isActive = id == activeTabId
		let position = positionAfterReadingAgain(isActive ? reading.ttsPosition : tabs[index].currentPosition, from: old, to: session)
		bookModels[id] = nil
		tabs[index].session = session
		tabs[index].currentPosition = position
		configManager.setDocumentPosition(path: tabs[index].url.path(percentEncoded: false), position: position)
		guard isActive else { return }
		// What is queued to be spoken is the old reading's text, so reading aloud starts again from the same place in the new one.
		let wasSpeaking = reading.ttsManager.isSpeaking
		reading.ttsManager.stop()
		reading.loadSegment(for: tabs[index])
		if wasSpeaking { reading.playCurrentSegment() }
	}

	private func addTab(url: URL, session: DocumentSession, scopeStarted: Bool) -> UUID {
		let path = url.path(percentEncoded: false)
		let title = session.title().isEmpty ? url.deletingPathExtension().lastPathComponent : session.title()
		var tab = DocumentTab(title: title, url: url, session: session)
		if let start = session.startPosition() {
			configManager.seedDocumentPosition(path: path, position: start)
		}
		tab.currentPosition = configManager.getDocumentPosition(path: path)
		tab.securityScopeURL = scopeStarted ? url : nil
		tabs.append(tab)
		return tab.id
	}

	private func reportOpenFailure(_ error: Error, url: URL) {
		// The core starts a missing or wrong password's error with this marker, so the reader is asked for one rather than shown a failure.
		if "\(error)".contains("[password_required]") {
			navigation.passwordPromptUrl = url
			return
		}
		debugMessage = "Error opening '\(url.lastPathComponent)':\n\(error)\n\nPath: \(url.path(percentEncoded: false))"
	}

	/// Kept so a test can wait for the tabs a launch brings back.
	@ObservationIgnored private(set) var restoreTask: Task<Void, Never>?

	/// Parsed off the main thread, since a few large books opened here held up launch until iOS could kill the app.
	private func restoreDocuments() {
		var activePath = configManager.getAppString(key: "active_document", defaultValue: "")
		let requests: [RestoreRequest] = configManager.getOpenedDocuments().compactMap { path in
			guard let url = resolvedURL(forPath: path) else { return nil }
			let password = configManager.getDocumentPassword(path: url.path(percentEncoded: false))
			let pageLayout = configManager.pageLayoutFor(path: url.path(percentEncoded: false))
			return RestoreRequest(path: path, url: url, password: password, pageLayout: pageLayout, scopeStarted: url.startAccessingSecurityScopedResource())
		}
		restoreTask = Task {
			let results = await Task.detached {
				requests.map { request in
					Result { try Self.openSession(path: request.url.path(percentEncoded: false), password: request.password, pageLayout: request.pageLayout) }
				}
			}.value
			// A book opened while these parsed keeps the reader; the restored tabs go in behind it.
			let openedMeanwhile = !tabs.isEmpty
			var restored: [UUID] = []
			for (request, result) in zip(requests, results) {
				let path = request.url.path(percentEncoded: false)
				if path != request.path {
					movePathSettings(from: request.path, to: path)
					if activePath == request.path { activePath = path }
				}
				switch result {
				case .success(let session) where !tabs.contains(where: { $0.url == request.url }):
					restored.append(addTab(url: request.url, session: session, scopeStarted: request.scopeStarted))
					// Refreshed in case it went stale.
					saveBookmark(for: request.url, path: path)
				case .success:
					if request.scopeStarted { request.url.stopAccessingSecurityScopedResource() }
				case .failure(let error):
					if request.scopeStarted { request.url.stopAccessingSecurityScopedResource() }
					reportOpenFailure(error, url: request.url)
				}
			}
			guard !openedMeanwhile, !restored.isEmpty else { return }
			activeTabId = tabs.first(where: { $0.url.path(percentEncoded: false) == activePath })?.id ?? restored.last
			reading.updateNowPlaying()
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
		reading.bookmarksChanged()
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

	/// An open tab of it is closed first, as on Windows, so it isn't left open or brought back at the next launch, and closing it can't write back the position just removed.
	func removeRecentDocument(url: URL) {
		let path = url.path(percentEncoded: false)
		for tab in tabs where tab.url.path(percentEncoded: false) == path {
			closeTab(tab)
		}
		configManager.removeDocumentHistory(path: path)
		configManager.flush()
		UserDefaults.standard.removeObject(forKey: bookmarkKey(path))
		recentDocuments.removeAll { $0.url == url }
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

	/// A file in the app's own folder, such as one downloaded from a link, is found by its bookmark at a new path once an update moves the app's container, so what was kept under the old path moves to the new one. Otherwise closing it wouldn't clear it from the documents restored at launch.
	private func movePathSettings(from oldPath: String, to newPath: String) {
		configManager.renameDocumentPath(oldPath: oldPath, newPath: newPath)
		let defaults = UserDefaults.standard
		if let bookmark = defaults.data(forKey: bookmarkKey(oldPath)) { defaults.set(bookmark, forKey: bookmarkKey(newPath)) }
		defaults.removeObject(forKey: bookmarkKey(oldPath))
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

	func persistNavUnit(_ unit: NavigationUnit) {
		guard let path = activeTab?.url.path(percentEncoded: false) else { return }
		configManager.setDocumentNavigationUnit(path: path, unit: unit)
	}

	func savedNavUnit(for tab: DocumentTab) -> NavigationUnit? {
		configManager.getDocumentNavigationUnit(path: tab.url.path(percentEncoded: false))
	}

	var activeDocPath: String? { activeTab?.url.path(percentEncoded: false) }
}

enum NavUnit: Hashable {
	case segment(SegmentTypeFfi)
	case time(seconds: Int)
	case find
	case bookmark

	init(_ stored: NavigationUnit) {
		switch stored {
		case .segment(segmentType: let type): self = .segment(type)
		case .seconds(seconds: let seconds): self = .time(seconds: Int(seconds))
		}
	}

	/// Find has no stored form, since the search it steps through ends with the document; Bookmark has none so older versions and desktop never read one they don't know.
	var stored: NavigationUnit? {
		switch self {
		case .segment(let type): return .segment(segmentType: type)
		case .time(let seconds): return .seconds(seconds: UInt32(seconds))
		case .find, .bookmark: return nil
		}
	}

	var name: String {
		switch self {
		case .segment(let type): return segmentTypeName(type)
		case .time(let seconds): return seekAmountName(seconds)
		// TRANSLATORS: Name of the "Find" navigation unit, which moves between search matches
		case .find: return t("Find")
		// TRANSLATORS: Name of the "Bookmark" navigation unit, which moves between the reader's bookmarks and notes
		case .bookmark: return t("Bookmark")
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
	case line, page, percent, time
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

/// Restoring doesn't track the documents, since that would move each tab to the top of recents and reorder them by tab instead of by reading on every launch.
private struct RestoreRequest: Sendable {
	let path: String
	let url: URL
	let password: String
	let pageLayout: PageLayout
	let scopeStarted: Bool
}

private struct ReadAgainRequest: Sendable {
	let id: UUID
	let path: String
	let password: String
	let pageLayout: PageLayout
}

/// Kept on the same page and as far into it, since a page holds the same words whatever its layout, though not in the same order or lines.
func positionAfterReadingAgain(_ position: Int64, from old: DocumentSession, to new: DocumentSession) -> Int64 {
	// The core gives the end of the document as 100 percent.
	let (oldEnd, newEnd) = (old.positionFromPercent(percent: 100), new.positionFromPercent(percent: 100))
	let page = old.currentPage(position: position)
	let oldStart = page > 0 ? old.pageOffset(page: page) : 0
	let newStart = page > 0 ? new.pageOffset(page: page) : 0
	guard oldStart >= 0, newStart >= 0 else { return min(max(position, 0) * newEnd / max(oldEnd, 1), newEnd) }
	let oldNext = old.pageOffset(page: page + 1)
	let newNext = new.pageOffset(page: page + 1)
	let oldLength = (page > 0 && oldNext >= 0 ? oldNext : oldEnd) - oldStart
	let newLength = (page > 0 && newNext >= 0 ? newNext : newEnd) - newStart
	return min(newStart + max(position - oldStart, 0) * newLength / max(oldLength, 1), newEnd)
}
