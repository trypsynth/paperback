import UIKit
import XCTest
@testable import Paperback

@MainActor
final class ReadingScreenTests: XCTestCase {
	private func model(_ contents: String, as fileExtension: String = "html") throws -> BookModel {
		let url = FileManager.default.temporaryDirectory.appending(path: "\(UUID().uuidString).\(fileExtension)")
		try contents.write(to: url, atomically: true, encoding: .utf8)
		let session = try DocumentSession.newFfi(filePath: url.path(percentEncoded: false), password: "", forcedExtension: "", renderTablesInline: false)
		return BookModel(session: session, layout: ReadingLayout(session: session))
	}

	private func book(_ body: String) throws -> BookView {
		let view = BookView(model: try model("<html><head><title>Book</title></head><body>\(body)</body></html>"))
		view.frame = CGRect(x: 0, y: 0, width: 390, height: 600)
		view.layoutIfNeeded()
		return view
	}

	private func paragraphs(_ count: Int) -> String {
		(1...count).map { "<p>Paragraph \($0)</p>" }.joined()
	}

	func testTheContainerHoldsEveryBlockOfTheBook() throws {
		let view = try book(paragraphs(500))
		XCTAssertEqual(view.accessibilityElementCount(), 500)
		let last = try XCTUnwrap(view.accessibilityElement(at: 499) as? ParagraphElement)
		XCTAssertEqual(last.accessibilityLabel, "Paragraph 500")
		XCTAssertEqual(view.index(ofAccessibilityElement: last), 499)
	}

	func testAUnitOffThePageClaimsARectangleOffTheScreen() throws {
		let view = try book(paragraphs(500))
		XCTAssertTrue(view.bounds.intersects(view.frame(forUnit: 0)))
		XCTAssertFalse(view.bounds.intersects(view.frame(forUnit: 400)))
		XCTAssertLessThan(view.frame(forUnit: 400).minY, view.frame(forUnit: 450).minY)
	}

	func testFocusOffThePageTurnsThePageToIt() throws {
		let view = try book(paragraphs(500))
		view.unitBecameFocused(400)
		XCTAssertTrue(view.bounds.intersects(view.frame(forUnit: 400)))
		XCTAssertFalse(view.bounds.intersects(view.frame(forUnit: 0)))
		XCTAssertEqual(view.firstUnitOnPage, 400)
	}

	func testARecreatedElementIsTheOneVoiceOverSawBefore() throws {
		let view = try book(paragraphs(3))
		let first = try XCTUnwrap(view.element(forUnit: 1))
		let again = ParagraphElement(container: view, bookView: view, unit: 1, unit: view.model.unit(1))
		XCTAssertEqual(first, again)
		XCTAssertEqual(first.hash, again.hash)
	}

	func testAListIsOneContainerOfItsItems() throws {
		let view = try book("<p>Before</p><ul><li>One</li><li>Two</li></ul><p>After</p>")
		XCTAssertEqual(view.accessibilityElementCount(), 3)
		let list = try XCTUnwrap(view.accessibilityElement(at: 1) as? ListElement)
		XCTAssertEqual(list.accessibilityElementCount(), 2)
		XCTAssertEqual((list.accessibilityElement(at: 1) as? ParagraphElement)?.accessibilityLabel?.hasSuffix("Two"), true)
		XCTAssertEqual((view.accessibilityElement(at: 2) as? ParagraphElement)?.accessibilityLabel, "After")
	}

	func testATableIsADataTableWithItsHeaders() throws {
		let view = try book("<table><tr><th>Item</th><th>Cost</th></tr><tr><td>Tea</td><td></td></tr></table>")
		let table = try XCTUnwrap(view.accessibilityElement(at: 0) as? TableElement)
		XCTAssertEqual(table.accessibilityRowCount(), 2)
		XCTAssertEqual(table.accessibilityColumnCount(), 2)
		XCTAssertEqual((table.accessibilityDataTableCellElement(forRow: 1, column: 0) as? TableCellElement)?.accessibilityLabel, "Tea")
		XCTAssertEqual((table.accessibilityDataTableCellElement(forRow: 1, column: 1) as? TableCellElement)?.accessibilityLabel, t("Blank"))
		XCTAssertEqual((table.accessibilityHeaderElements(forColumn: 1)?.first as? TableCellElement)?.accessibilityLabel, "Cost")
	}

	func testAHeadingCarriesItsLevel() throws {
		let view = try book("<h2>Part</h2><p>Text</p>")
		let heading = try XCTUnwrap(view.accessibilityElement(at: 0) as? ParagraphElement)
		XCTAssertTrue(heading.accessibilityTraits.contains(.header))
		XCTAssertEqual(heading.accessibilityAttributedLabel?.attribute(.accessibilityTextHeadingLevel, at: 0, effectiveRange: nil) as? Int, 2)
	}

