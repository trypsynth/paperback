import XCTest
@testable import Paperback

final class ChapterProgressTextTests: XCTestCase {
	private func byWords(_ title: String, _ percent: Int32, _ wordsLeft: Int64) -> ChapterProgressFfi {
		ChapterProgressFfi(found: true, title: title, percent: percent, wordsLeft: wordsLeft, elapsedMs: -1, totalMs: -1)
	}

	func testATextChapterGivesItsPercentageAndTheWordsLeft() {
		XCTAssertEqual("Chapter 3, 42%, 850 words left", chapterProgressText(byWords("Chapter 3", 42, 850)))
	}

	func testOneWordLeftIsSingular() {
		XCTAssertEqual("Chapter 3, 99%, 1 word left", chapterProgressText(byWords("Chapter 3", 99, 1)))
	}

	func testAChapterWithoutATitleStartsAtItsPercentage() {
		XCTAssertEqual("39%, 6 words left", chapterProgressText(byWords(" ", 39, 6)))
	}

	func testARecordedChapterGivesItsTimes() {
		let progress = ChapterProgressFfi(found: true, title: "Chapter 3", percent: 34, wordsLeft: -1, elapsedMs: 250_000, totalMs: 720_000)
		XCTAssertEqual("Chapter 3, 4:10 of 12:00, 7:50 left", chapterProgressText(progress))
	}
}
