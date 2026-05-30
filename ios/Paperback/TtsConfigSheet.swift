import SwiftUI
import AVFoundation

struct TtsConfigSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Form {
				Section {
					Picker("Voice", selection: Binding(
						get: { viewModel.ttsManager.selectedVoiceIdentifier },
						set: { viewModel.ttsManager.selectedVoiceIdentifier = $0 }
					)) {
						Text("Default").tag(nil as String?)
						ForEach(viewModel.ttsManager.availableVoices, id: \.identifier) { voice in
							Text("\(voice.name) (\(voice.language))")
								.tag(voice.identifier as String?)
						}
					}
					.pickerStyle(.navigationLink)
				}
				Section("Playback") {
					LabeledContent("Rate") {
						Slider(
							value: Binding(
								get: { viewModel.ttsManager.speechRate },
								set: { viewModel.ttsManager.speechRate = $0 }
							),
							in: AVSpeechUtteranceMinimumSpeechRate...AVSpeechUtteranceMaximumSpeechRate
						)
					}
					LabeledContent("Pitch") {
						Slider(
							value: Binding(
								get: { viewModel.ttsManager.pitch },
								set: { viewModel.ttsManager.pitch = $0 }
							),
							in: 0.5...2.0
						)
					}
				}
			}
			.navigationTitle("TTS Settings")
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					Button("Done") { dismiss() }
				}
			}
		}
		.sheetAccessibilityFocus(title: "TTS Settings")
	}
}
