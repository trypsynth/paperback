import SwiftUI

struct TocView: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		Group {
			if let session = viewModel.activeSession {
				let entries = session.getToc()
				if entries.isEmpty {
					emptyView
				} else {
					let activePos = activePosition(in: entries)
					ScrollViewReader { proxy in
						List(entries, id: \.position) { entry in
							let isActive = entry.position == activePos
							Button {
								viewModel.goToPosition(entry.position)
								dismiss()
							} label: {
								HStack {
									Text(entry.title)
										.padding(.leading, CGFloat(max(0, entry.level - 1)) * 16)
										.fontWeight(isActive ? .semibold : .regular)
										.foregroundStyle(isActive ? Color.accentColor : Color.primary)
									Spacer()
								}
							}
							.accessibilityAddTraits(isActive ? .isSelected : [])
							.id(entry.position)
						}
						.onAppear {
							if let pos = activePos {
								proxy.scrollTo(pos, anchor: .center)
							}
						}
					}
				}
			} else {
				emptyView
			}
		}
		// TRANSLATORS: Navigation title of the table of contents screen
		.navigationTitle(t("Contents"))
		.navigationBarTitleDisplayMode(.inline)
		.sheetAccessibilityFocus(title: "Contents")
	}

	// The active entry is the last one whose position is at or before the current position.
	private func activePosition(in entries: [TocEntry]) -> Int64? {
		entries.last(where: { $0.position <= viewModel.ttsPosition })?.position
	}

	@ViewBuilder private var emptyView: some View {
		ContentUnavailableView(
			// TRANSLATORS: Title shown when the current document has no table of contents
			t("No Table of Contents"),
			systemImage: "list.bullet",
			// TRANSLATORS: Description shown below the "No Table of Contents" title
			description: Text(t("This document has no table of contents."))
		)
	}
}
