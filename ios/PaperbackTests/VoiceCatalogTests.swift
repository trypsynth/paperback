import AVFoundation
import XCTest
@testable import Paperback

final class VoiceCatalogTests: XCTestCase {
	/// Enumerating the installed voices is slow, so a catalog must not do it just because it
	/// was created; the app starts long before anyone looks at the voice list.
	func testCreatingTheCatalogDoesNotLoadTheVoices() {
		var loads = 0
		_ = VoiceCatalog { loads += 1; return [] }
		XCTAssertEqual(0, loads)
	}

	/// The Settings screen reads the list on every redraw, and a redraw happens on every swipe of
	/// the rate control. One load must serve all of them.
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
