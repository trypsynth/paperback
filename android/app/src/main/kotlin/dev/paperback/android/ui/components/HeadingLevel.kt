package dev.paperback.android.ui.components

import uniffi.paperback.MarkerType

internal fun headingLevelOf(markerType: MarkerType): Int? = when (markerType) {
	MarkerType.HEADING1 -> 1
	MarkerType.HEADING2 -> 2
	MarkerType.HEADING3 -> 3
	MarkerType.HEADING4 -> 4
	MarkerType.HEADING5 -> 5
	MarkerType.HEADING6 -> 6
	else -> null
}
