import SwiftUI

struct SleepTimerSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	private let presets = [5, 10, 15, 30, 45, 60]

	var body: some View {
		NavigationStack {
			List {
				if let remaining = viewModel.sleepTimerRemaining {
					// TRANSLATORS: Section header shown while a sleep timer is currently running
					Section(t("Active")) {
						LabeledContent(
							// TRANSLATORS: Label for the countdown showing how much time is left on the sleep timer
							t("Time remaining"),
							value: String(format: "%d:%02d", remaining / 60, remaining % 60)
						)
						Button(role: .destructive) {
							viewModel.cancelSleepTimer()
							dismiss()
						} label: {
							// TRANSLATORS: Button to cancel the currently running sleep timer
							Label(t("Cancel Timer"), systemImage: "timer.slash")
						}
					}
				}
				// TRANSLATORS: Section header listing the sleep timer duration presets to choose from
				Section(t("Set timer")) {
					ForEach(presets, id: \.self) { minutes in
						Button("\(minutes) minutes") {
							viewModel.setSleepTimer(seconds: minutes * 60)
							dismiss()
						}
					}
				}
			}
			// TRANSLATORS: Navigation bar title of the sleep timer sheet
			.navigationTitle(t("Sleep Timer"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button to close the sleep timer sheet without changing anything
					Button(t("Cancel")) { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "Sleep Timer")
	}
}
