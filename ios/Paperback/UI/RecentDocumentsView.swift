import SwiftUI
import UniformTypeIdentifiers

struct RecentDocumentsView: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var locateTarget: RecentDocument? = nil

	var body: some View {
		Group {
			if viewModel.recentDocuments.isEmpty {
				ContentUnavailableView(
					// TRANSLATORS: Title of the empty-state view shown when no documents have been opened yet
					t("No Recent Documents"),
					systemImage: "clock",
					// TRANSLATORS: Description text under the "No Recent Documents" empty-state title
					description: Text(t("Documents you open will appear here."))
				)
			} else {
				List {
					ForEach(viewModel.recentDocuments) { doc in
						RecentDocumentRow(
							doc: doc,
							showRemoveButton: false,
							onOpen: {
								viewModel.openDocument(url: doc.url)
								dismiss()
							},
							onRemove: { viewModel.removeRecentDocument(url: doc.url) },
							onLocate: { locateTarget = doc }
						)
						.swipeActions {
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
		// TRANSLATORS: Navigation bar title of the recent documents screen
		.navigationTitle(t("Recent Documents"))
		.navigationBarTitleDisplayMode(.inline)
		.fileImporter(
			isPresented: Binding(get: { locateTarget != nil }, set: { if !$0 { locateTarget = nil } }),
			allowedContentTypes: [.item],
			allowsMultipleSelection: false
		) { result in
			defer { locateTarget = nil }
			guard case .success(let urls) = result, let newURL = urls.first, let target = locateTarget else { return }
			viewModel.locateRecentDocument(target.url, at: newURL)
		}
	}
}
