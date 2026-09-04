import AVFoundation

// Lets an armed buffer's completion handler validate against a generation assigned later, at
// consume time, rather than one captured when the closure was created (see armNextBuffer).
private final class GenBox {
	var gen: Int? = nil
}

@MainActor
@Observable
final class TtsManager: NSObject {
	private let synthesizer = AVSpeechSynthesizer()
	private let prefetchSynthesizer = AVSpeechSynthesizer()
	private let prevPrefetchSynthesizer = AVSpeechSynthesizer()
	private let engine = AVAudioEngine()
	private let player = AVAudioPlayerNode()
	private var outputFormat: AVAudioFormat!

	private var speechGeneration = 0
	private var lastScheduledGen = -1

	// The next utterance's buffer, already handed to the player node while the current one is
	// still playing (gapless queueing) — AVAudioPlayerNode.scheduleBuffer()/play() measurably
	// slower under lock, so doing it ahead of time keeps that cost out of the audible gap.
	// speechGeneration is NOT bumped at arm time (only once this is actually consumed in
	// speak()) — bumping it early would invalidate the still-playing utterance's own pending
	// completion handler as "stale", permanently desyncing playback from position tracking.
	private var armedBox: GenBox? = nil
	private var armedText: String? = nil

	private struct PrefetchEntry {
		let text: String
		var buffer: AVAudioPCMBuffer?
	}
	// Ordered queue of upcoming paragraphs synthesized ahead of playback, nearest first.
	// Buffering more than one paragraph deep absorbs iOS's tendency to slow down background
	// speech synthesis while the screen is locked, which otherwise reintroduces an audible
	// gap between paragraphs (a single paragraph's playback time may not be enough lead time).
	private var prefetchQueue: [PrefetchEntry] = []
	private var prefetchGeneration = 0

	private var prevPrefetchedText: String? = nil
	private var prevPrefetchedBuffer: AVAudioPCMBuffer? = nil
	private var prevPrefetchGeneration = 0

	private var wasInterruptedWhilePlaying = false
	private var ignoreExternalPlayUntil: Date = .distantPast

	/// True within ~1.5 s of a new Bluetooth device connecting while paused.
	/// Lets us ignore the spurious play command some speakers send on auto-pair.
	var suppressExternalPlay: Bool { Date() < ignoreExternalPlayUntil }

	var isSpeaking = false {
		didSet { if oldValue != isSpeaking { onPlaybackStateChanged?() } }
	}
	var isPaused = false {
		didSet { if oldValue != isPaused { onPlaybackStateChanged?() } }
	}

	var speechRate: Float = AVSpeechUtteranceDefaultSpeechRate {
		didSet {
			guard oldValue != speechRate else { return }
			invalidatePrefetch()
			onSpeechRateChanged?(speechRate)
		}
	}
	var pitch: Float = 1.0 {
		didSet {
			guard oldValue != pitch else { return }
			invalidatePrefetch()
			onPitchChanged?(pitch)
		}
	}
	var selectedVoiceIdentifier: String? = nil {
		didSet {
			guard oldValue != selectedVoiceIdentifier else { return }
			invalidatePrefetch()
			onVoiceChanged?(selectedVoiceIdentifier)
		}
	}

	var availableVoices: [AVSpeechSynthesisVoice] { AVSpeechSynthesisVoice.speechVoices() }
	@ObservationIgnored var onUtteranceFinished: (() -> Void)?
	// Observation replaces the old Combine forwarding for redraws; these carry the side effects
	// that used to ride those sinks — refreshing Now Playing and persisting settings. One per
	// setting, so changing the rate does not rewrite the pitch and voice keys as well.
	@ObservationIgnored var onPlaybackStateChanged: (() -> Void)?
	@ObservationIgnored var onSpeechRateChanged: ((Float) -> Void)?
	@ObservationIgnored var onPitchChanged: ((Float) -> Void)?
	@ObservationIgnored var onVoiceChanged: ((String?) -> Void)?
	@ObservationIgnored var rules: [TtsRule] = [] {
		didSet { invalidatePrefetch() }
	}

