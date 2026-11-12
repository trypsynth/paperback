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
import androidx.compose.ui.semantics.LiveRegionMode
import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.liveRegion
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.progressBarRangeInfo
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.setProgress
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import dev.paperback.android.ui.components.PickerMenuItem
import dev.paperback.android.ui.components.pickerAnchorColors
import dev.paperback.android.ui.state.DocumentTabState
import dev.paperback.android.ui.state.lineIndexFor

const val GO_TO_LINE = "Line"
const val GO_TO_PAGE = "Page"
const val GO_TO_PERCENTAGE = "Percentage"
const val GO_TO_TIME = "Time"

/** Literal `t()` calls so the string extractor can find them; `t(mode)` on a variable would leave nothing to extract. */
private fun goToModeName(mode: String): String = when (mode) {
	// TRANSLATORS: Go To mode that jumps to a line number
	GO_TO_LINE -> t("Line")
	// TRANSLATORS: Go To mode that jumps to a page number
	GO_TO_PAGE -> t("Page")
	// TRANSLATORS: Go To mode that jumps to a time in an audiobook, such as 1:23:45
	GO_TO_TIME -> t("Time")
	// TRANSLATORS: Go To mode that jumps to a percentage through the document
	else -> t("Percentage")
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun GoToDialog(
	docState: DocumentTabState,
	onDismiss: () -> Unit,
	onGoTo: (Int) -> Unit,
	initialMode: String = GO_TO_LINE,
	onSeekPercent: (Int) -> Boolean = { false },
	audioElapsedMs: Long? = null,
	onSeekTime: (Long) -> Boolean = { false }
) {
	val maxLines = remember(docState.session) { docState.session.lineCount() }
	val maxPages = remember(docState.session) { docState.session.pageCountFfi() }
	// An audio-only book's text is one blank line per file, so line and page numbers mean nothing in it.
	val modes = remember(docState.isAudioOnly, maxPages) {
		buildList {
			if (!docState.isAudioOnly) {
				add(GO_TO_LINE)
				if (maxPages > 0) {
					add(GO_TO_PAGE)
				}
			}
			add(GO_TO_PERCENTAGE)
			// Only where every file's length is known, since a time can't be placed otherwise.
			if (docState.session.audioTimeKnownFfi()) add(GO_TO_TIME)
		}
	}
	val currentMs = audioElapsedMs ?: 0L
	val currentTime = remember(docState.session, currentMs) { docState.session.formatAudioTimeFfi(currentMs) }
	val totalTime = remember(docState.session) {
		docState.session.formatAudioTimeFfi(docState.session.audioTotalDurationMsFfi())
	}
	var timeValue by remember { mutableStateOf(currentTime) }
	var timeError by remember { mutableStateOf<String?>(null) }
	var selectedMode by remember { mutableStateOf(if (initialMode in modes) initialMode else modes.first()) }
	var inputValue by remember { mutableStateOf("") }
	var sliderPercent by remember { mutableIntStateOf(0) }
	var dropdownExpanded by remember { mutableStateOf(false) }
	val onSubmit = submit@{
		if (selectedMode == GO_TO_TIME) {
			when (val targetMs = docState.session.audioTimeEntryFfi(timeValue, currentMs)) {
				// TRANSLATORS: Announced when what was typed in the Go to Time dialog is not a time
				-1L -> timeError = t("Type a time like 1:23:45, 23:45 or 45.")
				// TRANSLATORS: Announced when the time typed in the Go to Time dialog is past the end of the recording, or before its start
				-2L, -3L -> timeError = t("Time out of range.")
				else -> if (onSeekTime(targetMs)) onDismiss()
			}
			return@submit
		}
		// For audio, a percentage means running time rather than a share of the placeholder text.
		val seeked = selectedMode == GO_TO_PERCENTAGE && onSeekPercent(sliderPercent)
		val targetPos = if (seeked) {
			null
		} else {
			when (selectedMode) {
				GO_TO_PERCENTAGE -> docState.session.positionFromPercent(sliderPercent)
				else -> inputValue.toLongOrNull()?.let { value ->
					when (selectedMode) {
						GO_TO_LINE -> docState.session.positionFromLine(value.coerceIn(1L, maxLines))
						GO_TO_PAGE ->
							if (maxPages > 0) {
								docState.session.pageOffset(value.toInt().coerceIn(1, maxPages))
							} else {
								null
							}
						else -> null
					}
				}
			}
		}
		if (seeked) {
			onDismiss()
		} else if (targetPos != null) {
			val targetLine = docState.session.lineFromPosition(targetPos)
			val indexToScroll = lineIndexFor(targetLine)
			onGoTo(indexToScroll)
			onDismiss()
		}
	}
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = t("Go To") },
		// TRANSLATORS: Title of the dialog for jumping to a specific line, page, or percentage through the document
		title = { Text(t("Go To")) },
		text = {
			Column {
				if (modes.size > 1) {
					ExposedDropdownMenuBox(
						expanded = dropdownExpanded,
						onExpandedChange = { dropdownExpanded = it },
					) {
						OutlinedButton(
							onClick = { dropdownExpanded = true },
							colors = pickerAnchorColors(),
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
								PickerMenuItem(label = goToModeName(mode), selected = mode == selectedMode) {
									selectedMode = mode
									dropdownExpanded = false
								}
							}
						}
					}
					Spacer(modifier = Modifier.height(16.dp))
				}
				if (selectedMode == GO_TO_PERCENTAGE) {
					// A hundred ticks read as a dotted line, so the range info lives out here and TalkBack still swipes a percent at a time.
					Column(
						modifier = Modifier.clearAndSetSemantics {
							contentDescription = goToModeName(GO_TO_PERCENTAGE)
							// TRANSLATORS: TalkBack state of a slider; {} is the number it is set to
							stateDescription = t("{} percent", sliderPercent.toString())
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = sliderPercent.toFloat(),
								range = 0f..100f,
								steps = 99
							)
							setProgress { targetValue ->
								sliderPercent = kotlin.math.round(targetValue).toInt()
								true
							}
						}
					) {
						Text("$sliderPercent%", style = MaterialTheme.typography.labelLarge)
						Slider(
							value = sliderPercent.toFloat(),
							onValueChange = { sliderPercent = kotlin.math.round(it).toInt() },
							valueRange = 0f..100f,
							steps = 0,
							modifier = Modifier.fillMaxWidth()
						)
					}
				} else if (selectedMode == GO_TO_TIME) {
					TextField(
						value = timeValue,
						onValueChange = {
							timeValue = it
							timeError = null
						},
						// TRANSLATORS: Label of the field in the Go to Time dialog. The first {} is the time played so far and the second is the recording's length, both written like 1:02:03
						label = { Text(t("Go to time ({}/{}):", currentTime, totalTime)) },
						isError = timeError != null,
						supportingText = timeError?.let { error ->
							{ Text(error, modifier = Modifier.semantics { liveRegion = LiveRegionMode.Polite }) }
						},
						keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Ascii, imeAction = ImeAction.Go),
						keyboardActions = KeyboardActions(onGo = { onSubmit() }),
						singleLine = true,
						modifier = Modifier.fillMaxWidth()
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
