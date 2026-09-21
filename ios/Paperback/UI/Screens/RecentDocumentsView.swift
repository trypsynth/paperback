import SwiftUI
import UniformTypeIdentifiers

struct RecentDocumentsView: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var locateTarget: RecentDocument? = nil
	@State private var confirmingClear = false

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
		.toolbar {
			if !viewModel.recentDocuments.isEmpty {
				ToolbarItem(placement: .topBarTrailing) {
					Button(role: .destructive) { confirmingClear = true } label: {
						// TRANSLATORS: Button that empties the recent documents list
						Label(t("Clear Recent Documents"), systemImage: "trash")
					}
				}
			}
		}
		// TRANSLATORS: Title of the dialog confirming that the recent documents list should be emptied
		.confirmationDialog(t("Clear Recent Documents"), isPresented: $confirmingClear, titleVisibility: .visible) {
			// TRANSLATORS: Button that empties the recent documents list
			Button(t("Clear"), role: .destructive) { viewModel.clearRecentDocuments() }
			// TRANSLATORS: Button to close the clear-recent-documents dialog without clearing
			Button(t("Cancel"), role: .cancel) { }
		} message: {
			// TRANSLATORS: Body of the dialog confirming that the recent documents list should be emptied
			Text(t("This empties the list. The documents themselves are not deleted."))
		}
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
