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
		// The titles are the desktop's menu names, so VoiceOver's Cmd-hold list reads them in the
		// reader's language without asking translators for a second copy of each.
		cmds += [
			cmd(" ", [], #selector(kbTogglePlayPause)),
			cmd("[", [], #selector(kbPrevSection), t("Previous Section")),
			cmd("]", [], #selector(kbNextSection), t("Next Section")),
			cmd("h", [], #selector(kbNextHeading), t("Next Heading")),
			cmd("h", .shift, #selector(kbPrevHeading), t("Previous Heading")),
			cmd("p", [], #selector(kbNextPage), t("Next Page")),
			cmd("p", .shift, #selector(kbPrevPage), t("Previous Page")),
			cmd("g", [], #selector(kbNextImage), t("Next Image")),
			cmd("g", .shift, #selector(kbPrevImage), t("Previous Image")),
			cmd("f", [], #selector(kbNextFigure), t("Next Figure")),
			cmd("f", .shift, #selector(kbPrevFigure), t("Previous Figure")),
			cmd("k", [], #selector(kbNextLink), t("Next Link")),
			cmd("k", .shift, #selector(kbPrevLink), t("Previous Link")),
			cmd("t", [], #selector(kbNextTable), t("Next Table")),
			cmd("t", .shift, #selector(kbPrevTable), t("Previous Table")),
			cmd("m", [], #selector(kbNextFormula), t("Next Formula")),
			cmd("m", .shift, #selector(kbPrevFormula), t("Previous Formula")),
			cmd("s", [], #selector(kbNextSeparator), t("Next Separator")),
			cmd("s", .shift, #selector(kbPrevSeparator), t("Previous Separator")),
			cmd("l", [], #selector(kbNextList), t("Next List")),
			cmd("l", .shift, #selector(kbPrevList), t("Previous List")),
			cmd("i", [], #selector(kbNextListItem), t("Next List Item")),
			cmd("i", .shift, #selector(kbPrevListItem), t("Previous List Item")),
			cmd(UIKeyCommand.f3, [], #selector(kbFindNext), t("Find Next")),
			cmd(UIKeyCommand.f3, .shift, #selector(kbFindPrev), t("Find Previous")),
			cmd(UIKeyCommand.f7, [], #selector(kbElements), t("Elements List...")),
			cmd("o", .command, #selector(kbOpenBook), t("Open...")),
			cmd("r", .command, #selector(kbRecents), t("Show All Recent Documents...")),
			cmd("f", .command, #selector(kbOpenFind), t("Find...")),
			cmd(",", .command, #selector(kbOpenSettings), t("Settings...")),
			cmd("t", .command, #selector(kbOpenToc), t("Table of Contents")),
			cmd("p", .command, #selector(kbOpenGoToPage), t("Go to Page...")),
			cmd("g", .command, #selector(kbOpenGoToLine), t("Go to Line...")),
			cmd("g", [.command, .shift], #selector(kbOpenGoToPercent), t("Go to Percent...")),
			cmd("w", .command, #selector(kbWordCount), t("Word Count")),
			cmd("i", .command, #selector(kbDocumentInfo), t("Document Info")),
			cmd("e", .command, #selector(kbExport), t("Export Document")),
			cmd("s", [.command, .shift], #selector(kbSleepTimer), t("Sleep Timer...")),
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

	@objc private func kbTogglePlayPause()  { onMain { $0.reading.togglePlayPause() } }

	@objc private func kbNextSection()    { onMain { $0.reading.navigateByType(.section,  direction: .next) } }
	@objc private func kbPrevSection()    { onMain { $0.reading.navigateByType(.section,  direction: .previous) } }
	@objc private func kbNextHeading()    { onMain { $0.reading.navigateByType(.heading,  direction: .next) } }
	@objc private func kbPrevHeading()    { onMain { $0.reading.navigateByType(.heading,  direction: .previous) } }
	@objc private func kbNextPage()       { onMain { $0.reading.navigateByType(.page,     direction: .next) } }
	@objc private func kbPrevPage()       { onMain { $0.reading.navigateByType(.page,     direction: .previous) } }
	@objc private func kbNextImage()      { onMain { $0.reading.navigateByType(.image,    direction: .next) } }
	@objc private func kbPrevImage()      { onMain { $0.reading.navigateByType(.image,    direction: .previous) } }
	@objc private func kbNextFigure()     { onMain { $0.reading.navigateByType(.figure,   direction: .next) } }
	@objc private func kbPrevFigure()     { onMain { $0.reading.navigateByType(.figure,   direction: .previous) } }
	@objc private func kbNextLink()       { onMain { $0.reading.navigateByType(.link,     direction: .next) } }
	@objc private func kbPrevLink()       { onMain { $0.reading.navigateByType(.link,     direction: .previous) } }
	@objc private func kbNextTable()      { onMain { $0.reading.navigateByType(.table,    direction: .next) } }
	@objc private func kbPrevTable()      { onMain { $0.reading.navigateByType(.table,    direction: .previous) } }
	@objc private func kbNextFormula()    { onMain { $0.reading.navigateByType(.formula,  direction: .next) } }
	@objc private func kbPrevFormula()    { onMain { $0.reading.navigateByType(.formula,  direction: .previous) } }
	@objc private func kbNextSeparator()  { onMain { $0.reading.navigateByType(.separator,direction: .next) } }
	@objc private func kbPrevSeparator()  { onMain { $0.reading.navigateByType(.separator,direction: .previous) } }
	@objc private func kbNextList()       { onMain { $0.reading.navigateByType(.list,     direction: .next) } }
	@objc private func kbPrevList()       { onMain { $0.reading.navigateByType(.list,     direction: .previous) } }
	@objc private func kbNextListItem()   { onMain { $0.reading.navigateByType(.listItem, direction: .next) } }
	@objc private func kbPrevListItem()   { onMain { $0.reading.navigateByType(.listItem, direction: .previous) } }

	@objc private func kbFindNext()       { onMain { $0.reading.findNext() } }
	@objc private func kbFindPrev()       { onMain { $0.reading.findPrev() } }
	@objc private func kbElements()       { onMain { $0.navigation.showElements = true } }

	@objc private func kbOpenBook()       { onMain { $0.navigation.showFilePicker = true } }
	@objc private func kbRecents()        { onMain { $0.navigation.showRecents = true } }
	@objc private func kbExport()         { onMain { $0.navigation.showExportDocument = true } }
	@objc private func kbOpenFind()       { onMain { $0.navigation.showFind = true } }
	@objc private func kbOpenSettings()   { onMain { $0.navigation.showSettings = true } }
	@objc private func kbOpenToc()        { onMain { $0.navigation.showToc = true } }
	@objc private func kbOpenGoToPage()   { onMain { $0.navigation.goToInitialMode = .page;    $0.navigation.showGoTo = true } }
	@objc private func kbOpenGoToLine()   { onMain { $0.navigation.goToInitialMode = .line;    $0.navigation.showGoTo = true } }
	@objc private func kbOpenGoToPercent(){ onMain { $0.navigation.goToInitialMode = .percent; $0.navigation.showGoTo = true } }
	@objc private func kbWordCount()      { onMain { $0.navigation.showWordCount = true } }
	@objc private func kbDocumentInfo()   { onMain { $0.navigation.showDocumentInfo = true } }
	@objc private func kbSleepTimer()     { onMain { $0.navigation.showSleepTimer = true } }
}
