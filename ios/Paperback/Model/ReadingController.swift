import SwiftUI
import UIKit
import MediaPlayer

// The document state reading needs but does not own: which document is open, and where the
// per-tab reading position and text-mode scroll offset get stored.
@MainActor
protocol ReadingContext: AnyObject {
	var activeSession: DocumentSession? { get }
	var activeTitle: String? { get }
	var activeLineScrollIndex: Int { get set }
	func persistPosition(_ position: Int64)
}

// Everything about reading the open document: playback and its prefetching, the reading
// position, segment navigation, search, the sleep timer, and the Now Playing/remote command
// surface. Split out of AppViewModel, which keeps tabs, recents, and config.
@MainActor
@Observable
final class ReadingController {
	@ObservationIgnored weak var context: ReadingContext?

	private var activeSession: DocumentSession? { context?.activeSession }

	// Called once the context is set, since both of these need it.
	func start() {
		if let data = UserDefaults.standard.data(forKey: "tts_rules"),
		   let loaded = try? JSONDecoder().decode([TtsRule].self, from: data) {
			ttsRules = loaded
			ttsManager.rules = loaded
		}
		ttsManager.onUtteranceFinished = { [weak self] in
			self?.advanceTtsAfterUtterance()
			self?.updateNowPlaying()
		}
		ttsManager.onPlaybackStateChanged = { [weak self] in
			self?.updateNowPlaying()
		}
		setupRemoteCommands()
	}

	var isTextMode: Bool = false
	// Tracks the first visible 0-indexed line in TextModeView; updated eagerly while scrolling.
	var textModeFirstLine: Int = 0

	let ttsManager = TtsManager()
	var ttsPosition: Int64 = 0
	var currentSegmentText: String = ""
	var currentSegmentType: SegmentType = .paragraph
	var ttsRules: [TtsRule] = [] {
		didSet {
			ttsManager.rules = ttsRules
			if let data = try? JSONEncoder().encode(ttsRules) {
				UserDefaults.standard.set(data, forKey: "tts_rules")
			}
		}
	}

	var activeSearchQuery: String? = nil
	var searchOptions = SearchOptions()

	var sleepTimerRemaining: Int? = nil
	private var sleepTimerTask: Task<Void, Never>? = nil

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
		guard let session = activeSession else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .next
		)
		if seg.text.isEmpty { return false }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		context?.persistPosition(seg.startPos)
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
		guard let session = activeSession else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: ffiSegmentType(currentSegmentType),
			direction: .previous
		)
		if seg.text.isEmpty || seg.startPos == ttsPosition { return false }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		context?.persistPosition(seg.startPos)
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
		guard let session = activeSession else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: continuousPlaybackSegmentType(),
			direction: .next
		)
		if seg.text.isEmpty { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		context?.persistPosition(seg.startPos)
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
		guard let session = activeSession else { return }
		let seg = session.getTextSegment(position: ttsPosition, segmentType: type, direction: direction)
		if seg.text.isEmpty { return }
		if direction == .previous && seg.startPos == ttsPosition { return }
		ttsPosition = seg.startPos
		currentSegmentText = seg.text
		context?.persistPosition(seg.startPos)
		if ttsManager.isSpeaking {
			ttsManager.speak(seg.text)
			prefetchAdjacentSegments(around: seg.startPos)
		} else {
			if ttsManager.isPaused { ttsManager.stop() }
			announceNavigationCue(seg.text)
		}
	}

	func setSleepTimer(seconds: Int) {
		cancelSleepTimer()
		sleepTimerRemaining = seconds
		sleepTimerTask = Task {
			while true {
				try? await Task.sleep(for: .seconds(1))
				if Task.isCancelled { return }
				guard let r = sleepTimerRemaining, r > 0 else {
					ttsManager.pause()
					UIApplication.shared.isIdleTimerDisabled = false
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
			context?.persistPosition(result.position)
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
			context?.persistPosition(result.position)
			refreshCurrentSegment()
		}
	}

	func goToLine(_ line: Int64) {
		guard let session = activeSession else { return }
		let pos = session.positionFromLine(line: line)
		ttsPosition = pos
		context?.persistPosition(pos)
		refreshCurrentSegment()
	}

	func goToPosition(_ position: Int64) {
		ttsPosition = position
		context?.persistPosition(position)
		refreshCurrentSegment()
	}

	func goToPage(_ page: Int32) {
		guard let session = activeSession else { return }
		let pos = session.pageOffset(page: page)
		ttsPosition = pos
		context?.persistPosition(pos)
		refreshCurrentSegment()
	}

	func goToPercent(_ percent: Int32) {
		guard let session = activeSession else { return }
		let pos = session.positionFromPercent(percent: percent)
		ttsPosition = pos
		context?.persistPosition(pos)
		refreshCurrentSegment()
	}

	func loadSegment(for tab: DocumentTab) {
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
		guard let session = activeSession else { return }
		let line = session.lineFromPosition(position: ttsPosition)
		let scrollIdx = max(0, Int(line) - 1)
		context?.activeLineScrollIndex = scrollIdx
		textModeFirstLine = scrollIdx
	}

	private func exitTextMode() {
		guard let session = activeSession else { return }
		let pos = session.positionFromLine(line: Int64(textModeFirstLine + 1))
		ttsPosition = pos
		context?.persistPosition(pos)
		refreshCurrentSegment()
		context?.activeLineScrollIndex = textModeFirstLine
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
		if let title = context?.activeTitle {
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
