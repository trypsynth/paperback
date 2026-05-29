import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			List {
				if let session = viewModel.activeSession {
					LabeledContent("Title", value: session.title().isEmpty
						? (viewModel.activeTab?.title ?? "—")
						: session.title())
					LabeledContent("Author", value: session.author().isEmpty ? "—" : session.author())
					LabeledContent("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
					let stats = session.getStatsFfi()
					LabeledContent("Words", value: stats.wordCount.formatted())
					LabeledContent("Pages", value: session.pageCountFfi().formatted())
				} else {
					LabeledContent("Title", value: viewModel.activeTab?.title ?? "—")
					LabeledContent("Author", value: "—")
					LabeledContent("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
					LabeledContent("Words", value: "—")
					LabeledContent("Pages", value: "—")
				}
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
