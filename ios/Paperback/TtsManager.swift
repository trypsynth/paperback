import AVFoundation

@MainActor
final class TtsManager: NSObject, ObservableObject {
	private let synthesizer = AVSpeechSynthesizer()

	@Published var isSpeaking = false
	@Published var isPaused = false
	@Published var speechRate: Float = AVSpeechUtteranceDefaultSpeechRate
	@Published var pitch: Float = 1.0
	@Published var selectedVoice: AVSpeechSynthesisVoice? = nil

	var availableVoices: [AVSpeechSynthesisVoice] {
		AVSpeechSynthesisVoice.speechVoices()
	}

	var onUtteranceFinished: (() -> Void)? = nil

	override init() {
		super.init()
		synthesizer.delegate = self
		try? AVAudioSession.sharedInstance().setCategory(
			.playback,
			mode: .spokenAudio,
			options: [.duckOthers, .mixWithOthers]
		)
		try? AVAudioSession.sharedInstance().setActive(true)
	}

	func speak(_ text: String) {
		synthesizer.stopSpeaking(at: .immediate)
		let utterance = AVSpeechUtterance(string: text)
		utterance.rate = speechRate
		utterance.pitchMultiplier = pitch
		utterance.voice = selectedVoice
		synthesizer.speak(utterance)
		isSpeaking = true
		isPaused = false
	}

	func pause() {
		synthesizer.pauseSpeaking(at: .word)
	}

	func resume() {
		guard isPaused else { return }
		synthesizer.continueSpeaking()
	}

	func stop() {
		synthesizer.stopSpeaking(at: .immediate)
		isSpeaking = false
		isPaused = false
	}
}

extension TtsManager: AVSpeechSynthesizerDelegate {
	nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
		Task { @MainActor in
			self.isSpeaking = false
			self.isPaused = false
			self.onUtteranceFinished?()
		}
	}

	nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didPause utterance: AVSpeechUtterance) {
		Task { @MainActor in
			self.isSpeaking = false
			self.isPaused = true
		}
	}

	nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didContinue utterance: AVSpeechUtterance) {
		Task { @MainActor in
			self.isSpeaking = true
			self.isPaused = false
		}
	}

	nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
		Task { @MainActor in
			self.isSpeaking = false
			self.isPaused = false
		}
	}
}
