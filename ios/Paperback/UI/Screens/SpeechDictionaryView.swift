import SwiftUI

/// Core's rules have no identity of their own, and the list needs one to keep a row's focus and animation through a move.
private struct EditableRule: Identifiable {
	let id = UUID()
	var rule: SpeechRule
}

struct SpeechDictionaryView: View {
	@Environment(AppViewModel.self) private var viewModel
	@State private var rules: [EditableRule] = []
	@State private var editingRule: EditableRule? = nil
	@State private var showAddRule = false
	@Environment(\.editMode) private var editMode

	var body: some View {
		Group {
			if rules.isEmpty {
				emptyState
			} else {
				List {
					ForEach(rules) { rule in
						RuleRow(rule: rule.rule)
							.contentShape(Rectangle())
							.onTapGesture { editingRule = rule }
							.accessibilityAction(named: "Move Up") { moveUp(rule) }
							.accessibilityAction(named: "Move Down") { moveDown(rule) }
							.accessibilityAction(named: "Delete") { delete(rule) }
					}
					.onMove(perform: move)
					.onDelete(perform: deleteAt)
				}
			}
		}
		// TRANSLATORS: Navigation title for the screen where users define text-replacement rules for TTS pronunciation
		.navigationTitle(t("Speech Dictionary"))
		.navigationBarTitleDisplayMode(.inline)
		.toolbar {
			ToolbarItem(placement: .primaryAction) {
				HStack(spacing: 16) {
					if !rules.isEmpty {
						// TRANSLATORS: Toolbar button toggling the rule list's edit mode; label reflects the action it will perform next (finish editing / start editing)
						Button(editMode?.wrappedValue.isEditing == true ? t("Done") : t("Edit")) {
							withAnimation {
								editMode?.wrappedValue = editMode?.wrappedValue.isEditing == true ? .inactive : .active
							}
						}
					}
					Button { showAddRule = true } label: {
						Image(systemName: "plus")
					}
					// TRANSLATORS: Accessibility label for the "+" button that adds a new speech dictionary rule
					.accessibilityLabel(t("Add Rule"))
				}
			}
		}
		.onAppear { rules = viewModel.reading.speechRules.map { EditableRule(rule: $0) } }
		.sheet(isPresented: $showAddRule) {
			RuleEditSheet(rule: nil) { added in change { $0.append(EditableRule(rule: added)) } }
				.environment(viewModel)
		}
		.sheet(item: $editingRule) { editing in
			RuleEditSheet(rule: editing.rule) { updated in
				change { list in
					if let idx = list.firstIndex(where: { $0.id == editing.id }) {
						list[idx].rule = updated
					}
				}
			}
			.environment(viewModel)
		}
	}

	private var emptyState: some View {
		VStack(spacing: 12) {
			Spacer()
			Image(systemName: "text.badge.plus")
				.font(.largeTitle)
				.foregroundStyle(.secondary)
				.accessibilityHidden(true)
			// TRANSLATORS: Message shown when no speech dictionary rules have been created yet
			Text(t("No Rules"))
				.font(.title2.bold())
			Spacer()
		}
		.padding()
	}

	private func change(_ edit: (inout [EditableRule]) -> Void) {
		edit(&rules)
		viewModel.reading.setSpeechRules(rules.map(\.rule))
	}

	private func moveUp(_ rule: EditableRule) {
		guard let idx = rules.firstIndex(where: { $0.id == rule.id }), idx > 0 else { return }
		change { $0.swapAt(idx, idx - 1) }
	}

	private func moveDown(_ rule: EditableRule) {
		guard let idx = rules.firstIndex(where: { $0.id == rule.id }),
		      idx < rules.count - 1 else { return }
		change { $0.swapAt(idx, idx + 1) }
	}

	private func delete(_ rule: EditableRule) {
		change { $0.removeAll { $0.id == rule.id } }
	}

	private func move(from source: IndexSet, to destination: Int) {
		change { $0.move(fromOffsets: source, toOffset: destination) }
	}

	private func deleteAt(_ offsets: IndexSet) {
		change { $0.remove(atOffsets: offsets) }
	}
}

private struct RuleRow: View {
	let rule: SpeechRule

	var body: some View {
		VStack(alignment: .leading, spacing: 3) {
			HStack(spacing: 6) {
				scopeBadge
				if !rule.enabled {
					Text("Off")
						.font(.caption)
						.foregroundStyle(.secondary)
				}
			}
			HStack(spacing: 4) {
				Text(rule.pattern.isEmpty ? "(empty)" : rule.pattern)
					.font(.system(.subheadline, design: .monospaced))
					.foregroundStyle(rule.pattern.isEmpty ? .tertiary : .primary)
					.lineLimit(1)
				Image(systemName: "arrow.right")
					.font(.caption2)
					.foregroundStyle(.tertiary)
					.accessibilityHidden(true)
				Text(rule.replacement.isEmpty ? "(empty)" : rule.replacement)
					.font(.system(.subheadline, design: .monospaced))
					.foregroundStyle(rule.replacement.isEmpty ? .tertiary : .primary)
					.lineLimit(1)
			}
			Text(rule.voiceFilter.label)
				.font(.caption)
				.foregroundStyle(.secondary)
		}
		.padding(.vertical, 2)
		.opacity(rule.enabled ? 1.0 : 0.5)
		.accessibilityElement(children: .ignore)
		.accessibilityLabel(a11yLabel)
	}

	private var scopeBadge: some View {
		let isWord = rule.scope == .word
		// TRANSLATORS: Badge on a rule row indicating whether the rule applies per-word or per-paragraph
		return Text(isWord ? t("Word") : t("Paragraph"))
			.font(.caption2.weight(.semibold))
			.padding(.horizontal, 5)
			.padding(.vertical, 2)
			.background(isWord ? Color.blue.opacity(0.15) : Color.purple.opacity(0.15))
			.foregroundStyle(isWord ? .blue : .purple)
			.clipShape(RoundedRectangle(cornerRadius: 4))
	}

	private var a11yLabel: String {
		let scope = rule.scope == .word ? "Word rule" : "Paragraph rule"
		let pat = rule.pattern.isEmpty ? "empty pattern" : rule.pattern
		let repl = rule.replacement.isEmpty ? "nothing" : rule.replacement
		let status = rule.enabled ? "" : ", disabled"
		return "\(scope): \(pat) to \(repl), \(rule.voiceFilter.label)\(status)"
	}
}
