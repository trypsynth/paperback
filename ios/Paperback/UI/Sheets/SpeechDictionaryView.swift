import SwiftUI

struct SpeechDictionaryView: View {
	@EnvironmentObject var viewModel: AppViewModel
	@State private var editingRule: TtsRule? = nil
	@State private var showAddRule = false
	@Environment(\.editMode) private var editMode

	var body: some View {
		Group {
			if viewModel.ttsRules.isEmpty {
				emptyState
			} else {
				List {
					ForEach(viewModel.ttsRules) { rule in
						RuleRow(rule: rule)
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
		.navigationTitle(t("Speech Dictionary"))
		.navigationBarTitleDisplayMode(.inline)
		.toolbar {
			ToolbarItem(placement: .primaryAction) {
				HStack(spacing: 16) {
					if !viewModel.ttsRules.isEmpty {
						Button(editMode?.wrappedValue.isEditing == true ? t("Done") : t("Edit")) {
							withAnimation {
								editMode?.wrappedValue = editMode?.wrappedValue.isEditing == true ? .inactive : .active
							}
						}
					}
					Button { showAddRule = true } label: {
						Image(systemName: "plus")
					}
					.accessibilityLabel(t("Add Rule"))
				}
			}
		}
		.sheet(isPresented: $showAddRule) {
			RuleEditSheet(rule: nil) { viewModel.ttsRules.append($0) }
				.environmentObject(viewModel)
		}
		.sheet(item: $editingRule) { rule in
			RuleEditSheet(rule: rule) { updated in
				if let idx = viewModel.ttsRules.firstIndex(where: { $0.id == updated.id }) {
					viewModel.ttsRules[idx] = updated
				}
			}
			.environmentObject(viewModel)
		}
	}

	private var emptyState: some View {
		VStack(spacing: 12) {
			Spacer()
			Image(systemName: "text.badge.plus")
				.font(.largeTitle)
				.foregroundStyle(.secondary)
				.accessibilityHidden(true)
			Text(t("No Rules"))
				.font(.title2.bold())
			Spacer()
		}
		.padding()
	}

	private func moveUp(_ rule: TtsRule) {
		guard let idx = viewModel.ttsRules.firstIndex(where: { $0.id == rule.id }), idx > 0 else { return }
		viewModel.ttsRules.swapAt(idx, idx - 1)
	}

	private func moveDown(_ rule: TtsRule) {
		guard let idx = viewModel.ttsRules.firstIndex(where: { $0.id == rule.id }),
		      idx < viewModel.ttsRules.count - 1 else { return }
		viewModel.ttsRules.swapAt(idx, idx + 1)
	}

	private func delete(_ rule: TtsRule) {
		viewModel.ttsRules.removeAll { $0.id == rule.id }
	}

	private func move(from source: IndexSet, to destination: Int) {
		viewModel.ttsRules.move(fromOffsets: source, toOffset: destination)
	}

	private func deleteAt(_ offsets: IndexSet) {
		viewModel.ttsRules.remove(atOffsets: offsets)
	}
}

private struct RuleRow: View {
	let rule: TtsRule

	var body: some View {
		VStack(alignment: .leading, spacing: 3) {
			HStack(spacing: 6) {
				scopeBadge
				if !rule.isEnabled {
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
		.opacity(rule.isEnabled ? 1.0 : 0.5)
		.accessibilityElement(children: .ignore)
		.accessibilityLabel(a11yLabel)
	}

	private var scopeBadge: some View {
		let isWord = rule.scope == .word
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
		let status = rule.isEnabled ? "" : ", disabled"
		return "\(scope): \(pat) to \(repl), \(rule.voiceFilter.label)\(status)"
	}
}
