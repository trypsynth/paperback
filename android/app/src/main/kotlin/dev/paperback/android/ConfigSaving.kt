package dev.paperback.android

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.launch
import uniffi.paperback.ConfigManagerFfi

private const val SAVE_DELAY_MS = 2_000L

private val saveScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)

private val saveLock = Any()

private var pendingSave: Job? = null
private val configWritten = MutableSharedFlow<Boolean>(extraBufferCapacity = 1)
internal val configWrites = configWritten.asSharedFlow()

/** Each save rewrites the whole file and reading asks for one every paragraph, so a burst becomes one write; [saveNow] catches what is pending when the app leaves the screen. */
fun ConfigManagerFfi.saveSoon() {
	synchronized(saveLock) {
		if (pendingSave != null) return
		pendingSave =
			saveScope.launch {
				delay(SAVE_DELAY_MS)
				synchronized(saveLock) { pendingSave = null }
				flush()
				configWritten.tryEmit(false)
			}
	}
}

fun ConfigManagerFfi.saveNow() {
	synchronized(saveLock) {
		pendingSave?.cancel()
		pendingSave = null
	}
	flush()
	configWritten.tryEmit(true)
}
