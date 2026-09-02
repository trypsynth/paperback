import SwiftUI

struct SleepTimerView: View {
	@Environment(AppViewModel.self) private var viewModel

	private enum Choice: Hashable {
		case preset(Int)
		case custom
	}

	private let presets = [5, 10, 15, 30, 45, 60]

	@State private var choice: Choice = .preset(15)
	@State private var customMinutesText = ""
	@FocusState private var customFieldFocused: Bool

	private var isRunning: Bool { viewModel.reading.sleepTimerRemaining != nil }

	private var resolvedMinutes: Int? {
		switch choice {
		case .preset(let minutes): return minutes
		case .custom: return Int(customMinutesText)
		}
	}

	// TRANSLATORS: Shown (and announced by VoiceOver as a single phrase) while the sleep timer
	// is counting down; {} is a time like "12:34"
	private func runningLabel(remaining: Int) -> String {
		let time = String(format: "%d:%02d", remaining / 60, remaining % 60)
		return t("Sleep timer running, {} remaining").replacingOccurrences(of: "{}", with: time)
	}

	// TRANSLATORS: Sleep timer duration option that reveals a field to type a custom number of
	// minutes; {} is the number typed so far, e.g. "Custom: 45 minutes" — shown once non-empty
	// so VoiceOver users don't need to swipe into the text field to hear the current value.
	// Three forms picked by the count's last digits (see nt() in Translations.swift): one =
	// counts ending in 1 except 11 (1, 21, 31, ...), few = counts ending in 2-4 except 12-14
	// (2, 3, 4, 22, 23, 24, ...), many = everything else (0, 5-20, 25-30, ...). The "many" form's
	// trailing character isn't a typo — see pluralManyMarker in Translations.swift.
	private var customRowLabel: String {
		guard !customMinutesText.isEmpty else { return t("Custom") }
		let minutes = Int(customMinutesText) ?? 0
		return nt(t("Custom: {} minute"), t("Custom: {} minutes"), t("Custom: {} minutes⁣"), minutes)
			.replacingOccurrences(of: "{}", with: customMinutesText)
	}

	var body: some View {
		Form {
			if let remaining = viewModel.reading.sleepTimerRemaining {
				Section {
					HStack {
						Spacer()
						Text(runningLabel(remaining: remaining))
						Spacer()
					}
					.accessibilityElement(children: .ignore)
					.accessibilityLabel(runningLabel(remaining: remaining))
				}
			}
			Section {
				// Deliberately not disabled while running: there's no reason to block picking
				// the NEXT duration while the current timer counts down, and disabling it made
				// VoiceOver announce the (non-interactive) "Duration" header as dimmed too.
				// TRANSLATORS: Label of the picker for choosing the sleep timer duration
				Picker(t("Duration"), selection: $choice) {
					ForEach(presets, id: \.self) { minutes in
						Text("\(minutes) minutes").tag(Choice.preset(minutes))
					}
					Text(customRowLabel).tag(Choice.custom)
				}
				.pickerStyle(.inline)
				if choice == .custom {
					// TRANSLATORS: Placeholder in the custom sleep timer duration field, asking for a number of minutes
					TextField(t("Minutes"), text: $customMinutesText)
						.keyboardType(.numberPad)
						.focused($customFieldFocused)
						.onChange(of: customMinutesText) { _, newValue in
							let filtered = newValue.filter(\.isNumber)
							if filtered != newValue { customMinutesText = filtered }
						}
						.toolbar {
							ToolbarItemGroup(placement: .keyboard) {
								Spacer()
								// TRANSLATORS: Button that dismisses the keyboard after typing a custom sleep timer duration
								Button(t("Done")) { customFieldFocused = false }
							}
						}
				}
			}
		}
		.safeAreaInset(edge: .bottom) {
			Button {
				if isRunning {
					viewModel.reading.cancelSleepTimer()
				} else if let minutes = resolvedMinutes, minutes > 0 {
					viewModel.reading.setSleepTimer(seconds: minutes * 60)
				}
			} label: {
				// TRANSLATORS: Button that starts the sleep timer with the selected duration, or stops it if one is already running
				Text(isRunning ? t("Stop Timer") : t("Start Timer"))
					.frame(maxWidth: .infinity)
			}
			.buttonStyle(.borderedProminent)
			.controlSize(.large)
			.tint(isRunning ? .red : .accentColor)
			.disabled(!isRunning && !((resolvedMinutes ?? 0) > 0))
			.padding()
			.background(.bar)
		}
		// TRANSLATORS: Navigation title of the Sleep Timer screen
		.navigationTitle(t("Sleep Timer"))
		.navigationBarTitleDisplayMode(.inline)
		.onChange(of: choice) { _, newValue in
			if newValue == .custom { customFieldFocused = true }
		}
		.sheetAccessibilityFocus(title: "Sleep Timer")
	}
}
