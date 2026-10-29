import UIKit

/// The readability settings, as the reading screen lays text out with them.
struct ReadingStyle: Equatable {
	var scalePercent = 100
	var lineSpacingChoice = 0
	var paragraphSpacingChoice = 0
	var alignmentChoice = 0
	var highContrast = false
}

/// The whole book as one accessibility container, so Read All never runs out of elements; a page of it is drawn, and turns to follow focus.
@MainActor
final class BookView: UIScrollView {
	let model: BookModel
	var style = ReadingStyle() {
		didSet {
			guard style != oldValue else { return }
			heights.removeAll()
			setNeedsLayout()
		}
	}

	private(set) var focusedUnit = 0
	private var pageStart = 0
	/// Frames of the units on the page, from `pageStart` on, in this view's coordinates.
	private var pageFrames: [CGRect] = []
	private var labels: [UILabel] = []
	/// The label drawing each unit on the page that isn't a table row.
	private var labelOfUnit: [Int: UILabel] = [:]
	private var liveElements: [Int: ParagraphElement] = [:]
	private var liveLinks: [Int: [Int: LinkElement]] = [:]
	private var liveBlocks: [Int: UIAccessibilityElement] = [:]
	private var liveCells: [Int: [Int: TableCellElement]] = [:]
	private let windowRadius = 128
	private var heights: [Int: CGFloat] = [:]
	private var measuredWidth: CGFloat = 0
	private let sizingLabel = UILabel()

	/// Whether focus landing on a unit is the reader's own move, asked before the page turns to it.
	var acceptsFocus: (Int) -> Bool = { _ in true }
	var onFocusChange: ((Int) -> Void)?
	/// A link to follow, as its unit and its index among the unit's links.
	var onFollowLink: ((Int, Int) -> Void)?
	/// A three-finger flick, forward or back, reporting whether there was a page to go to.
	var onPageScroll: ((Bool) -> Bool)?

	init(model: BookModel) {
		self.model = model
		super.init(frame: .zero)
		isAccessibilityElement = false
		// VoiceOver asks only a scroll view it can scroll for the three-finger tap's status, so scrolling is on and the content a point taller than the page, with the drag off and the offset pinned.
		isScrollEnabled = true
		panGestureRecognizer.isEnabled = false
		alwaysBounceVertical = false
		bounces = false
		scrollsToTop = false
		showsVerticalScrollIndicator = false
		showsHorizontalScrollIndicator = false
		contentInsetAdjustmentBehavior = .never
		sizingLabel.numberOfLines = 0
		registerForTraitChanges([UITraitPreferredContentSizeCategory.self, UITraitAccessibilityContrast.self, UITraitUserInterfaceStyle.self]) { (view: BookView, _) in
			view.heights.removeAll()
			view.setNeedsLayout()
		}
	}

	required init?(coder: NSCoder) { fatalError("not used") }

	// MARK: Style

	private var isHighContrast: Bool {
		style.highContrast || traitCollection.accessibilityContrast == .high
	}

	private var textColor: UIColor {
		guard isHighContrast else { return .label }
		return traitCollection.userInterfaceStyle == .dark ? .white : .black
	}

	private var pageColor: UIColor {
		guard isHighContrast else { return .systemBackground }
		return traitCollection.userInterfaceStyle == .dark ? .black : .white
	}

	/// Multiplies Dynamic Type rather than fixing a size, as the rest of the app's reading text does.
	private var bodySize: CGFloat {
		UIFontMetrics(forTextStyle: .body).scaledValue(for: 17, compatibleWith: traitCollection) * CGFloat(style.scalePercent) / 100
	}

	private var paragraphGap: CGFloat {
		switch style.paragraphSpacingChoice {
		case 1: return bodySize * 1.3
		case 2: return bodySize * 2
		default: return bodySize * 0.6
		}
	}

	private var alignment: NSTextAlignment {
		switch style.alignmentChoice {
		case 1: return .center
		case 2: return effectiveUserInterfaceLayoutDirection == .rightToLeft ? .left : .right
		default: return .natural
		}
	}

	private func font(for unit: ReadingUnitFfi, header: Bool = false) -> UIFont {
		var size = bodySize
		var traits: UIFontDescriptor.SymbolicTraits = []
		switch unit.kind {
		case .heading:
			size *= unit.level == 1 ? 1.5 : unit.level == 2 ? 1.3 : unit.level == 3 ? 1.15 : 1
			traits = .traitBold
		case .image:
			traits = .traitItalic
		case .caption:
			traits = .traitBold
		case .tableRow:
			size *= 0.9
			if header { traits = .traitBold }
		case .paragraph, .listItem:
			break
		}
		let font = UIFont.systemFont(ofSize: size)
		guard !traits.isEmpty, let descriptor = font.fontDescriptor.withSymbolicTraits(traits) else { return font }
		return UIFont(descriptor: descriptor, size: size)
	}

