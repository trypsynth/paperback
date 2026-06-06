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
					.frame(minWidth: 72, alignment: .leading)
			}
			.accessibilityLabel("Navigation unit")
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
			.padding(.leading, 20)

			Spacer()

			Button { viewModel.playPrevSegment(speak: viewModel.ttsManager.isSpeaking) } label: {
				Image(systemName: "backward.fill")
					.font(.title2)
			}
			.accessibilityLabel("Previous \(viewModel.currentSegmentType.rawValue)")
			.padding(.horizontal, 20)

			Button { viewModel.togglePlayPause() } label: {
				Image(systemName: viewModel.ttsManager.isSpeaking ? "pause.fill" : "play.fill")
					.font(.title)
					.frame(width: 44)
			}
			.accessibilityLabel(viewModel.ttsManager.isSpeaking ? "Pause" : "Play")
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
					let label = tryingNext ? "End of document" : "Beginning of document"
					Task { @MainActor in
						try? await Task.sleep(for: .milliseconds(150))
						UIAccessibility.post(notification: .announcement, argument: label)
					}
				}
			}

			Button { viewModel.playNextSegment(speak: viewModel.ttsManager.isSpeaking) } label: {
				Image(systemName: "forward.fill")
					.font(.title2)
			}
			.accessibilityLabel("Next \(viewModel.currentSegmentType.rawValue)")
			.padding(.horizontal, 20)

			Spacer()

			// Balance the segment picker on the left
			Color.clear.frame(width: 72)
				.padding(.trailing, 20)
		}
		.padding(.vertical, 12)
	}
}
