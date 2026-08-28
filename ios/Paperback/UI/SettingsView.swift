import SwiftUI
import AVFoundation

private let sampleText = "This is a sample of the current voice and speed settings."

private struct TtsSettingsSection<VoiceDestination: View>: View {
	@Bindable var ttsManager: TtsManager
	let onPlaySample: () -> Void
	@ViewBuilder let voiceDestination: () -> VoiceDestination

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
		// TRANSLATORS: Section header in Settings grouping text-to-speech voice/rate/pitch controls
		Section(t("Text to Speech")) {
			NavigationLink {
				voiceDestination()
			} label: {
				HStack {
					// TRANSLATORS: Row label for the current TTS voice, navigates to the voice picker
					Text(t("Voice"))
					Spacer()
					Text(selectedVoiceName)
						.foregroundStyle(.secondary)
						.lineLimit(1)
				}
			}
			VStack(alignment: .leading, spacing: 4) {
				HStack {
					// TRANSLATORS: Label above the speech rate slider (visual label; the slider itself has its own accessibility label)
					Text(t("Rate")).accessibilityHidden(true)
					Spacer()
					Text("\(ratePercent)%")
						.foregroundStyle(.secondary)
						.monospacedDigit()
						.accessibilityHidden(true)
				}
				Slider(
					value: $ttsManager.speechRate,
					in: AVSpeechUtteranceMinimumSpeechRate...AVSpeechUtteranceMaximumSpeechRate,
					step: (AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate) / 100
				)
				// TRANSLATORS: VoiceOver accessibility label for the speech rate slider
				.accessibilityLabel(t("Speech Rate"))
				.accessibilityValue("\(ratePercent)%")
			}
			VStack(alignment: .leading, spacing: 4) {
				HStack {
					// TRANSLATORS: Label above the speech pitch slider (visual label; the slider itself has its own accessibility label)
					Text(t("Pitch")).accessibilityHidden(true)
					Spacer()
					Text("\(pitchPercent)%")
						.foregroundStyle(.secondary)
						.monospacedDigit()
						.accessibilityHidden(true)
				}
				Slider(value: $ttsManager.pitch, in: 0.5...2.0, step: 0.015)
					.accessibilityLabel(t("Pitch"))
					.accessibilityValue("\(pitchPercent)%")
			}
			NavigationLink {
				SpeechDictionaryView()
			} label: {
				// TRANSLATORS: Row label navigating to the custom speech pronunciation dictionary
				Text(t("Speech Dictionary"))
			}
			Button(action: onPlaySample) {
				// TRANSLATORS: Button that reads a sample sentence aloud using the current voice/rate/pitch settings
				Label(t("Play Sample"), systemImage: "play.circle")
			}
		}
	}
}

struct SettingsView: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		Form {
			// TRANSLATORS: Section header in Settings grouping general app behavior toggles
			Section(t("Behavior")) {
				// TRANSLATORS: Toggle to reopen previously open documents on next launch
				Toggle(t("Restore last open documents"), isOn: Binding(
					get: { viewModel.restorePreviousDocuments },
					set: { viewModel.restorePreviousDocuments = $0 }
				))
				// TRANSLATORS: Toggle controlling whether an upward swipe advances (vs. reverses) navigation
				Toggle(t("Swipe up moves forward"), isOn: Binding(
					get: { viewModel.swipeUpMovesForward },
					set: { viewModel.swipeUpMovesForward = $0 }
				))
			}
			TtsSettingsSection(
				ttsManager: viewModel.reading.ttsManager,
				onPlaySample: { viewModel.reading.ttsManager.speakSample(sampleText) },
				voiceDestination: { voicePicker }
			)
		}
		// TRANSLATORS: Navigation bar title of the Settings screen
		.navigationTitle(t("Settings"))
		.navigationBarTitleDisplayMode(.inline)
	}

	private var voicePicker: some View {
		VoicePickerView(ttsManager: viewModel.reading.ttsManager) { identifier in
			let wasPlaying = viewModel.reading.ttsManager.isSpeaking
			let wasPaused = viewModel.reading.ttsManager.isPaused
			viewModel.reading.ttsManager.selectedVoiceIdentifier = identifier
			if wasPlaying {
				viewModel.reading.ttsManager.stop()
				viewModel.reading.playCurrentSegment()
			} else if wasPaused {
				viewModel.reading.ttsManager.stop()
			}
		}
	}
}
