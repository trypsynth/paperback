import UIKit

extension MagicTapWindow {
	private var appViewModel: AppViewModel? {
		(windowScene?.delegate as? SceneDelegate)?.appViewModel
	}

	private func onMain(_ action: @escaping @MainActor (AppViewModel) -> Void) {
		guard let vm = appViewModel else { return }
		Task { @MainActor in action(vm) }
	}

	override var keyCommands: [UIKeyCommand]? {
		var cmds = super.keyCommands ?? []
		cmds += [
			// Play / pause
			cmd(" ", [], #selector(kbTogglePlayPause)),

			// Section navigation (no shift modifier needed, matching desktop/Android)
			cmd("[", [], #selector(kbPrevSection)),
			cmd("]", [], #selector(kbNextSection)),

			// Heading: H / Shift+H
			cmd("h", [], #selector(kbNextHeading), "Next heading"),
			cmd("h", .shift, #selector(kbPrevHeading), "Previous heading"),

			// Page: P / Shift+P
			cmd("p", [], #selector(kbNextPage), "Next page"),
			cmd("p", .shift, #selector(kbPrevPage), "Previous page"),

			// Image: G / Shift+G
			cmd("g", [], #selector(kbNextImage), "Next image"),
			cmd("g", .shift, #selector(kbPrevImage), "Previous image"),

			// Figure: F / Shift+F
			cmd("f", [], #selector(kbNextFigure), "Next figure"),
			cmd("f", .shift, #selector(kbPrevFigure), "Previous figure"),

			// Link: K / Shift+K
			cmd("k", [], #selector(kbNextLink), "Next link"),
			cmd("k", .shift, #selector(kbPrevLink), "Previous link"),

			// Table: T / Shift+T
			cmd("t", [], #selector(kbNextTable), "Next table"),
			cmd("t", .shift, #selector(kbPrevTable), "Previous table"),

			// Separator: S / Shift+S
			cmd("s", [], #selector(kbNextSeparator), "Next separator"),
			cmd("s", .shift, #selector(kbPrevSeparator), "Previous separator"),

			// List: L / Shift+L
			cmd("l", [], #selector(kbNextList), "Next list"),
			cmd("l", .shift, #selector(kbPrevList), "Previous list"),

			// List item: I / Shift+I
			cmd("i", [], #selector(kbNextListItem), "Next list item"),
			cmd("i", .shift, #selector(kbPrevListItem), "Previous list item"),

			// Find next/prev via F3
			cmd(UIKeyCommand.f3, [], #selector(kbFindNext)),
			cmd(UIKeyCommand.f3, .shift, #selector(kbFindPrev)),

			// Elements list via F7
			cmd(UIKeyCommand.f7, [], #selector(kbElements)),

			// Cmd shortcuts (parity with desktop)
			cmd("f", .command, #selector(kbOpenFind), "Find…"),
			cmd(",", .command, #selector(kbOpenSettings), "Settings…"),
			cmd("t", .command, #selector(kbOpenToc), "Table of Contents"),
			cmd("p", .command, #selector(kbOpenGoToPage), "Go to Page…"),
			cmd("g", .command, #selector(kbOpenGoToLine), "Go to Line…"),
			cmd("g", [.command, .shift], #selector(kbOpenGoToPercent), "Go to Percent…"),
			cmd("w", .command, #selector(kbWordCount), "Word Count"),
			cmd("i", .command, #selector(kbDocumentInfo), "Document Info"),
			cmd("s", [.command, .shift], #selector(kbSleepTimer), "Sleep Timer…"),
		]
		return cmds
	}

	private func cmd(
		_ input: String,
		_ modifiers: UIKeyModifierFlags,
		_ action: Selector,
		_ title: String? = nil
	) -> UIKeyCommand {
		let k = UIKeyCommand(input: input, modifierFlags: modifiers, action: action)
		if let title { k.discoverabilityTitle = title }
		return k
	}

	// MARK: - Actions

	@objc private func kbTogglePlayPause()  { onMain { $0.togglePlayPause() } }

	@objc private func kbNextSection()    { onMain { $0.navigateByType(.section,  direction: .next) } }
	@objc private func kbPrevSection()    { onMain { $0.navigateByType(.section,  direction: .previous) } }
	@objc private func kbNextHeading()    { onMain { $0.navigateByType(.heading,  direction: .next) } }
	@objc private func kbPrevHeading()    { onMain { $0.navigateByType(.heading,  direction: .previous) } }
	@objc private func kbNextPage()       { onMain { $0.navigateByType(.page,     direction: .next) } }
	@objc private func kbPrevPage()       { onMain { $0.navigateByType(.page,     direction: .previous) } }
	@objc private func kbNextImage()      { onMain { $0.navigateByType(.image,    direction: .next) } }
	@objc private func kbPrevImage()      { onMain { $0.navigateByType(.image,    direction: .previous) } }
	@objc private func kbNextFigure()     { onMain { $0.navigateByType(.figure,   direction: .next) } }
	@objc private func kbPrevFigure()     { onMain { $0.navigateByType(.figure,   direction: .previous) } }
	@objc private func kbNextLink()       { onMain { $0.navigateByType(.link,     direction: .next) } }
	@objc private func kbPrevLink()       { onMain { $0.navigateByType(.link,     direction: .previous) } }
	@objc private func kbNextTable()      { onMain { $0.navigateByType(.table,    direction: .next) } }
	@objc private func kbPrevTable()      { onMain { $0.navigateByType(.table,    direction: .previous) } }
	@objc private func kbNextSeparator()  { onMain { $0.navigateByType(.separator,direction: .next) } }
	@objc private func kbPrevSeparator()  { onMain { $0.navigateByType(.separator,direction: .previous) } }
	@objc private func kbNextList()       { onMain { $0.navigateByType(.list,     direction: .next) } }
	@objc private func kbPrevList()       { onMain { $0.navigateByType(.list,     direction: .previous) } }
	@objc private func kbNextListItem()   { onMain { $0.navigateByType(.listItem, direction: .next) } }
	@objc private func kbPrevListItem()   { onMain { $0.navigateByType(.listItem, direction: .previous) } }

	@objc private func kbFindNext()       { onMain { $0.findNext() } }
	@objc private func kbFindPrev()       { onMain { $0.findPrev() } }
	@objc private func kbElements()       { onMain { $0.showElements = true } }

	@objc private func kbOpenFind()       { onMain { $0.showFind = true } }
	@objc private func kbOpenSettings()   { onMain { $0.showSettings = true } }
	@objc private func kbOpenToc()        { onMain { $0.showToc = true } }
	@objc private func kbOpenGoToPage()   { onMain { $0.goToInitialMode = .page;    $0.showGoTo = true } }
	@objc private func kbOpenGoToLine()   { onMain { $0.goToInitialMode = .line;    $0.showGoTo = true } }
	@objc private func kbOpenGoToPercent(){ onMain { $0.goToInitialMode = .percent; $0.showGoTo = true } }
	@objc private func kbWordCount()      { onMain { $0.showWordCount = true } }
	@objc private func kbDocumentInfo()   { onMain { $0.showDocumentInfo = true } }
	@objc private func kbSleepTimer()     { onMain { $0.showSleepTimer = true } }
}
