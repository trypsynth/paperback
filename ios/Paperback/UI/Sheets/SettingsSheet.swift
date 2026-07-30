import SwiftUI
import AVFoundation

private let sampleText = "This is a sample of the current voice and speed settings."

private struct TtsSettingsSection: View {
	@ObservedObject var ttsManager: TtsManager
	let onPlaySample: () -> Void

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
			NavigationLink(value: "voice") {
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
			NavigationLink(value: "speechDictionary") {
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

struct SettingsSheet: View {
	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var restore = true

	var body: some View {
		NavigationStack {
			Form {
				// TRANSLATORS: Section header in Settings grouping general app behavior toggles
				Section(t("Behavior")) {
					// TRANSLATORS: Toggle to reopen previously open documents on next launch
					Toggle(t("Restore last open documents"), isOn: $restore)
					// TRANSLATORS: Toggle controlling whether an upward swipe advances (vs. reverses) navigation
					Toggle(t("Swipe up moves forward"), isOn: Binding(
						get: { viewModel.swipeUpMovesForward },
						set: { viewModel.swipeUpMovesForward = $0 }
					))
				}
				TtsSettingsSection(
					ttsManager: viewModel.ttsManager,
					onPlaySample: { viewModel.ttsManager.speakSample(sampleText) }
				)
			}
			// TRANSLATORS: Navigation bar title of the Settings sheet
			.navigationTitle(t("Settings"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that saves and dismisses the Settings sheet
					Button(t("Done")) {
						viewModel.restorePreviousDocuments = restore
						dismiss()
					}
				}
			}
			.navigationDestination(for: String.self) { destination in
				if destination == "speechDictionary" {
					SpeechDictionaryView()
				} else {
					VoicePickerView(ttsManager: viewModel.ttsManager) { identifier in
						let wasPlaying = viewModel.ttsManager.isSpeaking
						let wasPaused = viewModel.ttsManager.isPaused
						viewModel.ttsManager.selectedVoiceIdentifier = identifier
						if wasPlaying {
							viewModel.ttsManager.stop()
							viewModel.playCurrentSegment()
						} else if wasPaused {
							viewModel.ttsManager.stop()
						}
					}
				}
			}
			.onAppear {
				restore = viewModel.restorePreviousDocuments
			}
		}
		.sheetAccessibilityFocus(title: "Settings")
	}
}
