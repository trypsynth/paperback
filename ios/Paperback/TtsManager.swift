import AVFoundation

@MainActor
final class TtsManager: NSObject, ObservableObject {
	private let synthesizer = AVSpeechSynthesizer()
	private let prefetchSynthesizer = AVSpeechSynthesizer()
	private let engine = AVAudioEngine()
	private let player = AVAudioPlayerNode()
	private var outputFormat: AVAudioFormat!

	private var speechGeneration = 0
	private var lastScheduledGen = -1

	private var prefetchedText: String? = nil
	private var prefetchedBuffer: AVAudioPCMBuffer? = nil
	private var prefetchGeneration = 0

	private var wasInterruptedWhilePlaying = false

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

	override init() {
		super.init()
		try? AVAudioSession.sharedInstance().setCategory(
			.playback, mode: .spokenAudio, options: [.duckOthers, .mixWithOthers]
		)
		try? AVAudioSession.sharedInstance().setActive(true)

		let hwRate = AVAudioSession.sharedInstance().sampleRate
		outputFormat = AVAudioFormat(
			standardFormatWithSampleRate: hwRate > 0 ? hwRate : 44100,
			channels: 1
		)!

		engine.attach(player)
		engine.connect(player, to: engine.mainMixerNode, format: outputFormat)
		try? engine.start()

		NotificationCenter.default.addObserver(
			self,
			selector: #selector(handleInterruption(_:)),
			name: AVAudioSession.interruptionNotification,
			object: AVAudioSession.sharedInstance()
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

	// MARK: - Session interruption

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
				let optionsValue = info[AVAudioSessionInterruptionOptionKey] as? UInt ?? 0
				let options = AVAudioSession.InterruptionOptions(rawValue: optionsValue)
				try? AVAudioSession.sharedInstance().setActive(true)
				if !engine.isRunning { try? engine.start() }
				if wasInterruptedWhilePlaying && options.contains(.shouldResume) {
					wasInterruptedWhilePlaying = false
					player.play()
					isSpeaking = true
					isPaused = false
				} else {
					wasInterruptedWhilePlaying = false
				}
			@unknown default:
				break
			}
		}
	}

	@objc private func handleMediaServicesReset() {
		Task { @MainActor [weak self] in
			guard let self else { return }
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
			try? AVAudioSession.sharedInstance().setActive(true)
			try? engine.start()
		}
	}

	// MARK: - Playback

	func speak(_ text: String) {
		// Use the prefetched buffer if it matches (no synthesis needed).
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

	func pause() {
		guard isSpeaking else { return }
		player.pause()
		isSpeaking = false
		isPaused = true
	}

	func resume() {
		guard isPaused else { return }
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
		synthesizer.stopSpeaking(at: .immediate)
		player.stop()
		isSpeaking = false
		isPaused = false
	}

	private func invalidatePrefetch() {
		prefetchGeneration += 1
		prefetchSynthesizer.stopSpeaking(at: .immediate)
		prefetchedText = nil
		prefetchedBuffer = nil
	}

	private func makeUtterance(_ text: String) -> AVSpeechUtterance {
		let u = AVSpeechUtterance(string: text)
		u.rate = speechRate
		u.pitchMultiplier = pitch
		u.voice = selectedVoiceIdentifier.flatMap { AVSpeechSynthesisVoice(identifier: $0) }
		return u
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
		let outCapacity = AVAudioFrameCount(Double(totalFrames) * ratio) + 1
		guard let outBuf = AVAudioPCMBuffer(pcmFormat: target, frameCapacity: outCapacity) else { return nil }

		var inputConsumed = false
		var error: NSError?
		converter.convert(to: outBuf, error: &error) { _, outStatus in
			if inputConsumed {
				outStatus.pointee = .noDataNow
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
