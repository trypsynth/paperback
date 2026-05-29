import SwiftUI

struct WordCountSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			List {
				if let stats = viewModel.activeSession?.getStatsFfi() {
					LabeledContent("Words", value: stats.wordCount.formatted())
					LabeledContent("Lines", value: stats.lineCount.formatted())
					LabeledContent("Characters", value: stats.charCount.formatted())
					LabeledContent("Characters (no spaces)", value: stats.charCountNoWhitespace.formatted())
				} else {
					LabeledContent("Words", value: "—")
					LabeledContent("Lines", value: "—")
					LabeledContent("Characters", value: "—")
					LabeledContent("Characters (no spaces)", value: "—")
				}
			}
			.navigationTitle("Word Count")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
	}
}
