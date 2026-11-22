import UIKit

/// What a note is being written for, fixed when the note sheet opens: playback is paused while it's up, and a note belongs to the moment the reader asked for it, not the moment they finished typing.
struct BookmarkNoteTarget: Identifiable {
	let id = UUID()
	/// Nil when saving the note makes a new bookmark.
	let existing: BookmarkFfi?
	let position: Int64
	let audioMs: Int64?
	/// The text marked, or for an audio-only book the time, shown above the note.
	let passage: String
	let resumeAfter: Bool
}

/// One document's bookmarks at one revision, text bookmarks sorted by where they start.
final class BookmarkSnapshot {
	let path: String
	let revision: Int
	let count: Int
	let text: [BookmarkFfi]
	/// The units holding a bookmark, for the Bookmarks rotor, worked out the first time it asks.
	var units: [Int]?

	init(path: String, revision: Int, bookmarks: [BookmarkFfi]) {
		self.path = path
		self.revision = revision
		count = bookmarks.count
		text = bookmarks.filter { $0.audioMs == nil }.sorted { $0.start < $1.start }
	}

	/// The first text bookmark starting in `start..<end`, or exactly at `start` for an empty range, as the core's `text_bookmark_in` finds it.
	func bookmark(from start: Int64, to end: Int64) -> BookmarkFfi? {
		var low = 0, high = text.count
		while low < high {
			let mid = (low + high) / 2
			if text[mid].start < start { low = mid + 1 } else { high = mid }
		}
		guard low < text.count else { return nil }
		let found = text[low]
		return found.start == start || found.start < end ? found : nil
	}
}

extension ReadingController {
	/// Read again from the core only when the bookmarks or the document change. Reading the revision here also lets a view showing bookmark state follow it.
	var bookmarks: BookmarkSnapshot? {
		let revision = bookmarksRevision
		guard let context, let path = context.activeDocPath else { return nil }
		if let snapshot = bookmarkSnapshot, snapshot.path == path, snapshot.revision == revision { return snapshot }
		let snapshot = BookmarkSnapshot(path: path, revision: revision, bookmarks: context.configManager.bookmarks(path: path))
		bookmarkSnapshot = snapshot
		return snapshot
	}

	/// Set only in an audio-only book, whose text is a blank line per chapter, so only the playback time tells two bookmarks apart.
	var bookmarkAudioMs: Int64? {
		isAudioOnly ? audioElapsedMs() : nil
	}

	var hasBookmarks: Bool {
		(bookmarks?.count ?? 0) > 0
	}

	/// Set only when it changes, so views showing it don't redraw for nothing.
	func refreshBookmarkHere() {
		let found = findBookmarkHere()
		if found != bookmarkHere { bookmarkHere = found }
	}

	/// The bookmark on the paragraph being read: a bookmark is placed at the start of the segment the reader is on, so one anywhere in the paragraph counts and switching from Line to Paragraph doesn't hide it.
	private func findBookmarkHere() -> BookmarkFfi? {
		guard let snapshot = bookmarks else { return nil }
		// An audio-only book's bookmarks are told apart by time, which moves as it plays, so the core finds the one within reach.
		if let audioMs = bookmarkAudioMs, let context {
			let found = context.configManager.bookmarkAt(path: snapshot.path, start: ttsPosition, end: ttsPosition, audioMs: audioMs)
			return found.found ? found : nil
		}
		let (start, end) = paragraphBounds(at: ttsPosition)
		return snapshot.bookmark(from: start, to: end)
	}

	/// The Bookmarks screen's rows for the active document.
	func bookmarkRows(_ filter: BookmarkFilterType) -> BookmarkRowsFfi? {
		_ = bookmarksRevision
		guard let context, let path = context.activeDocPath, let session = activeSession else { return nil }
		return session.bookmarkRows(config: context.configManager, path: path, position: ttsPosition, filter: filter)
	}

