import AVFoundation
import Foundation

struct TtsRule: Codable, Identifiable, Equatable {
	var id: UUID
	var isEnabled: Bool
	var scope: Scope
	var matchType: MatchType
	var pattern: String
	var replacement: String
	var wholeWord: Bool
	var voiceFilter: VoiceFilter

	init(
		id: UUID = UUID(),
		isEnabled: Bool = true,
		scope: Scope = .word,
		matchType: MatchType = .literal,
		pattern: String = "",
		replacement: String = "",
		wholeWord: Bool = false,
		voiceFilter: VoiceFilter = .all
	) {
		self.id = id
		self.isEnabled = isEnabled
		self.scope = scope
		self.matchType = matchType
		self.pattern = pattern
		self.replacement = replacement
		self.wholeWord = wholeWord
		self.voiceFilter = voiceFilter
	}

	enum Scope: String, Codable, CaseIterable {
		case word
		case paragraph
	}

	enum MatchType: String, Codable, CaseIterable {
		case literal
		case regex
	}

	enum VoiceFilter: Codable, Equatable {
		case all
		case language(String)
		case voices([String])

		private enum CodingKeys: String, CodingKey { case type, value, values }

		func encode(to encoder: Encoder) throws {
			var c = encoder.container(keyedBy: CodingKeys.self)
			switch self {
			case .all:
				try c.encode("all", forKey: .type)
			case .language(let lang):
				try c.encode("language", forKey: .type)
				try c.encode(lang, forKey: .value)
			case .voices(let ids):
				try c.encode("voices", forKey: .type)
				try c.encode(ids, forKey: .values)
			}
		}

		init(from decoder: Decoder) throws {
			let c = try decoder.container(keyedBy: CodingKeys.self)
			switch try c.decode(String.self, forKey: .type) {
			case "language":
				self = .language(try c.decode(String.self, forKey: .value))
			case "voices":
				self = .voices(try c.decode([String].self, forKey: .values))
			default:
				self = .all
			}
		}

		func matches(voiceId: String?) -> Bool {
			switch self {
			case .all:
				return true
			case .language(let lang):
				guard let id = voiceId, let voice = AVSpeechSynthesisVoice(identifier: id) else { return false }
				return voice.language == lang
			case .voices(let ids):
				guard let id = voiceId else { return false }
				return ids.contains(id)
			}
		}

		var label: String {
			switch self {
			case .all:
				return "All voices"
			case .language(let lang):
				let name = Locale.current.localizedString(forIdentifier: lang) ?? lang
				return "All \(name) voices"
			case .voices(let ids):
				if ids.isEmpty { return "No voices selected" }
				if ids.count == 1, let v = AVSpeechSynthesisVoice(identifier: ids[0]) { return v.name }
				return "\(ids.count) voices"
			}
		}
	}

	func apply(to text: String, voiceId: String?) -> String {
		guard isEnabled, voiceFilter.matches(voiceId: voiceId), !pattern.isEmpty else { return text }
		switch scope {
		case .word:
			if wholeWord {
				let esc = NSRegularExpression.escapedPattern(for: pattern)
				guard let rx = try? NSRegularExpression(pattern: "\\b\(esc)\\b") else { return text }
				return rx.stringByReplacingMatches(
					in: text, range: NSRange(text.startIndex..., in: text),
					withTemplate: NSRegularExpression.escapedTemplate(for: replacement)
				)
			}
			return text.replacingOccurrences(of: pattern, with: replacement)
		case .paragraph:
			if matchType == .regex {
				guard let rx = try? NSRegularExpression(pattern: pattern) else { return text }
				return rx.stringByReplacingMatches(
					in: text, range: NSRange(text.startIndex..., in: text),
					withTemplate: replacement
				)
			}
			return text.replacingOccurrences(of: pattern, with: replacement)
		}
	}
}
