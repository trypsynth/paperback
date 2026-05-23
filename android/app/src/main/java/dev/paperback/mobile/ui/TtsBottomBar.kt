package dev.paperback.mobile.ui

import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.ArrowForward
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
import androidx.compose.ui.semantics.onLongClick
import androidx.compose.ui.semantics.progressBarRangeInfo
import androidx.compose.ui.semantics.role
import androidx.compose.ui.semantics.setProgress
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp
import kotlin.math.roundToInt
import uniffi.paperback.SegmentTypeFfi

private const val SEEK_RANGE = 10000
// Zero-width space: satisfies TalkBack's non-null stateDescription check so it doesn't
// fall back to announcing the raw slider value, while reading aloud as nothing.
private const val ZWSP = "​"

fun getSegmentTypeName(type: SegmentTypeFfi): String =
	when (type) {
		SegmentTypeFfi.PARAGRAPH -> "Paragraph"
		SegmentTypeFfi.LINE -> "Line"
		SegmentTypeFfi.HEADING -> "Heading"
		SegmentTypeFfi.LINK -> "Link"
		SegmentTypeFfi.SECTION -> "Section"
		SegmentTypeFfi.PAGE -> "Page"
		SegmentTypeFfi.LIST -> "List"
		SegmentTypeFfi.LIST_ITEM -> "List Item"
		SegmentTypeFfi.TABLE -> "Table"
		SegmentTypeFfi.SEPARATOR -> "Separator"
	}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TtsBottomBar(
	isSpeaking: Boolean,
	onPlayPause: () -> Unit,
	onPrev: () -> Unit,
	onNext: () -> Unit,
	currentSegmentType: SegmentTypeFfi,
	onSegmentTypeChange: (SegmentTypeFfi) -> Unit,
	modifier: Modifier = Modifier
) {
	var dropdownExpanded by remember { mutableStateOf(false) }
	val types = SegmentTypeFfi.entries
	val segmentTypeName = getSegmentTypeName(currentSegmentType)
	val currentTypeIndex = types.indexOf(currentSegmentType)

	BottomAppBar(
		modifier = modifier,
		actions = {
			// Unit selector: swipe up/down to cycle through navigation units with wrap-around.
			var unitSeekPosition by remember { mutableIntStateOf(SEEK_RANGE / 2) }
			Box(
				modifier = Modifier.clearAndSetSemantics {
					contentDescription = "Navigation unit"
					stateDescription = segmentTypeName
					progressBarRangeInfo = ProgressBarRangeInfo(
						current = unitSeekPosition.toFloat(),
						range = 0f..SEEK_RANGE.toFloat(),
						steps = SEEK_RANGE - 1,
					)
					setProgress { targetValue ->
						val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
						when {
							newPos > unitSeekPosition -> onSegmentTypeChange(
								types[(currentTypeIndex + 1) % types.size]
							)
							newPos < unitSeekPosition -> onSegmentTypeChange(
								types[(currentTypeIndex - 1 + types.size) % types.size]
							)
						}
						unitSeekPosition = newPos
						true
					}
				},
				contentAlignment = Alignment.Center,
			) {
				Text(text = segmentTypeName)
			}

			IconButton(onClick = onPrev) {
				Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Previous $segmentTypeName")
			}

			// Play/pause: double-tap to play/pause, swipe up/down to seek by the current unit.
			Box {
				var seekPosition by remember { mutableIntStateOf(SEEK_RANGE / 2) }
				Box(
					modifier = Modifier
						.size(48.dp)
						.clip(CircleShape)
						.combinedClickable(
							onClick = onPlayPause,
							onLongClick = { dropdownExpanded = true },
						)
						.clearAndSetSemantics {
							role = Role.Button
							contentDescription = if (isSpeaking) "Pause, button" else "Play, button"
							stateDescription = ZWSP
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = seekPosition.toFloat(),
								range = 0f..SEEK_RANGE.toFloat(),
								steps = SEEK_RANGE - 1,
							)
							setProgress { targetValue ->
								val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
								when {
									newPos > seekPosition -> onNext()
									newPos < seekPosition -> onPrev()
								}
								seekPosition = newPos
								true
							}
							onClick(label = "Activate") { onPlayPause(); true }
							onLongClick(label = "Select navigation unit") { dropdownExpanded = true; true }
						},
					contentAlignment = Alignment.Center,
				) {
					if (isSpeaking) {
						Icon(Icons.Filled.Pause, contentDescription = null)
					} else {
						Icon(Icons.Filled.PlayArrow, contentDescription = null)
					}
				}
				DropdownMenu(
					expanded = dropdownExpanded,
					onDismissRequest = { dropdownExpanded = false },
				) {
					types.forEach { type ->
						DropdownMenuItem(
							text = { Text(getSegmentTypeName(type)) },
							onClick = {
								onSegmentTypeChange(type)
								dropdownExpanded = false
							},
						)
					}
				}
			}

			IconButton(onClick = onNext) {
				Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = "Next $segmentTypeName")
			}
		},
	)
}
