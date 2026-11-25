import Foundation

let legacySpeechRulesKey = "tts_rules"

/// A speech dictionary rule as iOS kept it in UserDefaults before the dictionary moved to core; read only to migrate it.
struct TtsRule: Decodable {
	var isEnabled: Bool
	var scope: Scope
	var matchType: MatchType
	var pattern: String
	var replacement: String
	var wholeWord: Bool
	var voiceFilter: VoiceFilter

	enum Scope: String, Decodable {
		case word
		case paragraph
	}

	enum MatchType: String, Decodable {
		case literal
		case regex
	}

	enum VoiceFilter: Decodable {
		case all
		case language(String)
		case voices([String])

		private enum CodingKeys: String, CodingKey { case type, value, values }

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
	}

	var speechRule: SpeechRule {
		let filter: Paperback.VoiceFilter
		switch voiceFilter {
		case .all: filter = .all
		case .language(let language): filter = .language(language)
		case .voices(let ids): filter = .voices(ids)
		}
		let isRegex = scope == .paragraph && matchType == .regex
		return SpeechRule(enabled: isEnabled, scope: scope == .word ? .word : .paragraph, matchType: matchType == .regex ? .regex : .literal, pattern: pattern, replacement: isRegex ? Self.nvdaReplacement(replacement) : replacement, wholeWord: wholeWord, voiceFilter: filter)
	}

	/// iOS read a regex replacement as an NSRegularExpression template after turning `\1` into `$1`, so `$1` was a group too and other escapes dropped their backslash; core reads only `\1`.
	static func nvdaReplacement(_ template: String) -> String {
		let chars = Array(template)
		var result = ""
		var i = 0
		while i < chars.count {
			let next: Character? = i + 1 < chars.count ? chars[i + 1] : nil
			if chars[i] == "\\" || chars[i] == "$", let next, ("0"..."9").contains(next) {
				result += "\\\(next)"
				i += 2
			} else if chars[i] == "\\", let next {
				result.append(next)
				i += 2
			} else {
				result.append(chars[i])
				i += 1
			}
		}
		return result
	}
}

/// Runs once: the key goes when the rules are in core, but stays if they can't be read, so a failed decode loses nothing.
func migrateLegacySpeechRules(from defaults: UserDefaults, into config: ConfigManagerFfi) {
	guard let data = defaults.data(forKey: legacySpeechRulesKey), let legacy = try? JSONDecoder().decode([TtsRule].self, from: data) else { return }
	if !legacy.isEmpty, config.getSpeechRules().isEmpty {
		config.setSpeechRules(rules: legacy.map(\.speechRule))
		config.flush()
	}
	defaults.removeObject(forKey: legacySpeechRulesKey)
}
