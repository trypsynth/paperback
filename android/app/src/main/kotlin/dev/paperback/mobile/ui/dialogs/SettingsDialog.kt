package dev.paperback.mobile.ui.dialogs

import android.speech.tts.TextToSpeech
import android.speech.tts.Voice
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.*
import androidx.compose.ui.unit.dp
import dev.paperback.mobile.t

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsDialog(
	initialRestorePreviousDocuments: Boolean,
	initialUseInAppFileBrowser: Boolean,
	initialSwipeUpMovesForward: Boolean,
	engines: List<TextToSpeech.EngineInfo>,
	currentEngine: String?,
	voices: List<Voice>,
	currentVoice: Voice?,
	currentRate: Int,
	currentPitch: Int,
	onEngineSelected: (String) -> Unit,
	onVoiceSelected: (Voice) -> Unit,
	onRateChanged: (Int) -> Unit,
	onPitchChanged: (Int) -> Unit,
	onPlaySample: () -> Unit,
	onSaveOptions: (Boolean, Boolean, Boolean) -> Unit,
	onDismiss: () -> Unit
) {
	var restorePreviousDocuments by remember { mutableStateOf(initialRestorePreviousDocuments) }
	var useInAppFileBrowser by remember { mutableStateOf(initialUseInAppFileBrowser) }
	var swipeUpMovesForward by remember { mutableStateOf(initialSwipeUpMovesForward) }

	var engineExpanded by remember { mutableStateOf(false) }
	var voiceExpanded by remember { mutableStateOf(false) }

	AlertDialog(
		modifier = Modifier.semantics { paneTitle = "Settings" },
		onDismissRequest = onDismiss,
		// TRANSLATORS: Title of the app settings dialog
		title = { Text(t("Settings")) },
		text = {
			Column(modifier = Modifier.fillMaxWidth().verticalScroll(rememberScrollState())) {
				// TRANSLATORS: Section heading for general (non-speech) settings
				Text(t("General"), style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(bottom = 8.dp))

				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = restorePreviousDocuments,
							onValueChange = { restorePreviousDocuments = it },
							role = Role.Switch
						).padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					// TRANSLATORS: Settings switch: reopen the last-read book automatically on launch
					Text(t("Restore last open book"), modifier = Modifier.weight(1f))
					Switch(
						checked = restorePreviousDocuments,
						onCheckedChange = null
					)
				}
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = useInAppFileBrowser,
							onValueChange = { useInAppFileBrowser = it },
							role = Role.Switch
						).padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					// TRANSLATORS: Settings switch: use the app's built-in file browser instead of the system document picker
					Text(t("Use in-app file browser (requires All Files permission)"), modifier = Modifier.weight(1f))
					Switch(
						checked = useInAppFileBrowser,
						onCheckedChange = null
					)
				}
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = swipeUpMovesForward,
							onValueChange = { swipeUpMovesForward = it },
							role = Role.Switch
						).padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					// TRANSLATORS: Settings switch: choose whether swiping up in the reading view moves forward or backward
					Text(t("Swipe up moves forward"), modifier = Modifier.weight(1f))
					Switch(
						checked = swipeUpMovesForward,
						onCheckedChange = null
					)
				}

				Spacer(modifier = Modifier.height(24.dp))
				// TRANSLATORS: Section heading for text-to-speech (read-aloud) settings
				Text(t("Text-to-Speech"), style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(bottom = 8.dp))

				ExposedDropdownMenuBox(
					expanded = engineExpanded,
					onExpandedChange = { engineExpanded = it }
				) {
					OutlinedButton(
						onClick = { engineExpanded = true },
						modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth()
					) {
						val selectedName = engines.find { it.name == currentEngine }?.label ?: currentEngine ?: "Default"
						Text("Speech Engine: $selectedName", modifier = Modifier.weight(1f))
						ExposedDropdownMenuDefaults.TrailingIcon(expanded = engineExpanded)
					}
					ExposedDropdownMenu(
						expanded = engineExpanded,
						onDismissRequest = { engineExpanded = false }
					) {
						engines.forEach { engine ->
							DropdownMenuItem(
								text = { Text(engine.label) },
								onClick = {
									onEngineSelected(engine.name)
									engineExpanded = false
								}
							)
						}
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				val isSystemDefault = currentEngine == dev.paperback.mobile.tts.TtsManager.SYSTEM_DEFAULT
				ExposedDropdownMenuBox(
					expanded = voiceExpanded,
					onExpandedChange = { if (!isSystemDefault) voiceExpanded = it }
				) {
					OutlinedButton(
						onClick = { voiceExpanded = true },
						modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth(),
						enabled = !isSystemDefault
					) {
						val voiceName = currentVoice?.name ?: "Default"
						Text("Voice: $voiceName", modifier = Modifier.weight(1f))
						ExposedDropdownMenuDefaults.TrailingIcon(expanded = voiceExpanded)
					}
					ExposedDropdownMenu(
						expanded = voiceExpanded,
						onDismissRequest = { voiceExpanded = false }
					) {
						voices.forEach { voice ->
							DropdownMenuItem(
								text = { Text(voice.name) },
								onClick = {
									onVoiceSelected(voice)
									voiceExpanded = false
								}
							)
						}
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				Column(
					modifier = Modifier.clearAndSetSemantics {
						// TRANSLATORS: TalkBack label for the speech rate slider
						contentDescription = t("Speech Rate")
						if (isSystemDefault) {
							// TRANSLATORS: TalkBack value announced when a TTS setting is following the system/engine default rather than a custom value
							stateDescription = t("System Default")
							disabled()
						} else {
							stateDescription = "$currentRate percent"
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = currentRate.toFloat(),
								range = 0f..100f,
								steps = 99
							)
							setProgress { targetValue ->
								onRateChanged(kotlin.math.round(targetValue).toInt())
								true
							}
						}
					}
				) {
					val rateText = if (isSystemDefault) "Speech Rate: System Default" else "Speech Rate: $currentRate%"
					Text(rateText, style = MaterialTheme.typography.labelLarge)
					Slider(
						value = if (isSystemDefault) 50f else currentRate.toFloat(),
						onValueChange = { onRateChanged(kotlin.math.round(it).toInt()) },
						valueRange = 0f..100f,
						steps = 99,
						enabled = !isSystemDefault
					)
				}
				Spacer(modifier = Modifier.height(16.dp))
				Column(
					modifier = Modifier.clearAndSetSemantics {
						// TRANSLATORS: TalkBack label for the speech pitch slider
						contentDescription = t("Pitch")
						if (isSystemDefault) {
							stateDescription = t("System Default")
							disabled()
						} else {
							stateDescription = "$currentPitch percent"
							progressBarRangeInfo = ProgressBarRangeInfo(
								current = currentPitch.toFloat(),
								range = 0f..100f,
								steps = 99
							)
							setProgress { targetValue ->
								onPitchChanged(kotlin.math.round(targetValue).toInt())
								true
							}
						}
					}
				) {
					val pitchText = if (isSystemDefault) "Pitch: System Default" else "Pitch: $currentPitch%"
					Text(pitchText, style = MaterialTheme.typography.labelLarge)
					Slider(
						value = if (isSystemDefault) 50f else currentPitch.toFloat(),
						onValueChange = { onPitchChanged(kotlin.math.round(it).toInt()) },
						valueRange = 0f..100f,
						steps = 99,
						enabled = !isSystemDefault
					)
				}
				Spacer(modifier = Modifier.height(16.dp))
				Button(onClick = onPlaySample, modifier = Modifier.fillMaxWidth()) {
					// TRANSLATORS: Button to speak a sample sentence using the currently selected TTS voice/rate/pitch
					Text(t("Play Sample"))
				}
			}
		},
		dismissButton = {
			TextButton(onClick = onDismiss) {
				Text("Cancel")
			}
		},
		confirmButton = {
			TextButton(onClick = { onSaveOptions(restorePreviousDocuments, useInAppFileBrowser, swipeUpMovesForward) }) {
				Text("Save")
			}
		}
	)
}
