import SwiftUI

/// A speech percentage shown as a menu of presets. A screen reader reads it as one adjustable
/// value: swipe up or down moves it by `speechPercentStep`, double-tap opens the presets.
struct SpeechPercentMenu<MenuLabel: View>: View {
	let accessibilityLabel: String
	@Binding var percent: Int
	let presets: [Int]
	@ViewBuilder let label: () -> MenuLabel

	var body: some View {
		Menu {
			ForEach(presets, id: \.self) { preset in
				Button {
					percent = preset
				} label: {
					if preset == percent {
						Label("\(preset)%", systemImage: "checkmark")
					} else {
						Text("\(preset)%")
					}
				}
			}
		} label: {
			label()
		}
		.accessibilityLabel(accessibilityLabel)
		.accessibilityValue("\(percent)%")
		.accessibilityRemoveTraits(.isButton)
		.accessibilityAdjustableAction { direction in
			switch direction {
			case .increment: percent += speechPercentStep
			case .decrement: percent -= speechPercentStep
			@unknown default: break
			}
		}
	}
}
