import SwiftUI

struct DocumentMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		Menu {
			Button { viewModel.showToc = true } label: {
				Label("Table of Contents", systemImage: "list.bullet")
			}
			Button { viewModel.showElements = true } label: {
				Label("Elements", systemImage: "list.dash")
			}
			Divider()
			Button { viewModel.showFind = true } label: {
				Label("Find", systemImage: "magnifyingglass")
			}
			Button {
				viewModel.goToInitialMode = .line
				viewModel.showGoTo = true
			} label: {
				Label("Go To…", systemImage: "arrow.right.circle")
			}
			Divider()
			Button { viewModel.showRecents = true } label: {
				Label("Recent Documents", systemImage: "clock")
			}
			Divider()
			Button { viewModel.showWordCount = true } label: {
				Label("Word Count", systemImage: "textformat.123")
			}
			Button { viewModel.showDocumentInfo = true } label: {
				Label("Document Info", systemImage: "info.circle")
			}
			Divider()
			Button { viewModel.showSleepTimer = true } label: {
				Label(
					viewModel.sleepTimerRemaining != nil ? "Sleep Timer (active)" : "Sleep Timer",
					systemImage: "timer"
				)
			}
			Button { viewModel.showSettings = true } label: {
				Label("Settings", systemImage: "gear")
			}
		} label: {
			Image(systemName: "ellipsis.circle")
		}
	}
}
