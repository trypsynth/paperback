import AVFoundation
import XCTest
@testable import Paperback

final class SpeechScaleTests: XCTestCase {
	func testTheSliderEndsMapToTheEndsOfTheEnginesRange() {
		XCTAssertEqual(AVSpeechUtteranceMinimumSpeechRate, speechRateForPercent(0), accuracy: 0.0001)
		XCTAssertEqual(AVSpeechUtteranceMaximumSpeechRate, speechRateForPercent(100), accuracy: 0.0001)
	}

	/// A rate from an older build, or nudged past an end by the reading bar's step, must not reach the engine as-is.
	func testAValueOutsideTheSlidersRangeIsBroughtBackInsideIt() {
		XCTAssertEqual(speechRateForPercent(100), speechRateForPercent(500), accuracy: 0.0001)
		XCTAssertEqual(speechRateForPercent(0), speechRateForPercent(-40), accuracy: 0.0001)
	}

	/// The reading bar and the settings slider both show this number, so a lossy round trip makes them disagree.
	func testEverySliderPositionSurvivesTheRoundTrip() {
		for percent in 0...100 {
			XCTAssertEqual(percent, percentForSpeechRate(speechRateForPercent(percent)), "at \(percent)%")
		}
	}

	func testSteppingPastAnEndSettlesThereRatherThanWrapping() {
		XCTAssertEqual(0, percentForSpeechRate(speechRateForPercent(-1)))
		XCTAssertEqual(100, percentForSpeechRate(speechRateForPercent(101)))
	}
}

final class PitchScaleTests: XCTestCase {
	func testThePercentEndsMapToTheEndsOfThePitchRange() {
		XCTAssertEqual(pitchRange.lowerBound, pitchForPercent(0), accuracy: 0.0001)
		XCTAssertEqual(pitchRange.upperBound, pitchForPercent(100), accuracy: 0.0001)
	}

	func testAValueOutsideThePercentRangeIsBroughtBackInsideIt() {
		XCTAssertEqual(pitchForPercent(100), pitchForPercent(500), accuracy: 0.0001)
		XCTAssertEqual(pitchForPercent(0), pitchForPercent(-40), accuracy: 0.0001)
	}

	func testEveryPercentSurvivesTheRoundTrip() {
		for percent in 0...100 {
			XCTAssertEqual(percent, percentForPitch(pitchForPercent(percent)), "at \(percent)%")
		}
	}

	func testSteppingPastAnEndSettlesThereRatherThanWrapping() {
		XCTAssertEqual(0, percentForPitch(pitchForPercent(-1)))
		XCTAssertEqual(100, percentForPitch(pitchForPercent(101)))
	}

	/// The default pitch of 1.0 is off the whole-percent grid, so a step could round back to where it started.
	func testOneStepUpFromTheDefaultPitchMovesToTheNextPercent() {
		let start = percentForPitch(1.0)
		XCTAssertEqual(33, start)
		XCTAssertEqual(34, percentForPitch(pitchForPercent(start + speechPercentStep)))
	}

	/// The menu marks the preset equal to the current percent, so a lossy preset would never show as chosen.
	func testEveryPitchPresetSurvivesTheRoundTrip() {
		for preset in pitchPresets {
			XCTAssertEqual(preset, percentForPitch(pitchForPercent(preset)), "preset \(preset)%")
		}
	}
}

final class PluralFormTests: XCTestCase {
	private let polish = "Plural-Forms: nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<12 || n%100>14) ? 1 : 2);"
	private let serbian = "Plural-Forms: nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<12 || n%100>14) ? 1 : 2);"
	private let czech = "Plural-Forms: nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;"

	private func form(_ count: Int, _ header: String?) -> String {
		nt("one", "few", "many", count, rule: PluralRule(header: header))
	}

	func testEnglishGivesTheSingularToOneAlone() {
		XCTAssertEqual("one", form(1, nil))
		XCTAssertEqual("many", form(0, nil))
		XCTAssertEqual("many", form(21, nil))
		XCTAssertEqual("many", form(101, nil))
	}

	func testPolishGivesTheSingularToOneAlone() {
		XCTAssertEqual("one", form(1, polish))
		XCTAssertEqual("few", form(22, polish))
		XCTAssertEqual("many", form(11, polish))
		XCTAssertEqual("many", form(21, polish))
		XCTAssertEqual("many", form(121, polish))
	}

	func testSerbianGivesTwentyOneTheSingular() {
		XCTAssertEqual("one", form(21, serbian))
		XCTAssertEqual("few", form(23, serbian))
		XCTAssertEqual("many", form(11, serbian))
		XCTAssertEqual("many", form(14, serbian))
	}

	func testCzechReadsItsParenthesizedRule() {
		XCTAssertEqual("one", form(1, czech))
		XCTAssertEqual("few", form(3, czech))
		XCTAssertEqual("many", form(21, czech))
		XCTAssertEqual("many", form(22, czech))
	}

	func testATwoFormLanguageUsesOnlyTheFirstTwoForms() {
		let german = "Plural-Forms: nplurals=2; plural=(n != 1);"
		XCTAssertEqual("one", form(1, german))
		XCTAssertEqual("few", form(0, german))
		XCTAssertEqual("few", form(21, german))
	}

	func testFrenchGivesZeroTheSingular() {
		let french = "Plural-Forms: nplurals=2; plural=(n > 1);"
		XCTAssertEqual("one", form(0, french))
		XCTAssertEqual("few", form(2, french))
	}

	func testAOneFormLanguageAlwaysUsesTheFirstForm() {
		let chinese = "Plural-Forms: nplurals=1; plural=0;"
		XCTAssertEqual("one", form(1, chinese))
		XCTAssertEqual("one", form(21, chinese))
	}

	func testTheRuleIsFoundAmongTheOtherHeaderLines() {
		XCTAssertEqual("many", form(21, "Language: pl\nMIME-Version: 1.0\n\(polish)\nX-Generator: Poedit\n"))
	}

	/// An untrustworthy header must not pick form 0 for every count.
	func testAMalformedHeaderFallsBack() {
		XCTAssertNil(PluralRule(header: "Language: pl\n"))
		XCTAssertNil(PluralRule(header: "Plural-Forms: nplurals=0; plural=0;"))
		XCTAssertNil(PluralRule(header: "Plural-Forms: nplurals=2; plural=5;")?.formIndex(1))
		XCTAssertEqual("many", form(21, "Plural-Forms: nplurals=3; plural=(n==1 ? 0 : ;"))
	}

	/// The strings file is generated, so this is what catches a header that stops reaching the app.
	func testEveryShippedCatalogueHasARuleThatWorks() {
		var checked = 0
		for language in Bundle.main.localizations {
			guard let path = Bundle.main.path(forResource: "Localizable", ofType: "strings", inDirectory: nil, forLocalization: language), let strings = NSDictionary(contentsOfFile: path) else {
				continue
			}
			XCTAssertNotNil(PluralRule(header: strings[""] as? String)?.formIndex(21), language)
			checked += 1
		}
		XCTAssertGreaterThan(checked, 0)
	}
}

final class TranslationMarkerTests: XCTestCase {
	func testTheInvisiblePluralMarkerIsStrippedFromUntranslatedText() {
		let marker = "\u{2063}"
		let key = "a string no catalogue will ever hold\(marker)"
		XCTAssertEqual("a string no catalogue will ever hold", t(key))
	}

	func testAStringWithoutTheMarkerIsLeftAlone() {
		let key = "another string no catalogue will ever hold"
		XCTAssertEqual(key, t(key))
	}
}
