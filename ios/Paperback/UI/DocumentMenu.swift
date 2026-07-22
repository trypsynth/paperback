import SwiftUI

struct DocumentMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		Menu {
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
			Divider()
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
			Divider()
			Button { viewModel.isTextMode.toggle() } label: {
				Label(
					// TRANSLATORS: Menu item toggling between spoken (TTS) reading mode and visual text reading mode; label reflects the mode it will switch TO
					viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode"),
					systemImage: viewModel.isTextMode ? "speaker.wave.2" : "text.alignleft"
				)
			}
			Divider()
			Button { viewModel.showSleepTimer = true } label: {
				Label(
					// TRANSLATORS: Menu item to open the sleep timer; the "(active)" variant is shown while a timer is currently counting down
					viewModel.sleepTimerRemaining != nil ? t("Sleep Timer (active)") : t("Sleep Timer"),
					systemImage: "timer"
				)
			}
			Button { viewModel.showSettings = true } label: {
				// TRANSLATORS: Menu item to open the app's settings
				Label(t("Settings"), systemImage: "gear")
			}
		} label: {
			Image(systemName: "ellipsis.circle")
		}
		// TRANSLATORS: VoiceOver accessibility label for the "..." button that opens this document actions menu
		.accessibilityLabel(t("More options"))
		.accessibilityRemoveTraits(.isButton)
		.accessibilityAction(named: "Settings") { viewModel.showSettings = true }
		.accessibilityAction(named: "Sleep Timer") { viewModel.showSleepTimer = true }
		.accessibilityAction(named: "Document Info") { viewModel.showDocumentInfo = true }
		.accessibilityAction(named: "Word Count") { viewModel.showWordCount = true }
		.accessibilityAction(named: "Recent Documents") { viewModel.showRecents = true }
		.accessibilityAction(named: "Go To") { viewModel.showGoTo = true }
		.accessibilityAction(named: "Find") { viewModel.showFind = true }
		.accessibilityAction(named: "Elements") { viewModel.showElements = true }
		.accessibilityAction(named: "Table of Contents") { viewModel.showToc = true }
		// TRANSLATORS: VoiceOver custom action name for toggling between TTS and text reading mode; same strings as the menu item above
		.accessibilityAction(named: viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")) {
			viewModel.isTextMode.toggle()
		}
	}
}
