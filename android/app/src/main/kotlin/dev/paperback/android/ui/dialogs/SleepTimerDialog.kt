package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreTime
import androidx.compose.material.icons.filled.Timer
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import dev.paperback.android.t

private val presets = listOf(5, 10, 15, 30, 45, 60)

@Composable
fun SleepTimerDialog(
	remainingSeconds: Int?,
	onSetTimer: (Int) -> Unit,
	onCancelTimer: () -> Unit,
	onDismiss: () -> Unit
) {
	var showCustomInput by remember { mutableStateOf(false) }
	var customMinutes by remember { mutableStateOf("") }
	val focusRequester = remember { FocusRequester() }

	LaunchedEffect(showCustomInput) {
		if (showCustomInput) {
			focusRequester.requestFocus()
		}
	}

	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Sleep Timer" },
		icon = { Icon(Icons.Filled.Timer, contentDescription = null) },
		// TRANSLATORS: Dialog title, switches between the main sleep timer view and the custom-minutes entry view
		title = { Text(if (showCustomInput) t("Custom Timer") else t("Sleep Timer")) },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
				if (showCustomInput) {
					OutlinedTextField(
						value = customMinutes,
						onValueChange = { newValue ->
							if (newValue.length <= 4 && newValue.all { it.isDigit() }) {
								customMinutes = newValue
							}
						},
						// TRANSLATORS: Label for the numeric input field where the user types a custom sleep timer duration in minutes
						label = { Text(t("Minutes")) },
						keyboardOptions = KeyboardOptions(
							keyboardType = KeyboardType.Number,
							imeAction = ImeAction.Done
						),
						keyboardActions = KeyboardActions(
							onDone = {
								val mins = customMinutes.toIntOrNull()
								if (mins != null && mins > 0) {
									onSetTimer(mins)
									onDismiss()
								}
							}
						),
						modifier = Modifier
							.fillMaxWidth()
							.focusRequester(focusRequester),
						singleLine = true
					)
					Spacer(modifier = Modifier.height(16.dp))
					Row(
						modifier = Modifier.fillMaxWidth(),
						horizontalArrangement = Arrangement.spacedBy(8.dp)
					) {
						OutlinedButton(
							onClick = { showCustomInput = false },
							modifier = Modifier.weight(1f)
						) {
							// TRANSLATORS: Button to return from the custom-minutes entry view to the main sleep timer view
							Text(t("Back"))
						}
						Button(
							onClick = {
								val mins = customMinutes.toIntOrNull()
								if (mins != null && mins > 0) {
									onSetTimer(mins)
									onDismiss()
								}
							},
							enabled = customMinutes.isNotEmpty() && (customMinutes.toIntOrNull() ?: 0) > 0,
							modifier = Modifier.weight(1f)
						) {
							// TRANSLATORS: Button to confirm and start the sleep timer with the custom minutes entered
							Text(t("Start"))
						}
					}
				} else {
					if (remainingSeconds != null) {
						val min = remainingSeconds / 60
						val sec = remainingSeconds % 60
						val timeText = "%d:%02d".format(min, sec)
						// TRANSLATORS: Sentence announced to screen readers with the sleep timer's remaining time; {} is replaced with e.g. "3:45"
						val remainingAnnouncement = t("Active: {} remaining", timeText)
						Surface(
							color = MaterialTheme.colorScheme.secondaryContainer,
							shape = MaterialTheme.shapes.medium,
							modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp)
						) {
							Row(
								modifier = Modifier
									.fillMaxWidth()
									.padding(16.dp)
									.clearAndSetSemantics { contentDescription = remainingAnnouncement },
								verticalAlignment = Alignment.CenterVertically
							) {
								Icon(
									Icons.Filled.Timer,
									contentDescription = null,
									tint = MaterialTheme.colorScheme.onSecondaryContainer
								)
								Spacer(modifier = Modifier.width(12.dp))
								Column {
									Text(
										text = timeText,
										style = MaterialTheme.typography.headlineSmall,
										color = MaterialTheme.colorScheme.onSecondaryContainer
									)
									Text(
										// TRANSLATORS: Unit label shown under the sleep timer's remaining time
										text = t("remaining"),
										style = MaterialTheme.typography.bodySmall,
										color = MaterialTheme.colorScheme.onSecondaryContainer
									)
								}
							}
						}
						OutlinedButton(
							onClick = {
								onCancelTimer()
								onDismiss()
							},
							modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp)
						) {
							// TRANSLATORS: Button to cancel the currently running sleep timer
							Text(t("Cancel Timer"))
						}
						// TRANSLATORS: Heading introducing the preset duration chips, shown while a sleep timer is already active
						Text(t("Change to:"), style = MaterialTheme.typography.labelMedium)
						Spacer(modifier = Modifier.height(8.dp))
					}
					FlowRow(
						horizontalArrangement = Arrangement.spacedBy(8.dp),
						verticalArrangement = Arrangement.spacedBy(8.dp),
						modifier = Modifier.fillMaxWidth()
					) {
						presets.forEach { minutes ->
							AssistChip(
								onClick = {
									onSetTimer(minutes)
									onDismiss()
								},
								// TRANSLATORS: Preset sleep timer duration chip; {} is replaced with the number of minutes
								label = { Text(t("{} minutes", "$minutes")) }
							)
						}
						AssistChip(
							onClick = { showCustomInput = true },
							// TRANSLATORS: Chip to switch to the custom-minutes entry view for the sleep timer
							label = { Text(t("Custom time...")) },
							leadingIcon = {
								Icon(
									Icons.Filled.MoreTime,
									contentDescription = null,
									modifier = Modifier.size(AssistChipDefaults.IconSize)
								)
							}
						)
					}
				}
			}
		},
		confirmButton = {},
		dismissButton = {
			// TRANSLATORS: Button to close the Sleep Timer dialog without changing anything
			TextButton(onClick = onDismiss) { Text(t("Close")) }
		}
	)
}
