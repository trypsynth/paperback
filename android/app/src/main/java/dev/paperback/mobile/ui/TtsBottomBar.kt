package dev.paperback.mobile.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.semantics
import uniffi.paperback.SegmentTypeFfi

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

	BottomAppBar(
		modifier = modifier,
		actions = {
			val segmentTypeName = getSegmentTypeName(currentSegmentType)

			IconButton(onClick = onPrev) {
				Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Previous $segmentTypeName")
			}

			IconButton(onClick = onPlayPause) {
				if (isSpeaking) {
					Icon(
						imageVector = Icons.Filled.Pause,
						contentDescription = "Pause"
					)
				} else {
					Icon(
						imageVector = Icons.Filled.PlayArrow,
						contentDescription = "Play"
					)
				}
			}

			IconButton(onClick = onNext) {
				Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = "Next $segmentTypeName")
			}

			Box {
				TextButton(
					onClick = { dropdownExpanded = true },
					modifier = Modifier.semantics {
						customActions = SegmentTypeFfi
							.values()
							.filter { it != currentSegmentType }
							.map { type ->
								CustomAccessibilityAction("Switch to ${getSegmentTypeName(type)}") {
									onSegmentTypeChange(type)
									true
								}
							}
					}
				) {
					Text(segmentTypeName)
				}
				DropdownMenu(
					expanded = dropdownExpanded,
					onDismissRequest = { dropdownExpanded = false }
				) {
					SegmentTypeFfi.values().forEach { type ->
						DropdownMenuItem(
							text = { Text(getSegmentTypeName(type)) },
							onClick = {
								onSegmentTypeChange(type)
								dropdownExpanded = false
							}
						)
					}
				}
			}
		}
	)
}
