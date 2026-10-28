import AVFoundation

/// The app's own voice, rate and pitch, and the open book's, which take their place wherever the book has them.
@MainActor
@Observable
final class SpeechSettings {
	@ObservationIgnored private var config: ConfigManagerFfi?
	@ObservationIgnored private var tts: TtsManager?
	private(set) var defaultVoice: String?
	private(set) var defaultRate = AVSpeechUtteranceDefaultSpeechRate
	private(set) var defaultPitch: Float = 1.0
	/// A voice of "" is the system's default voice, chosen for this book over the app's.
	private(set) var book = DocumentSpeech(voice: nil, ratePercent: nil, pitchPercent: nil)
	@ObservationIgnored private var bookPath: String?
	@ObservationIgnored private var applying = false

	/// Once the config is loaded, which the view model does after its own properties exist.
	func start(config: ConfigManagerFfi, tts: TtsManager) {
		self.config = config
		self.tts = tts
		let voice = config.getAppString(key: "tts_voice_identifier", defaultValue: "")
		defaultVoice = voice.isEmpty ? nil : voice
		defaultRate = Float(config.getAppString(key: "tts_speech_rate", defaultValue: "")) ?? AVSpeechUtteranceDefaultSpeechRate
		defaultPitch = Float(config.getAppString(key: "tts_pitch", defaultValue: "")) ?? 1.0
		apply()
		// The reading bar changes whatever is in effect: the book's own setting if it has one, else the app's.
		tts.onSpeechRateChanged = { [weak self] rate in
			guard let self, !applying else { return }
			if book.ratePercent != nil { setBookRatePercent(percentForSpeechRate(rate)) } else { setDefaultRate(rate) }
		}
		tts.onPitchChanged = { [weak self] pitch in
			guard let self, !applying else { return }
			if book.pitchPercent != nil { setBookPitchPercent(percentForPitch(pitch)) } else { setDefaultPitch(pitch) }
		}
	}

	/// The voice speaking now, nil for the system's default.
	var effectiveVoice: String? {
		guard let voice = book.voice else { return defaultVoice }
		// A voice chosen on another device, or since removed, falls back to the app's.
		guard voice.isEmpty || AVSpeechSynthesisVoice(identifier: voice) != nil else { return defaultVoice }
		return voice.isEmpty ? nil : voice
	}

	func load(bookAt path: String?) {
		bookPath = path
		book = path.flatMap { config?.getDocumentSpeech(path: $0) } ?? DocumentSpeech(voice: nil, ratePercent: nil, pitchPercent: nil)
		apply()
	}

	func setDefaultVoice(_ voice: String?) {
		defaultVoice = voice
		config?.setAppString(key: "tts_voice_identifier", value: voice ?? "")
		apply()
	}

	func setDefaultRate(_ rate: Float) {
		defaultRate = rate
		config?.setAppString(key: "tts_speech_rate", value: "\(rate)")
		apply()
	}

	func setDefaultPitch(_ pitch: Float) {
		defaultPitch = pitch
		config?.setAppString(key: "tts_pitch", value: "\(pitch)")
		apply()
	}

	/// nil follows the app's voice again.
	func setBookVoice(_ voice: String?) {
		book.voice = voice
		saveBook()
	}

	func setBookRatePercent(_ percent: Int?) {
		book.ratePercent = percent.map { Int32(min(max($0, 0), 100)) }
		saveBook()
	}

	func setBookPitchPercent(_ percent: Int?) {
		book.pitchPercent = percent.map { Int32(min(max($0, 0), 100)) }
		saveBook()
	}

	private func saveBook() {
		if let bookPath { config?.setDocumentSpeech(path: bookPath, speech: book) }
		apply()
	}

	private func apply() {
		guard let tts else { return }
		applying = true
		defer { applying = false }
		let voice = effectiveVoice
		if tts.selectedVoiceIdentifier != voice { restartingSpeech(tts) { tts.selectedVoiceIdentifier = voice } }
		tts.speechRate = book.ratePercent.map { speechRateForPercent(Int($0)) } ?? defaultRate
		tts.pitch = book.pitchPercent.map { pitchForPercent(Int($0)) } ?? defaultPitch
	}

	/// A voice change only reaches what is spoken next, so reading carries on in the new voice and a paused buffer of the old one is dropped.
	private func restartingSpeech(_ tts: TtsManager, _ change: () -> Void) {
		let wasPlaying = tts.isSpeaking
		let wasPaused = tts.isPaused
		change()
		if wasPlaying || wasPaused { tts.stop() }
		if wasPlaying { onRestart?() }
	}

	/// Starts the current segment again, which the reading controller owns.
	@ObservationIgnored var onRestart: (() -> Void)?
}
