import SwiftUI

struct TtsControlBar: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		HStack(spacing: 0) {
			Menu {
				ForEach(SegmentType.allCases, id: \.self) { type in
					Button {
						viewModel.reading.changeSegmentType(type)
					} label: {
						// TRANSLATORS: Name of a navigation/reading unit shown in the "jump by unit" picker (e.g. "Paragraph", "Line", "Heading", "Section")
						if type == viewModel.reading.currentSegmentType {
							Label(t(type.rawValue), systemImage: "checkmark")
						} else {
							Text(t(type.rawValue))
						}
					}
				}
			} label: {
				Text(t(viewModel.reading.currentSegmentType.rawValue))
					.font(.caption)
					.foregroundStyle(.secondary)
					.frame(width: 72, alignment: .leading)
			}
			// TRANSLATORS: Accessibility label for the control that picks which unit (sentence, paragraph, etc.) prev/next buttons navigate by
			.accessibilityLabel(t("Navigation unit"))
			.accessibilityValue(t(viewModel.reading.currentSegmentType.rawValue))
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAdjustableAction { direction in
				let types = SegmentType.allCases
				guard let idx = types.firstIndex(of: viewModel.reading.currentSegmentType) else { return }
				switch direction {
				case .increment:
					viewModel.reading.changeSegmentType(types[(idx + 1) % types.count])
				case .decrement:
					viewModel.reading.changeSegmentType(types[(idx - 1 + types.count) % types.count])
				@unknown default: break
				}
			}
			.padding(.leading, 16)

			Button { viewModel.reading.playPrevSegment(speak: viewModel.reading.ttsManager.isSpeaking) } label: {
				Image(systemName: "backward.fill").font(.title2)
			}
			// TRANSLATORS: Accessibility label for the "previous unit" button; {} is the current navigation unit name, e.g. "Previous Paragraph"
			.accessibilityLabel(t("Previous {}").replacingOccurrences(of: "{}", with: t(viewModel.reading.currentSegmentType.rawValue)))
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			Button { viewModel.reading.togglePlayPause() } label: {
				Image(systemName: viewModel.reading.ttsManager.isSpeaking ? "pause.fill" : "play.fill").font(.title)
			}
			// TRANSLATORS: Accessibility label for the play/pause button, which toggles between these two states
			.accessibilityLabel(viewModel.reading.ttsManager.isSpeaking ? t("Pause") : t("Play"))
			.accessibilityAdjustableAction { direction in
				let wasPlaying = viewModel.reading.ttsManager.isSpeaking
				let forward = viewModel.swipeUpMovesForward
				let tryingNext: Bool
				switch direction {
				case .increment: tryingNext = forward
				case .decrement: tryingNext = !forward
				@unknown default: return
				}
				let moved = tryingNext
					? viewModel.reading.playNextSegment(speak: wasPlaying, announce: !wasPlaying)
					: viewModel.reading.playPrevSegment(speak: wasPlaying, announce: !wasPlaying)
				if !moved {
					// TRANSLATORS: Accessibility announcement spoken when trying to navigate past the start or end of the document
					let label = tryingNext ? t("End of document") : t("Beginning of document")
					Task { @MainActor in
						try? await Task.sleep(for: .milliseconds(150))
						UIAccessibility.post(notification: .announcement, argument: label)
					}
				}
			}
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			Button { viewModel.reading.playNextSegment(speak: viewModel.reading.ttsManager.isSpeaking) } label: {
				Image(systemName: "forward.fill").font(.title2)
			}
			// TRANSLATORS: Accessibility label for the "next unit" button; {} is the current navigation unit name, e.g. "Next Paragraph"
			.accessibilityLabel(t("Next {}").replacingOccurrences(of: "{}", with: t(viewModel.reading.currentSegmentType.rawValue)))
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			// Balance the segment picker on the left
			Color.clear.frame(width: 72)
				.padding(.trailing, 16)
		}
		.padding(.vertical, 4)
	}
}
