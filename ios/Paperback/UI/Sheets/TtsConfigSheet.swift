import SwiftUI
import AVFoundation

struct VoicePickerView: View {
	@ObservedObject var ttsManager: TtsManager
	let onSelect: (String?) -> Void
	@Environment(\.dismiss) private var dismiss

	private var sections: [VoiceSection] { buildSections(from: ttsManager.availableVoices) }

	var body: some View {
		List {
			Section {
				voiceRow(label: t("Default"), identifier: nil)
			}
			ForEach(sections) { section in
				Section(languageLabel(section.language)) {
					ForEach(section.items) { item in
						voiceRow(label: item.label, identifier: item.identifier)
					}
				}
			}
		}
		.navigationTitle(t("Voice"))
		.navigationBarTitleDisplayMode(.inline)
	}

	private func voiceRow(label: String, identifier: String?) -> some View {
		let isSelected = ttsManager.selectedVoiceIdentifier == identifier
		return Button {
			onSelect(identifier)
			dismiss()
		} label: {
			HStack {
				Text(label).foregroundStyle(.primary)
				if isSelected {
					Spacer()
					Image(systemName: "checkmark").foregroundStyle(Color.accentColor)
				}
			}
		}
		.accessibilityAddTraits(isSelected ? .isSelected : [])
	}
}