	func preprocessText(_ text: String) -> String {
		guard !rules.isEmpty else { return text }
		var result = text
		for rule in rules where rule.scope == .paragraph {
			result = rule.apply(to: result, voiceId: selectedVoiceIdentifier)
		}
		for rule in rules where rule.scope == .word {
			result = rule.apply(to: result, voiceId: selectedVoiceIdentifier)
		}
		return result
	}

	override init() {
		super.init()
		// The audio session is deliberately left alone here. Configuring it at launch claims
		// the route and stops whatever else the device is playing before the user has asked
		// for any speech, so both the category and activation wait for playback.
		let hwRate = AVAudioSession.sharedInstance().sampleRate
		outputFormat = AVAudioFormat(
			standardFormatWithSampleRate: hwRate > 0 ? hwRate : 44100,
			channels: 1
		)!

		engine.attach(player)
		engine.connect(player, to: engine.mainMixerNode, format: outputFormat)

		NotificationCenter.default.addObserver(
			self,
			selector: #selector(handleInterruption(_:)),
			name: AVAudioSession.interruptionNotification,
			object: AVAudioSession.sharedInstance()
		)
		NotificationCenter.default.addObserver(
			self,
			selector: #selector(handleRouteChange(_:)),
			name: AVAudioSession.routeChangeNotification,
			object: AVAudioSession.sharedInstance()
		)
		NotificationCenter.default.addObserver(
			self,
			selector: #selector(handleEngineConfigurationChange),
			name: .AVAudioEngineConfigurationChange,
			object: engine
		)
		NotificationCenter.default.addObserver(
			self,
			selector: #selector(handleMediaServicesReset),
			name: AVAudioSession.mediaServicesWereResetNotification,
			object: nil
		)
	}

	func speakSample(_ text: String) {
		invalidatePrefetch()
		internalStop()
		speechGeneration += 1
		let gen = speechGeneration
		isSpeaking = true
		isPaused = false

		let acc = BufferAccumulator()
		synthesizer.write(makeUtterance(text)) { [weak self, acc] buffer in
			guard let pcm = buffer as? AVAudioPCMBuffer else { return }
			if pcm.frameLength > 0 {
				acc.buffers.append(pcm)
			} else {
				let buffers = acc.buffers
				DispatchQueue.main.async { [weak self] in
					guard let self, self.speechGeneration == gen else { return }
					self.scheduleConverted(buffers, gen: gen, suppress: true)
				}
			}
		}
	}

	@objc private func handleRouteChange(_ notification: Notification) {
		guard let info = notification.userInfo,
		      let reasonValue = info[AVAudioSessionRouteChangeReasonKey] as? UInt,
		      let reason = AVAudioSession.RouteChangeReason(rawValue: reasonValue) else { return }
		Task { @MainActor [weak self] in
			guard let self else { return }
			switch reason {
			case .oldDeviceUnavailable:
				// Pause when headphones are unplugged (standard iOS behavior).
				if isSpeaking {
					player.pause()
					isSpeaking = false
					isPaused = true
					try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
				}
			case .newDeviceAvailable:
				// Some Bluetooth speakers (e.g. JBL) fire a play command on auto-pair.
				// Gate external play commands for 1.5 s so that spurious command is dropped.
				if isPaused {
					ignoreExternalPlayUntil = Date().addingTimeInterval(1.5)
				}
			default:
				break
			}
		}
	}

