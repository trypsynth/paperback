package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.t

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
							Text(t("Start"))
						}
					}
				} else {
					if (remainingSeconds != null) {
						val min = remainingSeconds / 60
						val sec = remainingSeconds % 60
						Text(
							"Active: %d:%02d remaining".format(min, sec),
							style = MaterialTheme.typography.bodyLarge,
							modifier = Modifier.padding(bottom = 8.dp)
						)
						OutlinedButton(
							onClick = { onCancelTimer(); onDismiss() },
							modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp)
						) {
							Text(t("Cancel Timer"))
						}
						Text(t("Change to:"), style = MaterialTheme.typography.labelMedium)
						Spacer(modifier = Modifier.height(8.dp))
					}
					presets.chunked(2).forEach { row ->
						Row(
							modifier = Modifier.fillMaxWidth(),
							horizontalArrangement = Arrangement.spacedBy(8.dp)
						) {
							row.forEach { minutes ->
								OutlinedButton(
									onClick = { onSetTimer(minutes); onDismiss() },
									modifier = Modifier.weight(1f)
								) {
									Text("$minutes minutes")
								}
							}
							if (row.size == 1) {
								Spacer(modifier = Modifier.weight(1f))
							}
						}
						Spacer(modifier = Modifier.height(8.dp))
					}
					OutlinedButton(
						onClick = { showCustomInput = true },
						modifier = Modifier.fillMaxWidth()
					) {
						Text(t("Custom time..."))
					}
				}
			}
		},
		confirmButton = {},
		dismissButton = {
			TextButton(onClick = onDismiss) { Text(t("Close")) }
		}
	)
}
