import SwiftUI

struct BookmarkNoteSheet: View {
	let target: BookmarkNoteTarget
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@Environment(\.accessibilityVoiceOverEnabled) private var voiceOverEnabled
	@State private var text: String
	@FocusState private var fieldFocused: Bool

	init(target: BookmarkNoteTarget) {
		self.target = target
		_text = State(initialValue: target.existing?.note ?? "")
	}

	private var hadNote: Bool { !(target.existing?.note.isEmpty ?? true) }

	var body: some View {
		NavigationStack {
			Form {
				if !target.passage.isEmpty {
					// TRANSLATORS: Heading over the passage (or audiobook time) a bookmark note is attached to
					Section(t("Bookmarked")) {
						Text(target.passage)
							.lineLimit(4)
							.foregroundStyle(.secondary)
					}
				}
				// TRANSLATORS: Heading over the text field where a bookmark's note is typed
				Section(t("Note")) {
					// TRANSLATORS: Prompt label in the bookmark note dialog asking the user to type their note
					TextField(t("Enter bookmark note:"), text: $text, axis: .vertical)
						.lineLimit(4...12)
						.focused($fieldFocused)
				}
				if hadNote {
					Section {
						// TRANSLATORS: Button that clears a bookmark's note but keeps the bookmark
						Button(t("Delete Note"), role: .destructive) {
							viewModel.reading.saveNote("", for: target)
							dismiss()
						}
					}
				}
			}
			// TRANSLATORS: Title of the dialog for adding or editing a bookmark note
			.navigationTitle(t("Bookmark Note"))
			.navigationBarTitleDisplayMode(.inline)
			.toolbar {
				ToolbarItem(placement: .cancellationAction) {
					Button(t("Cancel")) { dismiss() }
				}
				ToolbarItem(placement: .confirmationAction) {
					// TRANSLATORS: Button that saves what was typed and closes the editor, such as a speech rule or a bookmark's note
					Button(t("Save")) {
						viewModel.reading.saveNote(text, for: target)
						dismiss()
					}
				}
			}
			.onAppear { if !voiceOverEnabled { fieldFocused = true } }
		}
		.sheetAccessibilityFocus(title: t("Bookmark Note"))
		.onDisappear { viewModel.reading.finishNote(target) }
	}
}
