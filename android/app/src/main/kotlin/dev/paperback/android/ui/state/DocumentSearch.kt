package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.SearchOptionsFfi

/** Doesn't run the search itself: where to start depends on Text or Read-Aloud mode, which only MainScreen knows. */
class DocumentSearch {
	private val _query = MutableStateFlow<String?>(null)

	val query: StateFlow<String?> = _query.asStateFlow()

	private val _options = MutableStateFlow<SearchOptionsFfi?>(null)

	val options: StateFlow<SearchOptionsFfi?> = _options.asStateFlow()

	private val _stepRequests = MutableSharedFlow<Boolean>(extraBufferCapacity = 1)

	/** True steps forward, false back. */
	val stepRequests: SharedFlow<Boolean> = _stepRequests.asSharedFlow()

	fun start(query: String, options: SearchOptionsFfi) {
		_query.value = query
		_options.value = options
	}

	/** Direction is deliberately not compared: stepping backward is the same search, and a new one would re-find the match the reader is on. */
	fun isSameAs(query: String, options: SearchOptionsFfi): Boolean {
		val running = _options.value ?: return false
		return _query.value == query &&
			running.matchCase == options.matchCase &&
			running.wholeWord == options.wholeWord &&
			running.regex == options.regex
	}

	fun clear() {
		_query.value = null
		_options.value = null
	}

	fun findNext() {
		_stepRequests.tryEmit(true)
	}

	fun findPrevious() {
		_stepRequests.tryEmit(false)
	}
}
