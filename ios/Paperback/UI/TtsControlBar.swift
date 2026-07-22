import SwiftUI

struct TtsControlBar: View {
	@EnvironmentObject var viewModel: AppViewModel

	var body: some View {
		HStack(spacing: 0) {
			Menu {
				ForEach(SegmentType.allCases, id: \.self) { type in
					Button {
						viewModel.changeSegmentType(type)
					} label: {
						if type == viewModel.currentSegmentType {
							Label(type.rawValue, systemImage: "checkmark")
						} else {
							Text(type.rawValue)
						}
					}
				}
			} label: {
				Text(viewModel.currentSegmentType.rawValue)
					.font(.caption)
					.foregroundStyle(.secondary)
					.frame(width: 72, alignment: .leading)
			}
			// TRANSLATORS: Accessibility label for the control that picks which unit (sentence, paragraph, etc.) prev/next buttons navigate by
			.accessibilityLabel(t("Navigation unit"))
			.accessibilityValue(viewModel.currentSegmentType.rawValue)
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAdjustableAction { direction in
				let types = SegmentType.allCases
				guard let idx = types.firstIndex(of: viewModel.currentSegmentType) else { return }
				switch direction {
				case .increment:
					viewModel.changeSegmentType(types[(idx + 1) % types.count])
				case .decrement:
					viewModel.changeSegmentType(types[(idx - 1 + types.count) % types.count])
				@unknown default: break
				}
			}
			.padding(.leading, 16)

			Button { viewModel.playPrevSegment(speak: viewModel.ttsManager.isSpeaking) } label: {
				Image(systemName: "backward.fill").font(.title2)
			}
			.accessibilityLabel("Previous \(viewModel.currentSegmentType.rawValue)")
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			Button { viewModel.togglePlayPause() } label: {
				Image(systemName: viewModel.ttsManager.isSpeaking ? "pause.fill" : "play.fill").font(.title)
			}
			// TRANSLATORS: Accessibility label for the play/pause button, which toggles between these two states
			.accessibilityLabel(viewModel.ttsManager.isSpeaking ? t("Pause") : t("Play"))
			.accessibilityAdjustableAction { direction in
				let wasPlaying = viewModel.ttsManager.isSpeaking
				let forward = viewModel.swipeUpMovesForward
				let tryingNext: Bool
				switch direction {
				case .increment: tryingNext = forward
				case .decrement: tryingNext = !forward
				@unknown default: return
				}
				let moved = tryingNext
					? viewModel.playNextSegment(speak: wasPlaying, announce: !wasPlaying)
					: viewModel.playPrevSegment(speak: wasPlaying, announce: !wasPlaying)
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

			Button { viewModel.playNextSegment(speak: viewModel.ttsManager.isSpeaking) } label: {
				Image(systemName: "forward.fill").font(.title2)
			}
			.accessibilityLabel("Next \(viewModel.currentSegmentType.rawValue)")
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			// Balance the segment picker on the left
			Color.clear.frame(width: 72)
				.padding(.trailing, 16)
		}
		.padding(.vertical, 4)
	}
}
