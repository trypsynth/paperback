import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			List {
				// TODO: populate title/author from DocumentSession once UniFFI is wired up
				LabeledContent("Title", value: viewModel.activeTab?.title ?? "—")
				LabeledContent("Author", value: "—")
				LabeledContent("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
				LabeledContent("Words", value: "—")
				LabeledContent("Pages", value: "—")
			}
			.navigationTitle("Document Info")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
	}
}
