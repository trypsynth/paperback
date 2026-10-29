import UIKit

@MainActor
protocol BookElement: AnyObject {
	var unit: Int { get }
	var bookView: BookView? { get }
}

extension ReadingUnitFfi {
	/// A unit that is nothing but one link, which double-tap follows.
	var isWholeLink: Bool {
		links.count == 1 && links[0].start == 0 && Int(links[0].length) == text.unicodeScalars.count
	}

	/// Where a link sits in the unit's text, which counts in Unicode scalars.
	func range(of link: ReadingLinkFfi) -> Range<String.Index>? {
		let scalars = text.unicodeScalars
		guard let start = scalars.index(scalars.startIndex, offsetBy: Int(link.start), limitedBy: scalars.endIndex),
		      let end = scalars.index(start, offsetBy: Int(link.length), limitedBy: scalars.endIndex) else { return nil }
		return start..<end
	}
}

/// Equal by unit, so VoiceOver takes a re-created element for the one it saw before the window let it go.
@MainActor
final class ParagraphElement: UIAccessibilityElement, BookElement {
	let unit: Int
	weak var bookView: BookView?
	private let isWholeLink: Bool
	/// UIKit holds an element's container weakly, and the book view may let a list go while VoiceOver is still in it.
	private let list: ListElement?

	init(container: Any, bookView: BookView, unit index: Int, unit: ReadingUnitFfi) {
		self.unit = index
		self.bookView = bookView
		isWholeLink = unit.isWholeLink
		list = container as? ListElement
		super.init(accessibilityContainer: container)
		switch unit.kind {
		case .heading:
			accessibilityAttributedLabel = NSAttributedString(string: unit.description.isEmpty ? unit.text : unit.description, attributes: [.accessibilityTextHeadingLevel: Int(unit.level)])
			accessibilityTraits = .header
		case .image:
			accessibilityLabel = unit.description.isEmpty ? t("Image") : unit.description
			accessibilityTraits = unit.description.isEmpty ? .staticText : .image
		case .paragraph, .listItem, .caption, .tableRow:
			accessibilityLabel = unit.text
			accessibilityTraits = .staticText
		}
		if isWholeLink { accessibilityTraits.insert(.link) }
	}

	nonisolated override func isEqual(_ object: Any?) -> Bool {
		(object as? ParagraphElement)?.unit == unit
	}

	nonisolated override var hash: Int { unit }

	/// Computed on demand so the frame always matches the page now showing.
	override var accessibilityFrame: CGRect {
		get {
			guard let bookView else { return .zero }
			return UIAccessibility.convertToScreenCoordinates(bookView.frame(forUnit: unit), in: bookView)
		}
		set {}
	}

	override func accessibilityElementDidBecomeFocused() {
		bookView?.unitBecameFocused(unit)
	}

	/// Handled even when there is nothing to do, or VoiceOver taps the middle of the paragraph and may land on a link in it.
	override func accessibilityActivate() -> Bool {
		if isWholeLink { bookView?.onFollowLink?(unit, 0) }
		return true
	}
}

/// Only the Links rotor lands here, so a paragraph stays one flick however many links it holds.
@MainActor
final class LinkElement: UIAccessibilityElement, BookElement {
	let unit: Int
	let link: Int
	weak var bookView: BookView?
	private let list: ListElement?

	init(container: Any, bookView: BookView, unit: Int, link: Int, text: String) {
		self.unit = unit
		self.link = link
		self.bookView = bookView
		list = container as? ListElement
		super.init(accessibilityContainer: container)
		accessibilityLabel = text
		accessibilityTraits = .link
	}

	nonisolated override func isEqual(_ object: Any?) -> Bool {
		guard let other = object as? LinkElement else { return false }
		return other.unit == unit && other.link == link
	}

	nonisolated override var hash: Int { unit &* 1024 &+ link }

	override var accessibilityFrame: CGRect {
		get {
			guard let bookView else { return .zero }
			return UIAccessibility.convertToScreenCoordinates(bookView.frame(forUnit: unit), in: bookView)
		}
		set {}
	}

	override func accessibilityElementDidBecomeFocused() {
		bookView?.unitBecameFocused(unit)
	}

	override func accessibilityActivate() -> Bool {
		bookView?.onFollowLink?(unit, link)
		return true
	}
}

@MainActor
final class ListElement: UIAccessibilityElement {
	let block: Int
	private let units: Range<Int>
	weak var bookView: BookView?

	init(bookView: BookView, block: Int, units: Range<Int>) {
		self.block = block
		self.units = units
		self.bookView = bookView
		super.init(accessibilityContainer: bookView)
		isAccessibilityElement = false
		accessibilityContainerType = .list
		// TRANSLATORS: Spoken on entering a list in text mode; {} is how many items it has. Which of the three forms is used depends on the language's plural rule, and the "many" form's trailing character isn't a typo.
		accessibilityLabel = nt(t("{} item"), t("{} items"), t("{} items⁣"), units.count).replacingOccurrences(of: "{}", with: "\(units.count)")
	}

	nonisolated override func isEqual(_ object: Any?) -> Bool {
		(object as? ListElement)?.block == block
	}

	nonisolated override var hash: Int { block }

