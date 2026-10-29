import AVFoundation

final class VoiceCatalog {
	private let load: () -> [AVSpeechSynthesisVoice]
	private var cached: [AVSpeechSynthesisVoice]?

	init(load: @escaping () -> [AVSpeechSynthesisVoice]) {
		self.load = load
	}

	var voices: [AVSpeechSynthesisVoice] {
		if let cached { return cached }
		let loaded = load()
		cached = loaded
		return loaded
	}

	func invalidate() {
		cached = nil
	}
}