	// Fires when AVAudioEngine stops due to a hardware reconfiguration (e.g. Bluetooth
	// device connects and changes the output format). Reconnect and restart the engine,
	// but never auto-resume: respect the current isSpeaking / isPaused state.
	@objc private func handleEngineConfigurationChange() {
		Task { @MainActor [weak self] in
			guard let self else { return }
			guard isSpeaking || isPaused else { return }
			let wasSpeaking = isSpeaking
			isSpeaking = false
			isPaused = false
			armedBox = nil
			armedText = nil
			engine.detach(player)
			engine.attach(player)
			let hwRate = AVAudioSession.sharedInstance().sampleRate
			outputFormat = AVAudioFormat(
				standardFormatWithSampleRate: hwRate > 0 ? hwRate : 44100,
				channels: 1
			)!
			engine.connect(player, to: engine.mainMixerNode, format: outputFormat)
			try? engine.start()
			if wasSpeaking {
				// The scheduled buffer was lost; advance to the next segment.
				onUtteranceFinished?()
			}
		}
	}

	@objc private func handleInterruption(_ notification: Notification) {
		guard let info = notification.userInfo,
		      let typeValue = info[AVAudioSessionInterruptionTypeKey] as? UInt,
		      let type = AVAudioSession.InterruptionType(rawValue: typeValue) else { return }

		Task { @MainActor [weak self] in
			guard let self else { return }
			switch type {
			case .began:
				if isSpeaking {
					wasInterruptedWhilePlaying = true
					player.pause()
					isSpeaking = false
					isPaused = true
				}
			case .ended:
				// Only reactivate if TTS was actually interrupted; a call ending while
				// Paperback was already stopped must not restart playback.
				guard wasInterruptedWhilePlaying else { return }
				wasInterruptedWhilePlaying = false
				let optionsValue = info[AVAudioSessionInterruptionOptionKey] as? UInt ?? 0
				let options = AVAudioSession.InterruptionOptions(rawValue: optionsValue)
				activateAudioSession()
				if !engine.isRunning { try? engine.start() }
				if options.contains(.shouldResume) {
					player.play()
					isSpeaking = true
					isPaused = false
				}
			@unknown default:
				break
			}
		}
	}

	@objc private func handleMediaServicesReset() {
		Task { @MainActor [weak self] in
			guard let self else { return }
			let wasActive = isSpeaking || isPaused
			isSpeaking = false
			isPaused = false
			armedBox = nil
			armedText = nil
			wasInterruptedWhilePlaying = false
			speechGeneration += 1
			invalidatePrefetch()

			let hwRate = AVAudioSession.sharedInstance().sampleRate
			outputFormat = AVAudioFormat(
				standardFormatWithSampleRate: hwRate > 0 ? hwRate : 44100,
				channels: 1
			)!
			engine.stop()
			engine.detach(player)
			engine.attach(player)
			engine.connect(player, to: engine.mainMixerNode, format: outputFormat)
			// Only reactivate if audio was actually playing/paused before the reset;
			// unconditionally starting the engine keeps the app alive in the background.
			if wasActive {
				activateAudioSession()
				try? engine.start()
			}
		}
	}

	// `isAutoAdvance` must be true only when this call is the natural continuation onto the
	// buffer already queued next (i.e. from the utterance-finished callback) — never for a
	// user-initiated seek that merely happens to target the same text as the armed buffer,
	// since that would silently no-op instead of cutting off the currently-playing audio.
	func speak(_ text: String, isAutoAdvance: Bool = false) {
		let text = preprocessText(text)
		// Already handed to the player node while the previous utterance was still playing
		// (see armNextBuffer) — it's already audibly playing (or about to be). Just assign it
		// the generation it's now logically current under; no re-scheduling needed.
		if isAutoAdvance, text == armedText, let box = armedBox {
			armedBox = nil
			armedText = nil
			speechGeneration += 1
			let gen = speechGeneration
			box.gen = gen
			lastScheduledGen = gen
			isSpeaking = true
			isPaused = false
			return
		}
		// Use a prefetched buffer if one is queued and ready (no synthesis needed).
		if let idx = prefetchQueue.firstIndex(where: { $0.text == text }), let cached = prefetchQueue[idx].buffer {
			prefetchQueue.removeSubrange(0...idx)
			internalStop()
			speechGeneration += 1
			let gen = speechGeneration
			isSpeaking = true
			isPaused = false
			schedule(cached, gen: gen, suppress: false)
			return
		}
		if text == prevPrefetchedText, let cached = prevPrefetchedBuffer {
			prevPrefetchedText = nil
			prevPrefetchedBuffer = nil
			internalStop()
			speechGeneration += 1
			let gen = speechGeneration
			isSpeaking = true
			isPaused = false
			schedule(cached, gen: gen, suppress: false)
			return
		}

		invalidatePrefetch()
		internalStop()
		speechGeneration += 1
		let gen = speechGeneration
		isSpeaking = true
		isPaused = false

		let acc = BufferAccumulator()
		synthesizer.write(makeUtterance(text)) { [weak self, acc] buffer in
			guard let pcm = buffer as? AVAudioPCMBuffer else { return }
			if pcm.frameLength > 0 {
				acc.buffers.append(pcm)
			} else {
				let buffers = acc.buffers
				DispatchQueue.main.async { [weak self] in
					guard let self, self.speechGeneration == gen else { return }
					self.scheduleConverted(buffers, gen: gen, suppress: false)
				}
			}
		}
	}

