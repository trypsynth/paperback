package dev.paperback.mobile.ui

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
		SegmentTypeFfi.IMAGE -> "Image"
		SegmentTypeFfi.FIGURE -> "Figure"
	}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TtsBottomBar(
	isSpeaking: Boolean,
	onPlayPause: () -> Unit,
	onPrev: () -> Unit,
	onNext: () -> Unit,
	onPrevButton: () -> Unit,
	onNextButton: () -> Unit,
	currentSegmentType: SegmentTypeFfi,
	supportedSegmentTypes: List<SegmentTypeFfi>,
	onSegmentTypeChange: (SegmentTypeFfi) -> Unit,
	modifier: Modifier = Modifier
) {
	var dropdownExpanded by remember { mutableStateOf(false) }
	val types = supportedSegmentTypes
	val segmentTypeName = getSegmentTypeName(currentSegmentType)
	val currentTypeIndex = types.indexOf(currentSegmentType)

	BottomAppBar(
		modifier = modifier,
		actions = {
			// Unit selector: chip for sighted users (tap to open menu), swipe slider for TalkBack.
			Box {
				FilterChip(
					selected = false,
					onClick = { dropdownExpanded = true },
					label = { Text(segmentTypeName) },
					trailingIcon = {
						Icon(Icons.Filled.ArrowDropDown, contentDescription = null)
					},
					modifier = Modifier.clearAndSetSemantics {
						contentDescription = "Navigation unit"
						stateDescription = segmentTypeName
						progressBarRangeInfo = ProgressBarRangeInfo(
							current = (SEEK_RANGE / 2).toFloat(),
							range = 0f..SEEK_RANGE.toFloat(),
							steps = SEEK_RANGE - 1,
						)
						setProgress { targetValue ->
							val current = SEEK_RANGE / 2
							val newPos = targetValue.roundToInt().coerceIn(0, SEEK_RANGE)
							val idx = if (currentTypeIndex == -1) 0 else currentTypeIndex
							when {
								newPos > current -> onSegmentTypeChange(
									types[(idx + 1) % types.size]
								)
								newPos < current -> onSegmentTypeChange(
									types[(idx - 1 + types.size) % types.size]
								)
							}
							true
						}
						onClick(label = "Select navigation unit") { dropdownExpanded = true; true }
					}
				)
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

			IconButton(onClick = onPrevButton) {
				Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Previous $segmentTypeName")
			}

			// Play/pause: tap to play/pause, swipe up/down (TalkBack) to seek by the current unit.
			Box(
				modifier = Modifier
					.size(48.dp)
					.clip(CircleShape)
					.combinedClickable(onClick = onPlayPause)
					.clearAndSetSemantics {
						role = Role.Button
						contentDescription = if (isSpeaking) "Pause" else "Play"
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
								newPos > current -> onNext()
								newPos < current -> onPrev()
							}
							true
						}
						onClick(label = "Activate") { onPlayPause(); true }
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
				Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = "Next $segmentTypeName")
			}
		},
	)
}
