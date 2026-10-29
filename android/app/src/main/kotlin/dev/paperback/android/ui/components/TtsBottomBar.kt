package dev.paperback.android.ui.components

import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.onClick
import androidx.compose.ui.semantics.progressBarRangeInfo
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.setProgress
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import dev.paperback.android.ui.state.NavUnit
import dev.paperback.android.ui.state.getNavUnitName
import dev.paperback.android.ui.state.getNextUnitLabel
import dev.paperback.android.ui.state.getPreviousUnitLabel
import kotlin.math.roundToInt

private const val SEEK_RANGE = 10000

private val RATE_PRESETS = listOf(25, 50, 75, 100)

// Same as iOS; bigger steps overshoot, and rate is tuned once rather than swept through.
private const val RATE_STEP = 1

// A non-empty stateDescription stops TalkBack announcing the raw slider value, while reading aloud as nothing.
private const val ZWSP = "​"

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TtsBottomBar(
	isSpeaking: Boolean,
	onPlayPause: () -> Unit,
	onPrev: () -> Unit,
	onNext: () -> Unit,
	onPrevButton: () -> Unit,
	onNextButton: () -> Unit,
	currentUnit: NavUnit,
	navUnits: List<NavUnit>,
	onNavUnitChange: (NavUnit) -> Unit,
	// Null before the speech engine has started.
	speechRatePercent: Int?,
	onSpeechRateChange: (Int) -> Unit,
	modifier: Modifier = Modifier,
	swipeUpMovesForward: Boolean = true,
	hidePrevNextButtons: Boolean = false
) {
	var dropdownExpanded by remember { mutableStateOf(false) }
	var rateMenuExpanded by remember { mutableStateOf(false) }
	val unitName = getNavUnitName(currentUnit)
	val currentUnitIndex = navUnits.indexOf(currentUnit)
	val prevLabel = when (currentUnit) {
		is NavUnit.Time ->
			// TRANSLATORS: TalkBack label for the read-aloud bar's back button when navigating audio by time; {} is an amount like "30 seconds"
			t("Back {}").replace("{}", unitName)
		is NavUnit.Find ->
			// TRANSLATORS: TalkBack label for the read-aloud bar's previous button when navigating by Find matches
			t("Find Previous")
		is NavUnit.Segment -> getPreviousUnitLabel(currentUnit.type)
	}
	val nextLabel = when (currentUnit) {
		is NavUnit.Time ->
			// TRANSLATORS: TalkBack label for the read-aloud bar's forward button when navigating audio by time; {} is an amount like "30 seconds"
			t("Forward {}").replace("{}", unitName)
		is NavUnit.Find ->
			// TRANSLATORS: TalkBack label for the read-aloud bar's next button when navigating by Find matches
			t("Find Next")
		is NavUnit.Segment -> getNextUnitLabel(currentUnit.type)
	}

	BottomAppBar(modifier = modifier) {
		// Overlaid rather than in a Row with spacers, which would centre the transport in the space left after the selector, off the bar's centre.
		Box(modifier = Modifier.fillMaxWidth()) {
			Box(modifier = Modifier.align(Alignment.CenterStart)) {
				FilterChip(
					selected = false,
					onClick = { dropdownExpanded = true },
					label = { Text(unitName) },
					trailingIcon = {
						Icon(Icons.Filled.ArrowDropDown, contentDescription = null)
					},
					modifier = Modifier.clearAndSetSemantics {
						// TRANSLATORS: TalkBack label for the control that seeks between reading/navigation units (paragraph, line, heading, etc.)
						contentDescription = t("Navigation unit")
						stateDescription = unitName
						progressBarRangeInfo = ProgressBarRangeInfo(
							current = (SEEK_RANGE / 2).toFloat(),
							range = 0f..SEEK_RANGE.toFloat(),
							steps = SEEK_RANGE - 1,
						)
						setProgress { targetValue ->
							val current = SEEK_RANGE / 2
							val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
							val idx = if (currentUnitIndex == -1) 0 else currentUnitIndex
							when {
								newPos > current -> onNavUnitChange(
									navUnits[(idx + 1) % navUnits.size]
								)
								newPos < current -> onNavUnitChange(
									navUnits[(idx - 1 + navUnits.size) % navUnits.size]
								)
							}
							true
						}
						// TRANSLATORS: TalkBack action on the read-aloud bar's unit selector, which opens the list of navigation units
						onClick(label = t("Select navigation unit")) {
							dropdownExpanded = true
							true
						}
					}
				)
				DropdownMenu(
					expanded = dropdownExpanded,
					onDismissRequest = { dropdownExpanded = false },
				) {
					navUnits.forEach { unit ->
						PickerMenuItem(label = getNavUnitName(unit), selected = unit == currentUnit) {
							onNavUnitChange(unit)
							dropdownExpanded = false
						}
					}
				}
			}

			Row(
				modifier = Modifier.align(Alignment.Center),
				verticalAlignment = Alignment.CenterVertically
			) {
				IconButton(
					onClick = onPrevButton,
					modifier = if (hidePrevNextButtons) Modifier.clearAndSetSemantics { } else Modifier
				) {
					Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = prevLabel)
				}

				Box(
					modifier = Modifier
						.size(48.dp)
						.clip(CircleShape)
						.combinedClickable(onClick = onPlayPause)
						.clearAndSetSemantics {
							role = Role.Button
							// TRANSLATORS: TalkBack label for the central play/pause control in the read-aloud bar
							contentDescription = if (isSpeaking) t("Pause") else t("Play")
							stateDescription = ZWSP
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = (SEEK_RANGE / 2).toFloat(),
								range = 0f..SEEK_RANGE.toFloat(),
								steps = SEEK_RANGE - 1,
							)
							setProgress { targetValue ->
								val current = SEEK_RANGE / 2
								val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
								when {
									newPos > current -> if (swipeUpMovesForward) onNext() else onPrev()
									newPos < current -> if (swipeUpMovesForward) onPrev() else onNext()
								}
								true
							}
							// No label, so TalkBack says its own "activate" in the reader's language.
							onClick(label = null) {
								onPlayPause()
								true
							}
						},
					contentAlignment = Alignment.Center,
				) {
					if (isSpeaking) {
						Icon(Icons.Filled.Pause, contentDescription = null)
					} else {
						Icon(Icons.Filled.PlayArrow, contentDescription = null)
					}
				}

				IconButton(
					onClick = onNextButton,
					modifier = if (hidePrevNextButtons) Modifier.clearAndSetSemantics { } else Modifier
				) {
					Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = nextLabel)
				}
			}

			if (speechRatePercent != null) {
				Box(modifier = Modifier.align(Alignment.CenterEnd)) {
					FilterChip(
						selected = false,
						onClick = { rateMenuExpanded = true },
						label = { Text("$speechRatePercent%") },
						modifier = Modifier.clearAndSetSemantics {
							// TRANSLATORS: TalkBack label for the read-aloud bar's speech rate control
							contentDescription = t("Speech Rate")
							stateDescription = "$speechRatePercent%"
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = (SEEK_RANGE / 2).toFloat(),
								range = 0f..SEEK_RANGE.toFloat(),
								steps = SEEK_RANGE - 1,
							)
							setProgress { targetValue ->
								val current = SEEK_RANGE / 2
								val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
								when {
									newPos > current -> onSpeechRateChange(speechRatePercent + RATE_STEP)
									newPos < current -> onSpeechRateChange(speechRatePercent - RATE_STEP)
								}
								true
							}
							// TRANSLATORS: TalkBack action on the read-aloud bar's speech rate control, which opens the list of rates
							onClick(label = t("Select speech rate")) {
								rateMenuExpanded = true
								true
							}
						}
					)
					DropdownMenu(
						expanded = rateMenuExpanded,
						onDismissRequest = { rateMenuExpanded = false },
					) {
						RATE_PRESETS.forEach { percent ->
							PickerMenuItem(label = "$percent%", selected = percent == speechRatePercent) {
								onSpeechRateChange(percent)
								rateMenuExpanded = false
							}
						}
					}
				}
			}
		}
	}
}
