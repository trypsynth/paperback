package dev.paperback.android

import android.view.KeyEvent
import dev.paperback.android.ui.dialogs.GO_TO_LINE
import dev.paperback.android.ui.dialogs.GO_TO_PAGE
import dev.paperback.android.ui.dialogs.GO_TO_PERCENTAGE
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

/** Kept as data so the key table can be tested without an Activity. */
sealed interface ReaderShortcut {
	object TogglePlayback : ReaderShortcut

	object PlayOnly : ReaderShortcut

	object PauseOnly : ReaderShortcut

	object HeadsetHook : ReaderShortcut

	object NextSegment : ReaderShortcut

	object PreviousSegment : ReaderShortcut

	object FindNext : ReaderShortcut

	object FindPrevious : ReaderShortcut

	object OpenFind : ReaderShortcut

	object OpenSettings : ReaderShortcut

	object OpenBook : ReaderShortcut

	object OpenFromUrl : ReaderShortcut

	object OpenToc : ReaderShortcut

	object OpenElements : ReaderShortcut

	object OpenWordCount : ReaderShortcut

	object OpenAllDocuments : ReaderShortcut

	object OpenDocumentInfo : ReaderShortcut

	object OpenSleepTimer : ReaderShortcut

	object ExportDocument : ReaderShortcut

	object ExportSettings : ReaderShortcut

	object ImportSettings : ReaderShortcut

	data class OpenGoTo(val mode: String) : ReaderShortcut

	data class Navigate(val type: SegmentTypeFfi, val direction: SegmentDirectionFfi) : ReaderShortcut
}

/** A Ctrl key the Ctrl set doesn't name is passed on rather than falling through to its unmodified meaning. */
fun shortcutFor(
	keyCode: Int,
	ctrlPressed: Boolean,
	shiftPressed: Boolean,
	altPressed: Boolean = false
): ReaderShortcut? {
	if (keyCode == KeyEvent.KEYCODE_F7) return ReaderShortcut.OpenElements
	if (ctrlPressed) return ctrlShortcutFor(keyCode, shiftPressed, altPressed)
	if (keyCode == KeyEvent.KEYCODE_F3) {
		return if (shiftPressed) ReaderShortcut.FindPrevious else ReaderShortcut.FindNext
	}
	return readingShortcutFor(keyCode, shiftPressed)
}

/** Matches the desktop app's Ctrl shortcuts. */
private fun ctrlShortcutFor(keyCode: Int, shiftPressed: Boolean, altPressed: Boolean): ReaderShortcut? =
	when (keyCode) {
		KeyEvent.KEYCODE_F -> ReaderShortcut.OpenFind
		KeyEvent.KEYCODE_COMMA -> ReaderShortcut.OpenSettings
		KeyEvent.KEYCODE_O -> if (altPressed) ReaderShortcut.OpenFromUrl else ReaderShortcut.OpenBook
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

private fun readingShortcutFor(keyCode: Int, shiftPressed: Boolean): ReaderShortcut? {
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

private fun navigate(type: SegmentTypeFfi, direction: SegmentDirectionFfi) = ReaderShortcut.Navigate(type, direction)