	private func attributes(font: UIFont) -> [NSAttributedString.Key: Any] {
		let paragraph = NSMutableParagraphStyle()
		paragraph.alignment = alignment
		switch style.lineSpacingChoice {
		case 1: paragraph.lineSpacing = font.pointSize * 0.5
		case 2: paragraph.lineSpacing = font.pointSize
		default: break
		}
		return [.font: font, .foregroundColor: textColor, .paragraphStyle: paragraph]
	}

	// MARK: Layout

	private var textArea: CGRect {
		let safe = bounds.inset(by: safeAreaInsets)
		return safe.inset(by: UIEdgeInsets(top: 16, left: layoutMargins.left, bottom: 16, right: layoutMargins.right))
	}

	private func configure(_ label: UILabel, forUnit index: Int) {
		let unit = model.unit(index)
		let text = NSMutableAttributedString(string: unit.text, attributes: attributes(font: font(for: unit)))
		for link in unit.links {
			guard let range = unit.range(of: link) else { continue }
			text.addAttributes([.foregroundColor: isHighContrast ? textColor : UIColor.link, .underlineStyle: NSUnderlineStyle.single.rawValue], range: NSRange(range, in: unit.text))
		}
		label.attributedText = text
		label.numberOfLines = 0
	}

	private func configure(_ label: UILabel, forCell cell: ParsedTable.Cell, of unit: ReadingUnitFfi) {
		label.attributedText = NSAttributedString(string: cell.text, attributes: attributes(font: font(for: unit, header: cell.isHeader)))
		label.numberOfLines = 0
	}

	private func height(ofUnit index: Int, width: CGFloat) -> CGFloat {
		if width != measuredWidth {
			heights.removeAll()
			measuredWidth = width
		}
		if let known = heights[index] { return known }
		var height: CGFloat = 0
		if let (table, row) = model.tableRow(index) {
			// A row is as tall as its tallest cell; cells spanning rows are measured in the row they start in.
			let column = width / CGFloat(table.columnCount)
			let unit = model.unit(index)
			for cell in table.rows[row] {
				configure(sizingLabel, forCell: cell, of: unit)
				height = max(height, sizingLabel.sizeThatFits(CGSize(width: column * CGFloat(cell.columnSpan) - 6, height: .greatestFiniteMagnitude)).height)
			}
			height = ceil(height)
		} else {
			configure(sizingLabel, forUnit: index)
			height = ceil(sizingLabel.sizeThatFits(CGSize(width: width, height: .greatestFiniteMagnitude)).height)
		}
		heights[index] = height
		return height
	}

	/// A table's rows sit close together.
	private func gap(after index: Int) -> CGFloat {
		if model.tableRow(index) != nil, index + 1 < model.count, model.tableRow(index + 1) != nil, model.blockOfUnit(index) == model.blockOfUnit(index + 1) {
			return paragraphGap / 3
		}
		return paragraphGap
	}

	private func pageFrames(from start: Int) -> [CGRect] {
		let area = textArea
		guard area.width > 0, start < model.count else { return [] }
		var frames: [CGRect] = []
		var y = area.minY
		var index = start
		while index < model.count {
			let height = height(ofUnit: index, width: area.width)
			if !frames.isEmpty, y + height > area.maxY { break }
			frames.append(CGRect(x: area.minX, y: y, width: area.width, height: height))
			y += height + gap(after: index)
			index += 1
		}
		return frames
	}

	private func pageStart(endingAt index: Int) -> Int {
		let area = textArea
		return pageStart(before: index, filling: area.height - height(ofUnit: index, width: area.width))
	}

	private func pageStart(before index: Int, filling room: CGFloat) -> Int {
		let width = textArea.width
		var used: CGFloat = 0
		var start = index
		while start > 0 {
			let previous = height(ofUnit: start - 1, width: width) + gap(after: start - 1)
			if used + previous > room { break }
			used += previous
			start -= 1
		}
		return start
	}

