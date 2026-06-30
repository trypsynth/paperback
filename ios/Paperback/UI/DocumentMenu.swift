import SwiftUI

struct DocumentMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		Menu {
			Button { viewModel.showToc = true } label: {
				Label(t("Table of Contents"), systemImage: "list.bullet")
			}
			Button { viewModel.showElements = true } label: {
				Label(t("Elements"), systemImage: "list.dash")
			}
			Divider()
			Button { viewModel.showFind = true } label: {
				Label(t("Find"), systemImage: "magnifyingglass")
			}
			Button {
				viewModel.showGoTo = true
			} label: {
				Label(t("Go To…"), systemImage: "arrow.right.circle")
			}
			Divider()
			Button { viewModel.showRecents = true } label: {
				Label(t("Recent Documents"), systemImage: "clock")
			}
			Divider()
			Button { viewModel.showWordCount = true } label: {
				Label(t("Word Count"), systemImage: "textformat.123")
			}
			Button { viewModel.showDocumentInfo = true } label: {
				Label(t("Document Info"), systemImage: "info.circle")
			}
			Divider()
			Button { viewModel.isTextMode.toggle() } label: {
				Label(
					viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode"),
					systemImage: viewModel.isTextMode ? "speaker.wave.2" : "text.alignleft"
				)
			}
			Divider()
			Button { viewModel.showSleepTimer = true } label: {
				Label(
					viewModel.sleepTimerRemaining != nil ? t("Sleep Timer (active)") : t("Sleep Timer"),
					systemImage: "timer"
				)
			}
			Button { viewModel.showSettings = true } label: {
				Label(t("Settings"), systemImage: "gear")
			}
		} label: {
			Image(systemName: "ellipsis.circle")
		}
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
		.accessibilityAction(named: viewModel.isTextMode ? t("Switch to TTS Mode") : t("Switch to Text Mode")) {
			viewModel.isTextMode.toggle()
		}
	}
}
