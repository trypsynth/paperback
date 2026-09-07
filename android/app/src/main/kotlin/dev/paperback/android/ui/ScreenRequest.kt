package dev.paperback.android.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * A pending request raised by something that can't carry it out itself (a keyboard shortcut in
 * MainActivity, a menu item deep inside MainScreen), consumed by whichever composable can:
 * MainNavigation for the screens that go on the back stack, MainScreen for the file picker Ctrl+O
 * asks for.
 *
 * Distinct from [DialogState] on purpose: nothing reads the flag to decide whether the thing it
 * asked for is currently showing. The back stack is what says that.
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

/**
 * Runs [action] once whenever [request] is raised, and consumes it.
 *
 * For the requests a keyboard shortcut leaves for MainScreen, which owns the file pickers the
 * shortcut needs. Raised while another screen is in front, one waits until MainScreen is back,
 * the same way a jump offset from the elements screen does.
 */
@Composable
fun OnScreenRequest(
	request: ScreenRequest,
	action: () -> Unit
) {
	val requested by request.isRequested.collectAsStateWithLifecycle()
	LaunchedEffect(requested) {
		if (requested) {
			action()
			request.consume()
		}
	}
}
