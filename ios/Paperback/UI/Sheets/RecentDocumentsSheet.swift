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
							// TRANSLATORS: Title of the empty-state view shown when no documents have been opened yet
							t("No Recent Documents"),
							systemImage: "clock",
							// TRANSLATORS: Description text under the "No Recent Documents" empty-state title
							description: Text(t("Documents you open will appear here."))
						)
					} else {
						// TRANSLATORS: Fallback empty-state title shown on iOS versions before 17, when no documents have been opened yet
						Text(t("No Recent Documents"))
							.foregroundStyle(.secondary)
							.frame(maxWidth: .infinity, maxHeight: .infinity)
					}
				} else {
					List {
						ForEach(viewModel.recentDocuments) { doc in
							Button {
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
									// TRANSLATORS: Swipe action to remove a document from the recent documents list
									Label(t("Remove"), systemImage: "trash")
								}
							}
						}
					}
				}
			}
			// TRANSLATORS: Navigation bar title of the recent documents sheet
			.navigationTitle(t("Recent Documents"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button to close the recent documents sheet
					Button(t("Done")) { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Recent Documents")
	}
}
