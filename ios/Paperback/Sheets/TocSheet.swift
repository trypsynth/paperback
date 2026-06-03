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
			.navigationTitle("Contents")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Contents")
	}

	// The active entry is the last one whose position is at or before the current position.
	private func activePosition(in entries: [TocEntry]) -> Int64? {
		entries.last(where: { $0.position <= viewModel.ttsPosition })?.position
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
