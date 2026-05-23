package dev.paperback.mobile.ui

import android.speech.tts.TextToSpeech
import android.speech.tts.Voice
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
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
	onEngineSelected: (String) -> Unit,
	onVoiceSelected: (Voice) -> Unit,
	onRateChanged: (Int) -> Unit,
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
				Text("Speech Engine", style = MaterialTheme.typography.labelLarge)
				Spacer(modifier = Modifier.height(4.dp))

				Box {
					OutlinedButton(onClick = { engineExpanded = true }, modifier = Modifier.fillMaxWidth()) {
						val selectedName =
							engines.find { it.name == currentEngine }?.label ?: currentEngine ?: "Default"
						Text(selectedName)
					}
					DropdownMenu(
						expanded = engineExpanded,
						onDismissRequest = { engineExpanded = false },
						modifier = Modifier.fillMaxWidth()
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

				Text("Voice", style = MaterialTheme.typography.labelLarge)
				Spacer(modifier = Modifier.height(4.dp))

				Box {
					OutlinedButton(onClick = {
						voiceExpanded = true
					}, modifier = Modifier.fillMaxWidth(), enabled = !isSystemDefault) {
						val voiceName = currentVoice?.name ?: "Default"
						Text(voiceName)
					}
					DropdownMenu(
						expanded = voiceExpanded,
						onDismissRequest = { voiceExpanded = false },
						modifier = Modifier.fillMaxWidth()
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

				Text("Speech Rate: $currentRate%", style = MaterialTheme.typography.labelLarge)
				Slider(
					value = currentRate.toFloat(),
					onValueChange = { onRateChanged(kotlin.math.round(it).toInt()) },
					valueRange = 0f..100f,
					steps = 99,
					enabled = !isSystemDefault,
					modifier = Modifier.semantics {
						stateDescription = "$currentRate percent"
					}
				)

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
