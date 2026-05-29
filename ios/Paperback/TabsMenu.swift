import SwiftUI

struct TabsMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		Menu {
			ForEach(viewModel.tabs) { tab in
				Button {
					viewModel.setActiveTab(tab)
				} label: {
					Label(
						tab.title,
						systemImage: tab.id == viewModel.activeTabId ? "checkmark" : "doc.text"
					)
				}
			}
			if let active = viewModel.activeTab {
				Divider()
				Button(role: .destructive) {
					viewModel.closeTab(active)
				} label: {
					Label("Close \"\(active.title)\"", systemImage: "xmark")
				}
			}
		} label: {
			Label("Tabs (\(viewModel.tabs.count))", systemImage: "square.on.square")
				.labelStyle(.iconOnly)
		}
	}
}
