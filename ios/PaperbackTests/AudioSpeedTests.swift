import AVFoundation
import XCTest
@testable import Paperback

@MainActor
final class AudioSpeedTests: XCTestCase {
	/// A second of silence, written fresh so the test needs no fixture.
	private func silence(seconds: Double = 1) throws -> URL {
		let url = FileManager.default.temporaryDirectory.appending(path: "\(UUID().uuidString).wav")
		let format = AVAudioFormat(standardFormatWithSampleRate: 8000, channels: 1)!
		let file = try AVAudioFile(forWriting: url, settings: format.settings)
		let frames = AVAudioFrameCount(seconds * 8000)
		let buffer = AVAudioPCMBuffer(pcmFormat: format, frameCapacity: frames)!
		buffer.frameLength = frames
		try file.write(from: buffer)
		return url
	}

	private func config() -> ConfigManagerFfi {
		let config = ConfigManagerFfi()
		_ = config.initialize(configPath: FileManager.default.temporaryDirectory.appending(path: "\(UUID().uuidString).toml").path(percentEncoded: false))
		return config
	}

	func testAPausedRecordingSeeksAndReportsItsTimeAndLength() throws {
		let player = try SourcePlayer(url: silence())
		XCTAssertEqual(player.duration, 1, accuracy: 0.01)
		player.currentTime = 0.4
		XCTAssertEqual(player.currentTime, 0.4, accuracy: 0.01)
		player.currentTime = 5
		XCTAssertEqual(player.currentTime, 1, accuracy: 0.01)
	}

	func testTheSpeedReachesTheRecordingPastWhatAVAudioPlayerCouldDo() throws {
		let player = try SourcePlayer(url: silence())
		player.rate = 3.5
		XCTAssertEqual(player.rate, 3.5)
	}

	func testARecordingPlaysToItsEndAndSaysSo() throws {
		let player = try SourcePlayer(url: silence(seconds: 0.3))
		player.rate = 4
		let finished = expectation(description: "finished")
		player.onFinished = { finished.fulfill() }
		XCTAssertTrue(player.play())
		wait(for: [finished], timeout: 5)
		XCTAssertEqual(player.currentTime, player.duration, accuracy: 0.01)
	}

	func testTheSpeedStaysInRangeAndIsRemembered() {
		let settings = config()
		let first = RecordedNarration(config: settings)
		first.setSpeed(9)
		XCTAssertEqual(first.speed, 4)
		first.setSpeed(0.1)
		XCTAssertEqual(first.speed, 0.5)
		first.setSpeed(2.5)
		XCTAssertEqual(RecordedNarration(config: settings).speed, 2.5)
	}

	func testSpeedsReadAsDesktopSaysThem() {
		XCTAssertEqual(audioSpeedLabel(1), "1x")
		XCTAssertEqual(audioSpeedLabel(2.5), "2.5x")
		XCTAssertEqual(audioSpeedLabel(1.25), "1.25x")
	}
}
