import AVFoundation
import OSLog

private let ttsLog = Logger(subsystem: "dev.paperback.ios", category: "tts")

/// Paragraph rules run before word rules so a paragraph rule can't overwrite what a word rule produced.
func applyRules(_ rules: [TtsRule], to text: String, voiceId: String?) -> String {
	guard !rules.isEmpty else { return text }
	var result = text
	for rule in rules where rule.scope == .paragraph {
		result = rule.apply(to: result, voiceId: voiceId)
	}
	for rule in rules where rule.scope == .word {
		result = rule.apply(to: result, voiceId: voiceId)
	}
	return result
}

/// The step is small because a pause that sounds right and one that drags differ by tens of milliseconds; past a second it is no longer a paragraph pause.
let paragraphPauseRangeMs = 0...1000
let paragraphPauseStepMs = 10

func speechRateForPercent(_ percent: Int) -> Float {
	let range = AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate
	let clamped = Float(min(max(percent, 0), 100)) / 100
	return AVSpeechUtteranceMinimumSpeechRate + clamped * range
}

func percentForSpeechRate(_ rate: Float) -> Int {
	let range = AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate
	return Int((((rate - AVSpeechUtteranceMinimumSpeechRate) / range) * 100).rounded())
}

let pitchRange: ClosedRange<Float> = 0.5...2.0

func pitchForPercent(_ percent: Int) -> Float {
	let range = pitchRange.upperBound - pitchRange.lowerBound
	let clamped = Float(min(max(percent, 0), 100)) / 100
	return pitchRange.lowerBound + clamped * range
}

func percentForPitch(_ pitch: Float) -> Int {
	let range = pitchRange.upperBound - pitchRange.lowerBound
	return Int((((pitch - pitchRange.lowerBound) / range) * 100).rounded())
}

let speechPercentStep = 1

let speechRatePresets = [25, 50, 75, 100]

/// The engine's 0.5 to 2.0 in steps of 0.25, as percentages of `pitchRange`.
let pitchPresets = [0, 16, 33, 50, 67, 83, 100]