	private func showPage(from start: Int) {
		backgroundColor = pageColor
		pageStart = min(max(start, 0), max(model.count - 1, 0))
		pageFrames = pageFrames(from: pageStart)
		labelOfUnit.removeAll()
		var used = 0
		func nextLabel() -> UILabel {
			if used == labels.count {
				let label = UILabel()
				label.isAccessibilityElement = false
				addSubview(label)
				labels.append(label)
			}
			used += 1
			return labels[used - 1]
		}
		for (offset, frame) in pageFrames.enumerated() {
			let index = pageStart + offset
			if let (table, row) = model.tableRow(index) {
				let unit = model.unit(index)
				for cell in table.rows[row] {
					let label = nextLabel()
					configure(label, forCell: cell, of: unit)
					label.frame = cellFrame(column: cell.column, span: cell.columnSpan, of: table.columnCount, in: frame).insetBy(dx: 3, dy: 0)
					label.isHidden = false
				}
			} else {
				let label = nextLabel()
				configure(label, forUnit: index)
				label.frame = frame
				label.isHidden = false
				labelOfUnit[index] = label
			}
		}
		for label in labels[used...] {
			label.isHidden = true
		}
	}

	/// Pinned: VoiceOver may try to scroll toward an element off the page, which would only slide the drawn page away.
	override var contentOffset: CGPoint {
		get { .zero }
		set { super.contentOffset = .zero }
	}

	override func setContentOffset(_ contentOffset: CGPoint, animated: Bool) {}

	override func layoutSubviews() {
		super.layoutSubviews()
		contentSize = CGSize(width: bounds.width, height: bounds.height + 1)
		showPage(from: pageStart)
		// A bigger text size or a rotation can push the focused unit off the page it was on.
		if !isOnPage(focusedUnit) { showPage(from: focusedUnit) }
	}

	private func isOnPage(_ index: Int) -> Bool {
		index >= pageStart && index < pageStart + pageFrames.count
	}

	/// A unit's real rectangle when it is on the page, else whole screen-heights above or below.
	func frame(forUnit index: Int) -> CGRect {
		if isOnPage(index) { return pageFrames[index - pageStart] }
		let screen = max(bounds.height, 1)
		let area = textArea
		let y = index < pageStart ? -CGFloat(pageStart - index) * screen : CGFloat(index - pageStart - pageFrames.count + 1) * screen
		return CGRect(x: area.minX, y: y, width: area.width, height: screen / 2)
	}

	func frame(forUnits units: Range<Int>) -> CGRect {
		let onPage = units.clamped(to: pageStart..<pageStart + pageFrames.count)
		guard let first = onPage.first, let last = onPage.last else { return frame(forUnit: units.lowerBound) }
		return frame(forUnit: first).union(frame(forUnit: last))
	}

	private func cellFrame(column: Int, span: Int, of columns: Int, in row: CGRect) -> CGRect {
		let width = row.width / CGFloat(max(columns, 1))
		return CGRect(x: row.minX + CGFloat(column) * width, y: row.minY, width: width * CGFloat(span), height: row.height)
	}

	func frame(forCellAt column: Int, span: Int, of columns: Int, rows: Range<Int>) -> CGRect {
		cellFrame(column: column, span: span, of: columns, in: frame(forUnits: rows))
	}

	func hit(_ point: CGPoint) -> (unit: Int, link: Int?)? {
		guard let offset = pageFrames.firstIndex(where: { $0.contains(point) }) else { return nil }
		let index = pageStart + offset
		guard let label = labelOfUnit[index] else { return (index, nil) }
		let unit = model.unit(index)
		guard !unit.links.isEmpty, let text = label.attributedText else { return (index, nil) }
		let storage = NSTextStorage(attributedString: text)
		let manager = NSLayoutManager()
		let container = NSTextContainer(size: label.bounds.size)
		container.lineFragmentPadding = 0
		container.maximumNumberOfLines = 0
		manager.addTextContainer(container)
		storage.addLayoutManager(manager)
		let location = convert(point, to: label)
		let glyph = manager.glyphIndex(for: location, in: container)
		guard manager.boundingRect(forGlyphRange: NSRange(location: glyph, length: 1), in: container).contains(location) else { return (index, nil) }
		let character = manager.characterIndexForGlyph(at: glyph)
		let link = unit.links.firstIndex { link in
			guard let range = unit.range(of: link) else { return false }
			return NSRange(range, in: unit.text).contains(character)
		}
		return (index, link)
	}

	// MARK: Moving through the book

	/// Brings `index` onto the page, turning forward so it starts the page or back so it ends it; a page turn asks for it to start the page either way.
	func reveal(_ index: Int, startingPage: Bool = false) {
		guard model.count > 0 else { return }
		let index = min(max(index, 0), model.count - 1)
		focusedUnit = index
		if pageFrames.isEmpty { pageStart = index }
		if startingPage {
			if index != pageStart { showPage(from: index) }
			return
		}
		guard !isOnPage(index) else { return }
		showPage(from: index < pageStart ? pageStart(endingAt: index) : index)
	}

