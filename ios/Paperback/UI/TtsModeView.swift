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
					// The text above is the real content. This only says where in the document
					// it sits, so VoiceOver reading both would say the same words twice.
					.accessibilityHidden(true)
			}
			if let remaining = viewModel.reading.sleepTimerRemaining {
				Text(String(format: "Sleep timer: %d:%02d", remaining / 60, remaining % 60))
					.font(.caption)
					.foregroundStyle(.secondary)
			}
			Spacer()
		}
	}

	/// The line under the caret, shown small beneath the spoken text as a place marker.
	///
	/// Left out when the spoken text already contains it, which is what happens whenever a
	/// paragraph is one line long: the same words would otherwise appear twice on screen, once
	/// full size and once again underneath.
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
