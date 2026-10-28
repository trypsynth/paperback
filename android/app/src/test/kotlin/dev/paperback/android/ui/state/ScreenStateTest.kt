package dev.paperback.android.ui.state

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

	// Opened from both a menu and a shortcut, so neither can be a toggle.
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

	// Left set, the request would push the same screen again on the next recomposition.
	@Test
	fun `consuming a request that was never made is harmless`() {
		val request = ScreenRequest()
		request.consume()
		assertFalse(request.isRequested.value)
	}
}

class MainScreenUiStateTest {
	// The index and tab list update separately as tabs close, so the index can briefly point at nothing.
	@Test
	fun `there is no active tab when the list is empty`() {
		assertNull(MainScreenUiState.Success(emptyList(), 0).activeTab)
		assertNull(MainScreenUiState.Success(emptyList(), -1).activeTab)
		assertNull(MainScreenUiState.Success(emptyList(), 3).activeTab)
	}
}
