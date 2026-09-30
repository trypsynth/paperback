import XCTest
@testable import Paperback

/// The speech dictionary rewrites text on its way to the synthesiser, so a rule that matches
/// more than the reader meant changes words they never asked about, silently and everywhere.
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

	/// Without whole-word matching a short pattern rewrites the inside of longer words, which is
	/// the classic way a speech dictionary starts mangling unrelated prose.
	func testWholeWordMatchingLeavesLongerWordsAlone() {
		let loose = rule(pattern: "cat", replacement: "dog")
		XCTAssertEqual("dog in a dogalogue", loose.apply(to: "cat in a catalogue", voiceId: nil))

		let strict = rule(pattern: "cat", replacement: "dog", wholeWord: true)
		XCTAssertEqual("dog in a catalogue", strict.apply(to: "cat in a catalogue", voiceId: nil))
	}

	/// A whole-word pattern is escaped before it becomes a regex, so punctuation in it matches
	/// itself rather than acting as a metacharacter.
	func testAWholeWordPatternTreatsPunctuationLiterally() {
		let r = rule(pattern: "a.b", replacement: "X", wholeWord: true)
		XCTAssertEqual("X", r.apply(to: "a.b", voiceId: nil))
		XCTAssertEqual("axb", r.apply(to: "axb", voiceId: nil))
	}

	/// The replacement is escaped too, so a dollar sign in it reads as itself instead of being
	/// taken for a capture group reference.
	func testAWholeWordReplacementIsNotTreatedAsATemplate() {
		let r = rule(pattern: "price", replacement: "$5", wholeWord: true)
		XCTAssertEqual("$5", r.apply(to: "price", voiceId: nil))
	}

	func testAParagraphRegexRuleCanUseCaptureGroups() {
		let r = rule(scope: .paragraph, matchType: .regex, pattern: "(\\d+)-(\\d+)", replacement: "$1 to $2")
		XCTAssertEqual("pages 10 to 12", r.apply(to: "pages 10-12", voiceId: nil))
	}

	/// An unparseable pattern must leave the text untouched rather than throw away the
	/// paragraph the reader was about to hear.
	func testAnInvalidRegexLeavesTheTextAlone() {
		let r = rule(scope: .paragraph, matchType: .regex, pattern: "([unclosed", replacement: "X")
		XCTAssertEqual("untouched", r.apply(to: "untouched", voiceId: nil))
	}

	/// A literal paragraph rule must not quietly behave as a regex, or a pattern containing a
	/// dot would match any character.
	func testALiteralParagraphRuleDoesNotActAsARegex() {
		let r = rule(scope: .paragraph, matchType: .literal, pattern: "a.b", replacement: "X")
		XCTAssertEqual("X", r.apply(to: "a.b", voiceId: nil))
		XCTAssertEqual("axb", r.apply(to: "axb", voiceId: nil))
	}

	func testADisabledRuleChangesNothing() {
		let r = rule(pattern: "cat", replacement: "dog", isEnabled: false)
		XCTAssertEqual("cat", r.apply(to: "cat", voiceId: nil))
	}

	/// An empty pattern would otherwise match at every position and splice the replacement
	/// between every character.
	func testAnEmptyPatternChangesNothing() {
		let r = rule(pattern: "", replacement: "X")
		XCTAssertEqual("hello", r.apply(to: "hello", voiceId: nil))
	}

	func testARuleLimitedToOtherVoicesDoesNotApply() {
		let r = rule(pattern: "cat", replacement: "dog", voiceFilter: .voices(["com.example.voice.a"]))
		XCTAssertEqual("cat", r.apply(to: "cat", voiceId: "com.example.voice.b"))
		XCTAssertEqual("dog", r.apply(to: "cat", voiceId: "com.example.voice.a"))
	}

	/// With no voice selected there is nothing for a voice-specific rule to match against, so it
	/// has to stay out of the way rather than apply to everything.
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

/// Rules are stored as JSON in UserDefaults, so a shape that does not survive a round trip loses
/// the reader's whole speech dictionary on the next launch.
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

	/// The filter is the one field with a hand-written Codable conformance, so each case needs
	/// checking rather than trusting the synthesised one.
	func testEveryVoiceFilterCaseSurvivesEncodingAndDecoding() throws {
		for filter in [TtsRule.VoiceFilter.all, .language("en-US"), .voices(["a", "b"])] {
			let original = TtsRule(pattern: "x", replacement: "y", voiceFilter: filter)
			XCTAssertEqual(filter, try roundTrip(original).voiceFilter, "\(filter) did not survive")
		}
	}
}
