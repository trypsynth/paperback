package dev.paperback.android.ui

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * A dialog's open/closed flag as one reusable unit, replacing the
 * MutableStateFlow<Boolean> + asStateFlow() + open()/close() pattern that used to be
 * hand-copied per dialog in MainScreenViewModel.
 */
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
