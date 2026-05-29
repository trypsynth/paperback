import SwiftUI

struct TtsModeView: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		VStack(spacing: 16) {
			Spacer()
			ScrollView {
				Text(
					viewModel.currentSegmentText.isEmpty
						? "Press play to start listening."
						: viewModel.currentSegmentText
				)
				.font(.body)
				.multilineTextAlignment(.leading)
				.padding(24)
				.frame(maxWidth: .infinity, alignment: .leading)
			}
			.frame(maxHeight: 400)
			if let remaining = viewModel.sleepTimerRemaining {
				Text(String(format: "Sleep timer: %d:%02d", remaining / 60, remaining % 60))
					.font(.caption)
					.foregroundStyle(.secondary)
			}
			Spacer()
		}
	}
}