	// Synthesise the given upcoming paragraphs (nearest first) in the background so they're
	// ready by the time speak() needs them. Buffers already synthesized for entries that
	// remain in the new list are kept; the queue is only reset when the immediate next
	// paragraph changes (navigation, voice/rate change, etc).
	func prefetch(upcoming texts: [String]) {
		let texts = texts.map { preprocessText($0) }
		guard texts != prefetchQueue.map(\.text) else { return }

		if texts.first != prefetchQueue.first?.text {
			prefetchGeneration += 1
			prefetchSynthesizer.stopSpeaking(at: .immediate)
			prefetchQueue = []
		}
		let gen = prefetchGeneration

		let cachedBuffers = Dictionary(
			prefetchQueue.compactMap { entry in entry.buffer.map { (entry.text, $0) } },
			uniquingKeysWith: { first, _ in first }
		)
		prefetchQueue = texts.map { PrefetchEntry(text: $0, buffer: cachedBuffers[$0]) }
		// The new front slot may already have a buffer carried over from the old queue
		// (e.g. it was slot 1 a moment ago) — try to arm it now rather than waiting for a
		// synthesis completion that already happened.
		tryArmFirstSlotIfReady()

		for text in texts where cachedBuffers[text] == nil {
			let acc = BufferAccumulator()
			prefetchSynthesizer.write(makeUtterance(text)) { [weak self, acc] buffer in
				guard let pcm = buffer as? AVAudioPCMBuffer else { return }
				if pcm.frameLength > 0 {
					acc.buffers.append(pcm)
				} else {
					let buffers = acc.buffers
					DispatchQueue.main.async { [weak self] in
						guard let self, self.prefetchGeneration == gen else { return }
						guard let idx = self.prefetchQueue.firstIndex(where: { $0.text == text }) else { return }
						self.prefetchQueue[idx].buffer = self.convertToOutput(buffers)
						self.tryArmFirstSlotIfReady()
					}
				}
			}
		}
	}

	// Synthesise `text` in the background so it's ready if the user navigates backward.
	func prefetchPrev(_ text: String) {
		let text = preprocessText(text)
		guard text != prevPrefetchedText else { return }
		prevPrefetchGeneration += 1
		prevPrefetchSynthesizer.stopSpeaking(at: .immediate)
		prevPrefetchedText = text
		prevPrefetchedBuffer = nil
		prevPrefetchGeneration += 1
		let gen = prevPrefetchGeneration

		let acc = BufferAccumulator()
		prevPrefetchSynthesizer.write(makeUtterance(text)) { [weak self, acc] buffer in
			guard let pcm = buffer as? AVAudioPCMBuffer else { return }
			if pcm.frameLength > 0 {
				acc.buffers.append(pcm)
			} else {
				let buffers = acc.buffers
				DispatchQueue.main.async { [weak self] in
					guard let self, self.prevPrefetchGeneration == gen else { return }
					self.prevPrefetchedBuffer = self.convertToOutput(buffers)
				}
			}
		}
	}