	var lastUnitOnPage: Int { pageStart + max(pageFrames.count - 1, 0) }
	var firstUnitOnPage: Int { pageStart }

	func turnPage(forward: Bool) {
		if forward {
			guard lastUnitOnPage + 1 < model.count else { return }
			showPage(from: lastUnitOnPage + 1)
		} else {
			guard pageStart > 0 else { return }
			showPage(from: pageStart(endingAt: pageStart - 1))
		}
		focusedUnit = pageStart
	}

	func unitBecameFocused(_ index: Int) {
		guard acceptsFocus(index) else { return }
		reveal(index)
		trimElements()
		onFocusChange?(index)
	}

	// MARK: Elements

	func focusTarget(forUnit index: Int) -> NSObject? {
		guard index >= 0, index < model.count else { return nil }
		guard let (_, row) = model.tableRow(index) else { return element(forUnit: index) }
		guard let table = blockElement(model.blockOfUnit(index)) as? TableElement, let first = table.table.rows[row].first else { return nil }
		return cellElement(row: row, column: first.column, table: table)
	}

	private func container(forUnit index: Int) -> Any {
		let block = model.blockOfUnit(index)
		return model.block(block).kind == .list ? (blockElement(block) ?? self) : self
	}

	func element(forUnit index: Int) -> ParagraphElement? {
		guard index >= 0, index < model.count, model.tableRow(index) == nil else { return nil }
		if let live = liveElements[index] { return live }
		let element = ParagraphElement(container: container(forUnit: index), bookView: self, unit: index, unit: model.unit(index))
		liveElements[index] = element
		trimElements()
		return element
	}

	func element(forLink link: Int, inUnit unit: Int) -> LinkElement? {
		guard unit >= 0, unit < model.count else { return nil }
		let links = model.unit(unit).links
		guard links.indices.contains(link) else { return nil }
		if let live = liveLinks[unit]?[link] { return live }
		let element = LinkElement(container: container(forUnit: unit), bookView: self, unit: unit, link: link, text: links[link].text)
		liveLinks[unit, default: [:]][link] = element
		return element
	}

	func blockElement(_ block: Int) -> NSObject? {
		guard block >= 0, block < model.blockCount else { return nil }
		let info = model.block(block)
		let units = model.units(ofBlock: block)
		switch info.kind {
		case .single:
			return element(forUnit: units.lowerBound)
		case .list:
			if let live = liveBlocks[block] { return live }
			let list = ListElement(bookView: self, block: block, units: units)
			liveBlocks[block] = list
			return list
		case .table:
			if let live = liveBlocks[block] { return live }
			let element = TableElement(bookView: self, block: block, table: model.table(Int(info.table)), firstUnit: units.lowerBound)
			liveBlocks[block] = element
			return element
		}
	}

	func cellElement(row: Int, column: Int, table: TableElement) -> TableCellElement? {
		let unit = table.firstUnit + row
		if let live = liveCells[unit]?[column] { return live }
		guard let cell = table.table.rows[row].first(where: { $0.column == column }) else { return nil }
		let element = TableCellElement(table: table, cell: cell, bookView: self)
		liveCells[unit, default: [:]][column] = element
		return element
	}

	private func trimElements() {
		guard liveElements.count + liveCells.count > windowRadius * 2 else { return }
		liveElements = liveElements.filter { abs($0.key - focusedUnit) <= windowRadius }
		liveLinks = liveLinks.filter { abs($0.key - focusedUnit) <= windowRadius }
		liveCells = liveCells.filter { abs($0.key - focusedUnit) <= windowRadius }
		liveBlocks = liveBlocks.filter { block, _ in
			let units = model.units(ofBlock: block)
			return abs(units.lowerBound - focusedUnit) <= windowRadius || units.contains(focusedUnit)
		}
	}

	// MARK: UIAccessibilityContainer

	override func accessibilityElementCount() -> Int {
		model.blockCount
	}

	override func accessibilityElement(at index: Int) -> Any? {
		blockElement(index)
	}

	override func index(ofAccessibilityElement element: Any) -> Int {
		switch element {
		case let list as ListElement: return list.block
		case let table as TableElement: return table.block
		case let element as BookElement where element.unit >= 0 && element.unit < model.count: return model.blockOfUnit(element.unit)
		default: return NSNotFound
		}
	}

