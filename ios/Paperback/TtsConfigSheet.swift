import SwiftUI
import AVFoundation

private let sampleText = "This is a sample of the current voice and speed settings."

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
	let onPlaySample: () -> Void
	let onDone: () -> Void

	private var selectedVoiceName: String {
		guard let id = ttsManager.selectedVoiceIdentifier,
		      let voice = ttsManager.availableVoices.first(where: { $0.identifier == id })
		else { return "Default" }
		return voice.name
	}

	private var ratePercent: Int {
		let range = AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate
		return Int(((ttsManager.speechRate - AVSpeechUtteranceMinimumSpeechRate) / range * 100).rounded())
	}

	private var pitchPercent: Int {
		Int(((ttsManager.pitch - 0.5) / 1.5 * 100).rounded())
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
				Slider(
					value: $ttsManager.speechRate,
					in: AVSpeechUtteranceMinimumSpeechRate...AVSpeechUtteranceMaximumSpeechRate,
					step: (AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate) / 100
				)
				.accessibilityLabel("Speech Rate")
				.accessibilityValue("\(ratePercent) percent")
				Slider(value: $ttsManager.pitch, in: 0.5...2.0, step: 0.015)
					.accessibilityLabel("Pitch")
					.accessibilityValue("\(pitchPercent) percent")
			}
			Section {
				Button(action: onPlaySample) {
					Label("Play Sample", systemImage: "play.circle")
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
				onPlaySample: {
					viewModel.ttsManager.speakSample(sampleText)
				},
				onDone: { dismiss() }
			)
		}
		.sheetAccessibilityFocus(title: "TTS Settings")
	}
}
