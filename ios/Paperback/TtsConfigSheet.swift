import SwiftUI
import AVFoundation

private struct VoicePickerView: View {
	@ObservedObject var ttsManager: TtsManager
	let onSelect: () -> Void

	var body: some View {
		List {
			voiceRow(name: "Default", identifier: nil)
			ForEach(ttsManager.availableVoices, id: \.identifier) { voice in
				voiceRow(name: "\(voice.name) (\(voice.language))", identifier: voice.identifier)
			}
		}
		.navigationTitle("Voice")
		.navigationBarTitleDisplayMode(.inline)
	}

	private func voiceRow(name: String, identifier: String?) -> some View {
		let isSelected = ttsManager.selectedVoiceIdentifier == identifier
		return Button {
			ttsManager.selectedVoiceIdentifier = identifier
			onSelect()
		} label: {
			HStack {
				Text(name)
					.foregroundStyle(Color.primary)
				if isSelected {
					Spacer()
					Image(systemName: "checkmark")
						.foregroundStyle(Color.accentColor)
				}
			}
		}
		.accessibilityAddTraits(isSelected ? .isSelected : [])
	}
}

struct TtsConfigSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var path = NavigationPath()

	private var selectedVoiceName: String {
		guard let id = viewModel.ttsManager.selectedVoiceIdentifier,
		      let voice = viewModel.ttsManager.availableVoices.first(where: { $0.identifier == id })
		else { return "Default" }
		return voice.name
	}

	var body: some View {
		NavigationStack(path: $path) {
			Form {
				Section {
					NavigationLink(value: "voice") {
						HStack {
							Text("Voice")
							Spacer()
							Text(selectedVoiceName)
								.foregroundStyle(Color.secondary)
								.lineLimit(1)
						}
					}
				}
				Section {
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
			.navigationDestination(for: String.self) { _ in
				VoicePickerView(ttsManager: viewModel.ttsManager) {
					path.removeLast()
				}
			}
		}
		.sheetAccessibilityFocus(title: "TTS Settings")
	}
}
