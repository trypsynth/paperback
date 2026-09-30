package dev.paperback.android

import android.view.KeyEvent
import dev.paperback.android.ui.dialogs.GO_TO_LINE
import dev.paperback.android.ui.dialogs.GO_TO_PAGE
import dev.paperback.android.ui.dialogs.GO_TO_PERCENTAGE
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.paperback.SegmentDirectionFfi
import uniffi.paperback.SegmentTypeFfi

class ReaderShortcutTest {
	private fun plain(keyCode: Int) = shortcutFor(keyCode, ctrlPressed = false, shiftPressed = false)

	private fun shift(keyCode: Int) = shortcutFor(keyCode, ctrlPressed = false, shiftPressed = true)

	private fun ctrl(keyCode: Int) = shortcutFor(keyCode, ctrlPressed = true, shiftPressed = false)

	private fun ctrlShift(keyCode: Int) = shortcutFor(keyCode, ctrlPressed = true, shiftPressed = true)

	// Several letters mean one thing with Ctrl and another on their own. Getting either side
	// wrong takes a shortcut away from a reader with no other way to reach the command.
	@Test
	fun `the letters that mean two things keep both meanings`() {
		assertEquals(ReaderShortcut.OpenFind, ctrl(KeyEvent.KEYCODE_F))
		assertEquals(navigate(SegmentTypeFfi.FIGURE), plain(KeyEvent.KEYCODE_F))
		assertEquals(ReaderShortcut.OpenToc, ctrl(KeyEvent.KEYCODE_T))
		assertEquals(navigate(SegmentTypeFfi.TABLE), plain(KeyEvent.KEYCODE_T))
		assertEquals(ReaderShortcut.OpenGoTo(GO_TO_PAGE), ctrl(KeyEvent.KEYCODE_P))
		assertEquals(navigate(SegmentTypeFfi.PAGE), plain(KeyEvent.KEYCODE_P))
		assertEquals(ReaderShortcut.OpenGoTo(GO_TO_LINE), ctrl(KeyEvent.KEYCODE_G))
		assertEquals(navigate(SegmentTypeFfi.IMAGE), plain(KeyEvent.KEYCODE_G))
	}

	@Test
	fun `shift picks the other go to mode and the other transfer direction`() {
		assertEquals(ReaderShortcut.OpenGoTo(GO_TO_PERCENTAGE), ctrlShift(KeyEvent.KEYCODE_G))
		assertEquals(ReaderShortcut.ExportDocument, ctrl(KeyEvent.KEYCODE_E))
		assertEquals(ReaderShortcut.ExportSettings, ctrlShift(KeyEvent.KEYCODE_E))
		assertEquals(ReaderShortcut.OpenDocumentInfo, ctrl(KeyEvent.KEYCODE_I))
		assertEquals(ReaderShortcut.ImportSettings, ctrlShift(KeyEvent.KEYCODE_I))
	}

	// Ctrl+S is the system's to keep; only Ctrl+Shift+S is ours.
	@Test
	fun `the sleep timer needs shift as well as ctrl`() {
		assertNull(ctrl(KeyEvent.KEYCODE_S))
		assertEquals(ReaderShortcut.OpenSleepTimer, ctrlShift(KeyEvent.KEYCODE_S))
	}

	@Test
	fun `every kind of element steps backward with shift`() {
		val keys = listOf(
			KeyEvent.KEYCODE_H,
			KeyEvent.KEYCODE_P,
			KeyEvent.KEYCODE_G,
			KeyEvent.KEYCODE_F,
			KeyEvent.KEYCODE_K,
			KeyEvent.KEYCODE_T,
			KeyEvent.KEYCODE_S,
			KeyEvent.KEYCODE_L,
			KeyEvent.KEYCODE_I
		)
		for (key in keys) {
			val forward = plain(key) as ReaderShortcut.Navigate
			val backward = shift(key) as ReaderShortcut.Navigate
			assertEquals(SegmentDirectionFfi.NEXT, forward.direction)
			assertEquals(SegmentDirectionFfi.PREVIOUS, backward.direction)
			assertEquals(forward.type, backward.type)
		}
	}

