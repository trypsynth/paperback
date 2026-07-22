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
import dev.paperback.mobile.t

private const val SEEK_RANGE = 10000
// Zero-width space: satisfies TalkBack's non-null stateDescription check so it doesn't
// fall back to announcing the raw slider value, while reading aloud as nothing.
private const val ZWSP = "​"

fun getSegmentTypeName(type: SegmentTypeFfi): String =
	// TRANSLATORS: Name of a navigation/reading unit shown in the "jump by unit" picker and read-aloud controls (e.g. "Paragraph", "Line", "Heading")
	when (type) {
		// TRANSLATORS: Name of the "paragraph" reading/navigation unit
		SegmentTypeFfi.PARAGRAPH -> t("Paragraph")
		// TRANSLATORS: Name of the "line" reading/navigation unit
		SegmentTypeFfi.LINE -> t("Line")
		// TRANSLATORS: Name of the "heading" reading/navigation unit
		SegmentTypeFfi.HEADING -> t("Heading")
		// TRANSLATORS: Name of the "link" reading/navigation unit
		SegmentTypeFfi.LINK -> t("Link")
		// TRANSLATORS: Name of the "section" reading/navigation unit
		SegmentTypeFfi.SECTION -> t("Section")
		// TRANSLATORS: Name of the "page" reading/navigation unit
		SegmentTypeFfi.PAGE -> t("Page")
		// TRANSLATORS: Name of the "list" reading/navigation unit
		SegmentTypeFfi.LIST -> t("List")
		// TRANSLATORS: Name of the "list item" reading/navigation unit
		SegmentTypeFfi.LIST_ITEM -> t("List Item")
		// TRANSLATORS: Name of the "table" reading/navigation unit
		SegmentTypeFfi.TABLE -> t("Table")
		// TRANSLATORS: Name of the "separator" reading/navigation unit
		SegmentTypeFfi.SEPARATOR -> t("Separator")
		// TRANSLATORS: Name of the "image" reading/navigation unit
		SegmentTypeFfi.IMAGE -> t("Image")
		// TRANSLATORS: Name of the "figure" reading/navigation unit
		SegmentTypeFfi.FIGURE -> t("Figure")
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
	swipeUpMovesForward: Boolean = true,
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
						// TRANSLATORS: TalkBack label for the control that seeks between reading/navigation units (paragraph, line, heading, etc.)
						contentDescription = t("Navigation unit")
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
