import SwiftUI

struct TabStripView: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		ScrollViewReader { proxy in
			ScrollView(.horizontal, showsIndicators: false) {
				HStack(spacing: 4) {
					ForEach(viewModel.tabs) { tab in
						TabChip(tab: tab)
							.id(tab.id)
					}
				}
				.padding(.horizontal, 8)
				.padding(.vertical, 6)
			}
			.accessibilityElement(children: .contain)
			.accessibilityLabel(t("Tabs"))
			.onChange(of: viewModel.activeTabId) { id in
				guard let id else { return }
				withAnimation { proxy.scrollTo(id, anchor: .center) }
			}
		}
		.frame(height: 44)
		.background(.bar)
		.overlay(alignment: .bottom) { Divider() }
	}
}

private struct TabChip: View {
	@EnvironmentObject var viewModel: AppViewModel
	let tab: DocumentTab

	private var isActive: Bool { tab.id == viewModel.activeTabId }

	var body: some View {
		ZStack(alignment: .trailing) {
			Button { viewModel.setActiveTab(tab) } label: {
				Text(tab.title)
					.lineLimit(1)
					.font(.subheadline)
					.frame(maxWidth: 140, alignment: .leading)
					.padding(.leading, 10)
					.padding(.trailing, 26)
					.padding(.vertical, 5)
					.background(
						RoundedRectangle(cornerRadius: 7)
							.fill(isActive
								? Color(.systemBackground)
								: Color(.secondarySystemFill))
					)
			}
			.foregroundStyle(isActive ? .primary : .secondary)
			.accessibilityLabel(tab.title)
			.accessibilityAddTraits(isActive ? .isSelected : [])
			.accessibilityAction(named: "Close") { viewModel.closeTab(tab) }

			Button { viewModel.closeTab(tab) } label: {
				Image(systemName: "xmark")
					.font(.system(size: 10, weight: .semibold))
					.foregroundStyle(.secondary)
					.padding(.trailing, 8)
					.padding(.vertical, 10)
			}
			.accessibilityHidden(true)
		}
	}
}
