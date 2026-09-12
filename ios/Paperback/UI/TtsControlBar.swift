import SwiftUI

struct TtsControlBar: View {
	@Environment(AppViewModel.self) private var viewModel

	// Find reads as "Find Previous"/"Find Next", matching the Find screen's own buttons, rather
	// than "Previous Find"/"Next Find".
	private var prevLabel: String {
		if viewModel.reading.currentNavUnit == .find {
			// TRANSLATORS: Accessibility label for the read-aloud bar's previous button when navigating by Find matches
			return t("Find Previous")
		}
		// TRANSLATORS: Accessibility label for the "previous unit" button; {} is the current navigation unit name, e.g. "Previous Paragraph"
		return t("Previous {}").replacingOccurrences(of: "{}", with: viewModel.reading.currentNavUnit.name)
	}

	private var nextLabel: String {
		if viewModel.reading.currentNavUnit == .find {
			// TRANSLATORS: Accessibility label for the read-aloud bar's next button when navigating by Find matches
			return t("Find Next")
		}
		// TRANSLATORS: Accessibility label for the "next unit" button; {} is the current navigation unit name, e.g. "Next Paragraph"
		return t("Next {}").replacingOccurrences(of: "{}", with: viewModel.reading.currentNavUnit.name)
	}

	var body: some View {
		HStack(spacing: 0) {
			Menu {
				ForEach(viewModel.reading.availableNavUnits, id: \.self) { unit in
					Button {
						viewModel.reading.changeNavUnit(unit)
					} label: {
						if unit == viewModel.reading.currentNavUnit {
							Label(unit.name, systemImage: "checkmark")
						} else {
							Text(unit.name)
						}
					}
				}
			} label: {
				Text(viewModel.reading.currentNavUnit.name)
					.font(.caption)
					.foregroundStyle(.secondary)
					.frame(width: 72, alignment: .leading)
			}
			// TRANSLATORS: Accessibility label for the control that picks which unit (sentence, paragraph, etc.) prev/next buttons navigate by
			.accessibilityLabel(t("Navigation unit"))
			// TRANSLATORS: VoiceOver accessibility value announcing the currently selected navigation unit for the control above
			.accessibilityValue(viewModel.reading.currentNavUnit.name)
			.accessibilityRemoveTraits(.isButton)
			.accessibilityAdjustableAction { direction in
				let units = viewModel.reading.availableNavUnits
				guard let idx = units.firstIndex(of: viewModel.reading.currentNavUnit) else { return }
				switch direction {
				case .increment:
					viewModel.reading.changeNavUnit(units[(idx + 1) % units.count])
				case .decrement:
					viewModel.reading.changeNavUnit(units[(idx - 1 + units.count) % units.count])
				@unknown default: break
				}
			}
			.padding(.leading, 16)

			Button { viewModel.reading.playPrevSegment(speak: viewModel.reading.ttsManager.isSpeaking) } label: {
				Image(systemName: "backward.fill").font(.title2)
			}
			.accessibilityLabel(prevLabel)
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
			.accessibilityLabel(nextLabel)
			.frame(maxWidth: .infinity, minHeight: 64)
			.contentShape(Rectangle())

			// Balance the segment picker on the left
			Color.clear.frame(width: 72)
				.padding(.trailing, 16)
		}
		.padding(.vertical, 4)
	}
}
