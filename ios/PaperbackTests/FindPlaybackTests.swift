import XCTest
@testable import Paperback

private final class FindPlaybackSession: DocumentSession, @unchecked Sendable {
	private let paragraphs = ["First match context", "Following paragraph", "Second match context", "Final paragraph"]
	private let matches: [Int64] = [5, 25]

	override func hasAudioFfi() -> Bool { false }
	override func readingStep(position: Int64, unit: StepUnitFfi, direction: SegmentDirectionFfi) -> ReadingStepFfi {
		let landed: Int64?
		switch unit {
		case .segment:
			let index = Int(position / 10) + (direction == .next ? 1 : direction == .previous ? -1 : 0)
			landed = paragraphs.indices.contains(index) ? Int64(index * 10) : nil
		case .find:
			switch direction {
			case .previous: landed = matches.last { $0 < position }
			case .next: landed = matches.first { $0 > position }
			case .current: landed = matches.first { $0 >= position }
			}
		}
		guard let landed else { return ReadingStepFfi(found: false, position: position, text: "") }
		return ReadingStepFfi(found: true, position: landed, text: paragraphs[Int(landed / 10)])
	}
	override func readingAhead(position: Int64) -> ReadingAheadFfi {
		let index = Int(position / 10)
		func text(_ i: Int) -> String { paragraphs.indices.contains(i) ? paragraphs[i] : "" }
		return ReadingAheadFfi(next: text(index + 1), afterNext: text(index + 2), previous: text(index - 1))
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

	// Stands in for a completed audio buffer so the assertions don't wait on real speech.
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
		context.activeSession = FindPlaybackSession(noHandle: .init())
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
