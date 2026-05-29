import SwiftUI

struct ElementsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Group {
				if #available(iOS 17, *) {
					ContentUnavailableView(
						"No Elements",
						systemImage: "list.bullet.indent",
						description: Text("Headings, images, and other elements will appear here.")
					)
				} else {
					Text("No Elements")
						.foregroundStyle(.secondary)
						.frame(maxWidth: .infinity, maxHeight: .infinity)
				}
			}
			.navigationTitle("Elements")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
	}
}
