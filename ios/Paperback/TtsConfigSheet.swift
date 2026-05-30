import SwiftUI
import AVFoundation

private struct VoicePickerView: View {
	@ObservedObject var ttsManager: TtsManager
	let onSelect: (String?) -> Void

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
			onSelect(identifier)
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

private struct TtsConfigForm: View {
	@ObservedObject var ttsManager: TtsManager
	@Binding var path: NavigationPath
	let onVoiceSelect: (String?) -> Void
	let onDone: () -> Void

	private var selectedVoiceName: String {
		guard let id = ttsManager.selectedVoiceIdentifier,
		      let voice = ttsManager.availableVoices.first(where: { $0.identifier == id })
		else { return "Default" }
		return voice.name
	}

	var body: some View {
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
				VStack(alignment: .leading) {
					Text("Rate")
					Slider(value: $ttsManager.speechRate,
					       in: AVSpeechUtteranceMinimumSpeechRate...AVSpeechUtteranceMaximumSpeechRate)
						.accessibilityLabel("Speech rate")
				}
				VStack(alignment: .leading) {
					Text("Pitch")
					Slider(value: $ttsManager.pitch, in: 0.5...2.0)
						.accessibilityLabel("Pitch")
				}
			}
		}
		.navigationTitle("TTS Settings")
		.navigationBarTitleDisplayMode(.inline)
		.toolbar {
			ToolbarItem(placement: .confirmationAction) {
				Button("Done") { onDone() }
			}
		}
		.navigationDestination(for: String.self) { _ in
			VoicePickerView(ttsManager: ttsManager, onSelect: onVoiceSelect)
		}
	}
}

struct TtsConfigSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var path = NavigationPath()

	var body: some View {
		NavigationStack(path: $path) {
			TtsConfigForm(
				ttsManager: viewModel.ttsManager,
				path: $path,
				onVoiceSelect: { identifier in
					let wasPlaying = viewModel.ttsManager.isSpeaking
					let wasPaused = viewModel.ttsManager.isPaused
					viewModel.ttsManager.selectedVoiceIdentifier = identifier
					path.removeLast()
					if wasPlaying {
						viewModel.ttsManager.stop()
						viewModel.playCurrentSegment()
					} else if wasPaused {
						viewModel.ttsManager.stop()
					}
				},
				onDone: { dismiss() }
			)
		}
		.sheetAccessibilityFocus(title: "TTS Settings")
	}
}
