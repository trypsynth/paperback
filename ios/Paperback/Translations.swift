import Foundation

/// U+2063, invisible and unspoken by VoiceOver: translations are keyed by English text, so it makes a "many" form differ from an identical "few" form (Bosnian "sekunde" vs "sekundi").
/// Deliberate, not stray whitespace: don't strip it from translatable strings.
private let pluralManyMarker = "⁣"

func t(_ key: String) -> String {
	let value = NSLocalizedString(key, comment: "")
	guard value == key, key.hasSuffix(pluralManyMarker) else { return value }
	return String(key.dropLast(pluralManyMarker.count))
}

/// Read from the catalogue header rather than hardcoded, like Android's: Bosnian gives 21 the form of 1, Polish gives it to 1 alone, though both want three forms.
struct PluralRule {
	private let forms: Int
	private let expression: String

	init?(header: String?) {
		guard let line = header?.split(whereSeparator: \.isNewline).first(where: { $0.hasPrefix("Plural-Forms:") }).map(String.init), let formsAt = line.range(of: "nplurals="), let expressionAt = line.range(of: "plural=", range: formsAt.upperBound..<line.endIndex), let forms = Int(line[formsAt.upperBound...].prefix(while: \.isNumber)), forms > 0 else {
			return nil
		}
		let expression = (line[expressionAt.upperBound...].split(separator: ";", omittingEmptySubsequences: false).first ?? "").trimmingCharacters(in: .whitespaces)
		guard !expression.isEmpty else {
			return nil
		}
		self.forms = forms
		self.expression = expression
	}

	/// Nil on anything unrecognized, so a malformed header can't silently pick form 0 for every count.
	func formIndex(_ count: Int) -> Int? {
		guard let index = Self.ternary(expression, count), (0..<forms).contains(index) else {
			return nil
		}
		return index
	}

	private static func ternary(_ expression: String, _ n: Int) -> Int? {
		let text = withoutEnclosingParentheses(expression)
		guard let question = topLevelIndex(of: "?", in: text) else {
			return Int(text) ?? boolean(text, n).map { $0 ? 1 : 0 }
		}
		guard let colon = topLevelIndex(of: ":", in: text, from: text.index(after: question)), let condition = boolean(String(text[..<question]), n) else {
			return nil
		}
		return ternary(String(condition ? text[text.index(after: question)..<colon] : text[text.index(after: colon)...]), n)
	}

	private static func boolean(_ expression: String, _ n: Int) -> Bool? {
		let text = withoutEnclosingParentheses(expression)
		if let or = topLevelIndex(of: "||", in: text) {
			guard let left = boolean(String(text[..<or]), n) else {
				return nil
			}
			return left ? true : boolean(String(text[text.index(or, offsetBy: 2)...]), n)
		}
		if let and = topLevelIndex(of: "&&", in: text) {
			guard let left = boolean(String(text[..<and]), n) else {
				return nil
			}
			return left ? boolean(String(text[text.index(and, offsetBy: 2)...]), n) : false
		}
		for op in ["==", "!=", ">=", "<=", ">", "<"] {
			guard let at = topLevelIndex(of: op, in: text) else {
				continue
			}
			guard let left = arithmetic(String(text[..<at]), n), let right = arithmetic(String(text[text.index(at, offsetBy: op.count)...]), n) else {
				return nil
			}
			switch op {
			case "==": return left == right
			case "!=": return left != right
			case ">=": return left >= right
			case "<=": return left <= right
			case ">": return left > right
			default: return left < right
			}
		}
		return nil
	}

	private static func arithmetic(_ expression: String, _ n: Int) -> Int? {
		let text = withoutEnclosingParentheses(expression)
		if let modulo = topLevelIndex(of: "%", in: text) {
			guard let left = arithmetic(String(text[..<modulo]), n), let right = arithmetic(String(text[text.index(after: modulo)...]), n), right != 0 else {
				return nil
			}
			return left % right
		}
		return text == "n" ? n : Int(text)
	}

	private static func withoutEnclosingParentheses(_ expression: String) -> String {
		var text = expression.trimmingCharacters(in: .whitespaces)
		while text.count > 1, text.hasPrefix("("), text.hasSuffix(")"), outerParenthesisSpansAll(text) {
			text = String(text.dropFirst().dropLast()).trimmingCharacters(in: .whitespaces)
		}
		return text
	}

	/// False for `(a) && (b)`, whose outer parentheses look enclosing but are two separate groups.
	private static func outerParenthesisSpansAll(_ text: String) -> Bool {
		var depth = 0
		for index in text.indices {
			if text[index] == "(" {
				depth += 1
			} else if text[index] == ")" {
				depth -= 1
				if depth == 0 {
					return text.index(after: index) == text.endIndex
				}
			}
		}
		return false
	}

	private static func topLevelIndex(of token: String, in text: String, from start: String.Index? = nil) -> String.Index? {
		var depth = 0
		var index = start ?? text.startIndex
		while index < text.endIndex {
			if text[index] == "(" {
				depth += 1
			} else if text[index] == ")" {
				depth -= 1
			}
			if depth == 0, text[index...].hasPrefix(token) {
				// "<" must not match inside "<=".
				let next = text.index(after: index)
				if token.count > 1 || next == text.endIndex || text[next] != "=" {
					return index
				}
			}
			index = text.index(after: index)
		}
		return nil
	}
}

/// The po header is the entry with the empty msgid, which `NSLocalizedString` can't look up, so it's read from the file. English has no catalogue.
private let catalogueRule: PluralRule? = {
	guard let path = Bundle.main.path(forResource: "Localizable", ofType: "strings"), let strings = NSDictionary(contentsOfFile: path) else {
		return nil
	}
	return PluralRule(header: strings[""] as? String)
}()

/// Callers must wrap each form in `t(...)`: the pot scanner only extracts plain `t("...")` calls, so a bare literal here is never translated.
/// English is what reaches the fallback, since it has no catalogue.
func nt(_ one: String, _ few: String, _ many: String, _ count: Int, rule: PluralRule? = catalogueRule) -> String {
	if let index = rule?.formIndex(count) {
		return [one, few, many][min(index, 2)]
	}
	return count == 1 ? one : many
}
