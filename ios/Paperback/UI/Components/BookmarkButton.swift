import SwiftUI

/// Tap marks or unmarks the paragraph being read; holding it, or VoiceOver's actions, reach the note and the list.
struct BookmarkButton: View {
	@Environment(AppViewModel.self) private var viewModel
	@State private var confirmingRemove = false

	/// No timer here even for an audio-only book: a toolbar that changes every second makes VoiceOver re-read the screen. Its state follows seeks and chapter changes instead.
	var body: some View {
		button(here: viewModel.reading.bookmarkHere)
	}

	private func button(here: BookmarkFfi?) -> some View {
		let hasNote = !(here?.note.isEmpty ?? true)
		return Menu {
			Button { openNote() } label: {
				Label(noteTitle(hasNote: hasNote), systemImage: "square.and.pencil")
			}
			Button { viewModel.navigation.showBookmarks = true } label: {
				// TRANSLATORS: Menu item / accessibility action opening the list of the document's bookmarks and notes
				Label(t("Bookmarks"), systemImage: "list.bullet")
			}
		} label: {
			Image(systemName: here == nil ? "bookmark" : "bookmark.fill")
		} primaryAction: {
			if hasNote { confirmingRemove = true } else { viewModel.reading.toggleBookmark() }
		}
		// TRANSLATORS: Accessibility label of the reading screen's button that marks or unmarks the current paragraph with a bookmark
		.accessibilityLabel(t("Bookmark"))
		// A menu is already announced as a pop-up button, so without this VoiceOver says "button" twice, as on the More options button.
		.accessibilityRemoveTraits(.isButton)
		.accessibilityAddTraits(here == nil ? [] : .isSelected)
		// TRANSLATORS: VoiceOver value of the bookmark button when the bookmark at the current paragraph has a note
		.accessibilityValue(hasNote ? t("Has note") : "")
		.accessibilityActions {
			Button(noteTitle(hasNote: hasNote)) { openNote() }
			Button(t("Bookmarks")) { viewModel.navigation.showBookmarks = true }
		}
		// TRANSLATORS: Title of the alert confirming removal of a bookmark that has a note
		.alert(t("Remove this bookmark and its note?"), isPresented: $confirmingRemove) {
			// TRANSLATORS: Button confirming removal of a bookmark and its note
			Button(t("Remove"), role: .destructive) { viewModel.reading.toggleBookmark() }
			Button(t("Cancel"), role: .cancel) { }
		} message: {
			Text(here?.note ?? "")
		}
	}

	private func openNote() {
		viewModel.navigation.bookmarkNote = viewModel.reading.noteTargetHere()
	}
}

// TRANSLATORS: Menu item / accessibility action to write a note on the current bookmark, or to change the one it has
func noteTitle(hasNote: Bool) -> String {
	hasNote ? t("Edit Note...") : t("Add Note...")
}