	func pause() {
		guard isSpeaking else { return }
		player.pause()
		isSpeaking = false
		isPaused = true
		try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
	}

	func resume() {
		guard isPaused else { return }
		// pause() deactivated the session so other apps could take the route back; resuming has
		// to claim it again, and no new buffer is scheduled here to do it for us.
		activateAudioSession()
		if !engine.isRunning { try? engine.start() }
		player.play()
		isSpeaking = true
		isPaused = false
	}

	func stop() {
		speechGeneration += 1
		invalidatePrefetch()
		internalStop()
		// Only tear the engine/session down on an explicit stop. Between back-to-back
		// paragraphs (the common case) we keep both alive: stopping and restarting the
		// AVAudioEngine/AVAudioSession on every utterance round-trips through CoreAudio,
		// which gets noticeably slower while the screen is locked and was the real cause
		// of the growing gap between paragraphs during background playback.
		engine.stop()
		try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
	}

	private func internalStop() {
		wasInterruptedWhilePlaying = false
		synthesizer.stopSpeaking(at: .immediate)
		player.stop()
		isSpeaking = false
		isPaused = false
		// player.stop() cancels anything already queued on the node, including an armed buffer.
		armedBox = nil
		armedText = nil
	}

	// Arms the front of prefetchQueue onto the player node if it's ready and it's currently
	// safe to do so: something must be actively playing, and its own buffer must already be
	// committed to the player node (lastScheduledGen == speechGeneration) rather than still
	// mid on-demand-synthesis — otherwise this could schedule the next paragraph ahead of the
	// current one, playing it first.
	private func tryArmFirstSlotIfReady() {
		guard armedBox == nil, isSpeaking, !isPaused, lastScheduledGen == speechGeneration else { return }
		guard let first = prefetchQueue.first, let buf = first.buffer else { return }
		let text = first.text
		prefetchQueue.removeFirst()
		armNextBuffer(buf, text: text)
	}

	// Hands `pcm` to the player node immediately so it plays back-to-back after whatever's
	// currently playing, with no stop/restart round-trip at the transition itself. Does NOT
	// touch speechGeneration yet — that only happens once speak() actually consumes this (see
	// the armedText check there), once the currently-playing utterance's own completion has
	// legitimately fired. Bumping it here, before that, would make the still-in-flight
	// completion look stale and get silently dropped, desyncing playback from position tracking.
	private func armNextBuffer(_ pcm: AVAudioPCMBuffer, text: String) {
		let box = GenBox()
		armedBox = box
		armedText = text
		player.scheduleBuffer(pcm) { [weak self] in
			DispatchQueue.main.async { [weak self] in
				guard let self, let gen = box.gen, self.speechGeneration == gen else { return }
				self.isSpeaking = false
				self.isPaused = false
				self.onUtteranceFinished?()
			}
		}
	}

	private func invalidatePrefetch() {
		prefetchGeneration += 1
		prefetchSynthesizer.stopSpeaking(at: .immediate)
		prefetchQueue = []
		prevPrefetchGeneration += 1
		prevPrefetchSynthesizer.stopSpeaking(at: .immediate)
		prevPrefetchedText = nil
		prevPrefetchedBuffer = nil
	}

	private func makeUtterance(_ text: String) -> AVSpeechUtterance {
		let u = AVSpeechUtterance(string: sanitizeForSpeech(text))
		u.rate = speechRate
		u.pitchMultiplier = pitch
		u.voice = selectedVoiceIdentifier.flatMap { AVSpeechSynthesisVoice(identifier: $0) }
		return u
	}

	// Soft hyphens (\u{00AD}) and null bytes cause AVSpeechSynthesizer to truncate utterances.
	// The synthesizer also parses its input as markup, so a bare `<` swallows everything up to
	// the next `>` (and an unterminated one swallows the rest of the utterance, wedging playback
	// until the app is restarted). Escaping the three markup characters makes them literal again.
	private func sanitizeForSpeech(_ text: String) -> String {
		var out = ""
		out.reserveCapacity(text.count)
		for scalar in text.unicodeScalars {
			switch scalar {
			case "\u{00}", "\u{AD}": continue
			case "&": out += "&amp;"
			case "<": out += "&lt;"
			case ">": out += "&gt;"
			default: out.unicodeScalars.append(scalar)
			}
		}
		return out
	}

