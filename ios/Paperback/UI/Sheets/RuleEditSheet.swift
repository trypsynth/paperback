import SwiftUI
import AVFoundation

struct RuleEditSheet: View {
	let original: TtsRule?
	let onSave: (TtsRule) -> Void

	@EnvironmentObject var viewModel: AppViewModel
	@Environment(\.dismiss) private var dismiss
	@State private var rule: TtsRule

	init(rule: TtsRule?, onSave: @escaping (TtsRule) -> Void) {
		self.original = rule
		self.onSave = onSave
		_rule = State(initialValue: rule ?? TtsRule())
	}

	var body: some View {
		NavigationStack {
			Form {
				// TRANSLATORS: Section header grouping the scope picker for a text-substitution rule
				Section(t("Rule")) {
					// TRANSLATORS: Label for the picker choosing whether a rule applies to a whole word or a whole paragraph
					Picker(t("Scope"), selection: $rule.scope) {
						// TRANSLATORS: Rule scope option: the rule matches within a single word
						Text(t("Word")).tag(TtsRule.Scope.word)
						// TRANSLATORS: Rule scope option: the rule matches within a whole paragraph
						Text(t("Paragraph")).tag(TtsRule.Scope.paragraph)
					}
					.pickerStyle(.wheel)
				}

				// TRANSLATORS: Section header grouping the find/replace pattern fields for a text-substitution rule
				Section(t("Pattern & Replacement")) {
					// TRANSLATORS: Placeholder for the text field where the user enters the text (or regex) to search for
					TextField(t("Pattern"), text: $rule.pattern)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.font(.system(.body, design: .monospaced))

					// TRANSLATORS: Placeholder for the text field where the user enters the replacement text
					TextField(t("Replacement"), text: $rule.replacement)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.font(.system(.body, design: .monospaced))
				}

				Section {
					if rule.scope == .word {
						// TRANSLATORS: Toggle limiting a word-scope rule to whole-word matches only (no partial-word matches)
						Toggle(t("Whole word only"), isOn: $rule.wholeWord)
					}
					if rule.scope == .paragraph {
						// TRANSLATORS: Toggle enabling regex matching for a paragraph-scope rule; "\1" is a regex backreference syntax and must stay untranslated
						Toggle(t("Regular expression (\\1 = first capture group)"), isOn: Binding(
							get: { rule.matchType == .regex },
							set: { rule.matchType = $0 ? .regex : .literal }
						))
					}
					// TRANSLATORS: Toggle for whether this text-substitution rule is currently active
					Toggle(t("Enabled"), isOn: $rule.isEnabled)
				}

				// TRANSLATORS: Section header for choosing which TTS voices a rule applies to
				Section(t("Apply to")) {
					NavigationLink(value: "voiceFilter") {
						HStack {
							// TRANSLATORS: Row label leading to the voice-filter picker for this rule
							Text(t("Voices"))
							Spacer()
							Text(rule.voiceFilter.label)
								.foregroundStyle(.secondary)
								.lineLimit(1)
						}
					}
				}
			}
			// TRANSLATORS: Navigation title shown when creating a new text-substitution rule vs. editing an existing one
			.navigationTitle(original == nil ? t("New Rule") : t("Edit Rule"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					// TRANSLATORS: Button to dismiss the rule editor without saving
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button to save the rule and dismiss the editor
					Button(t("Save")) {
						onSave(rule)
						dismiss()
					}
					.disabled(rule.pattern.isEmpty)
				}
			}
			.navigationDestination(for: String.self) { _ in
				VoiceFilterPicker(
					filter: $rule.voiceFilter,
					voices: viewModel.ttsManager.availableVoices
				)
			}
		}
	}
}

private struct VoiceFilterPicker: View {
	@Binding var filter: TtsRule.VoiceFilter
	let voices: [AVSpeechSynthesisVoice]
	@Environment(\.dismiss) private var dismiss

	private var sections: [VoiceSection] { buildSections(from: voices) }

	var body: some View {
		List {
			Section {
				// TRANSLATORS: Row label for selecting all TTS voices as the target of a rule, rather than a specific language or voice
				filterRow(label: t("All voices"), isSelected: filter == .all) {
					filter = .all
					dismiss()
				}
			}
			ForEach(sections) { section in
				let langFilter = TtsRule.VoiceFilter.language(section.language)
				Section(languageLabel(section.language)) {
					filterRow(
						label: "All \(languageLabel(section.language)) voices",
						isSelected: filter == langFilter
					) {
						filter = langFilter
						dismiss()
					}
					ForEach(section.items) { item in
						let selected = isVoiceSelected(item.identifier)
						filterRow(label: item.label, isSelected: selected) {
							toggleVoice(item.identifier)
							dismiss()
						}
					}
				}
			}
		}
		// TRANSLATORS: Navigation title for the screen where the user picks which voices a rule applies to
		.navigationTitle(t("Apply To"))
		.navigationBarTitleDisplayMode(.inline)
	}

	private func filterRow(label: String, isSelected: Bool, action: @escaping () -> Void) -> some View {
		Button(action: action) {
			HStack {
				Text(label).foregroundStyle(.primary)
				Spacer()
				if isSelected {
					Image(systemName: "checkmark").foregroundStyle(Color.accentColor)
				}
			}
		}
		.accessibilityAddTraits(isSelected ? .isSelected : [])
	}

	private func isVoiceSelected(_ id: String) -> Bool {
		if case .voices(let ids) = filter { return ids.contains(id) }
		return false
	}

	private func toggleVoice(_ id: String) {
		filter = .voices([id])
	}
}
