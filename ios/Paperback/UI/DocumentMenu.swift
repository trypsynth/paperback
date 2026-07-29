import SwiftUI

struct DocumentMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		if viewModel.activeTab != nil {
			fullMenu
		} else {
			emptyMenu
		}
	}

	private var fullMenuAccessibilityBase: some View {
		fullMenuButton
			// TRANSLATORS: VoiceOver accessibility label for the "..." button that opens this document actions menu
			.accessibilityLabel(t("More options"))
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAction(named: "Help") { viewModel.openHelpDocument() }
			.accessibilityAction(named: "Settings") { viewModel.showSettings = true }
			.accessibilityAction(named: "Sleep Timer") { viewModel.showSleepTimer = true }
			.accessibilityAction(named: "Document Info") { viewModel.showDocumentInfo = true }
			.accessibilityAction(named: "Word Count") { viewModel.showWordCount = true }
	}

	private var fullMenu: some View {
		fullMenuAccessibilityBase
			.accessibilityAction(named: "Recent Documents") { viewModel.showRecents = true }
			.accessibilityAction(named: "Go To") { viewModel.showGoTo = true }
			.accessibilityAction(named: "Find") { viewModel.showFind = true }
			.accessibilityAction(named: "Elements") { viewModel.showElements = true }
			.accessibilityAction(named: "Table of Contents") { viewModel.showToc = true }
			.accessibilityAction(named: toggleModeActionName) {
				viewModel.isTextMode.toggle()
			}
	}

	private var emptyMenu: some View {
		emptyMenuButton
			// TRANSLATORS: VoiceOver accessibility label for the "..." button that opens this document actions menu
			.accessibilityLabel(t("More options"))
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAction(named: "Help") { viewModel.openHelpDocument() }
			.accessibilityAction(named: "Settings") { viewModel.showSettings = true }
	}

	// TRANSLATORS: VoiceOver custom action name for toggling between TTS and text reading mode; same strings as the menu item above
	private var toggleModeActionName: String {
		viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")
	}

	private var fullMenuButton: some View {
		Menu {
			navigationItems
			Divider()
			infoItems
			Divider()
			modeItems
			Divider()
			helpAndSettingsItems
		} label: {
			Image(systemName: "ellipsis.circle")
		}
	}

	private var emptyMenuButton: some View {
		Menu {
			helpAndSettingsItems
		} label: {
			Image(systemName: "ellipsis.circle")
		}
	}

	@ViewBuilder private var navigationItems: some View {
		Button { viewModel.showToc = true } label: {
			// TRANSLATORS: Menu item to open the document's table of contents
			Label(t("Table of Contents"), systemImage: "list.bullet")
		}
		Button { viewModel.showElements = true } label: {
			// TRANSLATORS: Menu item to open the list of headings and links found in the document
			Label(t("Elements"), systemImage: "list.dash")
		}
		Divider()
		Button { viewModel.showFind = true } label: {
			// TRANSLATORS: Menu item to open find-in-document search
			Label(t("Find"), systemImage: "magnifyingglass")
		}
		Button {
			viewModel.showGoTo = true
		} label: {
			// TRANSLATORS: Menu item to jump to a specific page, line, or percentage in the document
			Label(t("Go To…"), systemImage: "arrow.right.circle")
		}
	}

	@ViewBuilder private var infoItems: some View {
		Button { viewModel.showRecents = true } label: {
			// TRANSLATORS: Menu item to open the list of recently opened documents
			Label(t("Recent Documents"), systemImage: "clock")
		}
		Divider()
		Button { viewModel.showWordCount = true } label: {
			// TRANSLATORS: Menu item to show word/character/line count statistics for the document
			Label(t("Word Count"), systemImage: "textformat.123")
		}
		Button { viewModel.showDocumentInfo = true } label: {
			// TRANSLATORS: Menu item to show document metadata (title, author, format, etc.)
			Label(t("Document Info"), systemImage: "info.circle")
		}
	}

	@ViewBuilder private var modeItems: some View {
		Button { viewModel.isTextMode.toggle() } label: {
			// TRANSLATORS: Menu item toggling between spoken (TTS) reading mode and visual text reading mode; label reflects the mode it will switch TO
			let title: String = viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")
			let icon: String = viewModel.isTextMode ? "speaker.wave.2" : "text.alignleft"
			Label(title, systemImage: icon)
		}
		Divider()
		Button { viewModel.showSleepTimer = true } label: {
			// TRANSLATORS: Menu item to open the sleep timer; the "(active)" variant is shown while a timer is currently counting down
			let title: String = viewModel.sleepTimerRemaining != nil ? t("Sleep Timer (active)") : t("Sleep Timer")
			Label(title, systemImage: "timer")
		}
	}

	@ViewBuilder private var helpAndSettingsItems: some View {
		Button { viewModel.openHelpDocument() } label: {
			// TRANSLATORS: Menu item to open the in-app help document
			Label(t("Help"), systemImage: "questionmark.circle")
		}
		Button { viewModel.showSettings = true } label: {
			// TRANSLATORS: Menu item to open the app's settings
			Label(t("Settings"), systemImage: "gear")
		}
	}
}