	private func scheduleConverted(_ buffers: [AVAudioPCMBuffer], gen: Int, suppress: Bool) {
		// AVSpeechSynthesizer sometimes fires the done signal twice; only schedule once per gen.
		guard lastScheduledGen != gen else { return }
		guard let pcm = convertToOutput(buffers) else {
			isSpeaking = false
			isPaused = false
			if !suppress { onUtteranceFinished?() }
			return
		}
		schedule(pcm, gen: gen, suppress: suppress)
	}

	/// Configures and activates the session, immediately before audio is actually produced.
	/// The category is set here rather than once at startup because `.playback` is not
	/// mixable: applying it interrupts other apps' audio, which must not happen just because
	/// Paperback was opened. Setting it again on every activation is cheap, and it is also
	/// what restores the configuration after a media services reset wipes it.
	private func activateAudioSession() {
		let session = AVAudioSession.sharedInstance()
		try? session.setCategory(.playback, mode: .spokenAudio)
		try? session.setActive(true)
	}

	private func schedule(_ pcm: AVAudioPCMBuffer, gen: Int, suppress: Bool) {
		lastScheduledGen = gen
		activateAudioSession()
		if !engine.isRunning { try? engine.start() }
		player.scheduleBuffer(pcm) { [weak self] in
			DispatchQueue.main.async { [weak self] in
				guard let self, self.speechGeneration == gen else { return }
				self.isSpeaking = false
				self.isPaused = false
				if !suppress { self.onUtteranceFinished?() }
			}
		}
		if !isPaused {
			player.play()
		}
	}

	// Concatenate synthesis chunks then convert to the hardware output format in one pass.
	private func convertToOutput(_ buffers: [AVAudioPCMBuffer]) -> AVAudioPCMBuffer? {
		guard let synthFormat = buffers.first?.format else { return nil }

		let totalFrames = buffers.reduce(AVAudioFrameCount(0)) { $0 + $1.frameLength }
		guard let synthBuf = AVAudioPCMBuffer(pcmFormat: synthFormat, frameCapacity: totalFrames) else { return nil }
		for buf in buffers {
			guard let src = buf.floatChannelData, let dst = synthBuf.floatChannelData else { continue }
			let n = Int(buf.frameLength)
			for ch in 0..<Int(synthFormat.channelCount) {
				memcpy(dst[ch].advanced(by: Int(synthBuf.frameLength)), src[ch], n * MemoryLayout<Float>.size)
			}
			synthBuf.frameLength += buf.frameLength
		}

		let target = outputFormat!
		if synthFormat == target { return synthBuf }

		guard let converter = AVAudioConverter(from: synthFormat, to: target) else { return nil }
		let ratio = target.sampleRate / synthFormat.sampleRate
		// Extra headroom so the resampler can flush its internal delay buffer via .endOfStream.
		let outCapacity = AVAudioFrameCount(Double(totalFrames) * ratio) + 512
		guard let outBuf = AVAudioPCMBuffer(pcmFormat: target, frameCapacity: outCapacity) else { return nil }

		var inputConsumed = false
		var error: NSError?
		converter.convert(to: outBuf, error: &error) { _, outStatus in
			if inputConsumed {
				// Signal end-of-stream so the resampler flushes its tail; .noDataNow would discard it.
				outStatus.pointee = .endOfStream
				return nil
			}
			outStatus.pointee = .haveData
			inputConsumed = true
			return synthBuf
		}

		return error == nil ? outBuf : nil
	}
}

private final class BufferAccumulator: @unchecked Sendable {
	var buffers: [AVAudioPCMBuffer] = []
}
