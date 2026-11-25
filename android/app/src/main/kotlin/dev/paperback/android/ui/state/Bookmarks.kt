package dev.paperback.android.ui.state

import dev.paperback.android.t
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.BookmarkFfi
import uniffi.paperback.BookmarkFilterType
import uniffi.paperback.BookmarkRowFfi
import uniffi.paperback.BookmarkRowsFfi
import uniffi.paperback.BookmarkStepFfi
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

/** What a note is being written for, fixed when the note dialog opens: playback pauses while it's up, and a note belongs to the moment the reader asked for it, not the moment they finished typing. */
data class BookmarkNoteTarget(
	/** Null when saving the note makes a new bookmark. */
	val existing: BookmarkFfi?,
	val position: Long,
	val audioMs: Long?,
	/** The text marked, or for an audio-only book the time, shown above the note. */
	val passage: String,
	val resumeAfter: Boolean
)

/**
 * The reader's bookmarks and notes in the active document. A bookmark is placed at the start of the segment being read, which reading aloud
 * keeps at a paragraph's start; one anywhere in the paragraph counts as here, so switching from Line to Paragraph doesn't hide it.
 */
class Bookmarks(
	private val config: ConfigManagerFfi,
	private val reading: ReadingController,
	private val activeTab: () -> DocumentTabState?,
	private val announcer: (String) -> Unit
) {
	private val _revision = MutableStateFlow(0)

	/** Bumped on every change, so screens reading bookmarks through the core know to read again. */
	val revision: StateFlow<Int> = _revision.asStateFlow()

	private val _noteTarget = MutableStateFlow<BookmarkNoteTarget?>(null)
	val noteTarget: StateFlow<BookmarkNoteTarget?> = _noteTarget.asStateFlow()

	init {
		reading.stepBookmark = { forward, speak, announce -> step(forward, speak = speak, announce = announce) }
	}

	/** Set only in an audio-only book, whose text is a blank line per chapter, so only the playback time tells two bookmarks apart. */
	private fun audioMs(tab: DocumentTabState): Long? = if (tab.isAudioOnly) reading.audioElapsedMs() else null

	/** The active document's bookmarks as last read from the core, so text mode's lines don't each ask it again as they're drawn. */
	private var snapshot: Triple<String, Int, List<BookmarkFfi>>? = null

	private fun textBookmarks(tab: DocumentTabState): List<BookmarkFfi> {
		val revision = _revision.value
		snapshot?.let { (uri, at, list) -> if (uri == tab.documentUri && at == revision) return list }
		val list = config.bookmarks(tab.documentUri).filter { it.audioMs == null }.sortedBy { it.start }
		snapshot = Triple(tab.documentUri, revision, list)
		return list
	}

	fun hasBookmarks(): Boolean {
		val tab = activeTab() ?: return false
		return config.hasBookmarks(tab.documentUri)
	}

	/** The bookmark starting on a text mode line, from the snapshot; the same test as the core's for a paragraph. */
	fun onLine(start: Long, end: Long): BookmarkFfi? {
		val tab = activeTab() ?: return null
		val list = textBookmarks(tab)
		var low = 0
		var high = list.size
		while (low < high) {
			val mid = (low + high) / 2
			if (list[mid].start < start) low = mid + 1 else high = mid
		}
		val found = list.getOrNull(low) ?: return null
		return found.takeIf { it.start == start || it.start < end }
	}

	fun here(): BookmarkFfi? {
		val tab = activeTab() ?: return null
		return inParagraphOf(tab, reading.ttsPosition.value, audioMs(tab))
	}

	private fun inParagraphOf(tab: DocumentTabState, position: Long, audioMs: Long?): BookmarkFfi? {
		val paragraph = tab.session.getTextSegment(position, SegmentTypeFfi.PARAGRAPH, SegmentDirectionFfi.CURRENT)
		val (start, end) = if (paragraph.found && paragraph.startPos <= position) {
			paragraph.startPos to maxOf(paragraph.endPos, position)
		} else {
			position to position
		}
		return config.bookmarkAt(tab.documentUri, start, end, audioMs).takeIf { it.found }
	}

	fun rows(filter: BookmarkFilterType): BookmarkRowsFfi? {
		val tab = activeTab() ?: return null
		return tab.session.bookmarkRows(config, tab.documentUri, reading.ttsPosition.value, filter)
	}

	/** True when it added one. Removing a bookmark that has a note is for the caller to confirm first. */
	fun toggle(): Boolean {
		val tab = activeTab() ?: return false
		return toggle(tab, reading.ttsPosition.value, here(), audioMs(tab))
	}

	/** For a line of text mode, which may not be the reading position. */
	fun toggleAt(start: Long, end: Long): Boolean {
		val tab = activeTab() ?: return false
		return toggle(tab, start, onLine(start, end), null)
	}

	private fun toggle(tab: DocumentTabState, position: Long, existing: BookmarkFfi?, audioMs: Long?): Boolean {
		if (existing != null) {
			config.removeBookmark(tab.documentUri, existing.start, existing.end, existing.audioMs)
		} else {
			config.addBookmark(tab.documentUri, position, audioMs, "")
		}
		changed()
		// TRANSLATORS: Announced after toggling a bookmark at the current selection off/on
		announcer(if (existing == null) t("Bookmark added.") else t("Bookmark removed."))
		return existing == null
	}

	fun openNoteHere() {
		val tab = activeTab() ?: return
		openNote(tab, reading.ttsPosition.value, here(), audioMs(tab))
	}

	fun openNoteAt(start: Long, end: Long) {
		val tab = activeTab() ?: return
		openNote(tab, start, onLine(start, end), null)
	}

	private fun openNote(tab: DocumentTabState, at: Long, existing: BookmarkFfi?, audioHere: Long?) {
		val wasPlaying = reading.isPlaying()
		if (wasPlaying) reading.pauseTts()
		val audioMs = existing?.audioMs ?: audioHere
		val position = existing?.start ?: at
		val passage = audioMs?.let { formatDuration(it) } ?: tab.session.getLineText(position).trim()
		_noteTarget.value = BookmarkNoteTarget(existing, position, audioMs, passage, wasPlaying)
	}

	fun openNoteFor(row: BookmarkRowFfi) {
		val existing = BookmarkFfi(true, row.start, row.end, row.note, row.audioMs)
		val passage = row.audioMs?.let { formatDuration(it) } ?: row.snippet
		_noteTarget.value = BookmarkNoteTarget(existing, row.start, row.audioMs, passage, resumeAfter = false)
	}

	/** An empty note on an existing bookmark leaves a plain bookmark. */
	fun saveNote(text: String, target: BookmarkNoteTarget) {
		val tab = activeTab() ?: return
		val note = text.trim()
		val existing = target.existing
		if (existing != null) {
			config.setBookmarkNote(tab.documentUri, existing.start, existing.end, existing.audioMs, note)
		} else {
			config.addBookmark(tab.documentUri, target.position, target.audioMs, note)
		}
		changed()
		val message = if (existing != null && note.isEmpty()) {
			// TRANSLATORS: Announced after clearing a bookmark's note, which keeps the bookmark
			t("Note deleted.")
		} else {
			// TRANSLATORS: Announced after saving a bookmark's note text
			t("Bookmark saved.")
		}
		announcer(message)
	}

	fun announce(message: String) = announcer(message)

	fun closeNote(target: BookmarkNoteTarget) {
		_noteTarget.value = null
		if (target.resumeAfter && !reading.isPlaying()) reading.resumeTts()
	}

	fun delete(row: BookmarkRowFfi) {
		val tab = activeTab() ?: return
		config.removeBookmark(tab.documentUri, row.start, row.end, row.audioMs)
		changed()
		// TRANSLATORS: Announced after toggling a bookmark at the current selection off/on
		announcer(t("Bookmark removed."))
	}

	/** Moves there and says what it landed on, the note before the text as desktop does. */
	fun goTo(row: BookmarkRowFfi) {
		val tab = activeTab() ?: return
		val speaking = reading.isPlaying() && !tab.hasAudio
		reading.jumpToBookmark(row.start, row.audioMs, speaking)
		if (speaking) return
		val place = row.audioMs?.let { formatDuration(it) } ?: row.snippet
		announcer(noteAndPlace(row.note, place))
	}

	/** The previous or next bookmark, or note with [notesOnly]. Reading carries on from it when [speak]; otherwise it's announced as desktop announces it. */
	fun step(forward: Boolean, notesOnly: Boolean = false, speak: Boolean? = null, announce: Boolean = true): Boolean {
		val tab = activeTab() ?: return false
		val wrap = config.getAppBool("navigation_wrap", false)
		val position = reading.ttsPosition.value
		val step = config.stepBookmark(tab.documentUri, position, audioMs(tab), wrap, forward, notesOnly)
		if (!step.found) {
			announcer(nothingToStepTo(step.any, forward, notesOnly))
			return false
		}
		// Speech reads the bookmarked paragraph itself, so it isn't announced over it; a recording plays on, so where it landed is said.
		val resume = speak ?: reading.isPlaying()
		reading.jumpToBookmark(step.start, step.audioMs, resume)
		if (announce && (!resume || tab.hasAudio)) announcer(landing(tab, step, forward))
		return true
	}

	private fun landing(tab: DocumentTabState, step: BookmarkStepFfi, forward: Boolean): String {
		val place = step.audioMs?.let { formatDuration(it) } ?: tab.session.getLineText(step.start).trim()
		val content = noteAndPlace(step.note, place)
		// TRANSLATORS: Announcement when landing on a bookmark; %s is the bookmark/line text, %d is the bookmark's 1-based index
		val text = t("%s - Bookmark %d").replace("%s", content).replace("%d", step.number.toString())
		if (!step.wrapped) return text
		// TRANSLATORS: Prefix announced when navigation wraps around past the end/start of the document; the trailing space is significant
		return (if (forward) t("Wrapping to start. ") else t("Wrapping to end. ")) + text
	}

	private fun changed() {
		_revision.value += 1
	}

	/** After bookmarks arrive from elsewhere, such as an imported .paperback file. */
	fun refresh() = changed()
}

/** The note before the text or time it marks, as desktop says a bookmark. */
private fun noteAndPlace(note: String, place: String): String = when {
	note.isEmpty() -> place
	place.isEmpty() -> note
	else -> "$note, $place"
}

internal fun nothingToStepTo(any: Boolean, forward: Boolean, notesOnly: Boolean): String = when {
	// TRANSLATORS: Announced when there are no bookmarks/notes at all to navigate to
	!any -> if (notesOnly) t("No notes.") else t("No bookmarks.")
	// TRANSLATORS: Announced when there is no next bookmark/note from the current position
	forward -> if (notesOnly) t("No next note.") else t("No next bookmark.")
	// TRANSLATORS: Announced when there is no previous note from the current position
	notesOnly -> t("No previous note.")
	// TRANSLATORS: Announced when there is no previous bookmark from the current position
	else -> t("No previous bookmark.")
}
