package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material.icons.filled.ExpandMore
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.SemanticsPropertyReceiver
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import dev.paperback.android.t

fun flattenVisibleTreeIndices(itemCount: Int, levelAt: (Int) -> Int, expandedIndices: Set<Int>): List<Int> {
	val result = mutableListOf<Int>()
	var skipLevelGreaterThan = Int.MAX_VALUE
	for (index in 0 until itemCount) {
		val level = levelAt(index)
		if (level > skipLevelGreaterThan) {
			continue
		} else {
			skipLevelGreaterThan = Int.MAX_VALUE
		}
		result.add(index)
		if (hasTreeChildren(itemCount, levelAt, index) && !expandedIndices.contains(index)) {
			skipLevelGreaterThan = level
		}
	}
	return result
}

fun hasTreeChildren(itemCount: Int, levelAt: (Int) -> Int, index: Int): Boolean =
	index + 1 < itemCount && levelAt(index + 1) > levelAt(index)

@Composable
fun TreeExpandChevron(hasChildren: Boolean, isExpanded: Boolean, onToggle: () -> Unit) {
	if (hasChildren) {
		Box(
			modifier = Modifier
				.size(36.dp)
				.pointerInput(Unit) {
					detectTapGestures(onTap = { onToggle() })
				},
			contentAlignment = Alignment.Center
		) {
			Icon(
				imageVector = if (isExpanded) {
					Icons.Filled.ExpandMore
				} else {
					Icons.AutoMirrored.Filled.KeyboardArrowRight
				},
				contentDescription = null,
				tint = MaterialTheme.colorScheme.onSurfaceVariant,
				modifier = Modifier.clearAndSetSemantics { }
			)
		}
	} else {
		Spacer(modifier = Modifier.width(36.dp))
	}
}

/** Call from within the row's own `semantics(mergeDescendants = true)` block. */
fun SemanticsPropertyReceiver.applyTreeExpandSemantics(
	hasChildren: Boolean,
	isExpanded: Boolean,
	onToggle: () -> Unit
) {
	if (hasChildren) {
		// TRANSLATORS: TalkBack state description for a table-of-contents or heading row announcing whether its children are shown
		stateDescription = if (isExpanded) t("Expanded") else t("Collapsed")
		customActions = listOf(
			CustomAccessibilityAction(
				// TRANSLATORS: TalkBack custom action toggling whether a table-of-contents or heading row's children are shown
				label = if (isExpanded) t("Collapse") else t("Expand"),
				action = {
					onToggle()
					true
				}
			)
		)
	}
}
