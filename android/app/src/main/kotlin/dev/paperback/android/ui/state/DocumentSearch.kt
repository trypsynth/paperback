package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.SearchOptionsFfi

/**
 * The search the reader currently has running, and the requests to step through its matches.
 *
 * The search itself is not run here: which document to search, and where in it to start, depend
 * on whether the reader is in Text mode or Read-Aloud mode, so [stepRequests] carries the
 * direction and MainScreen does the searching with what it knows.
 */
class DocumentSearch {
	private val _query = MutableStateFlow<String?>(null)

	/** What is being searched for, or null when no search is running. */
	val query: StateFlow<String?> = _query.asStateFlow()

	private val _options = MutableStateFlow<SearchOptionsFfi?>(null)

	/** The options the running search was started with, or null when none is. */
	val options: StateFlow<SearchOptionsFfi?> = _options.asStateFlow()

	private val _stepRequests = MutableSharedFlow<Boolean>(extraBufferCapacity = 1)

	/** Emits true to step to the next match, false for the previous one. */
	val stepRequests: SharedFlow<Boolean> = _stepRequests.asSharedFlow()

	fun start(
		query: String,
		options: SearchOptionsFfi
	) {
		_query.value = query
		_options.value = options
	}

	/**
	 * True when [query] and [options] describe the search that is already running. Direction is
	 * deliberately not compared: stepping backward through the matches of the search just run is
	 * still the same search, and treating it as a new one would re-find the match the reader is
	 * already sitting on.
	 */
	fun isSameAs(
		query: String,
		options: SearchOptionsFfi
	): Boolean {
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
