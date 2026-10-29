import AVFoundation
import Foundation

private let pollInterval: TimeInterval = 0.25

enum SeekSpill: Equatable {
	case withinFile
	case pastEnd(overflowMs: Int64)
	case beforeStart(underflowMs: Int64)
}

/// In the file's own time, because audio-only books give every clip a placeholder duration far longer than the recording, so elapsed-time arithmetic would only clamp.
func spillOf(rawMs: Int64, lengthMs: Int64, deltaMs: Int64) -> SeekSpill {
	guard lengthMs > 0 else { return .withinFile }
	let naiveMs = rawMs + deltaMs
	if naiveMs > lengthMs { return .pastEnd(overflowMs: naiveMs - lengthMs) }
	if naiveMs < 0 { return .beforeStart(underflowMs: -naiveMs) }
	return .withinFile
}

private enum SourceSeek {
	case fromStart(ms: Int64)
	case fromEnd(ms: Int64)
}

@MainActor
final class DaisyAudioPlayer {
	private var session: DocumentSession?
	private var docKey: String?

	private var player: SourcePlayer?
	private var currentSource: Int32?
	private var playing = false
	private var loadGeneration = 0

	/// A seek requested while paused, applied lazily on resume (see `play()`).
	private var pendingTargetMs: Int64?
	private var lastReportedClip: Int32?
	private var pollTimer: Timer?
	/// Only relative seeks report where they land; ones following the caret or restoring a saved position have nothing to say.
	private var reportNextSeek = false

	var onClipChanged: ((Int64) -> Void)?
	var onPlaybackStateChanged: ((Bool) -> Void)?
	var onRelativeSeekLanded: ((Int64) -> Void)?

	/// One speed for every recording; it survives loading the next file.
	var speed: Float = 1 {
		didSet { player?.rate = speed }
	}

	private var cacheDirectory: URL {
		FileManager.default.temporaryDirectory.appendingPathComponent("paperback_daisy_audio_cache")
	}

	var hasAudio: Bool { session?.hasAudioFfi() == true }
	var isPlaying: Bool { playing }

	/// `newDocKey` scopes the extracted-source cache so it doesn't collide with another document's.
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

	/// Only notifies when something was active: this runs on every tab switch, and firing unconditionally would force a paused state onto speech.
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

	@discardableResult
	func seekToPosition(_ position: Int64) -> Bool {
		guard let session else { return false }
		let point = session.audioPointForPositionFfi(position: position)
		guard point.found else { return false }
		return seekToMs(point.timeMs)
	}

	/// A paused seek into another file waits for resume so browsing doesn't load per keystroke; within the open file it applies at once, since `seekRelativeMs` measures from the decoder.
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
			// The decoder may have moved since the last seek even when the requested destination hasn't.
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

	private func spillAcrossFileBoundary(_ session: DocumentSession, deltaMs: Int64) -> Bool {
		guard let source = currentSource, let player else { return false }
		// With an unapplied target pending, the decoder's position is the wrong thing to measure from.
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

	/// Nil must not be treated as the start, since that would overwrite a stored position.
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

	/// Clamped because audio-only books' clips declare a placeholder duration hours longer than the recording.
	private func seekWithinPlayer(_ player: SourcePlayer, seekMs: Int64) -> Int64 {
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
		// Gives a second seek arriving before the decoder is ready something to measure from; a distance from the end needs the real length first.
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
		// Off the main thread because resolving can extract from the book's zip.
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
		let loaded: SourcePlayer
		do {
			loaded = try SourcePlayer(url: URL(fileURLWithPath: path))
		} catch {
			resetAfterLoadFailure()
			return
		}
		loaded.rate = speed
		loaded.onFinished = { [weak self, weak loaded] in
			guard let self, let loaded, self.player === loaded, let source = self.currentSource else { return }
			self.onSourceCompleted(source)
		}
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
		// The decoder is the authority on the resume point again (see `resumePointMs`).
		pendingTargetMs = nil
		reportClipAtSourcePosition(source: sourceIndex, rawMs: landedMs)
		reportSeeked(source: sourceIndex, rawMs: landedMs)
	}

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

	var elapsedMs: Int64? {
		guard let session, let source = currentSource, let player else { return nil }
		let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: Int64(player.currentTime * 1000))
		return elapsed >= 0 ? elapsed : nil
	}

	private func pollTick() {
		guard playing, let session, let source = currentSource, let player else { return }
		let rawMs = Int64(player.currentTime * 1000)
		let elapsed = session.audioElapsedForSourcePositionFfi(source: source, rawMs: rawMs)
		guard elapsed >= 0 else { return }
		let cursor = session.audioCursorAtElapsedFfi(elapsedMs: elapsed)
		if cursor.found { reportClip(cursor.clipIndex) }
	}

	/// For a seek landing where the poll loop won't visit on its own, a paused one most of all.
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

	/// `.playback` is not mixable, so applying it interrupts other apps and must wait until the reader asks for playback.
	private func activateAudioSession() {
		let audio = AVAudioSession.sharedInstance()
		try? audio.setCategory(.playback, mode: .spokenAudio)
		try? audio.setActive(true)
	}

	private func deactivateAudioSession() {
		try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
	}
}
