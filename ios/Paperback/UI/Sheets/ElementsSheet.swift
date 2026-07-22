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
							// TRANSLATORS: Label for the segmented control choosing between the Headings and Links tabs
							Picker(t("Type"), selection: $tab) {
								// TRANSLATORS: Tab showing the document's list of headings
								Text(t("Headings")).tag(0)
								// TRANSLATORS: Tab showing the document's list of links
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
			// TRANSLATORS: Navigation title of the Elements sheet (lists the document's headings and links)
			.navigationTitle(t("Elements"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that dismisses the Elements sheet
					Button(t("Done")) { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Elements")
	}

	@ViewBuilder private var emptyView: some View {
		if #available(iOS 17, *) {
			ContentUnavailableView(
				// TRANSLATORS: Title shown when a document has no headings or links to list in the Elements sheet
				t("No Elements"),
				systemImage: "list.bullet.indent",
				// TRANSLATORS: Description shown under the "No Elements" title explaining what would appear here
				description: Text(t("Headings, images, and other elements will appear here."))
			)
		} else {
			// TRANSLATORS: Title shown when a document has no headings or links to list in the Elements sheet
			Text(t("No Elements"))
				.foregroundStyle(.secondary)
				.frame(maxWidth: .infinity, maxHeight: .infinity)
		}
	}
}
