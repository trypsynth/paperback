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

/// The silence between paragraphs, which has to stay in proportion to how fast the speech is.
final class ParagraphPauseTests: XCTestCase {
	func testTheDefaultRateGetsTheBasePause() {
		XCTAssertEqual(paragraphPauseSeconds, paragraphPause(atRate: AVSpeechUtteranceDefaultSpeechRate), accuracy: 0.001)
	}

	/// The whole point of scaling: the same silence that reads as a paragraph break at the
	/// default rate is dead air at twice that, and barely a breath at half.
	func testReadingFasterShortensThePauseAndReadingSlowerLengthensIt() {
		let base = paragraphPause(atRate: AVSpeechUtteranceDefaultSpeechRate)
		let faster = paragraphPause(atRate: AVSpeechUtteranceDefaultSpeechRate * 1.5)
		let slower = paragraphPause(atRate: AVSpeechUtteranceDefaultSpeechRate * 0.75)
		XCTAssertLessThan(faster, base)
		XCTAssertGreaterThan(slower, base)
	}

	/// Both ends of the slider have to stay usable, so the scaling is clamped rather than
	/// running off to no gap at all or to a silence you would wonder about.
	func testThePauseStaysInsideItsRangeAtBothEndsOfTheSlider() {
		for percent in 0...100 {
			let pause = paragraphPause(atRate: speechRateForPercent(percent))
			XCTAssertGreaterThanOrEqual(pause, paragraphPauseRange.lowerBound, "at \(percent)%")
			XCTAssertLessThanOrEqual(pause, paragraphPauseRange.upperBound, "at \(percent)%")
		}
	}

	/// The minimum rate is zero, which would divide by it.
	func testTheSlowestRateDoesNotDivideByZero() {
		let pause = paragraphPause(atRate: AVSpeechUtteranceMinimumSpeechRate)
		XCTAssertEqual(paragraphPauseRange.upperBound, pause, accuracy: 0.001)
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
