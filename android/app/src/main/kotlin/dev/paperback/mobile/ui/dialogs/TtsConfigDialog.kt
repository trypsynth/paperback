package dev.paperback.mobile.ui.dialogs

import android.speech.tts.TextToSpeech
import android.speech.tts.Voice
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.ProgressBarRangeInfo
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.disabled
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.progressBarRangeInfo
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.setProgress
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.unit.dp

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TtsConfigDialog(
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
	onDismiss: () -> Unit
) {
	var engineExpanded by remember { mutableStateOf(false) }
	var voiceExpanded by remember { mutableStateOf(false) }

	AlertDialog(
		modifier = Modifier.semantics { paneTitle = "TTS Configuration" },
		onDismissRequest = onDismiss,
		title = { Text("TTS Configuration") },
		text = {
			Column(modifier = Modifier.fillMaxWidth()) {
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
						contentDescription = "Speech Rate"
						if (isSystemDefault) {
							stateDescription = "System Default"
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
						contentDescription = "Pitch"
						if (isSystemDefault) {
							stateDescription = "System Default"
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
					Text("Play Sample")
				}
			}
		},
		confirmButton = {
			TextButton(onClick = onDismiss) {
				Text("Done")
			}
		}
	)
}
