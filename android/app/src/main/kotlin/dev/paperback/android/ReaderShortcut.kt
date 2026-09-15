package dev.paperback.android

import android.view.KeyEvent
import dev.paperback.android.ui.dialogs.GO_TO_LINE
import dev.paperback.android.ui.dialogs.GO_TO_PAGE
import dev.paperback.android.ui.dialogs.GO_TO_PERCENTAGE
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

/**
 * What a key press means to the reading view. Working the key table out as data, rather than
 * acting on each key where it is matched, keeps the whole of it in one readable place and lets it
 * be checked without an Activity: for a reader working entirely from the keyboard, a shortcut
 * that quietly stops matching is not a small thing.
 */
sealed interface ReaderShortcut {
	/** Play, pause, or start reading, depending on what is happening now. */
	object TogglePlayback : ReaderShortcut

	/** Start playing, but leave playback alone if it is already going (the media Play key,
	 * which is a separate key from Play/Pause on a headset or a keyboard). */
	object PlayOnly : ReaderShortcut

	/** Stop playing, leaving already-stopped playback alone (the media Pause key). */
	object PauseOnly : ReaderShortcut

	/** A headset's single button, which counts clicks: one plays or pauses, two skips on, three
	 * goes back. */
	object HeadsetHook : ReaderShortcut

	object NextSegment : ReaderShortcut

	object PreviousSegment : ReaderShortcut

	object FindNext : ReaderShortcut

	object FindPrevious : ReaderShortcut

	object OpenFind : ReaderShortcut

	object OpenSettings : ReaderShortcut

	object OpenBook : ReaderShortcut

	object OpenToc : ReaderShortcut

	object OpenElements : ReaderShortcut

	object OpenWordCount : ReaderShortcut

	object OpenAllDocuments : ReaderShortcut

	object OpenDocumentInfo : ReaderShortcut

	object OpenSleepTimer : ReaderShortcut

	object ExportDocument : ReaderShortcut

	object ExportSettings : ReaderShortcut

	object ImportSettings : ReaderShortcut

	/** Opens Go To on one of its modes (line, page or percentage). */
	data class OpenGoTo(
		val mode: String
	) : ReaderShortcut

	/** Steps to the next or previous element of one kind. */
	data class Navigate(
		val type: SegmentTypeFfi,
		val direction: SegmentDirectionFfi
	) : ReaderShortcut
}

/**
 * The shortcut a key press means, or null to let the key through to whoever else wants it.
 *
 * F7 is looked at before the modifier, so it works however it is pressed; every other shortcut
 * belongs to either the Ctrl set or the unmodified set, and a key with Ctrl held that the Ctrl
 * set does not name is passed on rather than falling through to its unmodified meaning.
 */
fun shortcutFor(
	keyCode: Int,
	ctrlPressed: Boolean,
	shiftPressed: Boolean
): ReaderShortcut? {
	if (keyCode == KeyEvent.KEYCODE_F7) return ReaderShortcut.OpenElements
	if (ctrlPressed) return ctrlShortcutFor(keyCode, shiftPressed)
	if (keyCode == KeyEvent.KEYCODE_F3) {
		return if (shiftPressed) ReaderShortcut.FindPrevious else ReaderShortcut.FindNext
	}
	return readingShortcutFor(keyCode, shiftPressed)
}

/** The shortcuts held with Ctrl, which match the desktop app's. */
private fun ctrlShortcutFor(
	keyCode: Int,
	shiftPressed: Boolean
): ReaderShortcut? =
	when (keyCode) {
		KeyEvent.KEYCODE_F -> ReaderShortcut.OpenFind
		KeyEvent.KEYCODE_COMMA -> ReaderShortcut.OpenSettings
		KeyEvent.KEYCODE_O -> ReaderShortcut.OpenBook
		KeyEvent.KEYCODE_T -> ReaderShortcut.OpenToc
		KeyEvent.KEYCODE_P -> ReaderShortcut.OpenGoTo(GO_TO_PAGE)
		KeyEvent.KEYCODE_G -> ReaderShortcut.OpenGoTo(if (shiftPressed) GO_TO_PERCENTAGE else GO_TO_LINE)
		KeyEvent.KEYCODE_W -> ReaderShortcut.OpenWordCount
		KeyEvent.KEYCODE_R -> ReaderShortcut.OpenAllDocuments
		KeyEvent.KEYCODE_E -> if (shiftPressed) ReaderShortcut.ExportSettings else ReaderShortcut.ExportDocument
		KeyEvent.KEYCODE_I -> if (shiftPressed) ReaderShortcut.ImportSettings else ReaderShortcut.OpenDocumentInfo
		KeyEvent.KEYCODE_S -> if (shiftPressed) ReaderShortcut.OpenSleepTimer else null
		else -> null
	}

/**
 * The shortcuts pressed on their own: playback keys, and one key per kind of element, where
 * Shift turns "go to the next one" into "go back to the previous one".
 */
private fun readingShortcutFor(
	keyCode: Int,
	shiftPressed: Boolean
): ReaderShortcut? {
	val direction = if (shiftPressed) SegmentDirectionFfi.PREVIOUS else SegmentDirectionFfi.NEXT
	return when (keyCode) {
		KeyEvent.KEYCODE_HEADSETHOOK -> ReaderShortcut.HeadsetHook
		KeyEvent.KEYCODE_SPACE, KeyEvent.KEYCODE_MEDIA_PLAY_PAUSE -> ReaderShortcut.TogglePlayback
		KeyEvent.KEYCODE_MEDIA_PLAY -> ReaderShortcut.PlayOnly
		KeyEvent.KEYCODE_MEDIA_PAUSE -> ReaderShortcut.PauseOnly
		KeyEvent.KEYCODE_MEDIA_NEXT -> ReaderShortcut.NextSegment
		KeyEvent.KEYCODE_MEDIA_PREVIOUS -> ReaderShortcut.PreviousSegment
		// Sections get a bracket each rather than one key and Shift, matching the desktop app.
		KeyEvent.KEYCODE_LEFT_BRACKET -> navigate(SegmentTypeFfi.SECTION, SegmentDirectionFfi.PREVIOUS)
		KeyEvent.KEYCODE_RIGHT_BRACKET -> navigate(SegmentTypeFfi.SECTION, SegmentDirectionFfi.NEXT)
		KeyEvent.KEYCODE_H -> navigate(SegmentTypeFfi.HEADING, direction)
		KeyEvent.KEYCODE_P -> navigate(SegmentTypeFfi.PAGE, direction)
		KeyEvent.KEYCODE_G -> navigate(SegmentTypeFfi.IMAGE, direction)
		KeyEvent.KEYCODE_F -> navigate(SegmentTypeFfi.FIGURE, direction)
		KeyEvent.KEYCODE_K -> navigate(SegmentTypeFfi.LINK, direction)
		KeyEvent.KEYCODE_T -> navigate(SegmentTypeFfi.TABLE, direction)
		KeyEvent.KEYCODE_S -> navigate(SegmentTypeFfi.SEPARATOR, direction)
		KeyEvent.KEYCODE_L -> navigate(SegmentTypeFfi.LIST, direction)
		KeyEvent.KEYCODE_I -> navigate(SegmentTypeFfi.LIST_ITEM, direction)
		else -> null
	}
}

private fun navigate(
	type: SegmentTypeFfi,
	direction: SegmentDirectionFfi
) = ReaderShortcut.Navigate(type, direction)
