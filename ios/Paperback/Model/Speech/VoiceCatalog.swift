import AVFoundation

/// The installed voices, loaded on first read and kept until `invalidate` is called.
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

	/// Drops the cached list so the next read loads it again.
	func invalidate() {
		cached = nil
	}
}
