package dev.paperback.android.ui.dialogs

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
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import dev.paperback.android.ui.DocumentTabState

const val GO_TO_LINE = "Line"
const val GO_TO_PAGE = "Page"
const val GO_TO_PERCENTAGE = "Percentage"

/**
 * The display name for a Go To mode.
 *
 * Each name is a literal `t()` call so the string extractor can find it; the modes are chosen
 * from a list at runtime, and `t(mode)` on a variable would leave nothing to extract.
 */
private fun goToModeName(mode: String): String =
	when (mode) {
		// TRANSLATORS: Go To mode that jumps to a line number
		GO_TO_LINE -> t("Line")
		// TRANSLATORS: Go To mode that jumps to a page number
		GO_TO_PAGE -> t("Page")
		// TRANSLATORS: Go To mode that jumps to a percentage through the document
		else -> t("Percentage")
	}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun GoToDialog(
	docState: DocumentTabState,
	onDismiss: () -> Unit,
	onGoTo: (Int) -> Unit,
	initialMode: String = GO_TO_LINE
) {
	val maxLines = remember(docState.session) { docState.session.lineCount() }
	val maxPages = remember(docState.session) { docState.session.pageCountFfi() }
	// An audio-only book's text is one blank line per audio file, so a line or page number means
	// nothing in it. A percentage still places the reader in the recording.
	val modes = remember(docState.isAudioOnly, maxPages) {
		buildList {
			if (!docState.isAudioOnly) {
				add(GO_TO_LINE)
				if (maxPages > 0) {
					add(GO_TO_PAGE)
				}
			}
			add(GO_TO_PERCENTAGE)
		}
	}
	var selectedMode by remember { mutableStateOf(if (initialMode in modes) initialMode else modes.first()) }
	var inputValue by remember { mutableStateOf("") }
	var sliderPercent by remember { mutableIntStateOf(0) }
	var dropdownExpanded by remember { mutableStateOf(false) }
	val onSubmit = {
		val targetPos = when (selectedMode) {
			GO_TO_PERCENTAGE -> docState.session.positionFromPercent(sliderPercent)
			else -> inputValue.toLongOrNull()?.let { value ->
				when (selectedMode) {
					GO_TO_LINE -> docState.session.positionFromLine(value.coerceIn(1L, maxLines))
					GO_TO_PAGE ->
						if (maxPages > 0) docState.session.pageOffset(value.toInt().coerceIn(1, maxPages)) else null
					else -> null
				}
			}
		}
		if (targetPos != null) {
			val targetLine = docState.session.lineFromPosition(targetPos)
			val indexToScroll = (targetLine - 1).toInt().coerceAtLeast(0)
			onGoTo(indexToScroll)
			onDismiss()
		}
	}
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Go To" },
		// TRANSLATORS: Title of the dialog for jumping to a specific line, page, or percentage through the document
		title = { Text(t("Go To")) },
		text = {
			Column {
				// With one mode there is nothing to choose, so the selector is left out rather
				// than shown holding a single entry.
				if (modes.size > 1) {
					ExposedDropdownMenuBox(
						expanded = dropdownExpanded,
						onExpandedChange = { dropdownExpanded = it },
					) {
						OutlinedButton(
							onClick = { dropdownExpanded = true },
							modifier = Modifier
								.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable)
								.fillMaxWidth()
								.semantics {
									customActions = modes.filter { it != selectedMode }.map { mode ->
										CustomAccessibilityAction(goToModeName(mode)) {
											selectedMode = mode
											true
										}
									}
								}
						) {
							Text(goToModeName(selectedMode), modifier = Modifier.weight(1f))
							ExposedDropdownMenuDefaults.TrailingIcon(expanded = dropdownExpanded)
						}
						ExposedDropdownMenu(
							expanded = dropdownExpanded,
							onDismissRequest = { dropdownExpanded = false }
						) {
							modes.forEach { mode ->
								DropdownMenuItem(
									text = { Text(goToModeName(mode)) },
									onClick = {
										selectedMode = mode
										dropdownExpanded = false
									}
								)
							}
						}
					}
					Spacer(modifier = Modifier.height(16.dp))
				}
				if (selectedMode == GO_TO_PERCENTAGE) {
					Text("$sliderPercent%", style = MaterialTheme.typography.labelLarge)
					Slider(
						value = sliderPercent.toFloat(),
						onValueChange = { sliderPercent = kotlin.math.round(it).toInt() },
						valueRange = 0f..100f,
						steps = 99,
						modifier = Modifier.fillMaxWidth().semantics {
							stateDescription = "$sliderPercent percent"
						}
					)
				} else {
					TextField(
						value = inputValue,
						onValueChange = { inputValue = it.filter { char -> char.isDigit() } },
						// TRANSLATORS: Placeholder in the Go To dialog's number field; {} is the mode name ("Line", "Page", or "Percentage")
						label = { Text(t("Enter {}", goToModeName(selectedMode))) },
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
			}
		},
		confirmButton = {
			// TRANSLATORS: Button to jump to the entered line/page/percentage
			TextButton(onClick = onSubmit) { Text(t("Go")) }
		},
		dismissButton = {
			// TRANSLATORS: Button to close the Go To dialog without navigating
			TextButton(onClick = onDismiss) { Text(t("Cancel")) }
		}
	)
}
