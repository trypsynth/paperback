package dev.paperback.android.ui.state

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.TocEntry

/** One object rather than two flows so a recomposition can never pair a fresh active row with a stale expansion set and scroll to the wrong place. */
data class TocUiState(val expandedIndices: Set<Int> = emptySet(), val activeIndex: Int? = null)

/** Expansion survives between openings so a branch the reader opened is open again; only the active row is recomputed. */
class TocState {
	private val _state = MutableStateFlow(TocUiState())
	val state: StateFlow<TocUiState> = _state.asStateFlow()

	fun toggleExpanded(index: Int) {
		val expanded = _state.value.expandedIndices
		_state.value = _state.value.copy(
			expandedIndices = if (expanded.contains(index)) expanded - index else expanded + index
		)
	}

	fun pointAt(toc: List<TocEntry>, position: Long) {
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

/** A position before every entry counts as the first one, since that is the section being read. */
internal fun activeEntryFor(toc: List<TocEntry>, position: Long): Int? {
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

internal fun ancestorsOf(toc: List<TocEntry>, index: Int): Set<Int> {
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
