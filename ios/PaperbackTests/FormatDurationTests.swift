import XCTest
@testable import Paperback

final class FormatDurationTests: XCTestCase {
	func testUnderAMinuteReadsAsMinutesAndSeconds() {
		XCTAssertEqual("0:00", formatDuration(0))
		XCTAssertEqual("0:01", formatDuration(1_000))
		XCTAssertEqual("0:59", formatDuration(59_000))
	}

	func testAnHourOrMoreGrowsAnHoursField() {
		XCTAssertEqual("59:59", formatDuration(3_599_000))
		XCTAssertEqual("1:00:00", formatDuration(3_600_000))
		XCTAssertEqual("1:01:01", formatDuration(3_661_000))
	}

	/// Playback positions arrive as raw milliseconds, so a spoken "1:00" that was really 59.6
	/// seconds has to round rather than truncate, or the announced time trails the audio.
	func testMillisecondsRoundToTheNearestSecond() {
		XCTAssertEqual("0:00", formatDuration(499))
		XCTAssertEqual("0:01", formatDuration(500))
		XCTAssertEqual("1:00", formatDuration(59_500))
	}

	/// A relative seek can compute a target before the start of the book before it is clamped.
	func testANegativePositionReadsAsZeroRatherThanGoingBackwards() {
		XCTAssertEqual("0:00", formatDuration(-1))
		XCTAssertEqual("0:00", formatDuration(-100_000))
	}
}

final class ExportFormatTests: XCTestCase {
	/// The picker offers the name this builds, so a format whose extension does not match what
	/// is actually written would hand the reader a file their system opens with the wrong app.
	func testEveryFormatHasTheExtensionItsContentIsWrittenIn() {
		XCTAssertEqual("txt", ExportFormat.text.fileExtension)
		XCTAssertEqual("html", ExportFormat.html.fileExtension)
		XCTAssertEqual("md", ExportFormat.markdown.fileExtension)
	}

	/// A shared extension would make two entries in the picker write files that look identical.
	func testEveryFormatKnownToTheCoreHasAnExtensionOfItsOwn() {
		let all: [ExportFormat] = [.text, .html, .markdown]
		XCTAssertEqual(all.count, Set(all.map(\.fileExtension)).count)
	}
}

final class NavUnitTests: XCTestCase {
	/// Find only earns a place in the list once there is a query to step through, so a unit that
	/// is a segment must never compare equal to one that is a seek amount.
	func testUnitsOfDifferentKindsAreDistinct() {
		XCTAssertNotEqual(NavUnit.segment(.paragraph), NavUnit.time(seconds: 5))
		XCTAssertNotEqual(NavUnit.find, NavUnit.segment(.paragraph))
		XCTAssertEqual(NavUnit.segment(.heading), NavUnit.segment(.heading))
		XCTAssertNotEqual(NavUnit.segment(.heading), NavUnit.segment(.line))
	}

	/// `ensureNavUnitSupported` looks the current unit up in the offered list, so seek amounts
	/// have to compare by their length rather than by identity.
	func testSeekAmountsCompareByTheirLength() {
		XCTAssertEqual(NavUnit.time(seconds: 30), NavUnit.time(seconds: 30))
		XCTAssertNotEqual(NavUnit.time(seconds: 30), NavUnit.time(seconds: 60))
	}

	/// Every amount the reading bar offers needs a name of its own, or two entries read alike.
	func testEverySeekAmountOfferedHasItsOwnName() {
		let names = audioSeekAmountsSeconds.map { seekAmountName($0) }
		XCTAssertEqual(audioSeekAmountsSeconds.count, Set(names).count)
		XCTAssertFalse(names.contains { $0.isEmpty })
	}
}
