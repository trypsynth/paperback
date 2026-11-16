import SwiftUI

struct BookmarksView: View {
	@Environment(AppViewModel.self) private var viewModel
	@Environment(\.dismiss) private var dismiss
	@State private var filter: BookmarkFilterType = .all
	@State private var pendingDelete: BookmarkRowFfi?
	@AccessibilityFocusState private var focusedIndex: Int?

	var body: some View {
		Group {
			if viewModel.reading.hasBookmarks, let list = viewModel.reading.bookmarkRows(filter) {
				VStack(spacing: 0) {
					// TRANSLATORS: Label for the picker choosing whether the Bookmarks screen shows everything, only bookmarks, or only notes
					Picker(t("Show"), selection: $filter) {
						// TRANSLATORS: Bookmarks screen filter showing bookmarks and notes together
						Text(t("All")).tag(BookmarkFilterType.all)
						// TRANSLATORS: Bookmarks screen filter showing only bookmarks without a note
						Text(t("Bookmarks")).tag(BookmarkFilterType.bookmarksOnly)
						// TRANSLATORS: Bookmarks screen filter showing only bookmarks with a note
						Text(t("Notes")).tag(BookmarkFilterType.notesOnly)
					}
					.pickerStyle(.wheel)
					.labelsHidden()
					.frame(height: 120)
					if list.rows.isEmpty {
						ContentUnavailableView(
							// TRANSLATORS: Announced when there are no bookmarks/notes at all to navigate to
							filter == .notesOnly ? t("No notes.") : t("No bookmarks."),
							systemImage: "bookmark"
						)
					} else {
						rows(list)
					}
				}
			} else {
				emptyView
			}
		}
		// TRANSLATORS: Navigation title of the screen listing the document's bookmarks and notes
		.navigationTitle(t("Bookmarks"))
		.navigationBarTitleDisplayMode(.inline)
		// TRANSLATORS: Title of the alert confirming removal of a bookmark that has a note
		.alert(t("Remove this bookmark and its note?"), isPresented: Binding(
			get: { pendingDelete != nil },
			set: { if !$0 { pendingDelete = nil } }
		)) {
			Button(t("Remove"), role: .destructive) {
				if let row = pendingDelete { delete(row, confirmed: true) }
			}
			Button(t("Cancel"), role: .cancel) { }
		} message: {
			Text(pendingDelete?.note ?? "")
		}
	}

	private func rows(_ list: BookmarkRowsFfi) -> some View {
		let closest = Int(list.closestIndex)
		return ScrollViewReader { proxy in
			List(list.rows.indices, id: \.self) { index in
				row(list.rows[index])
					.accessibilityFocused($focusedIndex, equals: index)
					.id(index)
			}
			.onAppear {
				guard closest >= 0 else { return }
				proxy.scrollTo(closest, anchor: .center)
				// Same delay as sheetAccessibilityFocus, so the push has settled before VoiceOver moves.
				DispatchQueue.main.asyncAfter(deadline: .now() + 0.35) {
					focusedIndex = closest
				}
			}
		}
	}

	private func row(_ row: BookmarkRowFfi) -> some View {
		let time = row.audioMs.map(formatDuration)
		let main = row.note.isEmpty ? row.snippet : row.note
		let secondary = row.note.isEmpty ? "" : row.snippet
		return Button {
			viewModel.reading.goToBookmark(row)
			dismiss()
		} label: {
			HStack(alignment: .firstTextBaseline) {
				VStack(alignment: .leading, spacing: 2) {
					if !row.note.isEmpty {
						Image(systemName: "note.text")
							.font(.caption)
							.foregroundStyle(.secondary)
							.accessibilityHidden(true)
					}
					Text(main.isEmpty ? t("Bookmark") : main)
						.lineLimit(3)
					if !secondary.isEmpty {
						Text(secondary)
							.font(.subheadline)
							.foregroundStyle(.secondary)
							.lineLimit(2)
					}
				}
				Spacer()
				if let time {
					Text(time)
						.font(.subheadline)
						.monospacedDigit()
						.foregroundStyle(.secondary)
				}
			}
			.contentShape(Rectangle())
		}
		.buttonStyle(.plain)
		.accessibilityElement(children: .ignore)
		.accessibilityLabel([main, secondary, time ?? ""].filter { !$0.isEmpty }.joined(separator: ", "))
		.accessibilityAddTraits(.isButton)
		.swipeActions(edge: .trailing) {
			// TRANSLATORS: Swipe / accessibility action deleting a bookmark from the Bookmarks screen
			Button(t("Delete"), role: .destructive) { delete(row, confirmed: false) }
			Button(noteTitle(hasNote: !row.note.isEmpty)) { editNote(row) }
				.tint(.accentColor)
		}
		.accessibilityActions {
			Button(noteTitle(hasNote: !row.note.isEmpty)) { editNote(row) }
			Button(t("Delete")) { delete(row, confirmed: false) }
		}
	}

	private func editNote(_ row: BookmarkRowFfi) {
		viewModel.navigation.bookmarkNote = viewModel.reading.noteTarget(for: row)
	}

	private func delete(_ row: BookmarkRowFfi, confirmed: Bool) {
		guard confirmed || row.note.isEmpty else {
			pendingDelete = row
			return
		}
		pendingDelete = nil
		viewModel.reading.deleteBookmark(start: row.start, end: row.end, audioMs: row.audioMs)
	}

	@ViewBuilder private var emptyView: some View {
		ContentUnavailableView(
			// TRANSLATORS: Title shown when the current document has no bookmarks or notes
			t("No Bookmarks"),
			systemImage: "bookmark",
			// TRANSLATORS: Description shown under the "No Bookmarks" title
			description: Text(t("Tap the bookmark button while reading to mark your place."))
		)
		.sheetAccessibilityFocus(title: t("Bookmarks"))
	}
}
