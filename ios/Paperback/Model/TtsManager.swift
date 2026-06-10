import AVFoundation

@MainActor
final class TtsManager: NSObject, ObservableObject {
	private let synthesizer = AVSpeechSynthesizer()
	private let prefetchSynthesizer = AVSpeechSynthesizer()
	private let prevPrefetchSynthesizer = AVSpeechSynthesizer()
	private let engine = AVAudioEngine()
	private let player = AVAudioPlayerNode()
	private var outputFormat: AVAudioFormat!

	private var speechGeneration = 0
	private var lastScheduledGen = -1

	private var prefetchedText: String? = nil
	private var prefetchedBuffer: AVAudioPCMBuffer? = nil
	private var prefetchGeneration = 0

	private var prevPrefetchedText: String? = nil
	private var prevPrefetchedBuffer: AVAudioPCMBuffer? = nil
	private var prevPrefetchGeneration = 0

	private var wasInterruptedWhilePlaying = false
	private var ignoreExternalPlayUntil: Date = .distantPast

	/// True within ~1.5 s of a new Bluetooth device connecting while paused.
	/// Lets us ignore the spurious play command some speakers send on auto-pair.
	var suppressExternalPlay: Bool { Date() < ignoreExternalPlayUntil }

	@Published var isSpeaking = false
	@Published var isPaused = false

	@Published var speechRate: Float = AVSpeechUtteranceDefaultSpeechRate {
		didSet { if oldValue != speechRate { invalidatePrefetch() } }
	}
	@Published var pitch: Float = 1.0 {
		didSet { if oldValue != pitch { invalidatePrefetch() } }
	}
	@Published var selectedVoiceIdentifier: String? = nil {
		didSet { if oldValue != selectedVoiceIdentifier { invalidatePrefetch() } }
	}

	var availableVoices: [AVSpeechSynthesisVoice] { AVSpeechSynthesisVoice.speechVoices() }
	var onUtteranceFinished: (() -> Void)?
	var rules: [TtsRule] = [] {
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
		try? AVAudioSession.sharedInstance().setCategory(.playback, mode: .spokenAudio)

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

	// MARK: - Sample playback

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

	// MARK: - Session / route / engine notifications

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
				try? AVAudioSession.sharedInstance().setActive(true)
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
				try? AVAudioSession.sharedInstance().setActive(true)
				try? engine.start()
			}
		}
	}

	// MARK: - Playback

	func speak(_ text: String) {
		let text = preprocessText(text)
		// Use a prefetched buffer if one matches (no synthesis needed).
		if text == prefetchedText, let cached = prefetchedBuffer {
			prefetchedText = nil
			prefetchedBuffer = nil
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

	// Synthesise `text` in the background so it's ready when speak() is called next.
	func prefetch(_ text: String) {
		let text = preprocessText(text)
		guard text != prefetchedText else { return }
		invalidatePrefetch()
		prefetchedText = text
		prefetchGeneration += 1
		let gen = prefetchGeneration

		let acc = BufferAccumulator()
		prefetchSynthesizer.write(makeUtterance(text)) { [weak self, acc] buffer in
			guard let pcm = buffer as? AVAudioPCMBuffer else { return }
			if pcm.frameLength > 0 {
				acc.buffers.append(pcm)
			} else {
				let buffers = acc.buffers
				DispatchQueue.main.async { [weak self] in
					guard let self, self.prefetchGeneration == gen else { return }
					self.prefetchedBuffer = self.convertToOutput(buffers)
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
		if !engine.isRunning { try? engine.start() }
		player.play()
		isSpeaking = true
		isPaused = false
	}

	func stop() {
		speechGeneration += 1
		invalidatePrefetch()
		internalStop()
	}

	// MARK: - Private

	private func internalStop() {
		wasInterruptedWhilePlaying = false
		synthesizer.stopSpeaking(at: .immediate)
		player.stop()
		isSpeaking = false
		isPaused = false
		engine.stop()
		try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
	}

	private func invalidatePrefetch() {
		prefetchGeneration += 1
		prefetchSynthesizer.stopSpeaking(at: .immediate)
		prefetchedText = nil
		prefetchedBuffer = nil
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
	private func sanitizeForSpeech(_ text: String) -> String {
		text.unicodeScalars
			.filter { $0.value != 0x00 && $0.value != 0x00AD }
			.reduce(into: "") { $0.unicodeScalars.append($1) }
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

	private func schedule(_ pcm: AVAudioPCMBuffer, gen: Int, suppress: Bool) {
		lastScheduledGen = gen
		try? AVAudioSession.sharedInstance().setActive(true)
		if !engine.isRunning { try? engine.start() }
		player.scheduleBuffer(pcm) { [weak self] in
			DispatchQueue.main.async { [weak self] in
				guard let self, self.speechGeneration == gen else { return }
				self.isSpeaking = false
				self.isPaused = false
				if !suppress { self.onUtteranceFinished?() }
			}
		}
		if !isPaused { player.play() }
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
