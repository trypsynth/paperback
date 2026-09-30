import AVFoundation
import Observation
import XCTest
@testable import Paperback

/// Model both M4B chapters sharing one source and audio bundles with a source per section.
private final class SectionAudioSession: DocumentSession, @unchecked Sendable {
	var sharedSource = true
	var audioPath = ""

	override func hasAudioFfi() -> Bool { true }
	override func isAudioOnlyFfi() -> Bool { true }
	override func getSupportedSegmentTypesFfi() -> [SegmentTypeFfi] { [.section] }
	override func getToc() -> [TocEntry] {
		(0..<3).map { TocEntry(title: "Chapter \($0 + 1)", position: Int64($0), level: 1) }
	}
	override func getTextSegment(position: Int64, segmentType: SegmentTypeFfi, direction: SegmentDirectionFfi) -> TextSegmentFfi {
		let destination = position + (direction == .next ? 1 : direction == .previous ? -1 : 0)
		let found = (0..<3).contains(destination)
		return TextSegmentFfi(text: found ? "Chapter \(destination + 1)" : "", startPos: destination, endPos: destination + 1, found: found)
	}
	override func audioPointForPositionFfi(position: Int64) -> AudioPointFfi {
		AudioPointFfi(found: (0..<3).contains(position), position: position, timeMs: position * 10_000)
	}
	override func audioCursorAtElapsedFfi(elapsedMs: Int64) -> AudioCursorFfi {
		AudioCursorFfi(found: (0..<30_000).contains(elapsedMs), clipIndex: Int32(elapsedMs / 10_000), seekMs: sharedSource ? elapsedMs : elapsedMs % 10_000)
	}
	override func audioClipFfi(index: Int32) -> AudioClipFfi {
		AudioClipFfi(found: (0..<3).contains(index), source: sharedSource ? 0 : index, clipBeginMs: sharedSource ? Int64(index) * 10_000 : 0, clipEndMs: sharedSource ? Int64(index + 1) * 10_000 : 10_000, start: Int64(index), end: Int64(index + 1))
	}
	override func audioElapsedForSourcePositionFfi(source: Int32, rawMs: Int64) -> Int64 {
		(sharedSource ? 0 : Int64(source) * 10_000) + rawMs
	}
	override func audioSourceDirectPathFfi(index: Int32) -> String { audioPath }
	override func audioExtractSourceFfi(index: Int32, outputPath: String) -> Bool { false }
	override func audioTotalDurationMsFfi() -> Int64 { 30_000 }
	override func audioNextSourceAfterFfi(currentSource: Int32) -> Int32 { -1 }
}

private final class AudioPositionConfig: ConfigManagerFfi, @unchecked Sendable {
	var savedMs: Int64 = -1
	override func getDocumentAudioTimeFfi(path: String) -> Int64 { savedMs }
	override func setDocumentAudioTimeFfi(path: String, timeMs: Int64) { savedMs = timeMs }
	override func flush() {}
}

@MainActor
private final class AudioReadingContext: ReadingContext {
	var activeSession: DocumentSession?
	var activeTitle: String? { "Test book" }
	var activeLineScrollIndex = 0
	var position: Int64 = 0
	func persistPosition(_ position: Int64) { self.position = position }
}

@MainActor
final class RecordedNavigationTests: XCTestCase {
	func testPausedSectionNavigationSeeksAndPersistsForChaptersAndFiles() {
		for sharedSource in [true, false] {
			let session = SectionAudioSession(noHandle: .init())
			session.sharedSource = sharedSource
			let config = AudioPositionConfig(noHandle: .init())
			let narration = RecordedNarration(config: config)
			narration.attach(to: DocumentTab(title: "Book", url: URL(fileURLWithPath: "/book"), session: session))
			defer { narration.detach() }
			let context = AudioReadingContext()
			context.activeSession = session
			let reading = ReadingController()
			reading.context = context
			reading.narration = narration
			reading.currentNavUnit = .segment(.section)

			XCTAssertFalse(reading.playPrevSegment(speak: false))
			XCTAssertTrue(reading.playNextSegment(speak: false))
			XCTAssertEqual(config.savedMs, 10_000)
			XCTAssertEqual(context.position, 1)
			XCTAssertEqual(reading.currentSegmentText, "Chapter 2")
			XCTAssertFalse(reading.isPlayingNow)
			XCTAssertFalse(reading.ttsManager.isSpeaking)
			XCTAssertTrue(reading.playNextSegment(speak: false))
			XCTAssertEqual(config.savedMs, 20_000)
			XCTAssertFalse(reading.playNextSegment(speak: false))
			reading.navigateByType(.section, direction: .previous)
			XCTAssertEqual(config.savedMs, 10_000)
			XCTAssertEqual(reading.ttsPosition, 1)
			XCTAssertTrue(reading.playPrevSegment(speak: false))
			XCTAssertEqual(config.savedMs, 0)
		}
	}

