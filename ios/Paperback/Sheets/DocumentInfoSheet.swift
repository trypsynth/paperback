import SwiftUI

struct DocumentInfoSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			List {
				if let session = viewModel.activeSession {
					let title = session.title().isEmpty
						? (viewModel.activeTab?.title ?? "—")
						: session.title()
					let author = session.author()
					let stats = session.getStatsFfi()

					LabeledContent("Title", value: title)
					if !author.isEmpty {
						LabeledContent("Author", value: author)
					}
					LabeledContent("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
					LabeledContent("Words", value: stats.wordCount.formatted())
					LabeledContent("Lines", value: stats.lineCount.formatted())
					LabeledContent("Characters", value: stats.charCount.formatted())
					LabeledContent("Characters (no spaces)", value: stats.charCountNoWhitespace.formatted())
				} else {
					LabeledContent("Title", value: viewModel.activeTab?.title ?? "—")
					LabeledContent("File", value: viewModel.activeTab?.url.lastPathComponent ?? "—")
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
		.sheetAccessibilityFocus(title: "Document Info")
	}
}
