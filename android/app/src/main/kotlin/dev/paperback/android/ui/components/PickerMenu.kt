package dev.paperback.android.ui.components

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

/** The outlined default, onSurfaceVariant, makes a current value look disabled next to full-strength text. */
@Composable
fun pickerAnchorColors(): ButtonColors =
	ButtonDefaults.outlinedButtonColors(contentColor = MaterialTheme.colorScheme.onSurface)

@Composable
fun PickerMenuItem(label: String, selected: Boolean, onClick: () -> Unit) {
	DropdownMenuItem(
		// On the label so it merges into the click node, which is the one a screen reader reads out.
		text = { Text(label, modifier = Modifier.semantics { this.selected = selected }) },
		trailingIcon = if (selected) {
			{ Icon(Icons.Filled.Check, contentDescription = null) }
		} else {
			null
		},
		onClick = onClick
	)
}