	func testPlaybackStateIsObservableAndResetsAfterSourceFailure() async {
		let session = SectionAudioSession(noHandle: .init())
		let narration = RecordedNarration(config: AudioPositionConfig(noHandle: .init()))
		narration.attach(to: DocumentTab(title: "Book", url: URL(fileURLWithPath: "/book"), session: session))
		defer { narration.detach() }
		let invalidated = expectation(description: "Playback invalidates the view's observation")
		withObservationTracking {
			_ = narration.isPlaying
		} onChange: {
			invalidated.fulfill()
		}
		let failed = expectation(description: "Unresolvable audio returns to paused")
		narration.onPlayingChanged = { playing in
			if !playing { failed.fulfill() }
		}
		narration.play()
		XCTAssertTrue(narration.isPlaying)
		await fulfillment(of: [invalidated, failed], timeout: 3)
		XCTAssertFalse(narration.isPlaying)
		narration.onPlayingChanged = nil
	}

	func testPausedFileJumpOverridesLoadedDecoderAndRestoresDestination() async throws {
		let url = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString + ".wav")
		defer { try? FileManager.default.removeItem(at: url) }
		try writeSilentAudio(to: url)
		let session = SectionAudioSession(noHandle: .init())
		session.sharedSource = false
		session.audioPath = url.path
		let player = DaisyAudioPlayer()
		let config = AudioPositionConfig(noHandle: .init())
		let narration = RecordedNarration(config: config, player: player)
		let tab = DocumentTab(title: "Book", url: URL(fileURLWithPath: "/book"), session: session)
		narration.attach(to: tab)
		defer { narration.detach() }
		narration.play()
		// Wait for the real decoder to load and move beyond the pending start position.
		for _ in 0..<200 {
			if (player.resumePointMs() ?? 0) > 0 { break }
			try await Task.sleep(for: .milliseconds(10))
		}
		XCTAssertGreaterThan(player.resumePointMs() ?? 0, 0)
		narration.pause()
		XCTAssertTrue(narration.seekToPosition(1))
		XCTAssertEqual(player.resumePointMs(), 10_000)
		XCTAssertEqual(config.savedMs, 10_000)
		XCTAssertFalse(narration.isPlaying)
		// Returning to the still-loaded file must rewind it, even though its last seek
		// target was also zero before playback advanced.
		XCTAssertTrue(narration.seekToPosition(0))
		XCTAssertEqual(player.resumePointMs(), 0)
		XCTAssertTrue(narration.seekToPosition(1))
		narration.detach()
		narration.attach(to: tab)
		XCTAssertEqual(player.resumePointMs(), 10_000)
	}

	func testSectionNavigationWhilePlayingMovesAudioWithoutStartingSpeech() async throws {
		let url = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString + ".wav")
		defer { try? FileManager.default.removeItem(at: url) }
		try writeSilentAudio(to: url)
		for sharedSource in [true, false] {
			let session = SectionAudioSession(noHandle: .init())
			session.sharedSource = sharedSource
			session.audioPath = url.path
			let player = DaisyAudioPlayer()
			let narration = RecordedNarration(config: AudioPositionConfig(noHandle: .init()), player: player)
			narration.attach(to: DocumentTab(title: "Book", url: URL(fileURLWithPath: "/book"), session: session))
			defer { narration.detach() }
			let context = AudioReadingContext()
			context.activeSession = session
			let reading = ReadingController()
			reading.context = context
			reading.narration = narration
			reading.currentNavUnit = .segment(.section)
			reading.togglePlayPause()
			XCTAssertTrue(reading.isPlayingNow)
			XCTAssertTrue(reading.playNextSegment())
			for _ in 0..<200 {
				if (player.resumePointMs() ?? 0) > 10_000 { break }
				try await Task.sleep(for: .milliseconds(10))
			}
			XCTAssertGreaterThan(player.resumePointMs() ?? 0, 10_000)
			XCTAssertTrue(reading.isPlayingNow)
			XCTAssertFalse(reading.ttsManager.isSpeaking)
			XCTAssertEqual(reading.currentSegmentText, "Chapter 2")
			XCTAssertTrue(reading.playPrevSegment())
			XCTAssertLessThan(player.resumePointMs() ?? 30_000, 10_000)
			XCTAssertTrue(reading.isPlayingNow)
			reading.togglePlayPause()
			XCTAssertFalse(reading.isPlayingNow)
			narration.detach()
			XCTAssertFalse(reading.isPlayingNow)
		}
	}

	private func writeSilentAudio(to url: URL) throws {
		let format = try XCTUnwrap(AVAudioFormat(standardFormatWithSampleRate: 8_000, channels: 1))
		let buffer = try XCTUnwrap(AVAudioPCMBuffer(pcmFormat: format, frameCapacity: 240_000))
		buffer.frameLength = buffer.frameCapacity
		let samples = try XCTUnwrap(buffer.floatChannelData)[0]
		samples.update(repeating: 0, count: Int(buffer.frameLength))
		let file = try AVAudioFile(forWriting: url, settings: format.settings)
		try file.write(from: buffer)
	}
}
