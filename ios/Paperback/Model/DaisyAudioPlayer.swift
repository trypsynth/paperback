import AVFoundation
import Foundation

private let pollInterval: TimeInterval = 0.25

/// Where a relative seek lands relative to the file it starts in.
enum SeekSpill: Equatable {
	/// The seek stays inside this file, so it is an ordinary seek.
	case withinFile
	/// The seek runs off the end, this far into whatever plays next.
	case pastEnd(overflowMs: Int64)
	/// The seek runs off the front, this far back from the end of whatever plays before.
	case beforeStart(underflowMs: Int64)
}

/// Where seeking `deltaMs` from `rawMs` lands, in a file that really runs for `lengthMs`.
///
/// This works in the file's own time rather than the document's elapsed time on purpose: an
/// audiobook that is only a bundle of narration files gives every clip the same placeholder
/// duration, far longer than the recording it stands for, so elapsed-time arithmetic would
/// resolve back into the same file past its end, where a seek can only clamp. A file whose
/// length is not known yet can only be seeked within.
func spillOf(rawMs: Int64, lengthMs: Int64, deltaMs: Int64) -> SeekSpill {
	guard lengthMs > 0 else { return .withinFile }
	let naiveMs = rawMs + deltaMs
	if naiveMs > lengthMs { return .pastEnd(overflowMs: naiveMs - lengthMs) }
	if naiveMs < 0 { return .beforeStart(underflowMs: -naiveMs) }
	return .withinFile
}

/// Where in a source to start: an absolute offset, or a distance back from its real end.
private enum SourceSeek {
	case fromStart(ms: Int64)
	case fromEnd(ms: Int64)
}

/// Plays a book's recorded narration (including DAISY and M4B) against its timeline, like
/// Android's `DaisyAudioPlayer`. `onClipChanged` fires unconditionally, so the caller keeps the
/// currently-spoken-text display in step with playback without a sync toggle.
@MainActor
final class DaisyAudioPlayer: NSObject, AVAudioPlayerDelegate {
	private var session: DocumentSession?
	private var docKey: String?

	private var player: AVAudioPlayer?
	private var currentSource: Int32?
	private var playing = false
	private var loadGeneration = 0

	/// A seek requested while paused, applied lazily on resume (see `play()`).
	private var pendingTargetMs: Int64?
	private var lastReportedClip: Int32?
	private var pollTimer: Timer?
	/// Set for the duration of one `seekRelativeMs`, so only that seek reports where it lands.
	private var reportNextSeek = false

	/// Invoked with the start position of the clip currently narrating, whenever it changes.
	var onClipChanged: ((Int64) -> Void)?
	/// Invoked whenever play/pause state changes, including auto-advance and end of book.
	var onPlaybackStateChanged: ((Bool) -> Void)?
	/// Invoked once a `seekRelativeMs` has landed, with where it landed in document elapsed
	/// time. Only relative seeks report: the ones following the caret or restoring a saved
	/// position have nothing to say.
	var onRelativeSeekLanded: ((Int64) -> Void)?

	private var cacheDirectory: URL {
		FileManager.default.temporaryDirectory.appendingPathComponent("paperback_daisy_audio_cache")
	}

	var hasAudio: Bool { session?.hasAudioFfi() == true }
	var isPlaying: Bool { playing }

	/// Switches to narrating `newSession`, stopping whatever this player was doing. `newDocKey`
	/// scopes the extracted-source cache so it doesn't collide with another document's.
	func attach(session newSession: DocumentSession, docKey newDocKey: String) {
		stop()
		session = newSession
		docKey = newDocKey
		lastReportedClip = nil
	}

	func detach() {
		stop()
		session = nil
		docKey = nil
	}

	func play() {
		guard session != nil else { return }
		playing = true
		onPlaybackStateChanged?(true)
		let pending = pendingTargetMs
		if let pending {
			if !seekToMs(pending) { resetAfterLoadFailure() }
		} else if currentSource != nil {
			resumeLoadedPlayer()
		} else {
			if !seekToMs(0) { resetAfterLoadFailure() }
		}
	}

	func pause() {
		playing = false
		stopPolling()
		player?.pause()
		deactivateAudioSession()
		onPlaybackStateChanged?(false)
	}

	func toggle() {
		if playing { pause() } else { play() }
	}

