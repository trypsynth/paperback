import SwiftUI
import Combine
import UIKit
import MediaPlayer

@MainActor
final class AppViewModel: ObservableObject {
	// MARK: - Tabs
	@Published var tabs: [DocumentTab] = []
	// Stops TTS whenever the active document changes so a paused/playing utterance from the
	// previous book can never bleed into the next one (its buffer stays scheduled on the audio
	// node across pause() until something clears it — see TtsManager.pause()).
	@Published var activeTabId: UUID? = nil {
		didSet {
			guard activeTabId != oldValue else { return }
			ttsManager.stop()
		}
	}

	var activeTab: DocumentTab? {
		guard let id = activeTabId else { return nil }
		return tabs.first { $0.id == id }
	}

	var activeSession: DocumentSession? { activeTab?.session }

	// MARK: - Reading mode
	@Published var isTextMode: Bool = false
	// Tracks the first visible 0-indexed line in TextModeView; updated eagerly while scrolling.
	var textModeFirstLine: Int = 0

	// MARK: - TTS
	let ttsManager = TtsManager()
	@Published var ttsPosition: Int64 = 0
	@Published var currentSegmentText: String = ""
	@Published var currentSegmentType: SegmentType = .paragraph
	@Published var ttsRules: [TtsRule] = []

	// MARK: - Search
	@Published var activeSearchQuery: String? = nil
	@Published var searchOptions = SearchOptions()

	// MARK: - Sleep timer
	@Published var sleepTimerRemaining: Int? = nil
	private var sleepTimerTask: Task<Void, Never>? = nil

	// MARK: - Sheet visibility
	@Published var showToc = false
	@Published var showFind = false
	@Published var showGoTo = false
	@Published var goToInitialMode: GoToMode = .line
	@Published var showSettings = false
	@Published var showRecents = false
	@Published var showWordCount = false
	@Published var showDocumentInfo = false
	@Published var showSleepTimer = false
	@Published var showElements = false
	@Published var passwordPromptUrl: URL? = nil

	// MARK: - Settings
	@Published var restorePreviousDocuments = true
	@Published var swipeUpMovesForward = true

	// MARK: - Recents
	@Published var recentDocuments: [RecentDocument] = []

	// MARK: - Config
	let configManager = ConfigManagerFfi()
	private var cancellables = Set<AnyCancellable>()

