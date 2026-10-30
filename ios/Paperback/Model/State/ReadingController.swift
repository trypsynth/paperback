import SwiftUI
import UIKit
import MediaPlayer

@MainActor
protocol ReadingContext: AnyObject {
	var activeSession: DocumentSession? { get }
	var activeTitle: String? { get }
	func persistPosition(_ position: Int64)
}

@MainActor
@Observable
final class ReadingController {
	@ObservationIgnored weak var context: ReadingContext?

	private var activeSession: DocumentSession? { context?.activeSession }

	// Not in init, because both of these need the context to be set.
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

	var isTextMode: Bool = false {
		didSet { if isTextMode != oldValue { onTextModeChanged?(isTextMode) } }
	}
	@ObservationIgnored var onTextModeChanged: ((Bool) -> Void)?
	/// The reading screen, for places the app chose: VoiceOver follows a jump, but only the page follows reading aloud.
	@ObservationIgnored var onTextModeJump: ((Int64) -> Void)?
	@ObservationIgnored var onTextModeFollow: ((Int64) -> Void)?
	@ObservationIgnored private var textModeReporting = false

	let ttsManager = TtsManager()
	var narration: RecordedNarration?
	var hasAudio: Bool { activeSession?.hasAudioFfi() == true }
	var isAudioOnly: Bool { activeSession?.isAudioOnlyFfi() == true }
	var isPlayingNow: Bool { hasAudio ? narration?.isPlaying == true : ttsManager.isSpeaking }
	var ttsPosition: Int64 = 0 {
		didSet {
			spokeCurrentSegment = false
			if isTextMode, !textModeReporting { onTextModeFollow?(ttsPosition) }
		}
	}
	// After a Find match is read, play means "carry on from here", not "read that again".
	private var spokeCurrentSegment = false
	var currentSegmentText: String = ""
	var currentNavUnit: NavUnit = .segment(.paragraph)
	var stopAfterFindMatch: Bool = true {
		didSet {
			if stopAfterFindMatch != oldValue { onStopAfterFindMatchChanged?(stopAfterFindMatch) }
		}
	}
	@ObservationIgnored var onStopAfterFindMatchChanged: ((Bool) -> Void)?
	// Find reads as paragraph, since that's what a match's surrounding context is spoken as.
	var currentSegmentType: SegmentTypeFfi {
		if case .segment(let type) = currentNavUnit { return type }
		return .paragraph
	}
	var availableNavUnits: [NavUnit] {
		let supported = activeSession?.getSupportedSegmentTypesFfi() ?? [.paragraph, .line]
		var segments = supported.map { NavUnit.segment($0) }
		if hasAudio {
			// An audio-only book has no prose; each audio file being a section is all the structure there is.
			if isAudioOnly { segments = segments.filter { $0 == .segment(.section) } }
			segments = audioSeekAmountsSeconds.map { NavUnit.time(seconds: $0) } + segments
		}
		return activeSearchQuery == nil ? segments : segments + [.find]
	}
	// Otherwise switching from an EPUB on Heading to a plain text file leaves every previous/next press doing nothing.
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

	private func seekAudioByNavUnit(forward: Bool) -> Bool {
		guard case .time(let seconds) = currentNavUnit, hasAudio, let narration else { return false }
		let deltaMs = Int64(seconds) * 1000
		narration.seekRelativeMs(forward ? deltaMs : -deltaMs)
		return true
	}

	@discardableResult
	func playNextSegment(speak: Bool = true, announce: Bool = false) -> Bool {
		stepSegment(.next, speak: speak, announce: announce)
	}

	@discardableResult
	func playPrevSegment(speak: Bool = true, announce: Bool = false) -> Bool {
		stepSegment(.previous, speak: speak, announce: announce)
	}

	private func stepSegment(_ direction: SegmentDirectionFfi, speak: Bool, announce shouldAnnounce: Bool) -> Bool {
		let forward = direction == .next
		if seekAudioByNavUnit(forward: forward) { return true }
		if navigateByFind(forward: forward, speak: speak, announce: shouldAnnounce) { return true }
		if hasAudio { return navigateRecordedSegment(currentSegmentType, direction: direction) }
		guard let session = activeSession else { return false }
		let step = session.readingStep(position: ttsPosition, unit: .segment(segmentType: currentSegmentType), direction: direction)
		guard step.found else { return false }
		land(on: step)
		if speak {
			ttsManager.speak(step.text)
			prefetchAdjacentSegments(around: step.position)
		} else {
			// Discard any paused buffer so pressing play starts at the new position.
			if ttsManager.isPaused { ttsManager.stop() }
			if shouldAnnounce { announceNavigationCue(step.text) }
		}
		return true
	}

