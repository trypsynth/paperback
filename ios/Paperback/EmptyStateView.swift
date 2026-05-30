import SwiftUI

struct EmptyStateView: View {
	@EnvironmentObject var viewModel: AppViewModel
	let onOpenFile: () -> Void

	var body: some View {
		VStack(spacing: 0) {
			Spacer()
			Image(systemName: "book.closed")
				.font(.system(size: 64))
				.foregroundStyle(.secondary)
				.padding(.bottom, 16)
				.accessibilityHidden(true)
			Text("No document open")
				.font(.title2)
				.foregroundStyle(.secondary)
			Spacer()
			if !viewModel.recentDocuments.isEmpty {
				Divider()
				recentList
			}
		}
	}

	private var recentList: some View {
		VStack(alignment: .leading, spacing: 0) {
			HStack {
				Text("Recent")
					.font(.headline)
					.padding(.horizontal)
					.padding(.top, 12)
				Spacer()
				Button("All") { viewModel.showRecents = true }
					.padding(.horizontal)
					.padding(.top, 12)
			}
			ForEach(viewModel.recentDocuments.prefix(5)) { doc in
				Button {
					viewModel.openDocument(url: doc.url)
				} label: {
					HStack {
						Image(systemName: "doc.text")
							.foregroundStyle(.secondary)
							.accessibilityHidden(true)
						Text(doc.title)
							.foregroundStyle(.primary)
							.lineLimit(1)
						Spacer()
					}
					.padding(.horizontal)
					.padding(.vertical, 10)
				}
				Divider().padding(.leading)
			}
		}
	}
}