	/// The paragraph around `position`, or just `position` where there is none. Kept for the last position asked about, since a view showing bookmark state asks on every redraw.
	func paragraphBounds(at position: Int64) -> (Int64, Int64) {
		guard let session = activeSession else { return (position, position) }
		let id = ObjectIdentifier(session)
		if let cached = paragraphBoundsCache, cached.session == id, cached.position == position { return cached.bounds }
		let paragraph = session.getTextSegment(position: position, segmentType: .paragraph, direction: .current)
		let bounds = paragraph.found && paragraph.startPos <= position ? (paragraph.startPos, max(paragraph.endPos, position)) : (position, position)
		paragraphBoundsCache = (id, position, bounds)
		return bounds
	}

	/// True when it added one. Removing a bookmark that has a note is for the caller to confirm first.
	@discardableResult
	func toggleBookmark() -> Bool {
		toggleBookmark(existing: findBookmarkHere(), position: ttsPosition, audioMs: bookmarkAudioMs)
	}

	@discardableResult
	private func toggleBookmark(existing: BookmarkFfi?, position: Int64, audioMs: Int64?) -> Bool {
		guard let context, let path = context.activeDocPath else { return false }
		let added: Bool
		if let existing {
			context.configManager.removeBookmark(path: path, start: existing.start, end: existing.end, audioMs: existing.audioMs)
			added = false
		} else {
			context.configManager.addBookmark(path: path, position: position, audioMs: audioMs, note: "")
			added = true
		}
		bookmarksChanged()
		// TRANSLATORS: Announced after toggling a bookmark at the current selection off/on
		announce(added ? t("Bookmark added.") : t("Bookmark removed."))
		return added
	}

	/// Pauses playback until the sheet closes; see `BookmarkNoteTarget`.
	func noteTargetHere() -> BookmarkNoteTarget {
		noteTarget(existing: findBookmarkHere(), position: ttsPosition, audioMs: bookmarkAudioMs)
	}

	private func noteTarget(existing: BookmarkFfi?, position: Int64, audioMs: Int64?) -> BookmarkNoteTarget {
		let wasPlaying = isPlayingNow
		if wasPlaying { togglePlayPause() }
		let audioMs = existing?.audioMs ?? audioMs
		let start = existing?.start ?? position
		return BookmarkNoteTarget(
			existing: existing,
			position: start,
			audioMs: audioMs,
			passage: bookmarkPassage(start: start, audioMs: audioMs),
			resumeAfter: wasPlaying
		)
	}

	func noteTarget(for row: BookmarkRowFfi) -> BookmarkNoteTarget {
		let existing = BookmarkFfi(found: true, start: row.start, end: row.end, note: row.note, audioMs: row.audioMs)
		return BookmarkNoteTarget(existing: existing, position: row.start, audioMs: row.audioMs, passage: row.audioMs.map(formatDuration) ?? row.snippet, resumeAfter: false)
	}

	private func bookmarkPassage(start: Int64, audioMs: Int64?) -> String {
		if let audioMs { return formatDuration(audioMs) }
		return activeSession?.getLineText(position: start).trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
	}

	/// An empty note on an existing bookmark leaves a plain bookmark.
	func saveNote(_ note: String, for target: BookmarkNoteTarget) {
		guard let context, let path = context.activeDocPath else { return }
		let note = note.trimmingCharacters(in: .whitespacesAndNewlines)
		if let existing = target.existing {
			context.configManager.setBookmarkNote(path: path, start: existing.start, end: existing.end, audioMs: existing.audioMs, note: note)
		} else {
			context.configManager.addBookmark(path: path, position: target.position, audioMs: target.audioMs, note: note)
		}
		bookmarksChanged()
		if target.existing != nil && note.isEmpty {
			// TRANSLATORS: Announced after clearing a bookmark's note, which keeps the bookmark
			announce(t("Note deleted."))
		} else {
			// TRANSLATORS: Announced after saving a bookmark's note text
			announce(t("Bookmark saved."))
		}
	}

