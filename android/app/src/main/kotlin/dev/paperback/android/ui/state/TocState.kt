package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.TocEntry

/**
 * The table of contents screen's own state: which rows are expanded, and which row the reader is
 * currently inside. One object rather than two flows so a recomposition can never pair a fresh
 * active row with a stale expansion set and scroll to the wrong place.
 */
data class TocUiState(
	val expandedIndices: Set<Int> = emptySet(),
	val activeIndex: Int? = null
)

/**
 * Where the table of contents is pointing. Expansion survives from one opening of the screen to
 * the next, so a reader who opened a branch finds it open again; only the active row is worked
 * out afresh each time from wherever reading has got to.
 */
class TocState {
	private val _state = MutableStateFlow(TocUiState())
	val state: StateFlow<TocUiState> = _state.asStateFlow()

	fun toggleExpanded(index: Int) {
		val expanded = _state.value.expandedIndices
		_state.value = _state.value.copy(
			expandedIndices = if (expanded.contains(index)) expanded - index else expanded + index
		)
	}

	/**
	 * Points the list at `position`: the nearest entry at or before it becomes the active one,
	 * and its ancestors are expanded so it is actually on screen when the list opens.
	 */
	fun pointAt(
		toc: List<TocEntry>,
		position: Long
	) {
		val activeIndex = activeEntryFor(toc, position)
		if (activeIndex == null) {
			_state.value = _state.value.copy(activeIndex = null)
			return
		}
		_state.value = TocUiState(
			expandedIndices = _state.value.expandedIndices + ancestorsOf(toc, activeIndex),
			activeIndex = activeIndex
		)
	}
}

/**
 * The entry the reader is inside at `position`: the last one starting at or before it. Null for
 * an empty table of contents. A position ahead of every entry (a document whose first heading is
 * some way in) counts as being in the first one, since that is the section being read.
 */
internal fun activeEntryFor(
	toc: List<TocEntry>,
	position: Long
): Int? {
	if (toc.isEmpty()) return null
	var activeIndex = 0
	var bestDistance = Long.MAX_VALUE
	for (i in toc.indices) {
		if (toc[i].position <= position) {
			val distance = position - toc[i].position
			if (distance < bestDistance) {
				bestDistance = distance
				activeIndex = i
			}
		}
	}
	return activeIndex
}

/**
 * The entries `index` sits under: walking back up the list, each first entry shallower than the
 * one before it, until the top level. These are the rows that have to be expanded for `index` to
 * be visible at all.
 */
internal fun ancestorsOf(
	toc: List<TocEntry>,
	index: Int
): Set<Int> {
	val ancestors = mutableSetOf<Int>()
	var currentLevel = toc[index].level
	for (i in index - 1 downTo 0) {
		if (toc[i].level < currentLevel) {
			ancestors.add(i)
			currentLevel = toc[i].level
			if (currentLevel == 0) break
		}
	}
	return ancestors
}
