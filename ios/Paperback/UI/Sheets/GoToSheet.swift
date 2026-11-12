import SwiftUI

struct GoToSheet: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@Environment(\.accessibilityVoiceOverEnabled) private var voiceOverEnabled
	@State private var mode: GoToMode = .line
	@State private var lineValue = ""
	@State private var pageValue = ""
	@State private var percentValue: Double = 50
	@State private var timeValue = ""
	@State private var timeError: String?
	@FocusState private var fieldFocused: Bool

	private var session: DocumentSession? { viewModel.activeSession }
	private var hasPages: Bool { (session?.pageCountFfi() ?? 0) > 0 }
	/// An audio-only book's text is one blank line per file, so only a percentage means anything in it.
	private var isAudioOnly: Bool { viewModel.reading.isAudioOnly }
	/// A time can only be placed when every file's length is known.
	private var hasTime: Bool { session?.audioTimeKnownFfi() == true }
	private var currentMs: Int64 { viewModel.reading.audioElapsedMs() ?? 0 }

	var body: some View {
		NavigationStack {
			Form {
				Section {
					// TRANSLATORS: Label for the wheel picker that chooses whether Go To navigates by line, page, or percent
					Picker(t("Mode"), selection: $mode) {
						if !isAudioOnly {
							// TRANSLATORS: Go To mode option: navigate to a specific line number
							Text(t("Line")).tag(GoToMode.line)
						}
						if hasPages && !isAudioOnly {
							// TRANSLATORS: Go To mode option: navigate to a specific page number
							Text(t("Page")).tag(GoToMode.page)
						}
						// TRANSLATORS: Go To mode option: navigate to a percentage through the document
						Text(t("Percent")).tag(GoToMode.percent)
						if hasTime {
							// TRANSLATORS: Go To mode option: navigate to a time in an audiobook, such as 1:23:45
							Text(t("Time")).tag(GoToMode.time)
						}
					}
					.pickerStyle(.wheel)
					.labelsHidden()
					.frame(height: 120)
				}
				Section {
					switch mode {
					case .line:
						// TRANSLATORS: Placeholder for the line number input field in Go To
						TextField(t("Line number"), text: $lineValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .page:
						// TRANSLATORS: Placeholder for the page number input field in Go To
						TextField(t("Page number"), text: $pageValue)
							.keyboardType(.numberPad)
							.focused($fieldFocused)
					case .percent:
						HStack {
							Slider(value: $percentValue, in: 0...100, step: 1)
								// TRANSLATORS: VoiceOver label for the slider that picks a percentage through the document
								.accessibilityLabel(t("Percentage"))
								.accessibilityValue("\(Int(percentValue))%")
							Text("\(Int(percentValue))%")
								.monospacedDigit()
								.frame(width: 44, alignment: .trailing)
								.foregroundStyle(.secondary)
								.accessibilityHidden(true)
						}
					case .time:
						if let session {
							// TRANSLATORS: Label of the field in the Go to Time dialog. The first {} is the time played so far and the second is the recording's length, both written like 1:02:03
							let label = filling(
								t("Go to time ({}/{}):"),
								session.formatAudioTimeFfi(ms: currentMs),
								session.formatAudioTimeFfi(ms: session.audioTotalDurationMsFfi())
							)
							TextField(label, text: $timeValue)
								.keyboardType(.numbersAndPunctuation)
								.focused($fieldFocused)
								.onSubmit { go() }
								.onChange(of: timeValue) { timeError = nil }
							if let timeError {
								Text(timeError).foregroundStyle(.red)
							}
						}
					}
				}
			}
			// TRANSLATORS: Navigation title of the Go To sheet
			.navigationTitle(t("Go To"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button that dismisses the Go To sheet without navigating
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that performs the Go To navigation using the entered line/page/percent
					Button(t("Go")) { go() }
						.disabled(!canGo)
				}
			}
			.onAppear { populate() }
			.onChange(of: mode) { viewModel.navigation.goToInitialMode = mode }
		}
		.sheetAccessibilityFocus(title: t("Go To"))
	}

	/// Fills the `{}` placeholders in turn, as the desktop's `replacen` does with the same string.
	private func filling(_ template: String, _ values: String...) -> String {
		var result = template
		for value in values {
			if let range = result.range(of: "{}") { result.replaceSubrange(range, with: value) }
		}
		return result
	}

	private var canGo: Bool {
		switch mode {
		case .line:    return Int64(lineValue) != nil
		case .page:    return Int64(pageValue) != nil
		case .percent: return true
		case .time:    return !timeValue.trimmingCharacters(in: .whitespaces).isEmpty
		}
	}

	private func populate() {
		let initialMode = viewModel.navigation.goToInitialMode
		// A selection the picker doesn't offer would leave it showing nothing.
		mode = if initialMode == .time {
			hasTime ? .time : .percent
		} else if isAudioOnly {
			.percent
		} else if initialMode == .page && !hasPages {
			.line
		} else {
			initialMode
		}

		guard let session else { return }
		let pos = viewModel.reading.ttsPosition
		let status = session.getStatusInfo(position: pos)
		lineValue = "\(status.lineNumber)"
		percentValue = Double(status.percentage)
		if hasTime { timeValue = session.formatAudioTimeFfi(ms: currentMs) }
		if hasPages {
			pageValue = "\(session.currentPage(position: pos))"
		}

		if !voiceOverEnabled { fieldFocused = mode != .percent }
	}

	private func go() {
		switch mode {
		case .line:
			guard let n = Int64(lineValue) else { dismiss(); return }
			viewModel.reading.goToLine(n)
		case .page:
			guard let n = Int64(pageValue) else { dismiss(); return }
			viewModel.reading.goToPage(Int32(n), announce: true)
		case .percent:
			// Falls back to the text when the recording's file lengths are not all known.
			if !viewModel.reading.seekAudioToPercent(Int32(percentValue)) {
				viewModel.reading.goToPercent(Int32(percentValue), announce: true)
			}
		case .time:
			guard let session else { dismiss(); return }
			let targetMs = session.audioTimeEntryFfi(text: timeValue, currentMs: currentMs)
			guard targetMs >= 0 else {
				let error = targetMs == -1
					// TRANSLATORS: Announced when what was typed in the Go to Time dialog is not a time
					? t("Type a time like 1:23:45, 23:45 or 45.")
					// TRANSLATORS: Announced when the time typed in the Go to Time dialog is past the end of the recording, or before its start
					: t("Time out of range.")
				timeError = error
				UIAccessibility.post(notification: .announcement, argument: error)
				return
			}
			viewModel.reading.seekAudioToMs(targetMs)
		}
		dismiss()
	}
}
