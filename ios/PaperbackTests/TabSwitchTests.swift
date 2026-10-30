import XCTest
@testable import Paperback

@MainActor
final class TabSwitchTests: XCTestCase {
	private var dir: URL!

	override func setUpWithError() throws {
		dir = FileManager.default.temporaryDirectory.appending(path: UUID().uuidString)
		try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
	}

	override func tearDownWithError() throws {
		try? FileManager.default.removeItem(at: dir)
	}

	private func book(_ name: String, _ text: String) throws -> URL {
		let url = dir.appending(path: name)
		try text.write(to: url, atomically: true, encoding: .utf8)
		return url
	}

	func testClosingTheActiveTabReadsFromTheBookLeftOpen() throws {
		let first = try book("first.txt", "Alpha paragraph.")
		let second = try book("second.txt", "Beta paragraph.")
		let viewModel = AppViewModel()
		XCTAssertTrue(viewModel.openDocument(url: first, track: false))
		XCTAssertTrue(viewModel.openDocument(url: second, track: false))
		XCTAssertTrue(viewModel.reading.currentSegmentText.contains("Beta"))
		viewModel.closeTab(try XCTUnwrap(viewModel.activeTab))
		XCTAssertEqual(viewModel.activeTab?.url, first)
		XCTAssertTrue(viewModel.reading.currentSegmentText.contains("Alpha"), viewModel.reading.currentSegmentText)
		viewModel.closeTab(try XCTUnwrap(viewModel.activeTab))
	}

	func testOpeningABookAlreadyOpenReadsFromIt() throws {
		let first = try book("first.txt", "Alpha paragraph.")
		let second = try book("second.txt", "Beta paragraph.")
		let viewModel = AppViewModel()
		XCTAssertTrue(viewModel.openDocument(url: first, track: false))
		XCTAssertTrue(viewModel.openDocument(url: second, track: false))
		XCTAssertTrue(viewModel.openDocument(url: first, track: false))
		XCTAssertTrue(viewModel.reading.currentSegmentText.contains("Alpha"), viewModel.reading.currentSegmentText)
		for tab in viewModel.tabs where tab.url == first || tab.url == second { viewModel.closeTab(tab) }
	}

	func testRelaunchingKeepsRecentsInReadingOrder() throws {
		let paths = try [book("first.txt", "Alpha paragraph."), book("second.txt", "Beta paragraph.")].map { $0.path(percentEncoded: false) }
		let config = AppViewModel().configManager
		for path in paths { config.addOpenedDocument(path: path) }
		for path in paths.reversed() { config.addRecentDocument(path: path) }
		config.flush()
		let relaunched = AppViewModel()
		XCTAssertEqual(relaunched.tabs.filter { paths.contains($0.url.path(percentEncoded: false)) }.count, 2)
		XCTAssertEqual(Array(relaunched.configManager.getRecentDocuments().prefix(2)), paths)
		for tab in relaunched.tabs where paths.contains(tab.url.path(percentEncoded: false)) { relaunched.closeTab(tab) }
		for path in paths { relaunched.configManager.removeDocumentHistory(path: path) }
		relaunched.configManager.flush()
	}
}