// The generation is assigned when speak() consumes the armed buffer, not when its completion closure is created (see armNextBuffer).
@MainActor
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

	// Scheduled on the node while the current utterance plays, because scheduleBuffer()/play() is measurably slower under lock and would widen the gap.
	private var armedBox: GenBox? = nil
	private var armedText: String? = nil
	// A buffer on the player node can't be taken back, so one synthesized under old settings is dropped at the paragraph boundary instead (see `speak`).
	private var armedIsStale = false

	private struct PrefetchEntry {
		let text: String
		var buffer: AVAudioPCMBuffer?
	}
	// More than one paragraph deep because iOS slows background synthesis while locked, and one paragraph of lead time can leave an audible gap.
	private var prefetchQueue: [PrefetchEntry] = []
	private var prefetchGeneration = 0

	private var prevPrefetchedText: String? = nil
	private var prevPrefetchedBuffer: AVAudioPCMBuffer? = nil
	private var prevPrefetchGeneration = 0

	private var wasInterruptedWhilePlaying = false
	private var ignoreExternalPlayUntil: Date = .distantPast

	/// Some Bluetooth speakers send a spurious play command on auto-pair.
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
	/// One conversion shared by the settings slider and the reading bar so the two never disagree about what a number means.
	var speechRatePercent: Int {
		get { percentForSpeechRate(speechRate) }
		set { speechRate = speechRateForPercent(newValue) }
	}

	var pitch: Float = 1.0 {
		didSet {
			guard oldValue != pitch else { return }
			invalidatePrefetch()
			onPitchChanged?(pitch)
		}
	}
	var pitchPercent: Int {
		get { percentForPitch(pitch) }
		set { pitch = pitchForPercent(newValue) }
	}
	/// A reader setting because the natural gap is the voice's own tail: Eloquence leaves a clear break, Vocalizer barely a comma's worth.
	var paragraphPauseMs: Int = 0 {
		didSet {
			guard oldValue != paragraphPauseMs else { return }
			invalidatePrefetch()
			onParagraphPauseChanged?(paragraphPauseMs)
		}
	}
	var selectedVoiceIdentifier: String? = nil {
		didSet {
			guard oldValue != selectedVoiceIdentifier else { return }
			invalidatePrefetch()
			onVoiceChanged?(selectedVoiceIdentifier)
		}
	}

	@ObservationIgnored private let voiceCatalog = VoiceCatalog { AVSpeechSynthesisVoice.speechVoices() }
	var availableVoices: [AVSpeechSynthesisVoice] { voiceCatalog.voices }
	@ObservationIgnored var onUtteranceFinished: (() -> Void)?
	// One per setting, so changing the rate does not rewrite the pitch and voice keys as well.
	@ObservationIgnored var onPlaybackStateChanged: (() -> Void)?
	@ObservationIgnored var onSpeechRateChanged: ((Float) -> Void)?
	@ObservationIgnored var onPitchChanged: ((Float) -> Void)?
	@ObservationIgnored var onVoiceChanged: ((String?) -> Void)?
	@ObservationIgnored var onParagraphPauseChanged: ((Int) -> Void)?
	@ObservationIgnored var rules: [TtsRule] = [] {
		didSet { invalidatePrefetch() }
	}

	func preprocessText(_ text: String) -> String {
		applyRules(rules, to: text, voiceId: selectedVoiceIdentifier)
	}

	override init() {
		super.init()
		// The audio session is left alone until playback, since configuring it at launch stops whatever else the device is playing.
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
			selector: #selector(handleAvailableVoicesChanged(_:)),
			name: AVSpeechSynthesizer.availableVoicesDidChangeNotification,
			object: nil
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

	@objc private func handleAvailableVoicesChanged(_ notification: Notification) {
		Task { @MainActor [weak self] in
			self?.voiceCatalog.invalidate()
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
				if isSpeaking {
					player.pause()
					isSpeaking = false
					isPaused = true
					try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
				}
			case .newDeviceAvailable:
				// Some Bluetooth speakers (e.g. JBL) fire a play command on auto-pair.
				if isPaused {
					ignoreExternalPlayUntil = Date().addingTimeInterval(1.5)
				}
			default:
				break
			}
		}
	}

	@objc private func handleEngineConfigurationChange() {
		Task { @MainActor [weak self] in
			guard let self else { return }
			guard isSpeaking || isPaused else { return }
			let wasSpeaking = isSpeaking
			isSpeaking = false
			isPaused = false
			armedBox = nil
			armedText = nil
			armedIsStale = false
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
				// The reconfiguration dropped the scheduled buffer.
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
				// A call ending while Paperback was already stopped must not restart playback.
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
			armedIsStale = false
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
			// Starting the engine unconditionally would keep the app alive in the background.
			if wasActive {
				activateAudioSession()
				try? engine.start()
			}
		}
	}

	// `isAutoAdvance` only from the utterance-finished callback: a user seek to the same text as the armed buffer would otherwise no-op instead of cutting off the audio.
	func speak(_ text: String, isAutoAdvance: Bool = false) {
		let text = preprocessText(text)
		// A stale armed buffer is not claimed: falling through stops the node and takes the re-synthesized one, so a setting change is heard a paragraph sooner at the cost of a blip.
		if isAutoAdvance, !armedIsStale, text == armedText, let box = armedBox {
			armedBox = nil
			armedText = nil
			armedIsStale = false
			speechGeneration += 1
			let gen = speechGeneration
			box.gen = gen
			lastScheduledGen = gen
			isSpeaking = true
			isPaused = false
			return
		}
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
		// A carried-over front buffer has no synthesis completion left to arm it.
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
		// pause() deactivated the session, and no new buffer is scheduled here to reclaim it.
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
		// Only an explicit stop tears these down: restarting them per paragraph round-trips through CoreAudio, which slows while locked and caused growing gaps in background playback.
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
		armedIsStale = false
	}

	// The current paragraph must already be on the node (lastScheduledGen == speechGeneration), or the next one would be scheduled ahead of it and play first.
	private func tryArmFirstSlotIfReady() {
		guard armedBox == nil, isSpeaking, !isPaused, lastScheduledGen == speechGeneration else { return }
		guard let first = prefetchQueue.first, let buf = first.buffer else { return }
		let text = first.text
		prefetchQueue.removeFirst()
		armNextBuffer(buf, text: text)
	}

	// Must not bump speechGeneration: the playing utterance's completion would look stale and be dropped, desyncing playback from position tracking.
	private func armNextBuffer(_ pcm: AVAudioPCMBuffer, text: String) {
		let box = GenBox()
		armedBox = box
		armedText = text
		armedIsStale = false
		player.scheduleBuffer(pcm) { [weak self] in
			DispatchQueue.main.async { [weak self] in
				guard let self else { return }
				// No generation means speak() never claimed it, and nothing else will restart the chain.
				guard let gen = box.gen else {
					self.recoverFromUnclaimedArmedBuffer(box)
					return
				}
				guard self.speechGeneration == gen else { return }
				self.isSpeaking = false
				self.isPaused = false
				self.onUtteranceFinished?()
			}
		}
	}

	/// Background throttling while locked can delay the previous completion until this buffer has played through; left alone the book stalls while still reporting speaking (#761).
	private func recoverFromUnclaimedArmedBuffer(_ box: GenBox) {
		guard armedBox === box else { return }
		armedBox = nil
		armedText = nil
		armedIsStale = false
		// Drops the earlier completion if it turns up late, which would otherwise advance again and skip a paragraph.
		speechGeneration += 1
		isSpeaking = false
		isPaused = false
		ttsLog.error("Armed buffer finished before speak() claimed it; continuing from it (#761)")
		onUtteranceFinished?()
	}

	private func invalidatePrefetch() {
		// Leaving the armed buffer alone made a rate change take two paragraphs to be heard rather than one.
		armedIsStale = armedBox != nil
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

	// Soft hyphens and nulls truncate utterances, and the synthesizer parses markup, so a bare `<` swallows text up to the next `>` (or wedges playback if unterminated).
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

	/// Set here, not at startup, because `.playback` is not mixable and would interrupt other apps on launch; repeating it also restores it after a media services reset.
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

	private func convertToOutput(_ buffers: [AVAudioPCMBuffer]) -> AVAudioPCMBuffer? {
		joinAndConvert(buffers).flatMap(withParagraphPause)
	}

	/// In the buffer rather than `postUtteranceDelay` so the silence survives the gapless hand-off to the next paragraph.
	private func withParagraphPause(_ buffer: AVAudioPCMBuffer) -> AVAudioPCMBuffer? {
		let format = buffer.format
		let silenceFrames = AVAudioFrameCount(format.sampleRate * Double(paragraphPauseMs) / 1000)
		guard silenceFrames > 0,
			let padded = AVAudioPCMBuffer(pcmFormat: format, frameCapacity: buffer.frameLength + silenceFrames),
			let source = buffer.floatChannelData,
			let destination = padded.floatChannelData
		else {
			return buffer
		}
		let frames = Int(buffer.frameLength)
		for channel in 0..<Int(format.channelCount) {
			memcpy(destination[channel], source[channel], frames * MemoryLayout<Float>.size)
			// `AVAudioPCMBuffer` does not promise zeroed storage.
			memset(destination[channel].advanced(by: frames), 0, Int(silenceFrames) * MemoryLayout<Float>.size)
		}
		padded.frameLength = buffer.frameLength + silenceFrames
		return padded
	}

	private func joinAndConvert(_ buffers: [AVAudioPCMBuffer]) -> AVAudioPCMBuffer? {
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
		// The converter calls the input block synchronously, before convert() returns, so the buffer never leaves this thread.
		nonisolated(unsafe) let input = synthBuf
		converter.convert(to: outBuf, error: &error) { _, outStatus in
			if inputConsumed {
				// Signal end-of-stream so the resampler flushes its tail; .noDataNow would discard it.
				outStatus.pointee = .endOfStream
				return nil
			}
			outStatus.pointee = .haveData
			inputConsumed = true
			return input
		}

		return error == nil ? outBuf : nil
	}
}

private final class BufferAccumulator: @unchecked Sendable {
	var buffers: [AVAudioPCMBuffer] = []
}