	// A key repeated by mistake would take an element kind out of reach with no error anywhere.
	@Test
	fun `no two element keys go to the same kind of element`() {
		val keys = listOf(
			KeyEvent.KEYCODE_H,
			KeyEvent.KEYCODE_P,
			KeyEvent.KEYCODE_G,
			KeyEvent.KEYCODE_F,
			KeyEvent.KEYCODE_K,
			KeyEvent.KEYCODE_T,
			KeyEvent.KEYCODE_S,
			KeyEvent.KEYCODE_L,
			KeyEvent.KEYCODE_I,
			KeyEvent.KEYCODE_LEFT_BRACKET
		)
		val types = keys.map { (plain(it) as ReaderShortcut.Navigate).type }
		assertEquals(keys.size, types.distinct().size)
	}

	// Sections get a bracket each, so Shift must not turn "next section" into "previous".
	@Test
	fun `the section brackets take no notice of shift`() {
		assertEquals(navigate(SegmentTypeFfi.SECTION, SegmentDirectionFfi.NEXT), plain(KeyEvent.KEYCODE_RIGHT_BRACKET))
		assertEquals(navigate(SegmentTypeFfi.SECTION, SegmentDirectionFfi.NEXT), shift(KeyEvent.KEYCODE_RIGHT_BRACKET))
		val previous = navigate(SegmentTypeFfi.SECTION, SegmentDirectionFfi.PREVIOUS)
		assertEquals(previous, plain(KeyEvent.KEYCODE_LEFT_BRACKET))
		assertEquals(previous, shift(KeyEvent.KEYCODE_LEFT_BRACKET))
	}

	@Test
	fun `the elements list opens whatever is held down with it`() {
		assertEquals(ReaderShortcut.OpenElements, plain(KeyEvent.KEYCODE_F7))
		assertEquals(ReaderShortcut.OpenElements, ctrl(KeyEvent.KEYCODE_F7))
		assertEquals(ReaderShortcut.OpenElements, ctrlShift(KeyEvent.KEYCODE_F7))
	}

	@Test
	fun `find next and previous are F3 on its own`() {
		assertEquals(ReaderShortcut.FindNext, plain(KeyEvent.KEYCODE_F3))
		assertEquals(ReaderShortcut.FindPrevious, shift(KeyEvent.KEYCODE_F3))
		assertNull(ctrl(KeyEvent.KEYCODE_F3))
	}

	@Test
	fun `the playback keys mean what the transport says`() {
		assertEquals(ReaderShortcut.TogglePlayback, plain(KeyEvent.KEYCODE_SPACE))
		assertEquals(ReaderShortcut.TogglePlayback, plain(KeyEvent.KEYCODE_MEDIA_PLAY_PAUSE))
		assertEquals(ReaderShortcut.PlayOnly, plain(KeyEvent.KEYCODE_MEDIA_PLAY))
		assertEquals(ReaderShortcut.PauseOnly, plain(KeyEvent.KEYCODE_MEDIA_PAUSE))
		assertEquals(ReaderShortcut.NextSegment, plain(KeyEvent.KEYCODE_MEDIA_NEXT))
		assertEquals(ReaderShortcut.PreviousSegment, plain(KeyEvent.KEYCODE_MEDIA_PREVIOUS))
		assertEquals(ReaderShortcut.HeadsetHook, plain(KeyEvent.KEYCODE_HEADSETHOOK))
	}

	// Anything not in the table has to be handed on, or typing in the reading view would start
	// swallowing keys that belong to the system or to a text field.
	@Test
	fun `a key that means nothing here is passed on`() {
		assertNull(plain(KeyEvent.KEYCODE_A))
		assertNull(plain(KeyEvent.KEYCODE_Z))
		assertNull(ctrl(KeyEvent.KEYCODE_A))
		assertNull(ctrl(KeyEvent.KEYCODE_SPACE))
		assertNull(plain(KeyEvent.KEYCODE_ENTER))
	}

	private fun navigate(
		type: SegmentTypeFfi,
		direction: SegmentDirectionFfi = SegmentDirectionFfi.NEXT
	) = ReaderShortcut.Navigate(type, direction)
}
