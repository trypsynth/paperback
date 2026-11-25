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

final class VoiceBaseNameTests: XCTestCase {
	/// Voice names arrive as "Samantha (Enhanced)"; grouping by bare name keeps one voice's qualities together.
	func testATrailingQualitySuffixIsStripped() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha (Enhanced)"))
		XCTAssertEqual("Daniel", baseName(of: "Daniel (Premium)"))
	}

	func testANameWithNoSuffixIsLeftAlone() {
		XCTAssertEqual("Samantha", baseName(of: "Samantha"))
	}

	/// Only the last parenthetical is a quality marker; a name like "Alice (US)" keeps its own.
	func testOnlyTheLastParentheticalIsStripped() {
		XCTAssertEqual("Alice (US)", baseName(of: "Alice (US) (Enhanced)"))
	}

	func testATrailingBracketThatIsNotASuffixIsKept() {
		XCTAssertEqual("Weird)", baseName(of: "Weird)"))
	}

	private func baseName(of name: String) -> String { voiceBaseName(ofName: name) }
}
