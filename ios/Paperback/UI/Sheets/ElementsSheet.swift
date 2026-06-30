import SwiftUI

struct ElementsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var tab = 0

	var body: some View {
		NavigationStack {
			Group {
				if let session = viewModel.activeSession {
					let headings = session.getHeadingTreeFfi(position: viewModel.ttsPosition)
					let links = session.getLinkListFfi(position: viewModel.ttsPosition)
					if headings.items.isEmpty && links.items.isEmpty {
						emptyView
					} else {
						VStack(spacing: 0) {
							Picker(t("Type"), selection: $tab) {
								Text(t("Headings")).tag(0)
								Text(t("Links")).tag(1)
							}
							.pickerStyle(.segmented)
							.padding()
							if tab == 0 {
								List(headings.items, id: \.offset) { item in
									Button {
										viewModel.goToPosition(item.offset)
										dismiss()
									} label: {
										Text(item.text)
									}
								}
							} else {
								List(links.items, id: \.offset) { item in
									Button {
										viewModel.goToPosition(item.offset)
										dismiss()
									} label: {
										Text(item.text)
									}
								}
							}
						}
					}
				} else {
					emptyView
				}
			}
			.navigationTitle(t("Elements"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button(t("Done")) { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Elements")
	}

	@ViewBuilder private var emptyView: some View {
		if #available(iOS 17, *) {
			ContentUnavailableView(
				t("No Elements"),
				systemImage: "list.bullet.indent",
				description: Text(t("Headings, images, and other elements will appear here."))
			)
		} else {
			Text(t("No Elements"))
				.foregroundStyle(.secondary)
				.frame(maxWidth: .infinity, maxHeight: .infinity)
		}
	}
}
