import XCTest
@testable import Paperback

private final class FindPlaybackSession: DocumentSession, @unchecked Sendable {
	private let paragraphs = ["First match context", "Following paragraph", "Second match context", "Final paragraph"]
	private let matches: [Int64] = [5, 25]
	var searchedFrom: Int64?

	override func hasAudioFfi() -> Bool { false }
	override func getTextSegment(position: Int64, segmentType: SegmentTypeFfi, direction: SegmentDirectionFfi) -> TextSegmentFfi {
		let index = Int(position / 10) + (direction == .next ? 1 : direction == .previous ? -1 : 0)
		let found = paragraphs.indices.contains(index)
		return TextSegmentFfi(text: found ? paragraphs[index] : "", startPos: Int64(index * 10), endPos: Int64((index + 1) * 10), found: found)
	}
	override func searchFfi(query: String, startPosition: Int64, options: SearchOptionsFfi) -> SearchResultFfi {
		searchedFrom = startPosition
		let position = options.forward
			? matches.first { $0 >= startPosition }
			: matches.last { $0 < startPosition }
		return SearchResultFfi(found: position != nil, wrapped: false, position: position ?? 0)
	}
}

@MainActor
private final class FindPlaybackContext: ReadingContext {
	var activeSession: DocumentSession?
	var activeTitle: String? { "Test document" }
	var activeLineScrollIndex = 0
	var position: Int64 = 0
	func persistPosition(_ position: Int64) { self.position = position }
}

@MainActor
final class FindPlaybackTests: XCTestCase {
	private func reader(context: FindPlaybackContext) -> ReadingController {
		let reading = ReadingController()
		reading.context = context
		reading.start()
		reading.activeSearchQuery = "match"
		reading.currentNavUnit = .find
		reading.findNext()
		return reading
	}

	// Deliver the same state and callback as a completed audio buffer without waiting
	// for real speech, keeping navigation and continuation assertions deterministic.
	private func finishUtterance(_ reading: ReadingController) {
		reading.ttsManager.isSpeaking = false
		reading.ttsManager.isPaused = false
		reading.ttsManager.onUtteranceFinished?()
	}

	func testFindStopsByDefaultAndPlayContinuesWithoutRepeatingTheMatch() {
		let context = FindPlaybackContext()
		context.activeSession = FindPlaybackSession(noHandle: .init())
		let reading = reader(context: context)
		defer { reading.ttsManager.stop() }
		XCTAssertTrue(reading.stopAfterFindMatch)
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 5)
		XCTAssertFalse(reading.isPlayingNow)
		reading.togglePlayPause()
		XCTAssertEqual(reading.ttsPosition, 10)
		XCTAssertEqual(reading.currentSegmentText, "Following paragraph")
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 10)
		XCTAssertFalse(reading.isPlayingNow)
	}

	func testFindCanReadContinuouslyAndStillNavigateBetweenMatches() {
		let context = FindPlaybackContext()
		let session = FindPlaybackSession(noHandle: .init())
		context.activeSession = session
		let reading = reader(context: context)
		defer { reading.ttsManager.stop() }
		reading.stopAfterFindMatch = false
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 10)
		XCTAssertEqual(context.position, 10)
		XCTAssertEqual(reading.currentSegmentText, "Following paragraph")
		XCTAssertTrue(reading.isPlayingNow)
		XCTAssertEqual(reading.currentNavUnit, .find)

		// The next button still searches, rather than becoming paragraph navigation.
		XCTAssertTrue(reading.playNextSegment())
		XCTAssertEqual(session.searchedFrom, 11)
		XCTAssertEqual(reading.ttsPosition, 25)
		XCTAssertEqual(reading.currentSegmentText, "Second match context")
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 30)
		XCTAssertEqual(reading.currentSegmentText, "Final paragraph")
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 30)
		XCTAssertFalse(reading.isPlayingNow)
		XCTAssertEqual(reading.currentNavUnit, .find)

		XCTAssertTrue(reading.playPrevSegment(speak: false))
		XCTAssertEqual(reading.ttsPosition, 25)
		XCTAssertTrue(reading.playPrevSegment(speak: false))
		XCTAssertEqual(reading.ttsPosition, 5)
	}

	func testEnablingStopTakesEffectAtTheNextParagraphBoundary() {
		let context = FindPlaybackContext()
		context.activeSession = FindPlaybackSession(noHandle: .init())
		let reading = reader(context: context)
		defer { reading.ttsManager.stop() }
		reading.stopAfterFindMatch = false
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 10)
		reading.stopAfterFindMatch = true
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 10)
		XCTAssertFalse(reading.isPlayingNow)
	}

	func testStopSettingDoesNotInterruptParagraphNavigationPlayback() {
		let context = FindPlaybackContext()
		context.activeSession = FindPlaybackSession(noHandle: .init())
		let reading = reader(context: context)
		defer { reading.ttsManager.stop() }
		reading.changeNavUnit(.segment(.paragraph))
		finishUtterance(reading)
		XCTAssertEqual(reading.ttsPosition, 10)
		XCTAssertTrue(reading.isPlayingNow)
	}
}
