package dev.paperback.android.ui.components

import uniffi.paperback.MarkerType

/**
 * The heading level a marker stands for, or null when it is not a heading at all. Levels run 1 to
 * 6 as they do in HTML, and the reading view uses the number both to size the line and to tell a
 * screen reader how deep in the structure it is.
 */
internal fun headingLevelOf(markerType: MarkerType): Int? =
	when (markerType) {
		MarkerType.HEADING1 -> 1
		MarkerType.HEADING2 -> 2
		MarkerType.HEADING3 -> 3
		MarkerType.HEADING4 -> 4
		MarkerType.HEADING5 -> 5
		MarkerType.HEADING6 -> 6
		else -> null
	}
