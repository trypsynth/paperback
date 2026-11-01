import Foundation

/// A table's grid as the reading screen lays it out and VoiceOver reads it.
struct ParsedTable: Equatable {
	struct Cell: Equatable {
		var text: String
		var isHeader: Bool
		var row: Int
		var column: Int
		var rowSpan: Int
		var columnSpan: Int
	}

	var caption: String
	/// The cells starting in each row, left to right.
	var rows: [[Cell]]
	var columnCount: Int
	var headerRowCount: Int

	var rowCount: Int { rows.count }

	init(_ table: ReadingTableFfi) {
		caption = table.caption
		columnCount = max(Int(table.columnCount), 1)
		headerRowCount = Int(table.headerRowCount)
		var rows = Array(repeating: [Cell](), count: Int(table.rowCount))
		for cell in table.cells where rows.indices.contains(Int(cell.row)) {
			rows[Int(cell.row)].append(Cell(text: cell.text, isHeader: cell.isHeader, row: Int(cell.row), column: Int(cell.column), rowSpan: Int(cell.rowSpan), columnSpan: Int(cell.columnSpan)))
		}
		self.rows = rows
	}

	/// The cell covering a grid position, wherever its spans start it.
	func cell(row: Int, column: Int) -> Cell? {
		for candidates in rows.prefix(row + 1) {
			if let cell = candidates.first(where: { $0.row <= row && row < $0.row + $0.rowSpan && $0.column <= column && column < $0.column + $0.columnSpan }) {
				return cell
			}
		}
		return nil
	}

	func columnHeaders(_ column: Int) -> [Cell] {
		(0..<headerRowCount).compactMap { cell(row: $0, column: column) }.filter(\.isHeader)
	}

	func rowHeaders(_ row: Int) -> [Cell] {
		guard row >= headerRowCount, let first = cell(row: row, column: 0), first.isHeader else { return [] }
		return [first]
	}
}

/// The open document as the reading screen sees it, read from the core's layout a stretch at a time and kept only around where the reader is.
@MainActor
final class BookModel {
	let session: DocumentSession
	let layout: ReadingLayout
	let count: Int
	let blockCount: Int
	private var units: [Int: ReadingUnitFfi] = [:]
	private var blocks: [Int: ReadingBlockFfi] = [:]
	private var tables: [Int: ParsedTable] = [:]
	private static let chunk = 64
	private static let kept = 4096

	init(session: DocumentSession, layout: ReadingLayout) {
		self.session = session
		self.layout = layout
		count = Int(layout.unitCount())
		blockCount = Int(layout.blockCount())
	}

	lazy var headingUnits: [Int] = layout.headingUnits().map(Int.init)
	lazy var tableUnits: [Int] = layout.tableUnits().map(Int.init)
	lazy var sectionUnits: [Int] = layout.sectionUnits().map(Int.init)

	func unit(_ index: Int) -> ReadingUnitFfi {
		if let unit = units[index] { return unit }
		if units.count > Self.kept {
			units = units.filter { abs($0.key - index) < Self.kept / 2 }
		}
		let start = max(index - Self.chunk / 2, 0)
		for (offset, unit) in layout.units(start: Int64(start), count: Int64(Self.chunk)).enumerated() {
			units[start + offset] = unit
		}
		return units[index] ?? layout.units(start: Int64(index), count: 1)[0]
	}

	func block(_ index: Int) -> ReadingBlockFfi {
		if let block = blocks[index] { return block }
		if blocks.count > Self.kept { blocks.removeAll() }
		let block = layout.block(index: Int64(index))
		blocks[index] = block
		return block
	}

	func units(ofBlock index: Int) -> Range<Int> {
		let block = block(index)
		return Int(block.firstUnit)..<Int(block.firstUnit + block.unitCount)
	}

	func blockOfUnit(_ index: Int) -> Int {
		Int(unit(index).block)
	}

	func table(_ index: Int) -> ParsedTable {
		if let table = tables[index] { return table }
		let table = ParsedTable(layout.table(index: Int64(index)))
		tables[index] = table
		return table
	}

	/// The table and row a unit lays out, when it is a table row.
	func tableRow(_ index: Int) -> (table: ParsedTable, row: Int)? {
		let unit = unit(index)
		guard unit.kind == .tableRow, unit.table >= 0 else { return nil }
		return (table(Int(unit.table)), Int(unit.row))
	}

	func unit(forPosition position: Int64) -> Int {
		Int(layout.unitAtPosition(position: position))
	}

	func position(ofUnit index: Int) -> Int64 {
		guard index >= 0, index < count else { return 0 }
		return unit(index).position
	}

	/// The link after or before one of a unit's links, or after or before the unit itself with `link` -1.
	func linkStop(fromUnit unit: Int, link: Int, forward: Bool) -> (unit: Int, link: Int)? {
		let stop = layout.linkStop(unit: Int64(unit), link: Int64(link), forward: forward)
		return stop.found ? (Int(stop.unit), Int(stop.link)) : nil
	}

	/// The first stop after `index` going forward, or the last one before it going back.
	static func stop(in stops: [Int], from index: Int, forward: Bool) -> Int? {
		forward ? stops.first { $0 > index } : stops.last { $0 < index }
	}
}
