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

	private var paragraphPauseLabel: String {
		// TRANSLATORS: Value of the paragraph pause setting; {} is a number of milliseconds
		t("{} ms").replacingOccurrences(of: "{}", with: "\(ttsManager.paragraphPauseMs)")
	}

	/// The percent readout at the trailing edge of a slider row, which doubles as the
	/// presets menu and the screen reader's adjustable value for that row.
	private func percentMenu(_ label: String, percent: Binding<Int>, presets: [Int]) -> some View {
		SpeechPercentMenu(accessibilityLabel: label, percent: percent, presets: presets) {
			Text("\(percent.wrappedValue)%")
				.foregroundStyle(.secondary)
				.monospacedDigit()
		}
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
			// The sliders are for touch. A screen reader gets the percent menu next to each label
			// instead: one swipe per percent and the presets behind a double-tap, the same control
			// as on the read-aloud bar.
			VStack(alignment: .leading, spacing: 4) {
				HStack {
					// TRANSLATORS: Label above the speech rate slider (visual label; the percent control next to it has its own accessibility label)
					Text(t("Rate")).accessibilityHidden(true)
					Spacer()
					percentMenu(
						// TRANSLATORS: VoiceOver accessibility label for the speech rate control in Settings
						t("Speech Rate"),
						percent: $ttsManager.speechRatePercent,
						presets: speechRatePresets
					)
				}
				Slider(
					value: $ttsManager.speechRate,
					in: AVSpeechUtteranceMinimumSpeechRate...AVSpeechUtteranceMaximumSpeechRate,
					step: (AVSpeechUtteranceMaximumSpeechRate - AVSpeechUtteranceMinimumSpeechRate) / 100
				)
				.accessibilityHidden(true)
			}
			VStack(alignment: .leading, spacing: 4) {
				HStack {
					// TRANSLATORS: Label above the speech pitch slider (visual label; the percent control next to it has its own accessibility label)
					Text(t("Pitch")).accessibilityHidden(true)
					Spacer()
					percentMenu(
						// TRANSLATORS: VoiceOver accessibility label for the pitch control in Settings
						t("Pitch"),
						percent: $ttsManager.pitchPercent,
						presets: pitchPresets
					)
				}
				Slider(
					value: $ttsManager.pitch,
					in: pitchRange,
					step: (pitchRange.upperBound - pitchRange.lowerBound) / 100
				)
				.accessibilityHidden(true)
			}
			Stepper(
				value: $ttsManager.paragraphPauseMs,
				in: paragraphPauseRangeMs,
				step: paragraphPauseStepMs
			) {
				HStack {
					// TRANSLATORS: Row label for the setting that adds silence between paragraphs when reading aloud
					Text(t("Paragraph Pause"))
					Spacer()
					Text(paragraphPauseLabel)
						.foregroundStyle(.secondary)
						.monospacedDigit()
				}
			}
			.accessibilityValue(paragraphPauseLabel)
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

private struct ReadabilitySettingsSection: View {
	@Bindable var viewModel: AppViewModel

	var body: some View {
		// TRANSLATORS: Section header in Settings grouping controls for how document text is displayed
		Section(t("Readability")) {
			Stepper(value: $viewModel.textScalePercent, in: 70...300, step: 10) {
				HStack {
					// TRANSLATORS: Row label for the control that scales the size of document text
					Text(t("Text Size"))
					Spacer()
					Text("\(viewModel.textScalePercent)%")
						.foregroundStyle(.secondary)
						.monospacedDigit()
				}
			}
			.accessibilityValue("\(viewModel.textScalePercent)%")
			// TRANSLATORS: Label for the picker choosing how much space sits between lines of text
			Picker(t("Line Spacing"), selection: $viewModel.lineSpacingChoice) {
				// TRANSLATORS: Default spacing option, shown in the line and paragraph spacing pickers
				Text(t("Normal")).tag(0)
				// TRANSLATORS: 1.5x line spacing option
				Text(t("1.5\u{00d7}")).tag(1)
				// TRANSLATORS: Double line spacing option
				Text(t("Double")).tag(2)
			}
			// TRANSLATORS: Label for the picker choosing how much space sits between paragraphs
			Picker(t("Paragraph Spacing"), selection: $viewModel.paragraphSpacingChoice) {
				Text(t("Normal")).tag(0)
				// TRANSLATORS: Relaxed paragraph spacing option
				Text(t("Relaxed")).tag(1)
				// TRANSLATORS: Wide paragraph spacing option
				Text(t("Wide")).tag(2)
			}
			// TRANSLATORS: Label for the picker choosing whether the app follows the system light/dark setting or is pinned to one of them
			Picker(t("Appearance"), selection: $viewModel.appearanceChoice) {
				// TRANSLATORS: Appearance option following the phone's own light/dark setting
				Text(t("System")).tag(0)
				// TRANSLATORS: Appearance option pinning the app to its light colours
				Text(t("Light")).tag(1)
				// TRANSLATORS: Appearance option pinning the app to its dark colours
				Text(t("Dark")).tag(2)
			}
			// TRANSLATORS: Toggle that renders document text in pure black on white, or white on black
			Toggle(t("High Contrast Text"), isOn: $viewModel.highContrastText)
			// TRANSLATORS: Label for the picker choosing how document text is aligned
			Picker(t("Alignment"), selection: $viewModel.textAlignmentChoice) {
				// TRANSLATORS: Left text alignment option
				Text(t("Left")).tag(0)
				// TRANSLATORS: Center text alignment option
				Text(t("Center")).tag(1)
				// TRANSLATORS: Right text alignment option
				Text(t("Right")).tag(2)
			}
		}
	}
}

struct SettingsView: View {
	@Environment(AppViewModel.self) private var viewModel

	var body: some View {
		Form {
			Section {
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
				// TRANSLATORS: Toggle that removes the reading bar's previous/next buttons from the screen reader's swipe order, since the play button's swipe up/down does the same thing
				Toggle(t("Hide previous and next buttons"), isOn: Binding(
					get: { viewModel.hidePrevNextButtons },
					set: { viewModel.hidePrevNextButtons = $0 }
				))
			} header: {
				// TRANSLATORS: Section header in Settings grouping general app behavior toggles
				Text(t("Behavior"))
			} footer: {
				// TRANSLATORS: Footer explaining the "Swipe up moves forward" toggle above it
				Text(t("With VoiceOver, swipe up on the play button to move forward, or down to move back."))
			}
			ReadabilitySettingsSection(viewModel: viewModel)
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
