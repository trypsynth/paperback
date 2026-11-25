import UIKit
import XCTest
@testable import Paperback

@MainActor
final class PageLayoutTests: XCTestCase {
	private var dir: URL!

	override func setUpWithError() throws {
		dir = FileManager.default.temporaryDirectory.appending(path: UUID().uuidString)
		try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
	}

	override func tearDownWithError() throws {
		try? FileManager.default.removeItem(at: dir)
	}

	// Each page is one paragraph wrapped over several lines, which only By Columns, Every Line Apart keeps apart.
	private func pdf(_ name: String) throws -> URL {
		let url = dir.appending(path: name)
		let words = (1...60).map { "word\($0)" }.joined(separator: " ")
		let renderer = UIGraphicsPDFRenderer(bounds: CGRect(x: 0, y: 0, width: 612, height: 792))
		try renderer.writePDF(to: url) { context in
			for page in 1...3 {
				context.beginPage()
				("Page \(page) \(words)" as NSString).draw(in: CGRect(x: 72, y: 72, width: 200, height: 640), withAttributes: [.font: UIFont.systemFont(ofSize: 12)])
			}
		}
		return url
	}

	private func cleanUp(_ viewModel: AppViewModel, _ urls: [URL]) {
		viewModel.pageLayout = .automatic
		for tab in viewModel.tabs where urls.contains(tab.url) { viewModel.closeTab(tab) }
		for url in urls {
			viewModel.configManager.setDocumentPageLayout(path: url.path(percentEncoded: false), layout: nil)
			viewModel.configManager.removeDocumentHistory(path: url.path(percentEncoded: false))
		}
		viewModel.configManager.flush()
	}

	func testOnlyPdfAndXpsDocumentsOfferAPageLayout() throws {
		let text = dir.appending(path: "plain.txt")
		try "Alpha paragraph.".write(to: text, atomically: true, encoding: .utf8)
		let paged = try pdf("paged.pdf")
		let viewModel = AppViewModel()
		XCTAssertTrue(viewModel.openDocument(url: text, track: false))
		XCTAssertTrue(viewModel.activeDocumentPageLayout == nil)
		XCTAssertTrue(viewModel.openDocument(url: paged, track: false))
		XCTAssertEqual(viewModel.activeDocumentPageLayout, PageLayout??.some(nil))
		cleanUp(viewModel, [text, paged])
	}

	func testChoosingALayoutReadsTheDocumentAgainOnTheSamePage() async throws {
		let url = try pdf("columns.pdf")
		let viewModel = AppViewModel()
		XCTAssertTrue(viewModel.openDocument(url: url, track: false))
		let tab = try XCTUnwrap(viewModel.activeTab)
		let joined = try XCTUnwrap(tab.session)
		XCTAssertEqual(joined.pageCountFfi(), 3)
		viewModel.reading.goToPage(2)
		viewModel.setActiveDocumentPageLayout(.lines)
		await viewModel.readAgainTask?.value
		XCTAssertEqual(viewModel.activeDocumentPageLayout, PageLayout??.some(.lines))
		XCTAssertEqual(viewModel.activeTab?.id, tab.id, "read again in its own tab")
		let apart = try XCTUnwrap(viewModel.activeSession)
		XCTAssertFalse(apart === joined)
		XCTAssertGreaterThan(apart.lineCount(), joined.lineCount())
		XCTAssertEqual(apart.currentPage(position: viewModel.reading.ttsPosition), 2)
		XCTAssertEqual(viewModel.configManager.getDocumentPosition(path: url.path(percentEncoded: false)), viewModel.reading.ttsPosition)
		XCTAssertTrue(viewModel.reading.currentSegmentText.contains("Page 2"), viewModel.reading.currentSegmentText)
		viewModel.setActiveDocumentPageLayout(nil)
		await viewModel.readAgainTask?.value
		XCTAssertEqual(viewModel.activeDocumentPageLayout, PageLayout??.some(nil))
		XCTAssertEqual(viewModel.activeSession?.lineCount(), joined.lineCount())
		cleanUp(viewModel, [url])
	}

	func testTheSettingsLayoutReadsAgainOnlyDocumentsWithoutOneOfTheirOwn() async throws {
		let following = try pdf("following.pdf")
		let own = try pdf("own.pdf")
		let viewModel = AppViewModel()
		XCTAssertTrue(viewModel.openDocument(url: own, track: false))
		viewModel.setActiveDocumentPageLayout(.automatic)
		XCTAssertTrue(viewModel.openDocument(url: following, track: false))
		let ownBefore = viewModel.tabs.first { $0.url == own }?.session
		let followingBefore = try XCTUnwrap(viewModel.activeSession)
		viewModel.pageLayout = .lines
		await viewModel.readAgainTask?.value
		XCTAssertEqual(viewModel.configManager.getPageLayout(), .lines)
		XCTAssertTrue(viewModel.tabs.first { $0.url == own }?.session === ownBefore)
		XCTAssertGreaterThan(viewModel.activeSession?.lineCount() ?? 0, followingBefore.lineCount())
		cleanUp(viewModel, [following, own])
	}
}
