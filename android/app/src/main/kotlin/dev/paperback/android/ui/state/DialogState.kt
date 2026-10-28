package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

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
