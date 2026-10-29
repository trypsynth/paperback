import SwiftUI

struct TtsModeView: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		VStack(spacing: 16) {
			Spacer()
			ScrollView {
				Text(
					viewModel.reading.currentSegmentText.isEmpty
						// TRANSLATORS: Placeholder shown in the read-aloud view before the user has started playback
						? t("Press play to start listening.")
						: viewModel.reading.currentSegmentText
				)
				.readabilityStyle(viewModel)
				.padding(.horizontal, 24)
			}
			.frame(maxHeight: 400)
			.readingBackground(viewModel)
			if let lineText = positionLine {
				Text(lineText)
					.font(.caption)
					.foregroundStyle(.secondary)
					.lineLimit(1)
					.padding(.horizontal, 24)
					// The text above is the real content, so VoiceOver reading both would say the same words twice.
					.accessibilityHidden(true)
			}
			// A recording moves on between the clip changes the view is told about, so its share is read again every second while it plays.
			TimelineView(.periodic(from: .now, by: 1)) { _ in
				if let percent = viewModel.reading.progressPercent() {
					VStack(spacing: 16) {
						// The text below says the same number, so the bar is for the eye only.
						ProgressView(value: Double(percent), total: 100)
							.padding(.horizontal, 24)
							.accessibilityHidden(true)
						// TRANSLATORS: How far through the document the reader has got, shown under the read-aloud progress bar; {} is a whole-number percentage
						Text(t("{}% through").replacingOccurrences(of: "{}", with: "\(percent)"))
							.font(.caption)
							.foregroundStyle(.secondary)
					}
				}
			}
			if let remaining = viewModel.reading.sleepTimerRemaining {
				Text(String(format: "Sleep timer: %d:%02d", remaining / 60, remaining % 60))
					.font(.caption)
					.foregroundStyle(.secondary)
			}
			Spacer()
		}
	}

	/// Nil when the spoken text already contains it, as in a one-line paragraph, so the same words don't appear twice.
	private var positionLine: String? {
		guard let session = viewModel.activeSession else { return nil }
		let line = session.getLineText(position: viewModel.reading.ttsPosition)
			.trimmingCharacters(in: .whitespacesAndNewlines)
		if line.isEmpty || viewModel.reading.currentSegmentText.contains(line) {
			return nil
		}
		return line
	}
}
