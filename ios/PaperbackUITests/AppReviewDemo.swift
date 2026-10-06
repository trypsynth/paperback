import XCTest

// Walks the typical flow App Review asked to see recorded on a physical device. It expects the books from ios/Scripts/capture-screenshots.sh in the app's Documents folder, and pauses between steps so the recording can be followed.
final class AppReviewDemo: XCTestCase {
	private let app = XCUIApplication()

	override func setUp() {
		continueAfterFailure = false
	}

	func testTypicalFlow() {
		app.launch()
		pause(3)
		openBook("Pride and Prejudice")
		pause(3)
		tap("Play")
		pause(8)
		tap("Pause")
		pause(1)
		tap("Next Paragraph")
		pause(2)
		tap("Next Paragraph")
		pause(2)
		tap("Navigation unit")
		tap("Heading")
		pause(1)
		tap("Next Heading")
		pause(2)
		tap("Next Heading")
		pause(2)
		menu("Table of Contents")
		pause(2)
		tap("V")
		pause(3)
		menu("Switch to Text Mode")
		pause(3)
		app.swipeUp()
		pause(3)
		menu("Switch to TTS Mode")
		pause(2)
		openBook("Alice's Adventures in Wonderland")
		pause(3)
		tap("Pride and Prejudice")
		pause(3)
		menu("Recent Documents")
		pause(3)
		goBack()
		pause(1)
		menu("Settings")
		pause(3)
		app.swipeUp()
		pause(3)
		goBack()
		pause(2)
	}

	private func pause(_ seconds: Double) {
		Thread.sleep(forTimeInterval: seconds)
	}

	// Matches on label rather than type, because SwiftUI surfaces the same control as a button, a cell or a menu item depending on where it sits. Contents rows read "V, Level 1", hence the prefix.
	private func matches(_ label: String) -> XCUIElementQuery {
		app.descendants(matching: .any).matching(NSPredicate(format: "label == %@ OR label BEGINSWITH %@", label, "\(label), "))
	}

	// isHittable cannot go in a query predicate, and the same label can also sit on something hidden behind a sheet or menu.
	private func element(_ label: String) -> XCUIElement {
		let found = matches(label)
		for index in 0..<found.count where found.element(boundBy: index).isHittable {
			return found.element(boundBy: index)
		}
		return found.firstMatch
	}

	private func tap(_ label: String, file: StaticString = #filePath, line: UInt = #line) {
		XCTAssertTrue(matches(label).firstMatch.waitForExistence(timeout: 10), "no \"\(label)\" to tap", file: file, line: line)
		element(label).tap()
	}

	private func menu(_ item: String) {
		tap("More options")
		pause(1)
		tap(item)
	}

	private func goBack() {
		app.navigationBars.buttons.element(boundBy: 0).tap()
	}

	// The picker lists files by name without the extension. Recents is only populated after a first open, so the first book is found by browsing to the app's own folder.
	private func openBook(_ name: String) {
		tap("Open Book")
		pause(2)
		let file = app.descendants(matching: .any).matching(NSPredicate(format: "label BEGINSWITH %@", name)).firstMatch
		if !file.waitForExistence(timeout: 3) {
			for place in ["Browse", "On My iPad", "Paperback"] where element(place).waitForExistence(timeout: 3) {
				element(place).tap()
				pause(1)
			}
		}
		XCTAssertTrue(file.waitForExistence(timeout: 10), "no \"\(name)\" in the file picker")
		file.tap()
	}
}
