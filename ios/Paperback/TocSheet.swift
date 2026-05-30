import SwiftUI

struct TocSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Group {
				if let session = viewModel.activeSession {
					let entries = session.getToc()
					if entries.isEmpty {
						emptyView
					} else {
						List(entries, id: \.position) { entry in
							Button {
								viewModel.goToPosition(entry.position)
								dismiss()
							} label: {
								HStack {
									Text(entry.title)
										.padding(.leading, CGFloat(max(0, entry.level - 1)) * 16)
									Spacer()
								}
							}
						}
					}
				} else {
					emptyView
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
		.sheetAccessibilityFocus()
	}

	@ViewBuilder private var emptyView: some View {
		if #available(iOS 17, *) {
			ContentUnavailableView(
				"No Table of Contents",
				systemImage: "list.bullet",
				description: Text("This document has no table of contents.")
			)
		} else {
			Text("No Table of Contents")
				.foregroundStyle(.secondary)
				.frame(maxWidth: .infinity, maxHeight: .infinity)
		}
	}
}
