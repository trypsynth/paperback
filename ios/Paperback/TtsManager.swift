import AVFoundation

// MARK: - AudioQueue
// Thread-safe PCM buffer queue consumed by the AVAudioSourceNode render callback.
// NSLock is held only for pointer arithmetic + memcpy — never across a blocking call —
// so the brief critical section doesn't cause audio thread priority inversion in practice.
private final class AudioQueue: @unchecked Sendable {
	private var buffers: [AVAudioPCMBuffer] = []
	private var bufferIndex = 0
	private var frameOffset = 0
	private var paused = false
	private let lock = NSLock()

	func load(_ newBuffers: [AVAudioPCMBuffer]) {
		lock.lock()
		buffers = newBuffers
		bufferIndex = 0
		frameOffset = 0
		lock.unlock()
	}

	func setPaused(_ value: Bool) {
		lock.lock()
		paused = value
		lock.unlock()
	}

	func reset() {
		lock.lock()
		buffers.removeAll()
		bufferIndex = 0
		frameOffset = 0
		paused = false
		lock.unlock()
	}

	// Called on the audio render thread. Fills `abl` with up to `frameCount` frames.
	// Returns true once the queue is fully exhausted (triggers completion on main).
	func render(into abl: UnsafeMutablePointer<AudioBufferList>, frameCount: UInt32) -> Bool {
		lock.lock()
		defer { lock.unlock() }

		let ablPtr = UnsafeMutableAudioBufferListPointer(abl)
		for i in 0..<ablPtr.count {
			if let data = ablPtr[i].mData { memset(data, 0, Int(ablPtr[i].mDataByteSize)) }
		}

		if paused { return false }

		var needed = Int(frameCount)
		var written = 0

		while needed > 0 && bufferIndex < buffers.count {
			let buf = buffers[bufferIndex]
			let available = Int(buf.frameLength) - frameOffset
			let toCopy = min(available, needed)
			guard let src = buf.floatChannelData else { break }
			let channels = min(Int(buf.format.channelCount), ablPtr.count)
			for ch in 0..<channels {
				if let dst = ablPtr[ch].mData?.assumingMemoryBound(to: Float.self) {
					memcpy(dst.advanced(by: written), src[ch].advanced(by: frameOffset),
					       toCopy * MemoryLayout<Float>.size)
				}
			}
			written += toCopy
			needed -= toCopy
			frameOffset += toCopy
			if frameOffset >= Int(buf.frameLength) {
				bufferIndex += 1
				frameOffset = 0
			}
		}

		return bufferIndex >= buffers.count
	}
}

// MARK: - BufferAccumulator
// Collects PCM buffers on the synthesis background thread.
private final class BufferAccumulator: @unchecked Sendable {
	var buffers: [AVAudioPCMBuffer] = []
}

// MARK: - TtsManager
@MainActor
final class TtsManager: NSObject, ObservableObject {
	private let synthesizer = AVSpeechSynthesizer()
	private let engine = AVAudioEngine()
	private let audioQueue = AudioQueue()
	private var sourceNode: AVAudioSourceNode?
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
			.playback,
			mode: .spokenAudio,
			options: [.duckOthers, .mixWithOthers]
		)
		try? AVAudioSession.sharedInstance().setActive(true)
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
					self.startPlayback(buffers: buffers)
				}
			}
		}
	}

	func pause() {
		guard isSpeaking else { return }
		audioQueue.setPaused(true)
		isSpeaking = false
		isPaused = true
	}

	func resume() {
		guard isPaused else { return }
		audioQueue.setPaused(false)
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
		if engine.isRunning { engine.stop() }
		if let node = sourceNode {
			engine.detach(node)
			sourceNode = nil
		}
		audioQueue.reset()
		isSpeaking = false
		isPaused = false
	}

	private func startPlayback(buffers: [AVAudioPCMBuffer]) {
		guard !buffers.isEmpty, let format = buffers.first?.format else {
			isSpeaking = false
			if !suppressNextFinish { onUtteranceFinished?() }
			suppressNextFinish = false
			return
		}

		audioQueue.load(buffers)

		let gen = speechGeneration
		var completionFired = false

		let node = AVAudioSourceNode(format: format) { [weak self, audioQueue] _, _, frameCount, abl -> OSStatus in
			let exhausted = audioQueue.render(into: abl, frameCount: frameCount)
			if exhausted && !completionFired {
				completionFired = true
				DispatchQueue.main.async { [weak self] in
					guard let self, self.speechGeneration == gen else { return }
					self.handlePlaybackComplete()
				}
			}
			return noErr
		}

		sourceNode = node
		engine.attach(node)
		engine.connect(node, to: engine.mainMixerNode, format: format)

		do {
			try engine.start()
			isSpeaking = !isPaused
		} catch {
			isSpeaking = false
			isPaused = false
			if !suppressNextFinish { onUtteranceFinished?() }
			suppressNextFinish = false
		}
	}

	private func handlePlaybackComplete() {
		if engine.isRunning { engine.stop() }
		if let node = sourceNode {
			engine.detach(node)
			sourceNode = nil
		}
		isSpeaking = false
		isPaused = false
		if !suppressNextFinish { onUtteranceFinished?() }
		suppressNextFinish = false
	}
}
