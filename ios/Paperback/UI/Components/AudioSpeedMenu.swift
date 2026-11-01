import SwiftUI

/// The reading bar's speed control on a book with a recording, which changes the recording's speed rather than the voice's.
struct AudioSpeedMenu: View {
	let narration: RecordedNarration

	var body: some View {
		Menu {
			ForEach(audioSpeedPresets, id: \.self) { preset in
				Button { narration.setSpeed(preset) } label: {
					if preset == narration.speed {
						Label(audioSpeedLabel(preset), systemImage: "checkmark")
					} else {
						Text(audioSpeedLabel(preset))
					}
				}
			}
		} label: {
			Text(audioSpeedLabel(narration.speed))
				.font(.caption)
				.foregroundStyle(.secondary)
				.monospacedDigit()
				.frame(width: 72, alignment: .trailing)
		}
		// TRANSLATORS: VoiceOver label for the read-aloud bar's control that speeds up or slows down a book's recorded audio
		.accessibilityLabel(t("Audio Speed"))
		.accessibilityValue(audioSpeedLabel(narration.speed))
		.accessibilityRemoveTraits(.isButton)
		.accessibilityAdjustableAction { direction in
			switch direction {
			case .increment: narration.setSpeed(narration.speed + audioSpeedStep)
			case .decrement: narration.setSpeed(narration.speed - audioSpeedStep)
			@unknown default: break
			}
		}
	}
}
