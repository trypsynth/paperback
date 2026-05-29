import SwiftUI

struct WordCountSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			List {
				// TODO: populate from DocumentSession.getStatsFfi() once UniFFI is wired up
				LabeledContent("Words", value: "—")
				LabeledContent("Lines", value: "—")
				LabeledContent("Characters", value: "—")
				LabeledContent("Characters (no spaces)", value: "—")
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
