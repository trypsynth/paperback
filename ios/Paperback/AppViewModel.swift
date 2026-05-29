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

	// MARK: - Settings (will be backed by ConfigManagerFfi)
	@Published var restorePreviousDocuments = true

	// MARK: - Recents
	@Published var recentDocuments: [RecentDocument] = []

	init() {
		ttsManager.onUtteranceFinished = { [weak self] in
			self?.playNextSegment()
		}
	}

	// MARK: - Document management

	func openDocument(url: URL, password: String? = nil) {
		// TODO: call DocumentSession.newFfi() once UniFFI is wired up
		let title = url.deletingPathExtension().lastPathComponent
		if let existing = tabs.first(where: { $0.url == url }) {
			activeTabId = existing.id
			return
		}
		let tab = DocumentTab(title: title, url: url)
		tabs.append(tab)
		activeTabId = tab.id
		addRecentDocument(url: url, title: title)
	}

	func closeTab(_ tab: DocumentTab) {
		tabs.removeAll { $0.id == tab.id }
		if activeTabId == tab.id {
			activeTabId = tabs.last?.id
		}
	}

	func setActiveTab(_ tab: DocumentTab) {
		activeTabId = tab.id
	}

	// MARK: - Recents

	func addRecentDocument(url: URL, title: String) {
		recentDocuments.removeAll { $0.url == url }
		recentDocuments.insert(RecentDocument(title: title, url: url), at: 0)
	}

	func removeRecentDocument(url: URL) {
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
		// TODO: advance ttsPosition via DocumentSession and refresh segment text
	}

	func playPrevSegment() {
		// TODO: rewind ttsPosition via DocumentSession and refresh segment text
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
	}

	func clearSearch() {
		activeSearchQuery = nil
	}

	func findNext() {
		// TODO: implement via DocumentSession.searchFfi
	}

	func findPrev() {
		// TODO: implement via DocumentSession.searchFfi
	}
}

// MARK: - Supporting types

enum SegmentType: String, CaseIterable {
	case paragraph = "Paragraph"
	case line = "Line"
	case heading = "Heading"
	case sentence = "Sentence"
}

enum GoToMode {
	case line, page, percent
}

struct SearchOptions {
	var matchCase: Bool = false
	var wholeWord: Bool = false
	var regex: Bool = false
}