	/// Stops playback and releases the decoder, ahead of switching documents or going away.
	/// Only notifies when there was something to stop: `attach` and `detach` call this on every
	/// tab switch, including ones involving documents that never had audio, and firing
	/// unconditionally there would force a paused state onto speech that was never speaking.
	func stop() {
		let wasActive = playing || session != nil
		playing = false
		loadGeneration += 1
		stopPolling()
		player?.stop()
		player = nil
		currentSource = nil
		pendingTargetMs = nil
		reportNextSeek = false
		deactivateAudioSession()
		if wasActive { onPlaybackStateChanged?(false) }
	}

	/// Seeks to the point covering `position` in the text, if the timeline narrates it. Leaves
	/// the transport running or paused as it already was.
	@discardableResult
	func seekToPosition(_ position: Int64) -> Bool {
		guard let session else { return false }
		let point = session.audioPointForPositionFfi(position: position)
		guard point.found else { return false }
		return seekToMs(point.timeMs)
	}

	/// Seeks to `elapsedMs` into the overall document timeline. A paused target landing in some
	/// other file is only recorded and applied on resume, so browsing a paused book doesn't
	/// drive a full load per keystroke. A target inside the file already open is applied at
	/// once even while paused: it costs nothing and keeps the decoder's reported position
	/// honest for `seekRelativeMs`, which measures from it.
	@discardableResult
	func seekToMs(_ elapsedMs: Int64) -> Bool {
		guard let session else { return false }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsedMs)
		guard cursor.found else { return false }
		let clip = session.audioClipFfi(index: cursor.clipIndex)
		guard clip.found else { return false }
		if !playing, currentSource != clip.source || player == nil {
			// A new paused destination supersedes any source still loading in the background.
			loadGeneration += 1
			pendingTargetMs = elapsedMs
			reportClip(cursor.clipIndex)
			if reportNextSeek {
				reportNextSeek = false
				onRelativeSeekLanded?(elapsedMs)
			}
			return true
		}
		// Clip callbacks persist the destination before a different decoder has loaded.
		pendingTargetMs = elapsedMs
		reportClip(cursor.clipIndex)
		if currentSource == clip.source, let player, Int64(player.currentTime * 1000) == cursor.seekMs {
			// Compare with the decoder's actual position: it may have moved since the last
			// seek, even when the requested destination hasn't changed.
			if playing, !resumeLoadedPlayer() { return false }
			pendingTargetMs = nil
			return true
		}
		if currentSource == clip.source, player != nil {
			guard applySeek(source: clip.source, seekMs: cursor.seekMs) else { return false }
			pendingTargetMs = nil
		} else {
			loadSource(clip.source, seek: .fromStart(ms: cursor.seekMs))
		}
		return true
	}

	/// Moves playback `deltaMs` from wherever it is now, the time-unit equivalent of stepping
	/// by paragraph in a text document. A seek running off either end of the file now playing
	/// continues into its neighbour, measured against that file's own real length.
	@discardableResult
	func seekRelativeMs(_ deltaMs: Int64) -> Bool {
		guard let session else { return false }
		reportNextSeek = true
		if spillAcrossFileBoundary(session, deltaMs: deltaMs) { return true }
		let current = resumePointMs() ?? 0
		let target = deltaMs >= 0
			? min(current + deltaMs, session.audioTotalDurationMsFfi())
			: max(current + deltaMs, 0)
		let seeked = seekToMs(target)
		if !seeked { reportNextSeek = false }
		return seeked
	}

	/// Handles the part of `seekRelativeMs` that leaves the current file, loading the
	/// neighbouring source at the leftover offset.
	private func spillAcrossFileBoundary(_ session: DocumentSession, deltaMs: Int64) -> Bool {
		guard let source = currentSource, let player else { return false }
		// A recorded-but-unapplied target means the loaded decoder isn't where we logically
		// are, so its reported position is the wrong thing to measure against.
		guard pendingTargetMs == nil else { return false }
		let rawMs = Int64(player.currentTime * 1000)
		let lengthMs = Int64(player.duration * 1000)
		switch spillOf(rawMs: rawMs, lengthMs: lengthMs, deltaMs: deltaMs) {
		case .pastEnd(let overflowMs):
			let next = session.audioNextSourceAfterFfi(currentSource: source)
			guard next >= 0 else { return false }
			loadSource(next, seek: .fromStart(ms: overflowMs))
			return true
		case .beforeStart(let underflowMs):
			let previous = session.audioPreviousSourceBeforeFfi(currentSource: source)
			guard previous >= 0 else { return false }
			loadSource(previous, seek: .fromEnd(ms: underflowMs))
			return true
		case .withinFile:
			return false
		}
	}

	/// Where playback would resume right now. Nil means no position has been established yet;
	/// callers must not treat that as the start, since it would overwrite a stored position.
	func resumePointMs() -> Int64? {
		if let pendingTargetMs { return pendingTargetMs }
		if let session, let source = currentSource, let player {
			let rawMs = Int64(player.currentTime * 1000)
			let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: rawMs)
			if elapsed >= 0 { return elapsed }
		}
		return nil
	}

	@discardableResult
	private func resumeLoadedPlayer() -> Bool {
		activateAudioSession()
		guard player?.play() == true else {
			resetAfterLoadFailure()
			return false
		}
		onPlaybackStateChanged?(true)
		startPolling()
		return true
	}

	private func applySeek(source: Int32, seekMs: Int64) -> Bool {
		guard let player else { return false }
		let landedMs = seekWithinPlayer(player, seekMs: seekMs)
		if playing {
			guard resumeLoadedPlayer() else { return false }
		} else {
			player.pause()
		}
		reportSeeked(source: source, rawMs: landedMs)
		return true
	}

	/// Seeks `player` to `seekMs`, clamped to the file's real length, and reports where it
	/// actually went. The clamp matters for a bundle of narration files, whose clips declare a
	/// placeholder duration hours longer than the recording.
	private func seekWithinPlayer(_ player: AVAudioPlayer, seekMs: Int64) -> Int64 {
		let lengthMs = Int64(player.duration * 1000)
		let target = lengthMs > 0 ? min(max(seekMs, 0), lengthMs) : max(seekMs, 0)
		player.currentTime = Double(target) / 1000
		return target
	}

	private func reportSeeked(source: Int32, rawMs: Int64) {
		guard reportNextSeek, let session else { return }
		reportNextSeek = false
		let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: rawMs)
		if elapsed >= 0 { onRelativeSeekLanded?(elapsed) }
	}

	private func loadSource(_ sourceIndex: Int32, seek: SourceSeek) {
		guard let session else { return }
		currentSource = sourceIndex
		// Where we are between here and the decoder being ready, so a second seek arriving in
		// that window still has something to measure from. A distance back from the end has no
		// answer until the file's real length is known, so it stays unrecorded until then.
		switch seek {
		case .fromStart(let ms):
			let elapsed = session.audioElapsedForSourcePositionFfi(source: sourceIndex, rawMs: ms)
			pendingTargetMs = elapsed >= 0 ? elapsed : nil
		case .fromEnd:
			pendingTargetMs = nil
		}
		stopPolling()
		player?.stop()
		player = nil
		loadGeneration += 1
		let myGeneration = loadGeneration
		let key = docKey
		let cacheDir = cacheDirectory
		// Resolving a source can extract it from the book's zip, so it runs off the main
		// thread; the generation check on return discards a superseded load.
		Task.detached(priority: .userInitiated) {
			let path = Self.resolveSourcePath(session: session, index: sourceIndex, docKey: key, cacheDir: cacheDir)
			await MainActor.run { [weak self] in
				guard let self, myGeneration == self.loadGeneration else { return }
				guard let path else {
					self.resetAfterLoadFailure()
					return
				}
				self.startPlayer(path: path, sourceIndex: sourceIndex, seek: seek, generation: myGeneration)
			}
		}
	}

	/// Resolves source `index` to a real local file: the document's own path when it isn't
	/// zip-embedded, otherwise extracted once and cached.
	private nonisolated static func resolveSourcePath(
		session: DocumentSession,
		index: Int32,
		docKey: String?,
		cacheDir: URL
	) -> String? {
		let direct = session.audioSourceDirectPathFfi(index: index)
		if !direct.isEmpty { return direct }
		guard let docKey else { return nil }
		let cacheFile = cacheDir.appendingPathComponent("\(docKey)_\(index)")
		if !FileManager.default.fileExists(atPath: cacheFile.path) {
			try? FileManager.default.createDirectory(at: cacheDir, withIntermediateDirectories: true)
			guard session.audioExtractSourceFfi(index: index, outputPath: cacheFile.path) else { return nil }
		}
		return cacheFile.path
	}

	private func startPlayer(path: String, sourceIndex: Int32, seek: SourceSeek, generation: Int) {
		guard generation == loadGeneration else { return }
		let loaded: AVAudioPlayer
		do {
			loaded = try AVAudioPlayer(contentsOf: URL(fileURLWithPath: path))
		} catch {
			resetAfterLoadFailure()
			return
		}
		loaded.delegate = self
		loaded.prepareToPlay()
		player = loaded
		let requestedMs: Int64
		switch seek {
		case .fromStart(let ms):
			requestedMs = ms
		case .fromEnd(let ms):
			requestedMs = Int64(loaded.duration * 1000) - ms
		}
		let landedMs = seekWithinPlayer(loaded, seekMs: requestedMs)
		if playing {
			guard resumeLoadedPlayer() else { return }
		}
		// The decoder is now sitting where it was asked to, so it is the authority on the
		// resume point again (see `resumePointMs`).
		pendingTargetMs = nil
		reportClipAtSourcePosition(source: sourceIndex, rawMs: landedMs)
		reportSeeked(source: sourceIndex, rawMs: landedMs)
	}

	/// Releases a failed decoder while retaining the destination for a later retry.
	private func resetAfterLoadFailure() {
		pendingTargetMs = resumePointMs()
		player?.stop()
		player = nil
		currentSource = nil
		reportNextSeek = false
		playing = false
		stopPolling()
		deactivateAudioSession()
		onPlaybackStateChanged?(false)
	}

	/// Auto-advances to whichever source continues the narration, or stops at the end.
	private func onSourceCompleted(_ sourceIndex: Int32) {
		guard let session else { return }
		let next = session.audioNextSourceAfterFfi(currentSource: sourceIndex)
		if next >= 0 {
			loadSource(next, seek: .fromStart(ms: 0))
		} else {
			playing = false
			stopPolling()
			deactivateAudioSession()
			onPlaybackStateChanged?(false)
		}
	}

	nonisolated func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
		Task { @MainActor [weak self] in
			guard let self, let source = self.currentSource, self.player === player else { return }
			if flag {
				self.onSourceCompleted(source)
			} else {
				self.resetAfterLoadFailure()
			}
		}
	}

	private func startPolling() {
		stopPolling()
		let timer = Timer(timeInterval: pollInterval, repeats: true) { _ in
			Task { @MainActor [weak self] in self?.pollTick() }
		}
		RunLoop.main.add(timer, forMode: .common)
		pollTimer = timer
	}

	private func stopPolling() {
		pollTimer?.invalidate()
		pollTimer = nil
	}

	/// Keeps the reported "currently narrating" clip in step with natural playback advance.
	private func pollTick() {
		guard playing, let session, let source = currentSource, let player else { return }
		let rawMs = Int64(player.currentTime * 1000)
		let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: rawMs)
		guard elapsed >= 0 else { return }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsed)
		if cursor.found { reportClip(cursor.clipIndex) }
	}

	/// Reports whichever clip covers `rawMs` in `source`, for a seek that landed somewhere the
	/// poll loop won't visit on its own (a paused one, most of all).
	private func reportClipAtSourcePosition(source: Int32, rawMs: Int64) {
		guard let session else { return }
		let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: rawMs)
		guard elapsed >= 0 else { return }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsed)
		if cursor.found { reportClip(cursor.clipIndex) }
	}

	private func reportClip(_ clipIndex: Int32) {
		guard clipIndex != lastReportedClip, let session else { return }
		let clip = session.audioClipFfi(index: clipIndex)
		guard clip.found else { return }
		lastReportedClip = clipIndex
		onClipChanged?(clip.start)
	}

	/// Matches how speech claims the route: `.playback` is not mixable, so applying it
	/// interrupts other apps' audio and must wait until the reader has asked for playback.
	private func activateAudioSession() {
		let audio = AVAudioSession.sharedInstance()
		try? audio.setCategory(.playback, mode: .spokenAudio)
		try? audio.setActive(true)
	}

	private func deactivateAudioSession() {
		try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
	}
}
