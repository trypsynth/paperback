import SwiftUI

struct ElementsView: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var tab = 0

	var body: some View {
		Group {
			if let session = viewModel.activeSession {
				let headings = session.getHeadingTreeFfi(position: viewModel.reading.ttsPosition)
				let links = session.getLinkListFfi(position: viewModel.reading.ttsPosition)
				if headings.items.isEmpty && links.items.isEmpty {
					emptyView
				} else {
					VStack(spacing: 0) {
						// TRANSLATORS: Label for the wheel picker choosing between the Headings and Links tabs
						Picker(t("Type"), selection: $tab) {
							// TRANSLATORS: Tab showing the document's list of headings
							Text(t("Headings")).tag(0)
							// TRANSLATORS: Tab showing the document's list of links
							Text(t("Links")).tag(1)
						}
						.pickerStyle(.wheel)
						.labelsHidden()
						.frame(height: 120)
						if tab == 0 {
							List(headings.items, id: \.offset) { item in
								Button {
									viewModel.reading.goToPosition(item.offset)
									dismiss()
								} label: {
									Text(item.text)
								}
							}
						} else {
							List(links.items, id: \.offset) { item in
								Button {
									viewModel.reading.goToPosition(item.offset)
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
		// TRANSLATORS: Navigation title of the Elements screen (lists the document's headings and links)
		.navigationTitle(t("Elements"))
		.navigationBarTitleDisplayMode(.inline)
		.sheetAccessibilityFocus(title: "Elements")
	}

	@ViewBuilder private var emptyView: some View {
		ContentUnavailableView(
			// TRANSLATORS: Title shown when a document has no headings or links to list in the Elements sheet
			t("No Elements"),
			systemImage: "list.bullet.indent",
			// TRANSLATORS: Description shown under the "No Elements" title explaining what would appear here
			description: Text(t("Headings, images, and other elements will appear here."))
		)
	}
}
