import SwiftUI

struct TabsMenu: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		makeBody()
	}

	private var menuContent: some View {
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
		.accessibilityLabel("Tabs")
		.accessibilityRemoveTraits(.isButton)
	}

	// Builds the menu with dynamic per-tab accessibility actions.
	// Declaration order is LIFO in VoiceOver, so we declare:
	//   close first  → appears last
	//   tabs reversed → tab[0] appears first
	private func makeBody() -> some View {
		var view: AnyView = AnyView(menuContent)
		if let active = viewModel.activeTab {
			let a = active
			view = AnyView(view.accessibilityAction(named: "Close \"\(a.title)\"") {
				viewModel.closeTab(a)
			})
		}
		for tab in viewModel.tabs.reversed() {
			if tab.id == viewModel.activeTabId { continue }
			let t = tab
			view = AnyView(view.accessibilityAction(named: t.title) {
				viewModel.setActiveTab(t)
			})
		}
		return view
	}
}
