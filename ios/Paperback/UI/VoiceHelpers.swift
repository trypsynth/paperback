import AVFoundation

struct VoiceSection: Identifiable {
	var id: String { language }
	let language: String
	let items: [VoiceItem]
}

struct VoiceItem: Identifiable {
	let id: String
	let label: String
	let identifier: String
}

func voiceBaseName(_ voice: AVSpeechSynthesisVoice) -> String {
	let name = voice.name
	guard name.hasSuffix(")"), let range = name.range(of: " (", options: .backwards) else {
		return name
	}
	return String(name[..<range.lowerBound])
}

func qualityLabel(_ voice: AVSpeechSynthesisVoice) -> String {
	switch voice.quality {
	case .enhanced: return "Enhanced"
	case .premium: return "Premium"
	default: return "Standard"
	}
}

func languageLabel(_ code: String) -> String {
	Locale.current.localizedString(forIdentifier: code) ?? code
}

func buildSections(from voices: [AVSpeechSynthesisVoice]) -> [VoiceSection] {
	let nativeCode = Locale.current.language.languageCode?.identifier ?? ""

	var byLang: [String: [String: [AVSpeechSynthesisVoice]]] = [:]
	for voice in voices {
		byLang[voice.language, default: [:]][voiceBaseName(voice), default: []].append(voice)
	}

	return byLang.map { lang, families -> VoiceSection in
		let items = families
			.sorted { $0.key < $1.key }
			.flatMap { baseName, voices -> [VoiceItem] in
				let sorted = voices.sorted { $0.quality.rawValue < $1.quality.rawValue }
				if sorted.count == 1 {
					let v = sorted[0]
					return [VoiceItem(id: v.identifier, label: baseName, identifier: v.identifier)]
				}
				return sorted.map { v in
					VoiceItem(id: v.identifier, label: "\(baseName) — \(qualityLabel(v))", identifier: v.identifier)
				}
			}
		return VoiceSection(language: lang, items: items)
	}
	.sorted { a, b in
		let aCode = a.language.components(separatedBy: "-").first ?? ""
		let bCode = b.language.components(separatedBy: "-").first ?? ""
		let aNative = aCode == nativeCode
		let bNative = bCode == nativeCode
		if aNative != bNative { return aNative }
		return a.language < b.language
	}
}
