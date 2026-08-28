import SwiftUI

struct TabStripView: View {
	@Environment(AppViewModel.self) private var viewModel

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
			// TRANSLATORS: Accessibility label for the horizontal strip of open document tabs
			.accessibilityLabel(t("Tabs"))
			.onChange(of: viewModel.activeTabId) { _, id in
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
	@Environment(AppViewModel.self) private var viewModel
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
					.modifier(TabChipBackground(isActive: isActive))
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

// The active tab gets a Liquid Glass background on iOS 26+ instead of a flat fill, matching
// Safari's tab-chip look; earlier versions keep the plain filled pill.
private struct TabChipBackground: ViewModifier {
	let isActive: Bool

	func body(content: Content) -> some View {
		if #available(iOS 26, *), isActive {
			content.glassEffect(.regular, in: RoundedRectangle(cornerRadius: 14))
		} else {
			content.background(
				RoundedRectangle(cornerRadius: 14)
					.fill(isActive
						? Color(.systemBackground)
						: Color(.secondarySystemFill))
			)
		}
	}
}