	private func land(on step: ReadingStepFfi) {
		ttsPosition = step.position
		currentSegmentText = step.text
		context?.persistPosition(step.position)
	}

	private func advanceTtsAfterUtterance() {
		// The stop matters: prefetch may already have scheduled the following paragraph before this callback runs.
		if currentNavUnit == .find && stopAfterFindMatch {
			spokeCurrentSegment = true
			ttsManager.stop()
			return
		}
		speakNextContinuousSegment(isAutoAdvance: true)
	}

	// `isAutoAdvance` only from the utterance-finished callback: speak() treats it as audio already playing, so a press of play would no-op into silence.
	private func speakNextContinuousSegment(isAutoAdvance: Bool) {
		guard let session = activeSession else { return }
		let step = session.readingStep(position: ttsPosition, unit: .segment(segmentType: .paragraph), direction: .next)
		guard step.found else { return }
		land(on: step)
		ttsManager.speak(step.text, isAutoAdvance: isAutoAdvance)
		prefetchAdjacentSegments(around: step.position)
	}

	private func announceNavigationCue(_ text: String) {
		let words = text.split(whereSeparator: \.isWhitespace)
		announce(words.prefix(5).joined(separator: " "))
	}

	private func announce(_ text: String) {
		// Delayed so SwiftUI's layout-changed notification after a button tap doesn't interrupt it.
		Task { @MainActor in
			try? await Task.sleep(for: .milliseconds(150))
			UIAccessibility.post(notification: .announcement, argument: text)
		}
	}

	private func prefetchAdjacentSegments(around position: Int64) {
		guard let session = activeSession else { return }
		let ahead = session.readingAhead(position: position)
		ttsManager.prefetch(upcoming: [ahead.next, ahead.afterNext].filter { !$0.isEmpty })
		if !ahead.previous.isEmpty {
			ttsManager.prefetchPrev(ahead.previous)
		}
	}

	func changeNavUnit(_ unit: NavUnit) {
		currentNavUnit = unit
	}

	func navigateByType(_ type: SegmentTypeFfi, direction: SegmentDirectionFfi) {
		if hasAudio {
			navigateRecordedSegment(type, direction: direction)
			return
		}
		guard let session = activeSession else { return }
		let step = session.readingStep(position: ttsPosition, unit: .segment(segmentType: type), direction: direction)
		guard step.found else { return }
		land(on: step)
		if ttsManager.isSpeaking {
			ttsManager.speak(step.text)
			prefetchAdjacentSegments(around: step.position)
		} else {
			if ttsManager.isPaused { ttsManager.stop() }
			announceNavigationCue(step.text)
		}
	}

	@discardableResult
	private func navigateRecordedSegment(_ type: SegmentTypeFfi, direction: SegmentDirectionFfi) -> Bool {
		guard let session = activeSession, let narration else { return false }
		let step = session.readingStep(position: ttsPosition, unit: .segment(segmentType: type), direction: direction)
		guard step.found, narration.seekToPosition(step.position) else { return false }
		land(on: step)
		// Section titles can exceed the five-word cue used for ordinary text navigation.
		announce(step.text)
		return true
	}