	init() {
		setPdfiumLibraryPath(path: Bundle.main.bundlePath + "/Frameworks/libpdfium.framework")

		let configPath = configFilePath()
		_ = configManager.initialize(configPath: configPath)
		restorePreviousDocuments = configManager.getAppBool(key: "restore_previous_documents", defaultValue: true)
		swipeUpMovesForward = configManager.getAppBool(key: "swipe_up_moves_forward", defaultValue: true)

		let savedRate = configManager.getAppString(key: "tts_speech_rate", defaultValue: "")
		if let r = Float(savedRate) { ttsManager.speechRate = r }

		let savedPitch = configManager.getAppString(key: "tts_pitch", defaultValue: "")
		if let p = Float(savedPitch) { ttsManager.pitch = p }

		let savedVoice = configManager.getAppString(key: "tts_voice_identifier", defaultValue: "")
		if !savedVoice.isEmpty { ttsManager.selectedVoiceIdentifier = savedVoice }

		loadRecentsFromConfig()
		ttsManager.onUtteranceFinished = { [weak self] in
			self?.advanceTtsAfterUtterance()
			self?.updateNowPlaying()
		}
		ttsManager.$isSpeaking
			.dropFirst()
			.sink { [weak self] _ in
				self?.objectWillChange.send()
				self?.updateNowPlaying()
			}
			.store(in: &cancellables)
		ttsManager.$isPaused
			.dropFirst()
			.sink { [weak self] _ in
				self?.objectWillChange.send()
				self?.updateNowPlaying()
			}
			.store(in: &cancellables)
		$restorePreviousDocuments
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppBool(key: "restore_previous_documents", value: value)
			}
			.store(in: &cancellables)
		$swipeUpMovesForward
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppBool(key: "swipe_up_moves_forward", value: value)
			}
			.store(in: &cancellables)
		ttsManager.$speechRate
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppString(key: "tts_speech_rate", value: "\(value)")
			}
			.store(in: &cancellables)
		ttsManager.$pitch
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppString(key: "tts_pitch", value: "\(value)")
			}
			.store(in: &cancellables)
		ttsManager.$selectedVoiceIdentifier
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppString(key: "tts_voice_identifier", value: value ?? "")
			}
			.store(in: &cancellables)

		if let data = UserDefaults.standard.data(forKey: "tts_rules"),
		   let loaded = try? JSONDecoder().decode([TtsRule].self, from: data) {
			ttsRules = loaded
			ttsManager.rules = loaded
		}
		$ttsRules
			.dropFirst()
			.sink { [weak self] rules in
				self?.ttsManager.rules = rules
				if let data = try? JSONEncoder().encode(rules) {
					UserDefaults.standard.set(data, forKey: "tts_rules")
				}
			}
			.store(in: &cancellables)

		setupRemoteCommands()
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
					self?.togglePlayPause()
				}
			}
			.store(in: &cancellables)
		updateNowPlaying()
	}

	// MARK: - Document management

	@Published var debugMessage: String? = nil

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
			loadSegment(for: tab)
			updateNowPlaying()
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
			ttsPosition = savedPos
			refreshCurrentSegment()
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
			loadSegment(for: t)
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

	// MARK: - TTS

	func togglePlayPause() {
		if ttsManager.isSpeaking {
			ttsManager.pause()
		} else if ttsManager.isPaused {
			ttsManager.resume()
		} else {
			playCurrentSegment()
		}
		updateNowPlaying()
	}

	func playCurrentSegment() {
		guard !currentSegmentText.isEmpty else { return }
		ttsManager.speak(currentSegmentText)
		prefetchAdjacentSegments(around: ttsPosition)
	}

	@discardableResult
	func playNextSegment(speak: Bool = true, announce: Bool = false) -> Bool {
		guard let tab = activeTab, let session = tab.session else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .next
		)
		if seg.text.isEmpty { return false }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		if speak {
			ttsManager.speak(seg.text)
			prefetchAdjacentSegments(around: seg.startPos)
		} else {
			// Discard any paused buffer so pressing play starts at the new position.
			if ttsManager.isPaused { ttsManager.stop() }
			if announce { announceNavigationCue(seg.text) }
		}
		return true
	}

	@discardableResult
	func playPrevSegment(speak: Bool = true, announce: Bool = false) -> Bool {
		guard let tab = activeTab, let session = tab.session else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .previous
		)
		if seg.text.isEmpty || seg.startPos == ttsPosition { return false }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		if speak {
			ttsManager.speak(seg.text)
			prefetchAdjacentSegments(around: seg.startPos)
		} else {
			// Discard any paused buffer so pressing play starts at the new position.
			if ttsManager.isPaused { ttsManager.stop() }
			if announce { announceNavigationCue(seg.text) }
		}
		return true
	}

	// Advances playback after an utterance finishes. Unlike playNextSegment(), this always
	// walks by actual readable content rather than currentSegmentType: heading/section are marker
	// jumps that only return the marker's title text, so using them here would make continuous
	// playback read a heading, then skip straight to the next one, forever.
	private func advanceTtsAfterUtterance() {
		guard let tab = activeTab, let session = tab.session else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: continuousPlaybackSegmentType(),
			direction: .next
		)
		if seg.text.isEmpty { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		ttsManager.speak(seg.text, isAutoAdvance: true)
		prefetchAdjacentSegments(around: seg.startPos)
	}

	// The segment type continuous TTS playback should walk by, regardless of the user's chosen
	// navigation unit. Paragraph/line are real sequential content; heading/section are marker
	// jumps and must fall back to paragraph so playback doesn't skip the body between markers.
	private func continuousPlaybackSegmentType() -> SegmentTypeFfi {
		switch currentSegmentType {
		case .paragraph, .line: return ffiSegmentType(currentSegmentType)
		case .heading, .section: return .paragraph
		}
	}

	private func announceNavigationCue(_ text: String) {
		let words = text.split(whereSeparator: \.isWhitespace)
		let cue = words.prefix(5).joined(separator: " ")
		// Delay so SwiftUI's layout-changed accessibility notification fires first;
		// otherwise it interrupts the announcement when triggered by a button tap.
		Task { @MainActor in
			try? await Task.sleep(for: .milliseconds(150))
			UIAccessibility.post(notification: .announcement, argument: cue)
		}
	}

	private func prefetchAdjacentSegments(around position: Int64) {
		guard let session = activeSession else { return }
		let type = continuousPlaybackSegmentType()
		let next = session.getTextSegment(position: position, segmentType: type, direction: .next)
		var upcoming: [String] = []
		if !next.text.isEmpty {
			upcoming.append(next.text)
			let nextNext = session.getTextSegment(position: next.startPos, segmentType: type, direction: .next)
			if !nextNext.text.isEmpty {
				upcoming.append(nextNext.text)
			}
		}
		ttsManager.prefetch(upcoming: upcoming)

		let prev = session.getTextSegment(position: position, segmentType: type, direction: .previous)
		if !prev.text.isEmpty {
			ttsManager.prefetchPrev(prev.text)
		}
	}

	func changeSegmentType(_ type: SegmentType) {
		currentSegmentType = type
	}

	func navigateByType(_ type: SegmentTypeFfi, direction: SegmentDirectionFfi) {
		guard let tab = activeTab, let session = tab.session else { return }
		let seg = session.getTextSegment(position: ttsPosition, segmentType: type, direction: direction)
		if seg.text.isEmpty { return }
		if direction == .previous && seg.startPos == ttsPosition { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		if ttsManager.isSpeaking {
			ttsManager.speak(seg.text)
			prefetchAdjacentSegments(around: seg.startPos)
		} else {
			if ttsManager.isPaused { ttsManager.stop() }
			announceNavigationCue(seg.text)
		}
	}

	// MARK: - Sleep timer

	func setSleepTimer(seconds: Int) {
		cancelSleepTimer()
		sleepTimerRemaining = seconds
		sleepTimerTask = Task {
			while true {
				try? await Task.sleep(for: .seconds(1))
				if Task.isCancelled { return }
				guard let r = sleepTimerRemaining, r > 0 else {
					ttsManager.pause()
					return
				}
				sleepTimerRemaining = r - 1
			}
		}
	}

	func cancelSleepTimer() {
		sleepTimerTask?.cancel()
		sleepTimerTask = nil
		sleepTimerRemaining = nil
	}

	// MARK: - Search

	// Starts (or re-runs) a search and immediately jumps to the first match in the given
	// direction, matching desktop/Android: there's no separate "start search" step, pressing
	// Find Previous/Next both sets the active query and jumps in one action.
	func startSearch(query: String, options: SearchOptions, forward: Bool) {
		activeSearchQuery = query
		searchOptions = options
		if forward {
			findNext(fromQuery: query, options: options)
		} else {
			findPrev(fromQuery: query, options: options)
		}
	}

	func findNext(fromQuery: String? = nil, options: SearchOptions? = nil) {
		guard let session = activeSession else { return }
		let q = fromQuery ?? activeSearchQuery ?? ""
		let opts = options ?? searchOptions
		let result = session.searchFfi(
			query: q,
			startPosition: ttsPosition,
			options: SearchOptionsFfi(
				matchCase: opts.matchCase,
				wholeWord: opts.wholeWord,
				regex: opts.regex,
				forward: true
			)
		)
		if result.found {
			ttsPosition = result.position
			updateTabPosition(result.position)
			refreshCurrentSegment()
		}
	}

	func findPrev(fromQuery: String? = nil, options: SearchOptions? = nil) {
		guard let session = activeSession else { return }
		let q = fromQuery ?? activeSearchQuery ?? ""
		let opts = options ?? searchOptions
		let result = session.searchFfi(
			query: q,
			startPosition: ttsPosition,
			options: SearchOptionsFfi(
				matchCase: opts.matchCase,
				wholeWord: opts.wholeWord,
				regex: opts.regex,
				forward: false
			)
		)
		if result.found {
			ttsPosition = result.position
			updateTabPosition(result.position)
			refreshCurrentSegment()
		}
	}

	// MARK: - Navigation

	func goToLine(_ line: Int64) {
		guard let session = activeSession else { return }
		let pos = session.positionFromLine(line: line)
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
	}

	func goToPosition(_ position: Int64) {
		ttsPosition = position
		updateTabPosition(position)
		refreshCurrentSegment()
	}

	func goToPage(_ page: Int32) {
		guard let session = activeSession else { return }
		let pos = session.pageOffset(page: page)
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
	}

	func goToPercent(_ percent: Int32) {
		guard let session = activeSession else { return }
		let pos = session.positionFromPercent(percent: percent)
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
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

	private func loadSegment(for tab: DocumentTab) {
		guard let session = tab.session else { return }
		ttsPosition = tab.currentPosition
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .current
		)
		currentSegmentText = seg.text
	}

	private func refreshCurrentSegment() {
		guard let session = activeSession else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .current
		)
		currentSegmentText = seg.text
	}

	private func updateTabPosition(_ position: Int64) {
		guard let id = activeTabId,
		      let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
		tabs[idx].currentPosition = position
		let path = tabs[idx].url.path(percentEncoded: false)
		configManager.setDocumentPosition(path: path, position: position)
	}

	// Computes and stores the text-mode scroll position BEFORE flipping isTextMode, rather than
	// reacting to the flag afterward: TextModeView's initial scroll only runs once, on its own
	// .onAppear, when it first mounts. If isTextMode flipped first, it would mount and scroll
	// using the still-default (0) lineScrollIndex before this had a chance to update it, and
	// that one-shot scroll wouldn't re-run once the real position was computed a moment later —
	// landing the user at the start of the book instead of where they were reading.
	func toggleTextMode() {
		if isTextMode {
			exitTextMode()
			isTextMode = false
		} else {
			enterTextMode()
			isTextMode = true
		}
	}

	private func enterTextMode() {
		guard let session = activeSession,
		      let id = activeTabId,
		      let idx = tabs.firstIndex(where: { $0.id == id }) else { return }
		let line = session.lineFromPosition(position: ttsPosition)
		let scrollIdx = max(0, Int(line) - 1)
		tabs[idx].lineScrollIndex = scrollIdx
		textModeFirstLine = scrollIdx
	}

	private func exitTextMode() {
		guard let session = activeSession else { return }
		let pos = session.positionFromLine(line: Int64(textModeFirstLine + 1))
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
		if let id = activeTabId, let idx = tabs.firstIndex(where: { $0.id == id }) {
			tabs[idx].lineScrollIndex = textModeFirstLine
		}
	}

	private func setupRemoteCommands() {
		let center = MPRemoteCommandCenter.shared()

		center.playCommand.addTarget { [weak self] _ in
			guard let self, !ttsManager.suppressExternalPlay else { return .success }
			if ttsManager.isPaused { ttsManager.resume() }
			else if !ttsManager.isSpeaking { playCurrentSegment() }
			updateNowPlaying()
			return .success
		}
		center.pauseCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			ttsManager.pause()
			updateNowPlaying()
			return .success
		}
		center.togglePlayPauseCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			togglePlayPause()
			updateNowPlaying()
			return .success
		}
		center.nextTrackCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			playNextSegment(speak: ttsManager.isSpeaking)
			updateNowPlaying()
			return .success
		}
		center.previousTrackCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			playPrevSegment(speak: ttsManager.isSpeaking)
			updateNowPlaying()
			return .success
		}

		center.stopCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			ttsManager.stop()
			updateNowPlaying()
			return .success
		}

		// Disable commands that don't apply to a book reader
		center.skipForwardCommand.isEnabled = false
		center.skipBackwardCommand.isEnabled = false
		center.seekForwardCommand.isEnabled = false
		center.seekBackwardCommand.isEnabled = false
		center.changePlaybackRateCommand.isEnabled = false
	}

	func updateNowPlaying() {
		var info: [String: Any] = [
			MPMediaItemPropertyMediaType: MPMediaType.audioBook.rawValue,
			MPNowPlayingInfoPropertyPlaybackRate: ttsManager.isSpeaking ? 1.0 : 0.0,
			MPNowPlayingInfoPropertyDefaultPlaybackRate: 1.0,
		]
		if let title = activeTab?.title {
			info[MPMediaItemPropertyTitle] = title
		}
		info[MPMediaItemPropertyArtist] = "Paperback"
		MPNowPlayingInfoCenter.default().nowPlayingInfo = info
	}

	private func ffiSegmentType(_ type: SegmentType) -> SegmentTypeFfi {
		switch type {
		case .paragraph: return .paragraph
		case .line: return .line
		case .heading: return .heading
		case .section: return .section
		}
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
