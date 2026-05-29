import SwiftUI

struct SleepTimerSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	private let presets = [5, 10, 15, 30, 45, 60]

	var body: some View {
		NavigationStack {
			List {
				if let remaining = viewModel.sleepTimerRemaining {
					Section("Active") {
						LabeledContent(
							"Time remaining",
							value: String(format: "%d:%02d", remaining / 60, remaining % 60)
						)
						Button(role: .destructive) {
							viewModel.cancelSleepTimer()
							dismiss()
						} label: {
							Label("Cancel Timer", systemImage: "timer.slash")
						}
					}
				}
				Section("Set timer") {
					ForEach(presets, id: \.self) { minutes in
						Button("\(minutes) minutes") {
							viewModel.setSleepTimer(seconds: minutes * 60)
							dismiss()
						}
					}
				}
			}
			.navigationTitle("Sleep Timer")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button("Cancel") { dismiss() }
				}
			}
		}
	}
}
