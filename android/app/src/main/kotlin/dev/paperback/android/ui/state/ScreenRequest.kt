package dev.paperback.android.ui.state

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/** Distinct from [DialogState] on purpose: the back stack, not this flag, says whether the requested screen is showing. */
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

/** A request raised while another screen is in front waits until MainScreen is back. */
@Composable
fun OnScreenRequest(request: ScreenRequest, action: () -> Unit) {
	val requested by request.isRequested.collectAsStateWithLifecycle()
	LaunchedEffect(requested) {
		if (requested) {
			action()
			request.consume()
		}
	}
}
