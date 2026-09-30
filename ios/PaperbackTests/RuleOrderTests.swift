import XCTest
@testable import Paperback

/// The order rules run in decides what the reader hears when two of them touch the same text.
final class RuleOrderTests: XCTestCase {
	private func word(_ pattern: String, _ replacement: String) -> TtsRule {
		TtsRule(scope: .word, pattern: pattern, replacement: replacement)
	}

	private func paragraph(_ pattern: String, _ replacement: String) -> TtsRule {
		TtsRule(scope: .paragraph, pattern: pattern, replacement: replacement)
	}

	/// Paragraph rules run first so a word rule can refine what they produced. The other order
	/// would let a paragraph rule overwrite a word rule's work.
	func testParagraphRulesRunBeforeWordRules() {
		let rules = [
			word("middle", "final"),
			paragraph("start", "middle"),
		]
		XCTAssertEqual("final", applyRules(rules, to: "start", voiceId: nil))
	}

	/// Declaration order must not decide it either: the same two rules listed the other way
	/// round have to produce the same result.
	func testTheOrderDoesNotDependOnHowTheRulesAreListed() {
		let oneWay = [word("middle", "final"), paragraph("start", "middle")]
		let other = [paragraph("start", "middle"), word("middle", "final")]
		XCTAssertEqual(
			applyRules(oneWay, to: "start", voiceId: nil),
			applyRules(other, to: "start", voiceId: nil)
		)
	}

	/// Rules of the same scope apply in the order the reader put them in, so a later rule sees
	/// what an earlier one produced.
	func testRulesOfTheSameScopeApplyInListOrder() {
		let rules = [word("a", "b"), word("b", "c")]
		XCTAssertEqual("c", applyRules(rules, to: "a", voiceId: nil))
	}

	func testAnEmptyDictionaryLeavesTextAlone() {
		XCTAssertEqual("untouched", applyRules([], to: "untouched", voiceId: nil))
	}

	/// A disabled rule must not break the chain for the ones after it.
	func testADisabledRuleIsSkippedWithoutStoppingTheRest() {
		let rules = [
			TtsRule(isEnabled: false, scope: .word, pattern: "a", replacement: "x"),
			word("a", "b"),
		]
		XCTAssertEqual("b", applyRules(rules, to: "a", voiceId: nil))
	}

	/// The voice filter is checked per rule, so one rule being filtered out must not affect
	/// whether the others run.
	func testAFilteredOutRuleDoesNotAffectTheOthers() {
		let rules = [
			TtsRule(scope: .word, pattern: "a", replacement: "x", voiceFilter: .voices(["other"])),
			word("a", "b"),
		]
		XCTAssertEqual("b", applyRules(rules, to: "a", voiceId: "selected"))
	}
}

final class VoiceBaseNameTests: XCTestCase {
	/// Voice names arrive as "Samantha (Enhanced)". The picker groups by the bare name so the
	/// qualities of one voice sit together instead of reading as separate voices.
	func testATrailingQualitySuffixIsStripped() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha (Enhanced)"))
		XCTAssertEqual("Daniel", baseName(of: "Daniel (Premium)"))
	}

	func testANameWithNoSuffixIsLeftAlone() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha"))
	}

	/// Only the last parenthetical is a quality marker. A voice whose own name carries one, like
	/// "Alice (US)", keeps it when a quality is appended after.
	func testOnlyTheLastParentheticalIsStripped() {
		XCTAssertEqual("Alice (US)", baseName(of: "Alice (US) (Enhanced)"))
	}

	/// A name ending in a bracket with no opening " (" before it is not a suffix, so it stays.
	func testATrailingBracketThatIsNotASuffixIsKept() {
		XCTAssertEqual("Weird)", baseName(of: "Weird)"))
	}

	private func baseName(of name: String) -> String { voiceBaseName(ofName: name) }
}
