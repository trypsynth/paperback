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
	/// A recorded book's own narration, when the open document has one. Set by AppViewModel,
	/// which owns the config it persists playback position to.
	var narration: RecordedNarration?
	/// Whether the open document carries its own recording, in which case playback is that
	/// rather than speech.
	var hasAudio: Bool { activeSession?.hasAudioFfi() == true }
	/// A document whose text is only there to anchor the audio has nothing to step through but
	/// the recording itself.
	var isAudioOnly: Bool { activeSession?.isAudioOnlyFfi() == true }
	/// Whether the book is being read aloud right now, by its own recording or by speech.
	/// The lock screen and the remote commands both need this rather than either one alone.
	var isPlayingNow: Bool { hasAudio ? narration?.isPlaying == true : ttsManager.isSpeaking }
	var ttsPosition: Int64 = 0 {
		didSet { spokeCurrentSegment = false }
	}
	// Set once an utterance finishes with the cursor left where it was, which is what browsing
	// Find matches does. Pressing play then means "carry on from here", not "read that again".
	private var spokeCurrentSegment = false
	var currentSegmentText: String = ""
	var currentNavUnit: NavUnit = .segment(.paragraph)
	// The structural unit the FFI is asked for. Find isn't one, so it reads as paragraph: that's
	// what a match's surrounding context is spoken as.
	var currentSegmentType: SegmentTypeFfi {
		if case .segment(let type) = currentNavUnit { return type }
		return .paragraph
	}
	// The units this document actually offers: a plain text file has no headings or tables to
	// step through, so the core decides per document. Find only joins the list once there's a
	// query to step through.
	var availableNavUnits: [NavUnit] {
		let supported = activeSession?.getSupportedSegmentTypesFfi() ?? [.paragraph, .line]
		var segments = supported.map { NavUnit.segment($0) }
		if hasAudio {
			// A book that is only a bundle of narration files has no prose to step through, so
			// each underlying audio file being its own section is all the structure there is.
			if isAudioOnly { segments = segments.filter { $0 == .segment(.section) } }
			segments = audioSeekAmountsSeconds.map { NavUnit.time(seconds: $0) } + segments
		}
		return activeSearchQuery == nil ? segments : segments + [.find]
	}
	// Keeps the selected unit on something the newly active document supports. Switching from an
	// EPUB navigated by heading to a plain text file would otherwise leave Heading selected and
	// every previous/next press doing nothing.
	func ensureNavUnitSupported() {
		let units = availableNavUnits
		guard !units.isEmpty, !units.contains(currentNavUnit) else { return }
		currentNavUnit = units[0]
	}
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
		if let narration, hasAudio {
			if narration.isPlaying { narration.pause() } else { narration.play() }
			updateNowPlaying()
			return
		}
		if ttsManager.isSpeaking {
			ttsManager.pause()
		} else if ttsManager.isPaused {
			ttsManager.resume()
		} else if spokeCurrentSegment {
			speakNextContinuousSegment(isAutoAdvance: false)
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

	/// Handles previous/next for a document being navigated by elapsed time rather than by text
	/// unit. False when that isn't what's happening, leaving the ordinary text path to run.
	private func seekAudioByNavUnit(forward: Bool) -> Bool {
		guard case .time(let seconds) = currentNavUnit, hasAudio, let narration else { return false }
		let deltaMs = Int64(seconds) * 1000
		narration.seekRelativeMs(forward ? deltaMs : -deltaMs)
		return true
	}

	@discardableResult
	func playNextSegment(speak: Bool = true, announce: Bool = false) -> Bool {
		if seekAudioByNavUnit(forward: true) { return true }
		if navigateByFind(forward: true, speak: speak, announce: announce) { return true }
		guard let session = activeSession else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: currentSegmentType,
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
		if seekAudioByNavUnit(forward: false) { return true }
		if navigateByFind(forward: false, speak: speak, announce: announce) { return true }
		guard let session = activeSession else { return false }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: currentSegmentType,
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
		// Landing on a Find match should speak its context and then wait for the next button
		// press, not silently keep reading past it. Remember that it was read, so that pressing
		// play carries on from here rather than repeating the paragraph for ever.
		if currentNavUnit == .find {
			spokeCurrentSegment = true
			return
		}
		speakNextContinuousSegment(isAutoAdvance: true)
	}

	// Reads on from the cursor by actual content, whatever navigation unit is selected.
	// `isAutoAdvance` is only ever true from the utterance-finished callback: it hands playback
	// the buffer already queued behind the last one, and speak() treats it as audio that is
	// playing already. Passing it for a press of play would no-op into silence.
	private func speakNextContinuousSegment(isAutoAdvance: Bool) {
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
		ttsManager.speak(seg.text, isAutoAdvance: isAutoAdvance)
		prefetchAdjacentSegments(around: seg.startPos)
	}

	// The segment type continuous TTS playback should walk by, regardless of the user's chosen
	// navigation unit. Paragraph/line are real sequential content; every other unit is a marker
	// jump and must fall back to paragraph so playback doesn't skip the body between markers.
	private func continuousPlaybackSegmentType() -> SegmentTypeFfi {
		switch currentSegmentType {
		case .paragraph, .line: return currentSegmentType
		default: return .paragraph
		}
	}

	private func announceNavigationCue(_ text: String) {
		let words = text.split(whereSeparator: \.isWhitespace)
		announce(words.prefix(5).joined(separator: " "))
	}

	private func announce(_ text: String) {
		// Delay so SwiftUI's layout-changed accessibility notification fires first;
		// otherwise it interrupts the announcement when triggered by a button tap.
		Task { @MainActor in
			try? await Task.sleep(for: .milliseconds(150))
			UIAccessibility.post(notification: .announcement, argument: text)
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

	func changeNavUnit(_ unit: NavUnit) {
		currentNavUnit = unit
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
	// Find Previous/Next both sets the active query and jumps in one action. Selecting Find as
	// the navigation unit afterwards is what lets the reading bar's previous/next controls walk
	// the rest of the matches without reopening this screen.
	func startSearch(query: String, options: SearchOptions, forward: Bool) {
		// A fresh query searches from the reading position itself, so a match already under the
		// cursor counts. Re-running the query that is already active is the user asking for the
		// next one, so it steps past that match instead of landing on it again.
		let repeated = query == activeSearchQuery && options == searchOptions
		activeSearchQuery = query
		searchOptions = options
		// Text mode has no reading bar to select a unit in, and leaving it later shouldn't drop
		// the reader into Find without them asking for it.
		if !isTextMode { currentNavUnit = .find }
		// The Find screen stays up and VoiceOver focus stays in it, so the announcement is the
		// only sign the reading position moved at all.
		if findMatch(forward: forward, skipCurrent: repeated) {
			announceNavigationCue(currentSegmentText)
		} else if repeated {
			// TRANSLATORS: Announced when stepping to the next/previous Find match runs off the end of the document
			announce(t("No more matches."))
		} else {
			// TRANSLATORS: Announced when a Find query matches nothing anywhere in the document
			announce(t("No matches."))
		}
	}

	func findNext() {
		findMatch(forward: true, skipCurrent: true)
	}

	func findPrev() {
		findMatch(forward: false, skipCurrent: true)
	}

	// Moves to the next match of the active query, if there is one. Forward search is inclusive
	// of the start position, so stepping off a match we're sitting on has to nudge past it;
	// backward search is already exclusive and needs no such adjustment.
	@discardableResult
	private func findMatch(forward: Bool, skipCurrent: Bool) -> Bool {
		guard let session = activeSession, let query = activeSearchQuery else { return false }
		let start = forward && skipCurrent ? ttsPosition + 1 : ttsPosition
		let result = session.searchFfi(
			query: query,
			startPosition: start,
			options: SearchOptionsFfi(
				matchCase: searchOptions.matchCase,
				wholeWord: searchOptions.wholeWord,
				regex: searchOptions.regex,
				forward: forward
			)
		)
		guard result.found else { return false }
		ttsPosition = result.position
		context?.persistPosition(result.position)
		refreshCurrentSegment()
		return true
	}

	// Handles previous/next while Find is the chosen navigation unit, stepping between matches
	// instead of structural units. False when Find isn't the chosen unit, leaving the ordinary
	// segment path to run. Once it is, this always reports handled, even with no more matches:
	// there is nothing else for previous/next to fall back to.
	private func navigateByFind(forward: Bool, speak: Bool, announce shouldAnnounce: Bool) -> Bool {
		guard currentNavUnit == .find else { return false }
		guard activeSearchQuery != nil else { return true }
		guard findMatch(forward: forward, skipCurrent: true) else {
			// TRANSLATORS: Announced when stepping to the next/previous Find match runs off the end of the document
			announce(t("No more matches."))
			return true
		}
		if speak {
			ttsManager.speak(currentSegmentText)
			prefetchAdjacentSegments(around: ttsPosition)
		} else {
			// Discard any paused buffer so pressing play starts at the new position.
			if ttsManager.isPaused { ttsManager.stop() }
			if shouldAnnounce { announceNavigationCue(currentSegmentText) }
		}
		return true
	}

	func goToLine(_ line: Int64) {
		guard let session = activeSession else { return }
		let pos = session.positionFromLine(line: line)
		ttsPosition = pos
		context?.persistPosition(pos)
		refreshCurrentSegment()
	}

	/// `announce` is for a jump the reader asked for, as opposed to restoring a saved position
	/// when a document opens, which nobody wants read out.
	func goToPosition(_ position: Int64, announce shouldAnnounce: Bool = false) {
		jump(to: position, announce: shouldAnnounce)
	}

	func goToPage(_ page: Int32, announce shouldAnnounce: Bool = false) {
		guard let session = activeSession else { return }
		jump(to: session.pageOffset(page: page), announce: shouldAnnounce)
	}

	func goToPercent(_ percent: Int32, announce shouldAnnounce: Bool = false) {
		guard let session = activeSession else { return }
		jump(to: session.positionFromPercent(percent: percent), announce: shouldAnnounce)
	}

	// Lands the reader somewhere else in the document, the way navigateByType() does for a
	// structural step: carry on reading aloud from the new spot if the reader was already going,
	// and otherwise say where they arrived, since the sheet they picked from is dismissing and
	// nothing else reports the move. A paused buffer still holds the old spot's audio, so play
	// would resume where they left rather than where they just went; drop it.
	private func jump(to position: Int64, announce shouldAnnounce: Bool) {
		ttsPosition = position
		context?.persistPosition(position)
		refreshCurrentSegment()
		if hasAudio, let narration {
			// The recording is the playback, so moving the caret without moving it would leave
			// the reader listening to where they used to be.
			narration.seekToPosition(position)
			if shouldAnnounce { announceNavigationCue(currentSegmentText) }
			return
		}
		guard shouldAnnounce else { return }
		if ttsManager.isSpeaking {
			ttsManager.speak(currentSegmentText)
			prefetchAdjacentSegments(around: position)
		} else {
			if ttsManager.isPaused { ttsManager.stop() }
			announceNavigationCue(currentSegmentText)
		}
	}

	func loadSegment(for tab: DocumentTab) {
		guard let session = tab.session else { return }
		narration?.attach(to: tab)
		ensureNavUnitSupported()
		ttsPosition = tab.currentPosition
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: currentSegmentType,
			direction: .current
		)
		currentSegmentText = seg.text
	}

	/// The file whose name was last announced after a seek, so a seek staying inside the same
	/// file doesn't repeat it.
	private var lastAnnouncedAudioSource: Int32?

	/// Keeps the displayed text in step with the recording.
	///
	/// An audio-only book's buffer is one placeholder space per file with no newlines anywhere,
	/// so asking for the paragraph enclosing a position collapses to the whole buffer and
	/// reports it as starting at 0. Deriving the label from that would pin it to the first
	/// file's name for the life of the book, however far playback had moved; the section (that
	/// is, the file) holding the current position is the only label there is.
	func refreshSegmentForAudio() {
		if isAudioOnly {
			currentSegmentText = sectionTitle(at: ttsPosition)
			return
		}
		refreshCurrentSegment()
	}

	/// Speaks where a relative seek landed. An audiobook that is a bundle of narration files has
	/// no meaningful document-wide elapsed time (its clips carry placeholder durations), so its
	/// position reads as an offset into the file now playing, named whenever the file changes.
	func announceAudioSeekLanded(_ elapsedMs: Int64) {
		guard let session = activeSession else { return }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsedMs)
		guard cursor.found else { return }
		let clip = session.audioClipFfi(index: cursor.clipIndex)
		guard clip.found else { return }
		let time = formatDuration(isAudioOnly ? cursor.seekMs : elapsedMs)
		let fileChanged = lastAnnouncedAudioSource != clip.source
		lastAnnouncedAudioSource = clip.source
		let title = sectionTitle(at: clip.start)
		announce(fileChanged && !title.isEmpty ? "\(title), \(time)" : time)
	}

	/// The last table-of-contents entry at or before `position`, which for a recorded book names
	/// the file now playing.
	private func sectionTitle(at position: Int64) -> String {
		guard let session = activeSession else { return "" }
		return session.getToc().last { $0.position <= position }?.title ?? ""
	}

	private func refreshCurrentSegment() {
		guard let session = activeSession else { return }
		let seg = session.getTextSegment(
			position: ttsPosition,
			segmentType: currentSegmentType,
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
			if hasAudio {
				narration?.play()
			} else if ttsManager.isPaused {
				ttsManager.resume()
			} else if !ttsManager.isSpeaking {
				playCurrentSegment()
			}
			updateNowPlaying()
			return .success
		}
		center.pauseCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			if hasAudio { narration?.pause() } else { ttsManager.pause() }
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
			playNextSegment(speak: isPlayingNow)
			updateNowPlaying()
			return .success
		}
		center.previousTrackCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			playPrevSegment(speak: isPlayingNow)
			updateNowPlaying()
			return .success
		}

		center.stopCommand.addTarget { [weak self] _ in
			guard let self else { return .commandFailed }
			if hasAudio { narration?.pause() } else { ttsManager.stop() }
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
			MPNowPlayingInfoPropertyPlaybackRate: isPlayingNow ? 1.0 : 0.0,
			MPNowPlayingInfoPropertyDefaultPlaybackRate: 1.0,
		]
		if let title = context?.activeTitle {
			info[MPMediaItemPropertyTitle] = title
		}
		info[MPMediaItemPropertyArtist] = "Paperback"
		MPNowPlayingInfoCenter.default().nowPlayingInfo = info
	}
}
