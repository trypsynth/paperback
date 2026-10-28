import XCTest
@testable import Paperback

@MainActor
final class SleepTimerTests: XCTestCase {
	func testAnEndedTimerNoLongerCountsAsRunning() async throws {
		let reading = ReadingController()
		reading.setSleepTimer(seconds: 1)
		XCTAssertEqual(reading.sleepTimerRemaining, 1)
		try await Task.sleep(for: .seconds(3))
		XCTAssertNil(reading.sleepTimerRemaining)
	}
}