	override var accessibilityFrame: CGRect {
		get {
			guard let bookView else { return .zero }
			return UIAccessibility.convertToScreenCoordinates(bookView.frame(forUnits: units), in: bookView)
		}
		set {}
	}

	override func accessibilityElementCount() -> Int {
		units.count
	}

	override func accessibilityElement(at index: Int) -> Any? {
		bookView?.element(forUnit: units.lowerBound + index)
	}

	override func index(ofAccessibilityElement element: Any) -> Int {
		guard let unit = (element as? BookElement)?.unit, units.contains(unit) else { return NSNotFound }
		return unit - units.lowerBound
	}
}

@MainActor
final class TableElement: UIAccessibilityElement, UIAccessibilityContainerDataTable {
	let block: Int
	let table: ParsedTable
	/// Row r is unit `firstUnit + r`.
	let firstUnit: Int
	weak var bookView: BookView?
	private let order: [(row: Int, cell: Int)]

	init(bookView: BookView, block: Int, table: ParsedTable, firstUnit: Int) {
		self.block = block
		self.table = table
		self.firstUnit = firstUnit
		self.bookView = bookView
		order = table.rows.enumerated().flatMap { row, cells in cells.indices.map { (row, $0) } }
		super.init(accessibilityContainer: bookView)
		isAccessibilityElement = false
		accessibilityContainerType = .dataTable
	}

	nonisolated override func isEqual(_ object: Any?) -> Bool {
		(object as? TableElement)?.block == block
	}

	nonisolated override var hash: Int { block }

	override var accessibilityFrame: CGRect {
		get {
			guard let bookView else { return .zero }
			return UIAccessibility.convertToScreenCoordinates(bookView.frame(forUnits: firstUnit..<firstUnit + table.rowCount), in: bookView)
		}
		set {}
	}

	private func element(_ cell: ParsedTable.Cell) -> TableCellElement? {
		bookView?.cellElement(row: cell.row, column: cell.column, table: self)
	}

	override func accessibilityElementCount() -> Int {
		order.count
	}

	override func accessibilityElement(at index: Int) -> Any? {
		guard order.indices.contains(index) else { return nil }
		return element(table.rows[order[index].row][order[index].cell])
	}

	override func index(ofAccessibilityElement element: Any) -> Int {
		guard let cell = element as? TableCellElement else { return NSNotFound }
		return order.firstIndex { table.rows[$0.row][$0.cell].column == cell.column && $0.row == cell.row } ?? NSNotFound
	}

	func accessibilityDataTableCellElement(forRow row: Int, column: Int) -> (any UIAccessibilityContainerDataTableCell)? {
		table.cell(row: row, column: column).flatMap(element)
	}

	func accessibilityRowCount() -> Int {
		table.rowCount
	}

	func accessibilityColumnCount() -> Int {
		table.columnCount
	}

	func accessibilityHeaderElements(forRow row: Int) -> [any UIAccessibilityContainerDataTableCell]? {
		table.rowHeaders(row).compactMap(element)
	}

	func accessibilityHeaderElements(forColumn column: Int) -> [any UIAccessibilityContainerDataTableCell]? {
		table.columnHeaders(column).compactMap(element)
	}
}

@MainActor
final class TableCellElement: UIAccessibilityElement, BookElement, UIAccessibilityContainerDataTableCell {
	let row: Int
	let column: Int
	private let rowSpan: Int
	private let columnSpan: Int
	private let columnCount: Int
	/// The unit of the row the cell starts in.
	let unit: Int
	weak var bookView: BookView?
	/// UIKit holds an element's container weakly, and the book view may let a table go while VoiceOver is still in it.
	private let table: TableElement

	init(table: TableElement, cell: ParsedTable.Cell, bookView: BookView) {
		self.table = table
		row = cell.row
		column = cell.column
		rowSpan = cell.rowSpan
		columnSpan = cell.columnSpan
		columnCount = table.table.columnCount
		unit = table.firstUnit + cell.row
		self.bookView = bookView
		super.init(accessibilityContainer: table)
		// TRANSLATORS: Spoken for an empty table cell in text mode
		accessibilityLabel = cell.text.isEmpty ? t("Blank") : cell.text
		accessibilityTraits = .staticText
	}

	nonisolated override func isEqual(_ object: Any?) -> Bool {
		guard let other = object as? TableCellElement else { return false }
		return other.unit == unit && other.column == column
	}

	nonisolated override var hash: Int { unit &* 1024 &+ column }

	override var accessibilityFrame: CGRect {
		get {
			guard let bookView else { return .zero }
			return UIAccessibility.convertToScreenCoordinates(bookView.frame(forCellAt: column, span: columnSpan, of: columnCount, rows: unit..<unit + rowSpan), in: bookView)
		}
		set {}
	}

	func accessibilityRowRange() -> NSRange {
		NSRange(location: row, length: rowSpan)
	}

	func accessibilityColumnRange() -> NSRange {
		NSRange(location: column, length: columnSpan)
	}

	override func accessibilityElementDidBecomeFocused() {
		bookView?.unitBecameFocused(unit)
	}

	override func accessibilityActivate() -> Bool {
		true
	}
}
