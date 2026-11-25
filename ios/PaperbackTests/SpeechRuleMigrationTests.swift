import XCTest
@testable import Paperback

/// A rule lost or changed in the move to core is one the reader made by hand and won't notice is gone until a book mispronounces it again.
@MainActor
final class SpeechRuleMigrationTests: XCTestCase {
	private var suiteName = ""
	private var configPath = ""
	private var defaults: UserDefaults!
	private var config: ConfigManagerFfi!

	override func setUp() {
		super.setUp()
		suiteName = "speech-rules-\(UUID().uuidString)"
		defaults = UserDefaults(suiteName: suiteName)
		configPath = FileManager.default.temporaryDirectory.appending(path: "\(UUID().uuidString).ini").path(percentEncoded: false)
		config = ConfigManagerFfi()
		_ = config.initialize(configPath: configPath)
	}

	override func tearDown() {
		defaults.removePersistentDomain(forName: suiteName)
		super.tearDown()
	}

	private func rule(_ pattern: String, _ replacement: String, scope: RuleScope = .word, matchType: MatchType = .literal, wholeWord: Bool = false, enabled: Bool = true, voiceFilter: VoiceFilter = .all) -> SpeechRule {
		SpeechRule(enabled: enabled, scope: scope, matchType: matchType, pattern: pattern, replacement: replacement, wholeWord: wholeWord, voiceFilter: voiceFilter)
	}

	/// Exactly what the old JSONEncoder wrote, id and all.
	private let savedByOldIos = """
	[
		{"id":"6F1C2B9A-3D4E-4F50-8A61-7B8C9D0E1F20","isEnabled":true,"scope":"word","matchType":"literal","pattern":"Dr.","replacement":"Doctor","wholeWord":true,"voiceFilter":{"type":"language","value":"en-GB"}},
		{"id":"0A1B2C3D-4E5F-4061-8273-8495A6B7C8D9","isEnabled":false,"scope":"paragraph","matchType":"regex","pattern":"(\\\\d+)-(\\\\d+)","replacement":"\\\\1 to $2","wholeWord":false,"voiceFilter":{"type":"voices","values":["com.apple.voice.a","com.apple.voice.b"]}},
		{"id":"1B2C3D4E-5F60-4172-8384-95A6B7C8D9E0","isEnabled":true,"scope":"paragraph","matchType":"literal","pattern":"$5","replacement":"five dollars","wholeWord":false,"voiceFilter":{"type":"all"}}
	]
	"""

	func testTheOldRulesMoveToCoreInOrderAndTheOldCopyGoes() {
		defaults.set(Data(savedByOldIos.utf8), forKey: legacySpeechRulesKey)
		migrateLegacySpeechRules(from: defaults, into: config)
		XCTAssertEqual(config.getSpeechRules(), [
			rule("Dr.", "Doctor", wholeWord: true, voiceFilter: .language("en-GB")),
			rule(#"(\d+)-(\d+)"#, #"\1 to \2"#, scope: .paragraph, matchType: .regex, enabled: false, voiceFilter: .voices(["com.apple.voice.a", "com.apple.voice.b"])),
			rule("$5", "five dollars", scope: .paragraph),
		])
		XCTAssertNil(defaults.data(forKey: legacySpeechRulesKey))
	}

	/// The migration runs on every launch until the key is gone, so dropping unreadable rules would lose them for good.
	func testRulesThatCannotBeReadAreLeftWhereTheyAre() {
		let unreadable = Data("not json".utf8)
		defaults.set(unreadable, forKey: legacySpeechRulesKey)
		migrateLegacySpeechRules(from: defaults, into: config)
		XCTAssertEqual(defaults.data(forKey: legacySpeechRulesKey), unreadable)
		XCTAssertEqual(config.getSpeechRules(), [])
	}

	func testRulesAlreadyInCoreAreNotReplaced() {
		let existing = [rule("cat", "dog")]
		config.setSpeechRules(rules: existing)
		defaults.set(Data(savedByOldIos.utf8), forKey: legacySpeechRulesKey)
		migrateLegacySpeechRules(from: defaults, into: config)
		XCTAssertEqual(config.getSpeechRules(), existing)
		XCTAssertNil(defaults.data(forKey: legacySpeechRulesKey))
	}

	/// The migrated rules survive a restart, because launch is when the old copy is gone.
	func testTheMovedRulesAreSaved() {
		defaults.set(Data(savedByOldIos.utf8), forKey: legacySpeechRulesKey)
		migrateLegacySpeechRules(from: defaults, into: config)
		let reopened = ConfigManagerFfi()
		_ = reopened.initialize(configPath: configPath)
		XCTAssertEqual(reopened.getSpeechRules().count, 3)
	}

	/// iOS took `$1` as a group too, so a rule written that way must still say the number rather than "$1".
	func testARegexReplacementSaysWhatItSaidBefore() {
		XCTAssertEqual(TtsRule.nvdaReplacement("$1 to $2"), #"\1 to \2"#)
		XCTAssertEqual(TtsRule.nvdaReplacement(#"\1 to \2"#), #"\1 to \2"#)
		XCTAssertEqual(TtsRule.nvdaReplacement(#"costs \$5"#), "costs $5")
		XCTAssertEqual(TtsRule.nvdaReplacement("$ and \\"), "$ and \\")
		let migrated = rule(#"(\d+)-(\d+)"#, TtsRule.nvdaReplacement("pages $1 to $2"), scope: .paragraph, matchType: .regex)
		config.setSpeechRules(rules: [migrated])
		XCTAssertEqual(config.applySpeechRules(text: "10-12", voiceId: nil, voiceLanguage: nil), "pages 10 to 12")
	}

	func testSpeakingAppliesTheDictionaryInCore() {
		config.setSpeechRules(rules: [rule("Dr.", "Doctor"), rule("Who", "Whom", voiceFilter: .voices(["com.apple.voice.a"]))])
		let tts = TtsManager()
		XCTAssertEqual(tts.preprocessText("Dr. Who"), "Dr. Who")
		tts.config = config
		XCTAssertEqual(tts.preprocessText("Dr. Who"), "Doctor Who")
	}
}
