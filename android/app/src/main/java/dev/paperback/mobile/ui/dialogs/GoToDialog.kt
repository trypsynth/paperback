package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.ui.DocumentTabState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun GoToDialog(
	docState: DocumentTabState,
	onDismiss: () -> Unit,
	onGoTo: (Int) -> Unit
) {
	var selectedMode by remember { mutableStateOf("Line") }
	var inputValue by remember { mutableStateOf("") }
	var dropdownExpanded by remember { mutableStateOf(false) }
	
	val maxLines = remember(docState.session) { docState.session.lineCount() }
	val maxPages = remember(docState.session) { docState.session.pageCountFfi() }

	val onSubmit = {
		val value = inputValue.toLongOrNull()
		if (value != null) {
			val targetPos = when (selectedMode) {
				"Line" -> docState.session.positionFromLine(value.coerceIn(1L, maxLines))
				"Page" -> docState.session.pageOffsetFfi(value.toInt().coerceIn(1, maxPages))
				"Percentage" -> docState.session.positionFromPercentFfi(value.toInt().coerceIn(0, 100))
				else -> 0L
			}
			val targetLine = docState.session.lineFromPosition(targetPos)
			val indexToScroll = (targetLine - 1).toInt().coerceAtLeast(0)
			onGoTo(indexToScroll)
			onDismiss()
		}
	}

	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Go To" },
		title = { Text("Go To") },
		text = {
			Column {
				ExposedDropdownMenuBox(
					expanded = dropdownExpanded,
					onExpandedChange = { dropdownExpanded = it },
				) {
					OutlinedButton(
						onClick = { dropdownExpanded = true },
						modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth().semantics {
							customActions = listOfNotNull(
								if (selectedMode != "Line") {
									CustomAccessibilityAction("Switch to Go to Line") {
										selectedMode = "Line"
										true
									}
								} else {
									null
								},
								if (selectedMode != "Page") {
									CustomAccessibilityAction("Switch to Go to Page") {
										selectedMode = "Page"
										true
									}
								} else {
									null
								},
								if (selectedMode != "Percentage") {
									CustomAccessibilityAction("Switch to Go to Percentage") {
										selectedMode = "Percentage"
										true
									}
								} else {
									null
								}
							)
						}
					) {
						Text("Go To $selectedMode", modifier = Modifier.weight(1f))
						ExposedDropdownMenuDefaults.TrailingIcon(expanded = dropdownExpanded)
					}
					ExposedDropdownMenu(
						expanded = dropdownExpanded,
						onDismissRequest = { dropdownExpanded = false }
					) {
						DropdownMenuItem(
							text = { Text("Go To Line") },
							onClick = {
								selectedMode = "Line"
								dropdownExpanded = false
							}
						)
						DropdownMenuItem(
							text = { Text("Go To Page") },
							onClick = {
								selectedMode = "Page"
								dropdownExpanded = false
							}
						)
						DropdownMenuItem(
							text = { Text("Go To Percentage") },
							onClick = {
								selectedMode = "Percentage"
								dropdownExpanded = false
							}
						)
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				TextField(
					value = inputValue,
					onValueChange = { inputValue = it.filter { char -> char.isDigit() } },
					label = { Text("Enter $selectedMode") },
					keyboardOptions = KeyboardOptions(
						keyboardType = KeyboardType.Number,
						imeAction = ImeAction.Go
					),
					keyboardActions = KeyboardActions(
						onGo = { onSubmit() }
					),
					singleLine = true,
					modifier = Modifier.fillMaxWidth()
				)
			}
		},
		confirmButton = {
			TextButton(onClick = onSubmit) { Text("Go") }
		},
		dismissButton = {
			TextButton(onClick = onDismiss) { Text("Cancel") }
		}
	)
}