	func finishNote(_ target: BookmarkNoteTarget) {
		if target.resumeAfter, !isPlayingNow { togglePlayPause() }
	}

	/// Moves there and says what it landed on, the note before the text as desktop does.
	func goToBookmark(_ row: BookmarkRowFfi) {
		let speaking = isPlayingNow && !hasAudio
		goToBookmark(start: row.start, audioMs: row.audioMs, speak: speaking)
		guard !speaking else { return }
		let place = row.audioMs.map(formatDuration) ?? row.snippet
		announce(row.note.isEmpty ? place : (place.isEmpty ? row.note : "\(row.note), \(place)"))
	}

	func deleteBookmark(start: Int64, end: Int64, audioMs: Int64?) {
		guard let context, let path = context.activeDocPath else { return }
		context.configManager.removeBookmark(path: path, start: start, end: end, audioMs: audioMs)
		bookmarksChanged()
		// TRANSLATORS: Announced after toggling a bookmark at the current selection off/on
		announce(t("Bookmark removed."))
	}

	func bookmarksChanged() {
		bookmarksRevision += 1
		ensureNavUnitSupported()
		refreshBookmarkHere()
	}

	/// Moves to a bookmark and, in an audio-only book, to its time; the time comes after the position, since moving the position seeks to the start of the chapter.
	func goToBookmark(start: Int64, audioMs: Int64?, speak: Bool = false) {
		jump(to: start, announce: false)
		if isAudioOnly { refreshSegmentForAudio() }
		if let audioMs {
			seekAudioToMs(audioMs)
			refreshBookmarkHere()
		}
		guard !hasAudio else { return }
		if speak {
			ttsManager.speak(currentSegmentText)
			prefetchAdjacentSegments(around: start)
		} else if ttsManager.isPaused {
			// A paused buffer holds the old spot's speech.
			ttsManager.stop()
		}
	}

	/// The previous or next bookmark, or note with `notesOnly`. Reading carries on from it when `speak`; otherwise it's announced as desktop announces it.
	@discardableResult
	func stepBookmark(forward: Bool, notesOnly: Bool = false, speak: Bool? = nil, announce shouldAnnounce: Bool = true) -> Bool {
		guard let context, let path = context.activeDocPath else { return false }
		let wrap = context.configManager.getAppBool(key: "navigation_wrap", defaultValue: false)
		let step = context.configManager.stepBookmark(path: path, position: ttsPosition, audioMs: bookmarkAudioMs, wrap: wrap, next: forward, notesOnly: notesOnly)
		guard step.found else {
			announce(nothingToStepTo(any: step.any, forward: forward, notesOnly: notesOnly))
			return false
		}
		// Speech reads the bookmarked paragraph itself, so it isn't announced over it; a recording plays on, so where it landed is said.
		let speaking = (speak ?? isPlayingNow) && !hasAudio
		goToBookmark(start: step.start, audioMs: step.audioMs, speak: speaking)
		if shouldAnnounce, !speaking {
			announce(bookmarkLanding(step, forward: forward))
		}
		return true
	}

	private func bookmarkLanding(_ step: BookmarkStepFfi, forward: Bool) -> String {
		let place: String
		if let audioMs = step.audioMs {
			place = formatDuration(audioMs)
		} else {
			place = activeSession?.getLineText(position: step.start).trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
		}
		let content = step.note.isEmpty ? place : (place.isEmpty ? step.note : "\(step.note), \(place)")
		// TRANSLATORS: Announcement when landing on a bookmark; %s is the bookmark/line text, %d is the bookmark's 1-based index
		let landing = t("%s - Bookmark %d")
			.replacingOccurrences(of: "%s", with: content)
			.replacingOccurrences(of: "%d", with: "\(step.number)")
		guard step.wrapped else { return landing }
		// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
		return (forward ? t("Wrapping to start. ") : t("Wrapping to end. ")) + landing
	}