	/// Three-finger flicks move by page. Read All's own page turns are left alone, since the page already follows focus.
	override func accessibilityScroll(_ direction: UIAccessibilityScrollDirection) -> Bool {
		switch direction {
		case .up: onPageScroll?(true) ?? false
		case .down: onPageScroll?(false) ?? false
		default: false
		}
	}

	func visualPageStop(forward: Bool) -> Int? {
		if forward { return lastUnitOnPage + 1 < model.count ? lastUnitOnPage + 1 : nil }
		return pageStart > 0 ? pageStart(endingAt: pageStart - 1) : nil
	}

	/// Touch exploration asks only about the page showing; left to itself, VoiceOver would walk every element in the book to find the one under a finger.
	@available(iOS 18.0, *)
	override func accessibilityHitTest(_ point: CGPoint, event: UIEvent?) -> Any? {
		guard let offset = pageFrames.firstIndex(where: { $0.contains(point) }) else { return nil }
		let index = pageStart + offset
		guard let (table, row) = model.tableRow(index), let element = blockElement(model.blockOfUnit(index)) as? TableElement else {
			return element(forUnit: index)
		}
		let frame = pageFrames[offset]
		let column = min(Int((point.x - frame.minX) / (frame.width / CGFloat(table.columnCount))), table.columnCount - 1)
		guard let cell = table.cell(row: row, column: column) else { return nil }
		return cellElement(row: cell.row, column: cell.column, table: element)
	}

	// MARK: Rotors

	/// Only the rotors the book has something for, so a book without tables offers no Tables rotor.
	override var accessibilityCustomRotors: [UIAccessibilityCustomRotor]? {
		get {
			var present: [UIAccessibilityCustomRotor] = []
			if !model.headingUnits.isEmpty { present.append(headingsRotor) }
			if model.linkStop(fromUnit: 0, link: -1, forward: true) != nil { present.append(linksRotor) }
			if !model.tableUnits.isEmpty { present.append(tablesRotor) }
			if !model.sectionUnits.isEmpty { present.append(sectionsRotor) }
			return present
		}
		set {}
	}

	private lazy var headingsRotor = unitRotor(systemType: .heading) { view, from, forward in
		BookModel.stop(in: view.model.headingUnits, from: from, forward: forward)
	}
	private lazy var linksRotor = linkRotor()
	private lazy var tablesRotor = unitRotor(systemType: .table) { view, from, forward in
		view.structureStop(view.model.tableUnits, from: from, forward: forward)
	}
	// TRANSLATORS: Name of the VoiceOver rotor setting that moves between the entries of a document's table of contents in text mode
	private lazy var sectionsRotor = unitRotor(name: t("Sections")) { view, from, forward in
		BookModel.stop(in: view.model.sectionUnits, from: from, forward: forward)
	}

	/// The next or previous table, counted from the start of the one the origin is inside.
	private func structureStop(_ stops: [Int], from: Int, forward: Bool) -> Int? {
		let start = model.units(ofBlock: model.blockOfUnit(from)).lowerBound
		return BookModel.stop(in: stops, from: forward ? max(from, start) : start, forward: forward)
	}

	private func origin(of predicate: UIAccessibilityCustomRotorSearchPredicate) -> Int {
		(predicate.currentItem.targetElement as? BookElement)?.unit ?? focusedUnit
	}

	private func unitRotor(name: String? = nil, systemType: UIAccessibilityCustomRotor.SystemRotorType = .none, next: @escaping (BookView, Int, Bool) -> Int?) -> UIAccessibilityCustomRotor {
		let search: UIAccessibilityCustomRotor.Search = { [weak self] predicate in
			guard let self else { return nil }
			let from = origin(of: predicate)
			guard let target = next(self, from, predicate.searchDirection == .next), let element = focusTarget(forUnit: target) else { return nil }
			return UIAccessibilityCustomRotorItemResult(targetElement: element, targetRange: nil)
		}
		if let name { return UIAccessibilityCustomRotor(name: name, itemSearch: search) }
		return UIAccessibilityCustomRotor(systemType: systemType, itemSearch: search)
	}

	private func linkRotor() -> UIAccessibilityCustomRotor {
		UIAccessibilityCustomRotor(systemType: .link) { [weak self] predicate in
			guard let self else { return nil }
			let current = predicate.currentItem.targetElement
			let unit = (current as? BookElement)?.unit ?? focusedUnit
			let link = (current as? LinkElement)?.link ?? -1
			guard let found = model.linkStop(fromUnit: unit, link: link, forward: predicate.searchDirection == .next), let element = element(forLink: found.link, inUnit: found.unit) else { return nil }
			return UIAccessibilityCustomRotorItemResult(targetElement: element, targetRange: nil)
		}
	}
}
