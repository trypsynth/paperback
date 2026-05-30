import SwiftUI

struct RecentDocumentsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Group {
				if viewModel.recentDocuments.isEmpty {
					if #available(iOS 17, *) {
						ContentUnavailableView(
							"No Recent Documents",
							systemImage: "clock",
							description: Text("Documents you open will appear here.")
						)
					} else {
						Text("No Recent Documents")
							.foregroundStyle(.secondary)
							.frame(maxWidth: .infinity, maxHeight: .infinity)
					}
				} else {
					List {
						ForEach(viewModel.recentDocuments) { doc in
							Button {
								_ = doc.url.startAccessingSecurityScopedResource()
								viewModel.openDocument(url: doc.url)
								dismiss()
							} label: {
								VStack(alignment: .leading, spacing: 2) {
									Text(doc.title)
										.foregroundStyle(.primary)
									Text(doc.url.path(percentEncoded: false))
										.font(.caption)
										.foregroundStyle(.secondary)
										.lineLimit(1)
								}
							}
							.swipeActions(edge: .trailing) {
								Button(role: .destructive) {
									viewModel.removeRecentDocument(url: doc.url)
								} label: {
									Label("Remove", systemImage: "trash")
								}
							}
						}
					}
				}
			}
			.navigationTitle("Recent Documents")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Recent Documents")
	}
}
