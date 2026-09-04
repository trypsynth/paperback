package dev.paperback.android.ui

import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class DialogStateTest {
	@Test
	fun `a dialog starts closed unless it is asked to start open`() {
		assertFalse(DialogState().isOpen.value)
		assertTrue(DialogState(initiallyOpen = true).isOpen.value)
	}

	@Test
	fun `opening and closing move the flag`() {
		val dialog = DialogState()
		dialog.open()
		assertTrue(dialog.isOpen.value)
		dialog.close()
		assertFalse(dialog.isOpen.value)
	}

	// Both are reached from more than one place (a menu item and a keyboard shortcut, say), so
	// neither can be a toggle: a second open must leave it open, a second close closed.
	@Test
	fun `repeating an open or a close changes nothing`() {
		val dialog = DialogState()
		dialog.open()
		dialog.open()
		assertTrue(dialog.isOpen.value)
		dialog.close()
		dialog.close()
		assertFalse(dialog.isOpen.value)
	}
}

class ScreenRequestTest {
	@Test
	fun `a request stands until it is consumed`() {
		val request = ScreenRequest()
		assertFalse(request.isRequested.value)
		request.request()
		assertTrue(request.isRequested.value)
		request.consume()
		assertFalse(request.isRequested.value)
	}

	// MainNavigation consumes the request as soon as it has pushed the screen. Leaving it set
	// would push the same destination again on the next recomposition.
	@Test
	fun `consuming a request that was never made is harmless`() {
		val request = ScreenRequest()
		request.consume()
		assertFalse(request.isRequested.value)
	}
}

class MainScreenUiStateTest {
	// The index and the tab list are updated separately as tabs close, so activeTab has to cope
	// with an index that no longer points at anything rather than throwing mid-recomposition.
	@Test
	fun `there is no active tab when the list is empty`() {
		assertNull(MainScreenUiState.Success(emptyList(), 0).activeTab)
		assertNull(MainScreenUiState.Success(emptyList(), -1).activeTab)
		assertNull(MainScreenUiState.Success(emptyList(), 3).activeTab)
	}
}
