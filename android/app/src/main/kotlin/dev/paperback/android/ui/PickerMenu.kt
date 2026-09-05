package dev.paperback.android.ui

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.ButtonColors
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics

/**
 * Colours for a dropdown's anchor button.
 *
 * An outlined button draws its content in `onSurfaceVariant`, which is right for a button whose
 * label names an action but wrong for one whose label is a setting's current value: against the
 * full-strength text of the rows around it the value reads as greyed out, as if the control were
 * disabled. The value is drawn at the same strength as any other text instead, leaving the real
 * disabled colour to mean what it says.
 */
@Composable
fun pickerAnchorColors(): ButtonColors =
	ButtonDefaults.outlinedButtonColors(contentColor = MaterialTheme.colorScheme.onSurface)

/**
 * One option in a dropdown that picks a value, ticked when it is the value currently in force.
 *
 * Without the tick a menu of choices gives no sign of which one is already chosen, and the reader
 * has to close it and read the anchor to find out.
 */
@Composable
fun PickerMenuItem(
	label: String,
	selected: Boolean,
	onClick: () -> Unit
) {
	DropdownMenuItem(
		// The state rides on the label so it merges into the node the item's click builds, which is
		// the one a screen reader reads out.
		text = { Text(label, modifier = Modifier.semantics { this.selected = selected }) },
		trailingIcon = if (selected) {
			{ Icon(Icons.Filled.Check, contentDescription = null) }
		} else {
			null
		},
		onClick = onClick
	)
}
