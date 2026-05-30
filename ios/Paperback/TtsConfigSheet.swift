import SwiftUI
import AVFoundation

struct TtsConfigSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss

	var body: some View {
		NavigationStack {
			Form {
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
				Section("Voice") {
					Picker("Voice", selection: Binding(
						get: { viewModel.ttsManager.selectedVoice },
						set: { viewModel.ttsManager.selectedVoice = $0 }
					)) {
						Text("Default").tag(nil as AVSpeechSynthesisVoice?)
						ForEach(viewModel.ttsManager.availableVoices, id: \.identifier) { voice in
							Text("\(voice.name) (\(voice.language))")
								.tag(voice as AVSpeechSynthesisVoice?)
						}
					}
					.pickerStyle(.navigationLink)
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
		.sheetAccessibilityFocus()
	}
}
