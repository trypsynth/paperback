package dev.paperback.android.ui

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * A pending request to push a screen onto the back stack, raised by something that can't reach the
 * back stack itself (a keyboard shortcut in MainActivity, a menu item deep inside MainScreen) and
 * consumed by MainNavigation once it has navigated.
 *
 * Distinct from [DialogState] on purpose: these destinations are screens, not dialogs, and nothing
 * reads the flag to decide whether one is currently showing — the back stack is what says that.
 */
class ScreenRequest {
	private val _isRequested = MutableStateFlow(false)
	val isRequested: StateFlow<Boolean> = _isRequested.asStateFlow()

	fun request() {
		_isRequested.value = true
	}

	fun consume() {
		_isRequested.value = false
	}
}
