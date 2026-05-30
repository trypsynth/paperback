import AVFoundation

@MainActor
final class TtsManager: NSObject, ObservableObject {
	private let synthesizer = AVSpeechSynthesizer()
	private let engine = AVAudioEngine()
	private let player = AVAudioPlayerNode()
	private var outputFormat: AVAudioFormat!
	private var speechGeneration = 0
	private var suppressNextFinish = false

	@Published var isSpeaking = false
	@Published var isPaused = false
	@Published var speechRate: Float = AVSpeechUtteranceDefaultSpeechRate
	@Published var pitch: Float = 1.0
	@Published var selectedVoiceIdentifier: String? = nil

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
	}

	func speak(_ text: String) {
		internalStop()
		suppressNextFinish = false
		speechGeneration += 1
		let gen = speechGeneration

		let utterance = AVSpeechUtterance(string: text)
		utterance.rate = speechRate
		utterance.pitchMultiplier = pitch
		utterance.voice = selectedVoiceIdentifier.flatMap { AVSpeechSynthesisVoice(identifier: $0) }

		isSpeaking = true
		isPaused = false

		let acc = BufferAccumulator()
		synthesizer.write(utterance) { [weak self, acc] buffer in
			guard let pcm = buffer as? AVAudioPCMBuffer else { return }
			if pcm.frameLength > 0 {
				acc.buffers.append(pcm)
			} else {
				let buffers = acc.buffers
				DispatchQueue.main.async { [weak self] in
					guard let self, self.speechGeneration == gen else { return }
					self.scheduleAndPlay(buffers: buffers, gen: gen)
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
		suppressNextFinish = true
		speechGeneration += 1
		internalStop()
	}

	// MARK: - Private

	private func internalStop() {
		synthesizer.stopSpeaking(at: .immediate)
		player.stop()
		isSpeaking = false
		isPaused = false
	}

	private func scheduleAndPlay(buffers: [AVAudioPCMBuffer], gen: Int) {
		guard let pcm = convertToOutput(buffers) else {
			isSpeaking = false
			if !suppressNextFinish { onUtteranceFinished?() }
			suppressNextFinish = false
			return
		}

		if !engine.isRunning { try? engine.start() }

		player.scheduleBuffer(pcm) { [weak self] in
			DispatchQueue.main.async { [weak self] in
				guard let self, self.speechGeneration == gen else { return }
				self.handlePlaybackComplete()
			}
		}
		if !isPaused { player.play() }
	}

	private func handlePlaybackComplete() {
		isSpeaking = false
		isPaused = false
		if !suppressNextFinish { onUtteranceFinished?() }
		suppressNextFinish = false
	}

	// Concatenates raw synthesis buffers then converts to the hardware output format in one pass.
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
