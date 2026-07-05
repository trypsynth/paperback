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
				Section(t("Rule")) {
					Picker(t("Scope"), selection: $rule.scope) {
						Text(t("Word")).tag(TtsRule.Scope.word)
						Text(t("Paragraph")).tag(TtsRule.Scope.paragraph)
					}
					.pickerStyle(.wheel)
				}

				Section(t("Pattern & Replacement")) {
					TextField(t("Pattern"), text: $rule.pattern)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.font(.system(.body, design: .monospaced))

					TextField(t("Replacement"), text: $rule.replacement)
						.autocorrectionDisabled()
						.textInputAutocapitalization(.never)
						.font(.system(.body, design: .monospaced))
				}

				Section {
					if rule.scope == .word {
						Toggle(t("Whole word only"), isOn: $rule.wholeWord)
					}
					if rule.scope == .paragraph {
						Toggle(t("Regular expression (\\1 = first capture group)"), isOn: Binding(
							get: { rule.matchType == .regex },
							set: { rule.matchType = $0 ? .regex : .literal }
						))
					}
					Toggle(t("Enabled"), isOn: $rule.isEnabled)
				}

				Section(t("Apply to")) {
					NavigationLink(value: "voiceFilter") {
						HStack {
							Text(t("Voices"))
							Spacer()
							Text(rule.voiceFilter.label)
								.foregroundStyle(.secondary)
								.lineLimit(1)
						}
					}
				}
			}
			.navigationTitle(original == nil ? t("New Rule") : t("Edit Rule"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
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
