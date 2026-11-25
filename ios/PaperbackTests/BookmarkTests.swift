import XCTest
@testable import Paperback

/// Four paragraphs ten positions apart, each read as one segment.
private final class BookmarkSession: DocumentSession, @unchecked Sendable {
	private let paragraphs = ["One", "Two", "Three", "Four"]

	override func hasAudioFfi() -> Bool { false }
	override func isAudioOnlyFfi() -> Bool { false }
	override func getSupportedSegmentTypesFfi() -> [SegmentTypeFfi] { [.paragraph, .line] }
	override func readingStep(position: Int64, unit: StepUnitFfi, direction: SegmentDirectionFfi) -> ReadingStepFfi {
		let index = Int(position / 10) + (direction == .next ? 1 : direction == .previous ? -1 : 0)
		guard paragraphs.indices.contains(index) else { return ReadingStepFfi(found: false, position: position, text: "") }
		return ReadingStepFfi(found: true, position: Int64(index * 10), text: paragraphs[index])
	}
	override func readingAhead(position: Int64) -> ReadingAheadFfi {
		ReadingAheadFfi(next: "", afterNext: "", previous: "")
	}
	override func getTextSegment(position: Int64, segmentType: SegmentTypeFfi, direction: SegmentDirectionFfi) -> TextSegmentFfi {
		let index = Int(position / 10)
		guard paragraphs.indices.contains(index) else { return TextSegmentFfi(text: "", startPos: 0, endPos: 0, found: false) }
		return TextSegmentFfi(text: paragraphs[index], startPos: Int64(index * 10), endPos: Int64(index * 10 + 9), found: true)
	}
	override func getLineText(position: Int64) -> String {
		let index = Int(position / 10)
		return paragraphs.indices.contains(index) ? paragraphs[index] : ""
	}
}

@MainActor
private final class BookmarkContext: ReadingContext {
	var activeSession: DocumentSession? = BookmarkSession(noHandle: .init())
	var activeTitle: String? { "Test document" }
	let configManager = ConfigManagerFfi()
	var activeDocPath: String? { "/test/book.txt" }
	func persistPosition(_ position: Int64) {}
	func persistNavUnit(_ unit: NavigationUnit) {}
	func savedNavUnit(for tab: DocumentTab) -> NavigationUnit? { nil }

	init() {
		let dir = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
		try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
		_ = configManager.initialize(configPath: dir.appendingPathComponent("config.toml").path)
	}
}

@MainActor
final class BookmarkTests: XCTestCase {
	private var context: BookmarkContext!

	private func reader(at position: Int64 = 0) -> ReadingController {
		context = BookmarkContext()
		let reading = ReadingController()
		reading.context = context
		reading.goToPosition(position)
		return reading
	}

	/// Reading by line lands mid-paragraph, and the bookmark made there must still show when reading by paragraph.
	func testABookmarkAnywhereInTheParagraphIsTheOneHere() {
		let reading = reader(at: 13)
		XCTAssertNil(reading.bookmarkHere)
		XCTAssertTrue(reading.toggleBookmark())
		XCTAssertEqual(reading.bookmarkHere?.start, 13)
		reading.goToPosition(10)
		XCTAssertEqual(reading.bookmarkHere?.start, 13)
		reading.goToPosition(20)
		XCTAssertNil(reading.bookmarkHere)
	}

	func testTogglingTwiceRemovesTheBookmark() {
		let reading = reader(at: 10)
		reading.toggleBookmark()
		XCTAssertFalse(reading.toggleBookmark())
		XCTAssertNil(reading.bookmarkHere)
		XCTAssertFalse(reading.hasBookmarks)
	}

	/// The unit is offered only while there is something to step to, and leaving it falls back rather than stranding the bar on nothing.
	func testTheBookmarkUnitComesAndGoesWithTheBookmarks() {
		let reading = reader(at: 20)
		XCTAssertFalse(reading.availableNavUnits.contains(.bookmark))
		reading.toggleBookmark()
		XCTAssertTrue(reading.availableNavUnits.contains(.bookmark))
		reading.changeNavUnit(.bookmark)
		reading.toggleBookmark()
		XCTAssertNotEqual(reading.currentNavUnit, .bookmark)
	}

