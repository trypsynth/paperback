import AVFoundation
import XCTest
@testable import Paperback

final class VoiceCatalogTests: XCTestCase {
	/// Enumerating voices is slow, and the app starts long before anyone looks at the list.
	func testCreatingTheCatalogDoesNotLoadTheVoices() {
		var loads = 0
		_ = VoiceCatalog { loads += 1; return [] }
		XCTAssertEqual(0, loads)
	}

	/// Settings reads the list on every redraw, which every swipe of the rate control causes.
	func testRepeatedReadsLoadTheVoicesOnce() {
		var loads = 0
		let catalog = VoiceCatalog { loads += 1; return [] }
		_ = catalog.voices
		_ = catalog.voices
		_ = catalog.voices
		XCTAssertEqual(1, loads)
	}

	func testInvalidatingReloadsOnTheNextRead() {
		var loads = 0
		let catalog = VoiceCatalog { loads += 1; return [] }
		_ = catalog.voices
		catalog.invalidate()
		XCTAssertEqual(1, loads)
		_ = catalog.voices
		XCTAssertEqual(2, loads)
	}
}
