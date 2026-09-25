import AVFoundation
import XCTest
@testable import Paperback

final class SpeechScaleTests: XCTestCase {
	func testTheSliderEndsMapToTheEndsOfTheEnginesRange() {
		XCTAssertEqual(AVSpeechUtteranceMinimumSpeechRate, speechRateForPercent(0), accuracy: 0.0001)
		XCTAssertEqual(AVSpeechUtteranceMaximumSpeechRate, speechRateForPercent(100), accuracy: 0.0001)
	}

	/// A rate stored from an older build, or nudged past an end by the reading bar's step, must
	/// come back inside the range rather than reach the engine as-is.
	func testAValueOutsideTheSlidersRangeIsBroughtBackInsideIt() {
		XCTAssertEqual(speechRateForPercent(100), speechRateForPercent(500), accuracy: 0.0001)
		XCTAssertEqual(speechRateForPercent(0), speechRateForPercent(-40), accuracy: 0.0001)
	}

	/// The reading bar and the settings slider both show this number, so a rate that does not
	/// survive the round trip would make one of them disagree with the other.
	func testEverySliderPositionSurvivesTheRoundTrip() {
		for percent in 0...100 {
			XCTAssertEqual(percent, percentForSpeechRate(speechRateForPercent(percent)), "at \(percent)%")
		}
	}

	/// The reading bar steps by 1 and clamps at the ends, so holding the swipe at an end must
	/// settle rather than wrap around to the other one.
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

	/// The engine's default pitch of 1.0 is not on the whole-percent grid. One step up from it
	/// must land on the next percent rather than round back to where it started.
	func testOneStepUpFromTheDefaultPitchMovesToTheNextPercent() {
		let start = percentForPitch(1.0)
		XCTAssertEqual(33, start)
		XCTAssertEqual(34, percentForPitch(pitchForPercent(start + speechPercentStep)))
	}

	/// The menu marks the preset equal to the current percent, so a preset that did not survive
	/// the round trip could be chosen and then never show as chosen.
	func testEveryPitchPresetSurvivesTheRoundTrip() {
		for preset in pitchPresets {
			XCTAssertEqual(preset, percentForPitch(pitchForPercent(preset)), "preset \(preset)%")
		}
	}
}

final class PluralFormTests: XCTestCase {
	private func form(_ count: Int) -> String {
		nt("one", "few", "many", count)
	}

	func testACountEndingInOneTakesTheSingularForm() {
		XCTAssertEqual("one", form(1))
		XCTAssertEqual("one", form(21))
		XCTAssertEqual("one", form(101))
	}

	/// Eleven ends in a 1 but is not singular in these languages, which is the whole reason the
	/// rule looks at the last two digits as well as the last one.
	func testElevenIsNotSingularDespiteEndingInOne() {
		XCTAssertEqual("many", form(11))
		XCTAssertEqual("many", form(111))
	}

	func testCountsEndingTwoToFourTakeTheFewForm() {
		XCTAssertEqual("few", form(2))
		XCTAssertEqual("few", form(4))
		XCTAssertEqual("few", form(23))
	}

	/// Twelve through fourteen end in 2 to 4 but are not "few", the same trap as eleven.
	func testTwelveThroughFourteenAreNotFew() {
		XCTAssertEqual("many", form(12))
		XCTAssertEqual("many", form(13))
		XCTAssertEqual("many", form(14))
	}

	func testEverythingElseTakesTheManyForm() {
		XCTAssertEqual("many", form(0))
		XCTAssertEqual("many", form(5))
		XCTAssertEqual("many", form(20))
	}
}

final class TranslationMarkerTests: XCTestCase {
	/// U+2063 marks an English string whose plural "many" form would otherwise be byte-identical
	/// to its "few" form. It must never reach the interface, so an untranslated string has it
	/// stripped rather than shown.
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
