import XCTest
@testable import Paperback

final class RuleOrderTests: XCTestCase {
	private func word(_ pattern: String, _ replacement: String) -> TtsRule {
		TtsRule(scope: .word, pattern: pattern, replacement: replacement)
	}

	private func paragraph(_ pattern: String, _ replacement: String) -> TtsRule {
		TtsRule(scope: .paragraph, pattern: pattern, replacement: replacement)
	}

	/// The other order would let a paragraph rule overwrite a word rule's work.
	func testParagraphRulesRunBeforeWordRules() {
		let rules = [
			word("middle", "final"),
			paragraph("start", "middle"),
		]
		XCTAssertEqual("final", applyRules(rules, to: "start", voiceId: nil))
	}

	func testTheOrderDoesNotDependOnHowTheRulesAreListed() {
		let oneWay = [word("middle", "final"), paragraph("start", "middle")]
		let other = [paragraph("start", "middle"), word("middle", "final")]
		XCTAssertEqual(
			applyRules(oneWay, to: "start", voiceId: nil),
			applyRules(other, to: "start", voiceId: nil)
		)
	}

	func testRulesOfTheSameScopeApplyInListOrder() {
		let rules = [word("a", "b"), word("b", "c")]
		XCTAssertEqual("c", applyRules(rules, to: "a", voiceId: nil))
	}

	func testAnEmptyDictionaryLeavesTextAlone() {
		XCTAssertEqual("untouched", applyRules([], to: "untouched", voiceId: nil))
	}

	func testADisabledRuleIsSkippedWithoutStoppingTheRest() {
		let rules = [
			TtsRule(isEnabled: false, scope: .word, pattern: "a", replacement: "x"),
			word("a", "b"),
		]
		XCTAssertEqual("b", applyRules(rules, to: "a", voiceId: nil))
	}

	func testAFilteredOutRuleDoesNotAffectTheOthers() {
		let rules = [
			TtsRule(scope: .word, pattern: "a", replacement: "x", voiceFilter: .voices(["other"])),
			word("a", "b"),
		]
		XCTAssertEqual("b", applyRules(rules, to: "a", voiceId: "selected"))
	}
}

final class VoiceBaseNameTests: XCTestCase {
	/// Voice names arrive as "Samantha (Enhanced)"; grouping by bare name keeps one voice's qualities together.
	func testATrailingQualitySuffixIsStripped() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha (Enhanced)"))
		XCTAssertEqual("Daniel", baseName(of: "Daniel (Premium)"))
	}

	func testANameWithNoSuffixIsLeftAlone() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha"))
	}

	/// Only the last parenthetical is a quality marker; a name like "Alice (US)" keeps its own.
	func testOnlyTheLastParentheticalIsStripped() {
		XCTAssertEqual("Alice (US)", baseName(of: "Alice (US) (Enhanced)"))
	}

	func testATrailingBracketThatIsNotASuffixIsKept() {
		XCTAssertEqual("Weird)", baseName(of: "Weird)"))
	}

	private func baseName(of name: String) -> String { voiceBaseName(ofName: name) }
}
