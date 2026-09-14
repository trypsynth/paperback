package dev.paperback.android.ui.state

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

private const val SECONDS_PER_MINUTE = 60

/**
 * Counts down to stopping playback, in whole seconds so the status line can read the time left
 * out as it goes. Starting a timer while one is already running replaces it, so the reader
 * always gets the length they last asked for rather than whichever timer was set first.
 *
 * [onExpired] stops playback. It runs before [expired] is emitted, so anything listening for the
 * end (an announcement, say) can count on playback already being stopped by the time it hears.
 */
class SleepTimer(
	private val scope: CoroutineScope,
	private val onExpired: () -> Unit
) {
	private val _remaining = MutableStateFlow<Int?>(null)

	/** Seconds left, or null when no timer is running. */
	val remaining: StateFlow<Int?> = _remaining.asStateFlow()

	private val _expired = MutableSharedFlow<Unit>(extraBufferCapacity = 1)

	/** Emitted once each time a timer runs out. Cancelling one emits nothing. */
	val expired: SharedFlow<Unit> = _expired.asSharedFlow()

	private var job: Job? = null

	fun start(minutes: Int) {
		job?.cancel()
		job = scope.launch {
			var secondsLeft = minutes * SECONDS_PER_MINUTE
			_remaining.value = secondsLeft
			while (secondsLeft > 0) {
				delay(1000)
				secondsLeft--
				_remaining.value = secondsLeft
			}
			_remaining.value = null
			onExpired()
			_expired.emit(Unit)
		}
	}

	fun cancel() {
		job?.cancel()
		job = null
		_remaining.value = null
	}
}