	func testAParagraphThatIsOneLinkIsALink() throws {
		let view = try book("<p><a href=\"https://example.com\">Example</a></p><p>Go <a href=\"https://example.com\">here</a> now</p>")
		XCTAssertTrue((view.accessibilityElement(at: 0) as? ParagraphElement)?.accessibilityTraits.contains(.link) == true)
		XCTAssertFalse((view.accessibilityElement(at: 1) as? ParagraphElement)?.accessibilityTraits.contains(.link) == true)
		XCTAssertEqual(view.element(forLink: 0, inUnit: 1)?.accessibilityLabel, "here")
	}

	func testOnlyTheRotorsTheBookHasSomethingForAreOffered() throws {
		let plain = try book(paragraphs(3))
		XCTAssertEqual(plain.accessibilityCustomRotors?.count, 0)
		let structured = try book("<h1>A</h1><p><a href=\"#b\">to B</a></p><table><tr><td>1</td><td>2</td></tr></table><h2 id=\"b\">B</h2>")
		let types = structured.accessibilityCustomRotors?.map(\.systemRotorType) ?? []
		XCTAssertTrue(types.contains(.heading))
		XCTAssertTrue(types.contains(.link))
		XCTAssertTrue(types.contains(.table))
	}

	private func reader(_ model: BookModel) -> ReadingViewController {
		let controller = ReadingViewController(model: model, position: 0)
		controller.loadViewIfNeeded()
		controller.view.frame = CGRect(x: 0, y: 0, width: 390, height: 600)
		controller.view.layoutIfNeeded()
		return controller
	}

	private let threePages = #"{\rtf1\ansi One\par\page Two\par\page Three\par}"#

	func testTheThreeFingerTapNamesTheBooksPageWhenItHasPages() throws {
		let book = try model(threePages, as: "rtf")
		let controller = reader(book)
		XCTAssertEqual(controller.accessibilityScrollStatus(for: controller.bookView), "0%, page 1")
		controller.jump(toPosition: book.position(ofUnit: book.count - 1))
		XCTAssertEqual(controller.accessibilityScrollStatus(for: controller.bookView)?.hasSuffix("page 3"), true)
	}

	func testTheThreeFingerTapIsThePercentageWithoutPages() throws {
		let controller = reader(try model("<html><body><p>One</p><p>Two</p></body></html>"))
		XCTAssertEqual(controller.accessibilityScrollStatus(for: controller.bookView), "0 percent")
	}

	private func page(_ controller: ReadingViewController) -> String? {
		controller.accessibilityScrollStatus(for: controller.bookView)?.components(separatedBy: ", ").last
	}

	func testThreeFingerFlicksTurnTheBooksPagesAndStopAtItsEdges() throws {
		let controller = reader(try model(threePages, as: "rtf"))
		XCTAssertTrue(controller.bookView.accessibilityScroll(.up))
		XCTAssertEqual(page(controller), "page 2")
		XCTAssertTrue(controller.bookView.accessibilityScroll(.up))
		XCTAssertEqual(page(controller), "page 3")
		XCTAssertFalse(controller.bookView.accessibilityScroll(.up))
		XCTAssertTrue(controller.bookView.accessibilityScroll(.down))
		XCTAssertTrue(controller.bookView.accessibilityScroll(.down))
		XCTAssertEqual(page(controller), "page 1")
		XCTAssertFalse(controller.bookView.accessibilityScroll(.down))
	}

	func testAPageStartingInsideAParagraphIsTurnedToAndPast() throws {
		let book = try model(#"{\rtf1\ansi One\page two\par Three\par\page Four\par}"#, as: "rtf")
		let controller = reader(book)
		XCTAssertTrue(controller.bookView.accessibilityScroll(.up))
		XCTAssertEqual(page(controller), "page 2")
		XCTAssertEqual(controller.bookView.focusedUnit, 0)
		XCTAssertTrue(controller.bookView.accessibilityScroll(.up))
		XCTAssertEqual(page(controller), "page 3")
		XCTAssertEqual(controller.bookView.focusedUnit, book.count - 1)
		XCTAssertTrue(controller.bookView.accessibilityScroll(.down))
		XCTAssertEqual(page(controller), "page 2")
		XCTAssertTrue(controller.bookView.accessibilityScroll(.down))
		XCTAssertEqual(page(controller), "page 1")
	}

	func testWithoutPagesAFlickTurnsTheScreensPage() throws {
		let controller = reader(try model("<html><body>\(paragraphs(300))</body></html>"))
		XCTAssertTrue(controller.bookView.accessibilityScroll(.up))
		XCTAssertGreaterThan(controller.bookView.firstUnitOnPage, 0)
		XCTAssertTrue(controller.bookView.accessibilityScroll(.down))
		XCTAssertEqual(controller.bookView.firstUnitOnPage, 0)
	}
}