	private func nothingToStepTo(any: Bool, forward: Bool, notesOnly: Bool) -> String {
		if !any {
			// TRANSLATORS: Announced when there are no bookmarks/notes at all to navigate to
			return notesOnly ? t("No notes.") : t("No bookmarks.")
		}
		if forward {
			// TRANSLATORS: Announced when there is no next bookmark/note from the current position
			return notesOnly ? t("No next note.") : t("No next bookmark.")
		}
		// TRANSLATORS: Announced when there is no previous note from the current position
		if notesOnly { return t("No previous note.") }
		// TRANSLATORS: Announced when there is no previous bookmark from the current position
		return t("No previous bookmark.")
	}
}

// MARK: Text mode

/// All answered from the snapshot, without the core: VoiceOver asks for these on every move.
extension ReadingController {
	/// The units holding a bookmark, for the Bookmarks rotor. An audio-only book's bookmarks all sit on its chapters' blank lines, so it offers none.
	func bookmarkUnits(in model: BookModel) -> [Int] {
		guard !isAudioOnly, let snapshot = bookmarks else { return [] }
		if let units = snapshot.units { return units }
		let units = Array(Set(snapshot.text.map { model.unit(forPosition: $0.start) })).sorted()
		snapshot.units = units
		return units
	}

	private func bookmark(inUnit unit: Int, of model: BookModel) -> (BookmarkFfi?, start: Int64, end: Int64)? {
		guard !isAudioOnly, let snapshot = bookmarks, unit >= 0, unit < model.count else { return nil }
		let start = model.position(ofUnit: unit)
		let end = unit + 1 < model.count ? max(model.position(ofUnit: unit + 1), start) : Int64.max
		return (snapshot.bookmark(from: start, to: end), start, end)
	}

	func bookmarkValue(forUnit unit: Int, in model: BookModel) -> String? {
		guard let found = bookmark(inUnit: unit, of: model), let bookmark = found.0 else { return nil }
		// TRANSLATORS: VoiceOver value of a paragraph in text mode that has a bookmark on it
		if bookmark.note.isEmpty { return t("Bookmark") }
		// TRANSLATORS: VoiceOver label for the note shown under the passage being read; {} is the note's text
		return t("Note: {}").replacingOccurrences(of: "{}", with: bookmark.note)
	}

	/// Act on the unit itself, which is the one VoiceOver reads them on.
	func bookmarkActions(forUnit unit: Int, in model: BookModel) -> [UIAccessibilityCustomAction] {
		guard let found = bookmark(inUnit: unit, of: model) else { return [] }
		let (existing, start, end) = found
		let hasNote = !(existing?.note.isEmpty ?? true)
		// TRANSLATORS: VoiceOver action on a paragraph in text mode that marks or unmarks it with a bookmark
		let toggleName = existing == nil ? t("Add Bookmark") : t("Remove Bookmark")
		return [
			UIAccessibilityCustomAction(name: toggleName) { [weak self] _ in
				guard let self else { return false }
				// Looked up again, in case it changed since VoiceOver asked for the actions.
				let existing = bookmarks?.bookmark(from: start, to: end)
				// A note goes with its bookmark, so removing one that has a note is asked first, as from the bookmark button.
				if let existing, !existing.note.isEmpty {
					onRequestBookmarkRemoval?(existing)
				} else {
					toggleBookmark(existing: existing, position: start, audioMs: nil)
				}
				return true
			},
			UIAccessibilityCustomAction(name: noteTitle(hasNote: hasNote)) { [weak self] _ in
				guard let self else { return false }
				onRequestBookmarkNote?(noteTarget(existing: bookmarks?.bookmark(from: start, to: end), position: start, audioMs: nil))
				return true
			},
		]
	}
}
