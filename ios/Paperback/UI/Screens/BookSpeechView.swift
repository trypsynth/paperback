import SwiftUI

/// Voice, rate and pitch for the open book, each either its own or the one in Settings.
struct BookSpeechView: View {
	@Environment(AppViewModel.self) private var viewModel

	private var speech: SpeechSettings { viewModel.speech }
	private var ttsManager: TtsManager { viewModel.reading.ttsManager }

	var body: some View {
		Form {
			Section {
				// TRANSLATORS: Switch in Speech for This Book giving the open book a voice of its own instead of the one in Settings
				Toggle(t("Own Voice"), isOn: Binding(get: { speech.book.voice != nil }, set: { speech.setBookVoice($0 ? (speech.effectiveVoice ?? "") : nil) }))
				if let voice = speech.book.voice {
					NavigationLink {
						VoicePickerView(ttsManager: ttsManager, selected: voice.isEmpty ? nil : voice) { identifier in
							speech.setBookVoice(identifier ?? "")
						}
					} label: {
						HStack {
							Text(t("Voice"))
							Spacer()
							Text(voiceName(voice.isEmpty ? nil : voice, in: ttsManager.availableVoices))
								.foregroundStyle(.secondary)
								.lineLimit(1)
						}
					}
				}
				// TRANSLATORS: Switch in Speech for This Book giving the open book a speech rate of its own instead of the one in Settings
				Toggle(t("Own Rate"), isOn: Binding(get: { speech.book.ratePercent != nil }, set: { speech.setBookRatePercent($0 ? percentForSpeechRate(speech.defaultRate) : nil) }))
				if let percent = speech.book.ratePercent {
					SpeechRateRow(rate: Binding(get: { speechRateForPercent(Int(percent)) }, set: { speech.setBookRatePercent(percentForSpeechRate($0)) }))
				}
				// TRANSLATORS: Switch in Speech for This Book giving the open book a pitch of its own instead of the one in Settings
				Toggle(t("Own Pitch"), isOn: Binding(get: { speech.book.pitchPercent != nil }, set: { speech.setBookPitchPercent($0 ? percentForPitch(speech.defaultPitch) : nil) }))
				if let percent = speech.book.pitchPercent {
					SpeechPitchRow(pitch: Binding(get: { pitchForPercent(Int(percent)) }, set: { speech.setBookPitchPercent(percentForPitch($0)) }))
				}
			} footer: {
				// TRANSLATORS: Footer in Speech for This Book explaining what a switch left off does
				Text(t("Whatever is switched off follows Settings."))
			}
			Button {
				ttsManager.speakSample(speechSampleText)
			} label: {
				Label(t("Play Sample"), systemImage: "play.circle")
			}
		}
		// TRANSLATORS: Title of the screen, and the document menu item opening it, that gives the open book its own voice, rate and pitch
		.navigationTitle(t("Speech for This Book"))
		.navigationBarTitleDisplayMode(.inline)
	}
}
