import SwiftUI

struct WordCountSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			VStack {
				if let stats = viewModel.activeSession?.getStatsFfi() {
					Text("This document contains \(stats.wordCount.formatted()) words.")
						.font(.body)
						.multilineTextAlignment(.center)
						.padding()
				}
				Spacer()
			}
			.navigationTitle("Word Count")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Word Count")
	}
}
