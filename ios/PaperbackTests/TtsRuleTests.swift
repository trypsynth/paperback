import XCTest
@testable import Paperback

final class TtsRuleTests: XCTestCase {
	private func rule(
		scope: TtsRule.Scope = .word,
		matchType: TtsRule.MatchType = .literal,
		pattern: String,
		replacement: String,
		wholeWord: Bool = false,
		isEnabled: Bool = true,
		voiceFilter: TtsRule.VoiceFilter = .all
	) -> TtsRule {
		TtsRule(
			isEnabled: isEnabled,
			scope: scope,
			matchType: matchType,
			pattern: pattern,
			replacement: replacement,
			wholeWord: wholeWord,
			voiceFilter: voiceFilter
		)
	}

	func testASimpleWordRuleRewritesEveryOccurrence() {
		let r = rule(pattern: "Dr.", replacement: "Doctor")
		XCTAssertEqual("Doctor Who met Doctor Song", r.apply(to: "Dr. Who met Dr. Song", voiceId: nil))
	}

	func testWholeWordMatchingLeavesLongerWordsAlone() {
		let loose = rule(pattern: "cat", replacement: "dog")
		XCTAssertEqual("dog in a dogalogue", loose.apply(to: "cat in a catalogue", voiceId: nil))

		let strict = rule(pattern: "cat", replacement: "dog", wholeWord: true)
		XCTAssertEqual("dog in a catalogue", strict.apply(to: "cat in a catalogue", voiceId: nil))
	}

	func testAWholeWordPatternTreatsPunctuationLiterally() {
		let r = rule(pattern: "a.b", replacement: "X", wholeWord: true)
		XCTAssertEqual("X", r.apply(to: "a.b", voiceId: nil))
		XCTAssertEqual("axb", r.apply(to: "axb", voiceId: nil))
	}

	func testAWholeWordReplacementIsNotTreatedAsATemplate() {
		let r = rule(pattern: "price", replacement: "$5", wholeWord: true)
		XCTAssertEqual("$5", r.apply(to: "price", voiceId: nil))
	}

	func testAParagraphRegexRuleCanUseCaptureGroups() {
		let r = rule(scope: .paragraph, matchType: .regex, pattern: "(\\d+)-(\\d+)", replacement: "$1 to $2")
		XCTAssertEqual("pages 10 to 12", r.apply(to: "pages 10-12", voiceId: nil))
	}

	func testABackslashCaptureGroupWorksAsTheEditorSays() {
		let r = rule(scope: .paragraph, matchType: .regex, pattern: #"(\d+)-(\d+)"#, replacement: #"\1 to \2"#)
		XCTAssertEqual("pages 10 to 12", r.apply(to: "pages 10-12", voiceId: nil))
	}

	func testAnInvalidRegexLeavesTheTextAlone() {
		let r = rule(scope: .paragraph, matchType: .regex, pattern: "([unclosed", replacement: "X")
		XCTAssertEqual("untouched", r.apply(to: "untouched", voiceId: nil))
	}

	func testALiteralParagraphRuleDoesNotActAsARegex() {
		let r = rule(scope: .paragraph, matchType: .literal, pattern: "a.b", replacement: "X")
		XCTAssertEqual("X", r.apply(to: "a.b", voiceId: nil))
		XCTAssertEqual("axb", r.apply(to: "axb", voiceId: nil))
	}

	func testADisabledRuleChangesNothing() {
		let r = rule(pattern: "cat", replacement: "dog", isEnabled: false)
		XCTAssertEqual("cat", r.apply(to: "cat", voiceId: nil))
	}

	/// An empty pattern would otherwise splice the replacement between every character.
	func testAnEmptyPatternChangesNothing() {
		let r = rule(pattern: "", replacement: "X")
		XCTAssertEqual("hello", r.apply(to: "hello", voiceId: nil))
	}

	func testARuleLimitedToOtherVoicesDoesNotApply() {
		let r = rule(pattern: "cat", replacement: "dog", voiceFilter: .voices(["com.example.voice.a"]))
		XCTAssertEqual("cat", r.apply(to: "cat", voiceId: "com.example.voice.b"))
		XCTAssertEqual("dog", r.apply(to: "cat", voiceId: "com.example.voice.a"))
	}

	func testAVoiceSpecificRuleDoesNotApplyWhenNoVoiceIsSelected() {
		let r = rule(pattern: "cat", replacement: "dog", voiceFilter: .voices(["com.example.voice.a"]))
		XCTAssertEqual("cat", r.apply(to: "cat", voiceId: nil))
	}

	func testARuleForAllVoicesAppliesWhateverIsSelected() {
		let r = rule(pattern: "cat", replacement: "dog", voiceFilter: .all)
		XCTAssertEqual("dog", r.apply(to: "cat", voiceId: nil))
		XCTAssertEqual("dog", r.apply(to: "cat", voiceId: "com.example.voice.a"))
	}
}

/// A shape that doesn't survive the UserDefaults round trip loses the reader's whole speech dictionary on the next launch.
final class TtsRuleCodingTests: XCTestCase {
	private func roundTrip(_ rule: TtsRule) throws -> TtsRule {
		let data = try JSONEncoder().encode(rule)
		return try JSONDecoder().decode(TtsRule.self, from: data)
	}

	func testARuleSurvivesEncodingAndDecoding() throws {
		let original = TtsRule(
			isEnabled: false,
			scope: .paragraph,
			matchType: .regex,
			pattern: "(\\d+)",
			replacement: "number $1",
			wholeWord: true,
			voiceFilter: .all
		)
		XCTAssertEqual(original, try roundTrip(original))
	}

	/// The filter is the one field with a hand-written Codable conformance.
	func testEveryVoiceFilterCaseSurvivesEncodingAndDecoding() throws {
		for filter in [TtsRule.VoiceFilter.all, .language("en-US"), .voices(["a", "b"])] {
			let original = TtsRule(pattern: "x", replacement: "y", voiceFilter: filter)
			XCTAssertEqual(filter, try roundTrip(original).voiceFilter, "\(filter) did not survive")
		}
	}
}
