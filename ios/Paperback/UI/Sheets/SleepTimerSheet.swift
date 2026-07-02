import SwiftUI

struct SleepTimerSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	private let presets = [5, 10, 15, 30, 45, 60]

	var body: some View {
		NavigationStack {
			List {
				if let remaining = viewModel.sleepTimerRemaining {
					Section(t("Active")) {
						LabeledContent(
							t("Time remaining"),
							value: String(format: "%d:%02d", remaining / 60, remaining % 60)
						)
						Button(role: .destructive) {
							viewModel.cancelSleepTimer()
							dismiss()
						} label: {
							Label(t("Cancel Timer"), systemImage: "timer.slash")
						}
					}
				}
				Section(t("Set timer")) {
					ForEach(presets, id: \.self) { minutes in
						Button("\(minutes) minutes") {
							viewModel.setSleepTimer(seconds: minutes * 60)
							dismiss()
						}
					}
				}
			}
			.navigationTitle(t("Sleep Timer"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Sleep Timer")
	}
}
