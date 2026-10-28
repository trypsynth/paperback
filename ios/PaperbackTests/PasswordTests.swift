import XCTest
@testable import Paperback

@MainActor
final class PasswordTests: XCTestCase {
	/// The core's own encrypted fixture, whose password is "password"; the simulator reads it straight from the checkout.
	private let protected = URL(fileURLWithPath: #filePath).deletingLastPathComponent().appending(path: "../../crates/paperback-core/src/parser/odf_crypto/lo-legacy-aes-cbc.odt").standardized

	func testAProtectedDocumentAsksForItsPasswordAndKeepsTheRightOne() throws {
		try XCTSkipUnless(FileManager.default.fileExists(atPath: protected.path(percentEncoded: false)), "needs the checkout's fixture")
		let viewModel = AppViewModel()
		let path = protected.path(percentEncoded: false)
		viewModel.configManager.setDocumentPassword(path: path, password: "")
		XCTAssertFalse(viewModel.openDocument(url: protected, track: false))
		XCTAssertEqual(viewModel.navigation.passwordPromptUrl, protected)
		XCTAssertNil(viewModel.debugMessage)
		XCTAssertFalse(viewModel.openDocument(url: protected, password: "wrong", track: false))
		XCTAssertNil(viewModel.debugMessage)
		XCTAssertTrue(viewModel.openDocument(url: protected, password: "password", track: false))
		XCTAssertEqual(viewModel.configManager.getDocumentPassword(path: path), "password")
		if let tab = viewModel.tabs.first(where: { $0.url == protected }) { viewModel.closeTab(tab) }
		viewModel.configManager.setDocumentPassword(path: path, password: "")
	}
}
