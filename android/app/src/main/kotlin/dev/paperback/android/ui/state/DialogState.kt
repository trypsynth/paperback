package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/** A dialog's open/closed flag, so each dialog does not need its own flow and open and close functions. */
class DialogState(initiallyOpen: Boolean = false) {
	private val _isOpen = MutableStateFlow(initiallyOpen)
	val isOpen: StateFlow<Boolean> = _isOpen.asStateFlow()

	fun open() {
		_isOpen.value = true
	}

	fun close() {
		_isOpen.value = false
	}
}