	func testNextAndPreviousStepBetweenBookmarks() {
		let reading = reader(at: 10)
		reading.toggleBookmark()
		reading.goToPosition(30)
		reading.toggleBookmark()
		reading.goToPosition(0)
		reading.changeNavUnit(.bookmark)
		reading.playNextSegment(speak: false)
		XCTAssertEqual(reading.ttsPosition, 10)
		reading.playNextSegment(speak: false)
		XCTAssertEqual(reading.ttsPosition, 30)
		reading.playPrevSegment(speak: false)
		XCTAssertEqual(reading.ttsPosition, 10)
	}

	func testANoteOnANewSpotMakesTheBookmarkAndClearingItKeepsTheBookmark() {
		let reading = reader(at: 20)
		reading.saveNote("  remember  ", for: reading.noteTargetHere())
		XCTAssertEqual(reading.bookmarkHere?.note, "remember")
		reading.saveNote("", for: reading.noteTargetHere())
		XCTAssertEqual(reading.bookmarkHere?.note, "")
	}

	/// Text mode answers VoiceOver from the snapshot, so it has to find what the core finds.
	func testTheSnapshotFindsWhatTheCoreFinds() {
		let bookmark = { (start: Int64) in BookmarkFfi(found: true, start: start, end: start, note: "", audioMs: nil) }
		let snapshot = BookmarkSnapshot(path: "p", revision: 0, bookmarks: [bookmark(30), bookmark(13), BookmarkFfi(found: true, start: 20, end: 20, note: "", audioMs: 5000)])
		XCTAssertEqual(snapshot.count, 3)
		XCTAssertEqual(snapshot.bookmark(from: 10, to: 20)?.start, 13)
		XCTAssertNil(snapshot.bookmark(from: 20, to: 30), "an audio bookmark is not a text one, and 30 is past the range")
		XCTAssertEqual(snapshot.bookmark(from: 30, to: 30)?.start, 30)
		XCTAssertNil(snapshot.bookmark(from: 31, to: 40))
	}

	/// A change has to reach the snapshot, or text mode would keep announcing a bookmark that's gone.
	func testTheSnapshotFollowsChanges() {
		let reading = reader(at: 10)
		XCTAssertFalse(reading.hasBookmarks)
		reading.toggleBookmark()
		XCTAssertTrue(reading.hasBookmarks)
		XCTAssertNotNil(reading.bookmarkHere)
		reading.toggleBookmark()
		XCTAssertFalse(reading.hasBookmarks)
	}

	/// The toolbar's button reads only this, so a move that doesn't change it must not redraw the toolbar, which makes VoiceOver re-read the screen.
	func testMovingWithoutChangingTheBookmarkDoesNotRedrawItsViews() {
		let reading = reader(at: 10)
		reading.toggleBookmark()
		let redrawn = Flag()
		withObservationTracking { _ = reading.bookmarkHere } onChange: { redrawn.set() }
		reading.goToPosition(15)
		XCTAssertFalse(redrawn.value, "same paragraph, same bookmark")
		reading.goToPosition(20)
		XCTAssertTrue(redrawn.value, "left the bookmarked paragraph")
	}

	func testNotesOnlySkipsPlainBookmarks() {
		let reading = reader(at: 10)
		reading.toggleBookmark()
		reading.goToPosition(30)
		reading.saveNote("later", for: reading.noteTargetHere())
		reading.goToPosition(0)
		XCTAssertTrue(reading.stepBookmark(forward: true, notesOnly: true))
		XCTAssertEqual(reading.ttsPosition, 30)
	}
}

/// Set from the observation callback, which Swift treats as running concurrently.
private final class Flag: @unchecked Sendable {
	private(set) var value = false
	func set() { value = true }
}
