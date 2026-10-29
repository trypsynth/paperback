import UIKit

@MainActor
final class ReadingViewController: UIViewController, UIScrollViewAccessibilityDelegate {
	let bookView: BookView
	private var model: BookModel { bookView.model }
	private var currentUnit: Int
	/// Kept exact, so a place partway into a paragraph isn't moved to its start just by opening.
	private var currentPosition: Int64

	var onPositionChange: ((Int64) -> Void)?

	/// VoiceOver focuses something by itself on appearing and on coming back to the front, often the first element, which mustn't count as the reading position.
	private var restoreTarget = 0
	private var restoreUntil: Date?
	private var returningToFront = false

	init(model: BookModel, position: Int64) {
		bookView = BookView(model: model)
		currentPosition = position
		currentUnit = model.unit(forPosition: position)
		super.init(nibName: nil, bundle: nil)
	}

	required init?(coder: NSCoder) { fatalError("not used") }

	override func loadView() {
		view = bookView
	}

	override func viewDidLoad() {
		super.viewDidLoad()
		bookView.acceptsFocus = { [weak self] unit in
			self?.acceptsFocus(at: unit) ?? true
		}
		bookView.onFocusChange = { [weak self] unit in
			self?.setCurrent(unit)
		}
		bookView.onFollowLink = { [weak self] unit, link in
			self?.followLink(link, inUnit: unit)
		}
		bookView.onPageScroll = { [weak self] forward in
			self?.turnPage(forward: forward) ?? false
		}
		bookView.delegate = self
		view.addGestureRecognizer(UITapGestureRecognizer(target: self, action: #selector(handleTap(_:))))
		for direction in [UISwipeGestureRecognizer.Direction.left, .right] {
			let swipe = UISwipeGestureRecognizer(target: self, action: #selector(handleSwipe(_:)))
			swipe.direction = direction
			view.addGestureRecognizer(swipe)
		}
		NotificationCenter.default.addObserver(self, selector: #selector(willEnterForeground), name: UIApplication.willEnterForegroundNotification, object: nil)
		NotificationCenter.default.addObserver(self, selector: #selector(didBecomeActive), name: UIApplication.didBecomeActiveNotification, object: nil)
		// Laid out on the saved place before VoiceOver's first query, so the restored unit owns the screen from the start.
		bookView.reveal(currentUnit, startingPage: true)
		restoreTarget = currentUnit
		restoreUntil = Date().addingTimeInterval(3)
	}

	override func viewDidAppear(_ animated: Bool) {
		super.viewDidAppear(animated)
		restoreFocus()
	}

	func restoreFocus() {
		restoreTarget = currentUnit
		restoreUntil = Date().addingTimeInterval(3)
		postRestoreFocus()
	}

	private func postRestoreFocus() {
		guard UIAccessibility.isVoiceOverRunning, let element = bookView.focusTarget(forUnit: restoreTarget) else { return }
		UIAccessibility.post(notification: .screenChanged, argument: element)
	}

	private func acceptsFocus(at unit: Int) -> Bool {
		guard let until = restoreUntil else { return true }
		// A flick from the target counts too: VoiceOver may already have been on it and never said it landed.
		if abs(unit - restoreTarget) <= 1 || Date() >= until {
			restoreUntil = nil
			return true
		}
		postRestoreFocus()
		return false
	}

	@objc private func willEnterForeground() {
		returningToFront = true
	}

	/// Only after the app was in the background: becoming active after Control Center or a call banner leaves focus where it is, so Read All carries on.
	@objc private func didBecomeActive() {
		guard returningToFront else { return }
		returningToFront = false
		guard view.window != nil, view.window?.rootViewController?.presentedViewController == nil else { return }
		restoreFocus()
	}

	private func setCurrent(_ unit: Int) {
		guard unit != currentUnit else { return }
		currentUnit = unit
		currentPosition = model.position(ofUnit: unit)
		onPositionChange?(currentPosition)
	}

	/// A place the app chose, such as a table of contents entry or a search result, with VoiceOver following.
	func jump(toPosition position: Int64) {
		let unit = model.unit(forPosition: position)
		currentPosition = position
		currentUnit = unit
		bookView.reveal(unit)
		restoreTarget = unit
		restoreUntil = nil
		guard view.window != nil, UIAccessibility.isVoiceOverRunning, let element = bookView.focusTarget(forUnit: unit) else { return }
		UIAccessibility.post(notification: .layoutChanged, argument: element)
	}

	/// Keeps the page on what is being read aloud without taking VoiceOver focus there.
	func follow(position: Int64) {
		let unit = model.unit(forPosition: position)
		currentPosition = position
		currentUnit = unit
		bookView.reveal(unit)
	}

	private func followLink(_ link: Int, inUnit unit: Int) {
		let links = model.unit(unit).links
		guard links.indices.contains(link) else { return }
		let result = model.session.activateLink(position: links[link].position)
		guard result.found else { return }
		switch result.action {
		case .internal:
			jump(toPosition: result.offset)
			onPositionChange?(result.offset)
		case .external:
			if let url = URL(string: result.url) { UIApplication.shared.open(url) }
		case .notFound:
			break
		}
	}

	// MARK: Pages

	/// What the three-finger tap speaks.
	private func status() -> String {
		let percent = model.session.getStatusInfo(position: currentPosition).percentage
		let page = model.session.currentPage(position: currentPosition)
		// The same words as desktop's position shortcut, whose translations this shares.
		guard page > 0 else { return t("{} percent").replacingOccurrences(of: "{}", with: "\(percent)") }
		return t("%s, page %d").replacingOccurrences(of: "%s", with: "\(percent)%").replacingOccurrences(of: "%d", with: "\(page)")
	}

	func accessibilityScrollStatus(for scrollView: UIScrollView) -> String? {
		status()
	}

	/// The book's own pages when it has them, else the screen's.
	private func turnPage(forward: Bool) -> Bool {
		let landing: (unit: Int, position: Int64)?
		if model.session.pageOffset(page: 1) >= 0 {
			landing = bookPageTurn(forward: forward)
		} else {
			landing = bookView.visualPageStop(forward: forward).map { ($0, model.position(ofUnit: $0)) }
		}
		guard let landing else {
			// The same status again tells VoiceOver it is at the edge.
			UIAccessibility.post(notification: .pageScrolled, argument: status())
			return false
		}
		bookView.reveal(landing.unit, startingPage: true)
		currentUnit = landing.unit
		currentPosition = landing.position
		onPositionChange?(landing.position)
		if let element = bookView.focusTarget(forUnit: landing.unit) {
			UIAccessibility.post(notification: .layoutChanged, argument: element)
		}
		UIAccessibility.post(notification: .pageScrolled, argument: status())
		return true
	}

	/// Counted from the exact place, so a page starting partway into a paragraph is turned to like any other.
	private func bookPageTurn(forward: Bool) -> (unit: Int, position: Int64)? {
		let session = model.session
		let page = Int(session.currentPage(position: currentPosition))
		let target: Int
		if forward {
			target = page + 1
		} else {
			// Back to the start of the page being read first, unless the reader is already at it.
			target = page > 0 && session.pageOffset(page: Int32(page)) < currentPosition ? page : page - 1
		}
		guard target >= 1 else { return nil }
		let position = session.pageOffset(page: Int32(target))
		guard position >= 0 else { return nil }
		return (model.unit(forPosition: position), position)
	}

	// MARK: Without VoiceOver

	/// A link follows; elsewhere, the outer quarters of the page turn it.
	@objc private func handleTap(_ gesture: UITapGestureRecognizer) {
		let point = gesture.location(in: bookView)
		if let hit = bookView.hit(point), let link = hit.link {
			followLink(link, inUnit: hit.unit)
			return
		}
		let width = bookView.bounds.width
		guard point.x < width / 4 || point.x > width * 3 / 4 else { return }
		let forward = (point.x > width / 2) != (view.effectiveUserInterfaceLayoutDirection == .rightToLeft)
		turnPageWithoutVoiceOver(forward: forward)
	}

	@objc private func handleSwipe(_ gesture: UISwipeGestureRecognizer) {
		turnPageWithoutVoiceOver(forward: (gesture.direction == .left) != (view.effectiveUserInterfaceLayoutDirection == .rightToLeft))
	}

	private func turnPageWithoutVoiceOver(forward: Bool) {
		bookView.turnPage(forward: forward)
		setCurrent(bookView.firstUnitOnPage)
	}
}
