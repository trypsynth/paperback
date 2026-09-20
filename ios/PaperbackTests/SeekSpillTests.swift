import XCTest
@testable import Paperback

final class SeekSpillTests: XCTestCase {
	func testASeekLandingInsideTheFileStaysInIt() {
		XCTAssertEqual(.withinFile, spillOf(rawMs: 30_000, lengthMs: 45_000, deltaMs: 10_000))
	}

	func testLandingExactlyOnTheEndStillCountsAsInside() {
		XCTAssertEqual(.withinFile, spillOf(rawMs: 35_000, lengthMs: 45_000, deltaMs: 10_000))
	}

	func testThePartOfAForwardSeekPastTheEndBelongsToTheNextFile() {
		XCTAssertEqual(.pastEnd(overflowMs: 5_000), spillOf(rawMs: 40_000, lengthMs: 45_000, deltaMs: 10_000))
	}

	func testThePartOfABackwardSeekBeforeTheStartBelongsToThePreviousFile() {
		XCTAssertEqual(.beforeStart(underflowMs: 7_000), spillOf(rawMs: 3_000, lengthMs: 45_000, deltaMs: -10_000))
	}

	func testLandingExactlyOnTheStartStaysInThisFile() {
		XCTAssertEqual(.withinFile, spillOf(rawMs: 10_000, lengthMs: 45_000, deltaMs: -10_000))
	}

	/// The decoder reports no length until it has loaded. Spilling on that would send the reader
	/// to another file over a length of zero, which every forward seek runs past.
	func testAFileOfUnknownLengthCanOnlyBeSeekedWithin() {
		XCTAssertEqual(.withinFile, spillOf(rawMs: 0, lengthMs: 0, deltaMs: 10_000))
		XCTAssertEqual(.withinFile, spillOf(rawMs: 0, lengthMs: -1, deltaMs: 10_000))
	}

	/// A seek amount can be set as high as an hour, over narration files of a few minutes.
	func testASeekFarPastTheEndReportsTheWholeOverflow() {
		XCTAssertEqual(.pastEnd(overflowMs: 3_555_000), spillOf(rawMs: 0, lengthMs: 45_000, deltaMs: 3_600_000))
	}
}
