import SwiftUI

struct SearchControlBar: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		HStack(spacing: 12) {
			Image(systemName: "magnifyingglass")
				.foregroundStyle(.secondary)
			Text(viewModel.activeSearchQuery ?? "")
				.lineLimit(1)
				.foregroundStyle(.secondary)
				.frame(maxWidth: .infinity, alignment: .leading)
			Button { viewModel.findPrev() } label: {
				Image(systemName: "chevron.up")
			}
			Button { viewModel.findNext() } label: {
				Image(systemName: "chevron.down")
			}
			Button { viewModel.clearSearch() } label: {
				Image(systemName: "xmark.circle.fill")
					.foregroundStyle(.secondary)
			}
		}
		.padding(.horizontal, 16)
		.padding(.vertical, 10)
	}
}
