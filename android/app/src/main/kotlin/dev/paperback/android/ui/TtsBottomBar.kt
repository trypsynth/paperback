package dev.paperback.android.ui

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
import kotlin.math.roundToInt

private const val SEEK_RANGE = 10000

// Zero-width space: satisfies TalkBack's non-null stateDescription check so it doesn't
// fall back to announcing the raw slider value, while reading aloud as nothing.
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
	modifier: Modifier = Modifier,
	swipeUpMovesForward: Boolean = true
) {
	var dropdownExpanded by remember { mutableStateOf(false) }
	val unitName = getNavUnitName(currentUnit)
	val currentUnitIndex = navUnits.indexOf(currentUnit)
	// A time unit means the prev/next controls seek the recording rather than stepping through
	// text, so they read as "back"/"forward" by that amount instead of "previous"/"next" thing.
	val isTimeUnit = currentUnit is NavUnit.Time
	val prevLabel = if (isTimeUnit) {
		// TRANSLATORS: TalkBack label for the read-aloud bar's back button when navigating audio by time; {} is an amount like "30 seconds"
		t("Back {}").replace("{}", unitName)
	} else {
		// TRANSLATORS: TalkBack label for the read-aloud bar's previous button; {} is a unit name like "Paragraph"
		t("Previous {}").replace("{}", unitName)
	}
	val nextLabel = if (isTimeUnit) {
		// TRANSLATORS: TalkBack label for the read-aloud bar's forward button when navigating audio by time; {} is an amount like "30 seconds"
		t("Forward {}").replace("{}", unitName)
	} else {
		// TRANSLATORS: TalkBack label for the read-aloud bar's next button; {} is a unit name like "Paragraph"
		t("Next {}").replace("{}", unitName)
	}

	BottomAppBar(
		modifier = modifier,
		actions = {
			// Unit selector: chip for sighted users (tap to open menu), swipe slider for TalkBack.
			Box {
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
						onClick(label = "Select navigation unit") {
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
						DropdownMenuItem(
							text = { Text(getNavUnitName(unit)) },
							onClick = {
								onNavUnitChange(unit)
								dropdownExpanded = false
							},
						)
					}
				}
			}

			IconButton(onClick = onPrevButton) {
				Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = prevLabel)
			}

			// Play/pause: tap to play/pause, swipe up/down (TalkBack) to seek by the current unit.
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
						onClick(label = "Activate") {
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

			IconButton(onClick = onNextButton) {
				Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = nextLabel)
			}
		},
	)
}
