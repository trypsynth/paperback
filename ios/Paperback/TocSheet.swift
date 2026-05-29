import SwiftUI

struct TocSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Group {
				if #available(iOS 17, *) {
					ContentUnavailableView(
						"No Table of Contents",
						systemImage: "list.bullet",
						description: Text("This document has no table of contents.")
					)
				} else {
					List {
						// TODO: populate from DocumentSession.getToc() once UniFFI is wired up
						Text("No Table of Contents")
							.foregroundStyle(.secondary)
					}
				}
			}
			.navigationTitle("Contents")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
	}
}