	func setSleepTimer(seconds: Int) {
		cancelSleepTimer()
		sleepTimerRemaining = seconds
		sleepTimerTask = Task {
			while true {
				try? await Task.sleep(for: .seconds(1))
				if Task.isCancelled { return }
				guard let r = sleepTimerRemaining, r > 0 else {
					// A recording plays on its own player, which pausing speech wouldn't reach.
					if hasAudio { narration?.pause() } else { ttsManager.pause() }
					updateNowPlaying()
					UIApplication.shared.isIdleTimerDisabled = false
					sleepTimerRemaining = nil
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

	func startSearch(query: String, options: SearchOptions, forward: Bool) {
		// A fresh query counts a match under the cursor; re-running the active one means "next", so it steps past it.
		let repeated = query == activeSearchQuery && options == searchOptions
		activeSearchQuery = query
		searchOptions = options
		// Text mode has no reading bar, and leaving it later shouldn't drop the reader into Find unasked.
		if !isTextMode { currentNavUnit = .find }
		// VoiceOver focus stays in the Find screen, so the announcement is the only sign the position moved.
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

	@discardableResult
	private func findMatch(forward: Bool, skipCurrent: Bool) -> Bool {
		guard let session = activeSession, let query = activeSearchQuery else { return false }
		let options = SearchOptionsFfi(matchCase: searchOptions.matchCase, wholeWord: searchOptions.wholeWord, regex: searchOptions.regex, forward: forward)
		let direction: SegmentDirectionFfi = !forward ? .previous : skipCurrent ? .next : .current
		let step = session.readingStep(position: ttsPosition, unit: .find(query: query, options: options), direction: direction)
		guard step.found else { return false }
		land(on: step)
		return true
	}

	// Always reports handled once Find is the unit, even with no more matches, since there's nothing for previous/next to fall back to.
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

	/// `announce` is off for restoring a saved position on open, which nobody wants read out.
	func goToPosition(_ position: Int64, announce shouldAnnounce: Bool = false) {
		jump(to: position, announce: shouldAnnounce)
	}

	func goToPage(_ page: Int32, announce shouldAnnounce: Bool = false) {
		guard let session = activeSession else { return }
		jump(to: session.pageOffset(page: page), announce: shouldAnnounce)
	}

	/// Through the running time because an audiobook's text is one line per file, which treats a two minute file and an hour long one as equal shares.
	@discardableResult
	func seekAudioToPercent(_ percent: Int32) -> Bool {
		guard hasAudio, let session = activeSession, let narration else { return false }
		let targetMs = session.audioElapsedForPercentFfi(percent: percent)
		guard targetMs >= 0 else { return false }
		return narration.seekToMs(targetMs)
	}

	/// Audio-only books are measured by running time, since their text is one placeholder line per file.
	func progressPercent() -> Int? {
		guard let session = activeSession else { return nil }
		if isAudioOnly {
			let point = session.audioPointForPositionFfi(position: ttsPosition)
			guard let elapsed = narration?.elapsedMs ?? (point.found ? point.timeMs : nil) else { return nil }
			let percent = session.audioProgressPercentFfi(elapsedMs: elapsed)
			return percent >= 0 ? Int(percent) : nil
		}
		return Int(session.getStatusInfo(position: ttsPosition).percentage)
	}

	func goToPercent(_ percent: Int32, announce shouldAnnounce: Bool = false) {
		guard let session = activeSession else { return }
		jump(to: session.positionFromPercent(percent: percent), announce: shouldAnnounce)
	}

	// Announces because the sheet the reader picked from is dismissing and nothing else reports the move; a paused buffer is dropped since it holds the old spot's audio.
	private func jump(to position: Int64, announce shouldAnnounce: Bool) {
		ttsPosition = position
		context?.persistPosition(position)
		refreshCurrentSegment()
		if isTextMode { onTextModeJump?(position) }
		if hasAudio, let narration {
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
		currentSegmentText = session.readingStep(position: ttsPosition, unit: .segment(segmentType: currentSegmentType), direction: .current).text
	}

	/// So a seek staying inside the same file doesn't repeat its name.
	private var lastAnnouncedAudioSource: Int32?

	/// An audio-only buffer has no newlines, so the enclosing paragraph is the whole buffer and would pin the label to the first file's name.
	func refreshSegmentForAudio() {
		if isAudioOnly {
			currentSegmentText = activeSession?.sectionTitleAt(position: ttsPosition) ?? ""
			return
		}
		refreshCurrentSegment()
	}

	/// An audio-only book's clips carry placeholder durations, so its position reads as an offset into the current file instead.
	func announceAudioSeekLanded(_ elapsedMs: Int64) {
		guard let session = activeSession else { return }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsedMs)
		guard cursor.found else { return }
		let clip = session.audioClipFfi(index: cursor.clipIndex)
		guard clip.found else { return }
		let time = formatDuration(isAudioOnly ? cursor.seekMs : elapsedMs)
		let fileChanged = lastAnnouncedAudioSource != clip.source
		lastAnnouncedAudioSource = clip.source
		let title = session.sectionTitleAt(position: clip.start)
		announce(fileChanged && !title.isEmpty ? "\(title), \(time)" : time)
	}

	private func refreshCurrentSegment() {
		guard let session = activeSession else { return }
		currentSegmentText = session.readingStep(position: ttsPosition, unit: .segment(segmentType: currentSegmentType), direction: .current).text
	}

	func toggleTextMode() {
		isTextMode.toggle()
	}

	/// Where VoiceOver focus or the page went in text mode, which is the reading position.
	func textModeMoved(to position: Int64) {
		textModeReporting = true
		ttsPosition = position
		textModeReporting = false
		context?.persistPosition(position)
		refreshCurrentSegment()
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
