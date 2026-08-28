import SwiftUI
import UniformTypeIdentifiers

struct EmptyStateView: View {
	@Environment(AppViewModel.self) private var viewModel
	let onOpenFile: () -> Void
	@State private var locateTarget: RecentDocument? = nil

	var body: some View {
		VStack(spacing: 0) {
			emptyContent
			if !viewModel.recentDocuments.isEmpty {
				Divider()
				recentList
			}
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

	@ViewBuilder private var emptyContent: some View {
		ContentUnavailableView(
			// TRANSLATORS: Shown in the main reading area when no document is currently open
			t("No document open"),
			systemImage: "book.closed"
		)
		.frame(maxWidth: .infinity, maxHeight: .infinity)
	}

	private var recentList: some View {
		VStack(alignment: .leading, spacing: 0) {
			// TRANSLATORS: Heading above the short list of recently opened documents shown when no document is open
			Text(t("Recent Documents"))
				.font(.headline)
				.padding(.horizontal)
				.padding(.top, 12)
				.accessibilityAddTraits(.isHeader)
			ForEach(viewModel.recentDocuments.prefix(5)) { doc in
				RecentDocumentRow(
					doc: doc,
					showClosedStatus: false,
					onOpen: { viewModel.openDocument(url: doc.url) },
					onRemove: { viewModel.removeRecentDocument(url: doc.url) },
					onLocate: { locateTarget = doc }
				)
				.padding(.horizontal)
				Divider().padding(.leading)
			}
			// TRANSLATORS: Button below the short recent-documents preview that opens the full Recent Documents list
			Button(t("Show All")) { viewModel.showRecents = true }
				.padding()
		}
	}
}
