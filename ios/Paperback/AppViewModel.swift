import SwiftUI
import Combine

@MainActor
final class AppViewModel: ObservableObject {
	// MARK: - Tabs
	@Published var tabs: [DocumentTab] = []
	@Published var activeTabId: UUID? = nil

	var activeTab: DocumentTab? {
		guard let id = activeTabId else { return nil }
		return tabs.first { $0.id == id }
	}

	var activeSession: DocumentSession? { activeTab?.session }

	// MARK: - Reading mode
	@Published var isTextMode: Bool = false

	// MARK: - TTS
	let ttsManager = TtsManager()
	@Published var ttsPosition: Int64 = 0
	@Published var currentSegmentText: String = ""
	@Published var currentSegmentType: SegmentType = .paragraph

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

	// MARK: - Recents
	@Published var recentDocuments: [RecentDocument] = []

	// MARK: - Config
	let configManager = ConfigManagerFfi()
	private var cancellables = Set<AnyCancellable>()

	init() {
		let configPath = configFilePath()
		_ = configManager.initialize(configPath: configPath)
		restorePreviousDocuments = configManager.getAppBool(key: "restore_previous_documents", defaultValue: true)
		loadRecentsFromConfig()
		ttsManager.onUtteranceFinished = { [weak self] in
			self?.playNextSegment()
		}
		$restorePreviousDocuments
			.dropFirst()
			.sink { [weak self] value in
				self?.configManager.setAppBool(key: "restore_previous_documents", value: value)
			}
			.store(in: &cancellables)
		if restorePreviousDocuments {
			for path in configManager.getOpenedDocuments() {
				tryRestoreDocument(path: path)
			}
		}
		NotificationCenter.default.publisher(for: .pbMagicTap)
			.sink { [weak self] _ in
				Task { @MainActor [weak self] in
					self?.togglePlayPause()
				}
			}
			.store(in: &cancellables)
	}

	// MARK: - Document management

	@Published var debugMessage: String? = nil

	func openDocument(url: URL, password: String? = nil) {
		if let existing = tabs.first(where: { $0.url == url }) {
			activeTabId = existing.id
			return
		}
		let path = url.path(percentEncoded: false)
		let pass = password ?? configManager.getDocumentPassword(path: path)
		do {
			let session = try DocumentSession.newFfi(
				filePath: path,
				password: pass,
				forcedExtension: ""
			)
			let title = session.title().isEmpty
				? url.deletingPathExtension().lastPathComponent
				: session.title()
			let savedPos = configManager.getDocumentPosition(path: path)
			var tab = DocumentTab(title: title, url: url, session: session)
			tab.currentPosition = savedPos
			tabs.append(tab)
			activeTabId = tab.id
			configManager.addRecentDocument(path: path)
			configManager.addOpenedDocument(path: path)
			loadRecentsFromConfig()
			loadSegment(for: tab)
		} catch {
			debugMessage = "Error opening '\(url.lastPathComponent)':\n\(error)\n\nPath: \(path)"
		}
	}

	func closeTab(_ tab: DocumentTab) {
		let path = tab.url.path(percentEncoded: false)
		if tab.session != nil {
			configManager.setDocumentPosition(path: path, position: tab.currentPosition)
		}
		configManager.removeOpenedDocument(path: path)
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
		recentDocuments = paths.compactMap { path -> RecentDocument? in
			let url = URL(fileURLWithPath: path)
			guard url.path != path || FileManager.default.fileExists(atPath: path) else { return nil }
			let title = url.deletingPathExtension().lastPathComponent
			return RecentDocument(title: title, url: url)
		}
	}

	func addRecentDocument(url: URL, title: String) {
		configManager.addRecentDocument(path: url.path(percentEncoded: false))
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
	}

	func playCurrentSegment() {
		guard !currentSegmentText.isEmpty else { return }
		ttsManager.speak(currentSegmentText)
	}

	func playNextSegment() {
		guard let tab = activeTab, let session = tab.session else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .next
		)
		if seg.text.isEmpty { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		ttsManager.speak(seg.text)
	}

	func playPrevSegment() {
		guard let tab = activeTab, let session = tab.session else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .previous
		)
		if seg.text.isEmpty { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		updateTabPosition(seg.startPos)
		ttsManager.speak(seg.text)
	}

	func changeSegmentType(_ type: SegmentType) {
		currentSegmentType = type
		guard let tab = activeTab, let session = tab.session else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(type),
			direction: .current
		)
		currentSegmentText = seg.text
		ttsPosition = seg.startPos
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

	func startSearch(query: String, options: SearchOptions) {
		activeSearchQuery = query
		searchOptions = options
		findNext(fromQuery: query, options: options)
	}

	func clearSearch() {
		activeSearchQuery = nil
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

	func findPrev() {
		guard let session = activeSession else { return }
		let q = activeSearchQuery ?? ""
		let result = session.searchFfi(
			query: q,
			startPosition: ttsPosition,
			options: SearchOptionsFfi(
				matchCase: searchOptions.matchCase,
				wholeWord: searchOptions.wholeWord,
				regex: searchOptions.regex,
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
		let pos = session.pageOffsetFfi(page: page)
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
	}

	func goToPercent(_ percent: Int32) {
		guard let session = activeSession else { return }
		let pos = session.positionFromPercentFfi(percent: percent)
		ttsPosition = pos
		updateTabPosition(pos)
		refreshCurrentSegment()
	}

	// MARK: - Private helpers

	private func tryRestoreDocument(path: String) {
		guard FileManager.default.fileExists(atPath: path) else { return }
		let url = URL(fileURLWithPath: path)
		guard let session = try? DocumentSession.newFfi(
			filePath: path,
			password: configManager.getDocumentPassword(path: path),
			forcedExtension: ""
		) else { return }
		let title = session.title().isEmpty
			? url.deletingPathExtension().lastPathComponent
			: session.title()
		var tab = DocumentTab(title: title, url: url, session: session)
		tab.currentPosition = configManager.getDocumentPosition(path: path)
		tabs.append(tab)
		if activeTabId == nil { activeTabId = tab.id }
		loadSegment(for: tab)
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
