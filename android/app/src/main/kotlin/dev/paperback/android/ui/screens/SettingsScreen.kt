package dev.paperback.android.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.*
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import dev.paperback.android.ui.components.PickerMenuItem
import dev.paperback.android.ui.components.pickerAnchorColors
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.ReaderSettings
import kotlin.math.roundToInt

private val MIN_SCALE = ReaderSettings.MIN_TEXT_SCALE_PERCENT.toFloat()

private val MAX_SCALE = ReaderSettings.MAX_TEXT_SCALE_PERCENT.toFloat()

/** Discrete slider positions between the bounds, one per [ReaderSettings.TEXT_SCALE_PERCENT_STEP]. */
private const val SCALE_STEPS =
	(ReaderSettings.MAX_TEXT_SCALE_PERCENT - ReaderSettings.MIN_TEXT_SCALE_PERCENT) /
		ReaderSettings.TEXT_SCALE_PERCENT_STEP - 1

/** Rounds a raw slider value so the text size only ever lands on a whole step. */
private fun snapScale(value: Float): Int =
	(value / ReaderSettings.TEXT_SCALE_PERCENT_STEP).roundToInt() *
		ReaderSettings.TEXT_SCALE_PERCENT_STEP

private val PERCENT_RANGE = 0f..100f

/** One step per whole percent, so TalkBack's swipe-to-adjust moves by one rather than jumping. */
private const val PERCENT_ACCESSIBILITY_STEPS = 99
private const val PERCENT_MIDPOINT = 50f

/**
 * A labelled dropdown over a fixed list of options, where the stored value is the option's index.
 * The readability choices all share the desktop's index meanings, so the index is the setting.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ChoiceSetting(
	label: String,
	options: List<String>,
	selectedIndex: Int,
	onSelect: (Int) -> Unit
) {
	var expanded by remember { mutableStateOf(false) }
	val selectedLabel = options.getOrElse(selectedIndex) { options.first() }
	ExposedDropdownMenuBox(
		expanded = expanded,
		onExpandedChange = { expanded = it }
	) {
		OutlinedButton(
			onClick = { expanded = true },
			colors = pickerAnchorColors(),
			modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth()
		) {
			Text("$label: $selectedLabel", modifier = Modifier.weight(1f))
			ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded)
		}
		ExposedDropdownMenu(
			expanded = expanded,
			onDismissRequest = { expanded = false }
		) {
			options.forEachIndexed { index, option ->
				PickerMenuItem(label = option, selected = index == selectedIndex) {
					onSelect(index)
					expanded = false
				}
			}
		}
	}
}

/**
 * A speech setting on a 0 to 100 scale, with the slider semantics a screen reader needs to
 * announce and change it: the label and the percentage are folded into the group so TalkBack
 * reads one control rather than a heading and a slider separately.
 *
 * A setting following the engine's own default has nothing to show, so it reads as the system
 * default and reports itself disabled rather than showing a figure the reader cannot change.
 */
@Composable
private fun PercentSetting(
	label: String,
	percentage: Int,
	isSystemDefault: Boolean,
	onChange: (Int) -> Unit
) {
	Column(
		modifier = Modifier.clearAndSetSemantics {
			contentDescription = label
			if (isSystemDefault) {
				// TRANSLATORS: TalkBack value announced when a TTS setting is following the system/engine default rather than a custom value
				stateDescription = t("System Default")
				disabled()
			} else {
				stateDescription = t("{} percent", percentage.toString())
				progressBarRangeInfo = ProgressBarRangeInfo(
					current = percentage.toFloat(),
					range = PERCENT_RANGE,
					steps = PERCENT_ACCESSIBILITY_STEPS
				)
				setProgress { targetValue ->
					onChange(targetValue.roundToInt())
					true
				}
			}
		}
	) {
		val readout = if (isSystemDefault) t("System Default") else "$percentage%"
		Text("$label: $readout", style = MaterialTheme.typography.labelLarge)
		Slider(
			value = if (isSystemDefault) PERCENT_MIDPOINT else percentage.toFloat(),
			onValueChange = { onChange(it.roundToInt()) },
			valueRange = PERCENT_RANGE,
			steps = 0,
			enabled = !isSystemDefault
		)
	}
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
	viewModel: MainScreenViewModel = viewModel(),
	onDismiss: () -> Unit
) {
	val settings = viewModel.settings
	val restorePreviousDocuments by settings.restorePreviousDocuments.state.collectAsStateWithLifecycle()
	val useInAppFileBrowser by settings.useInAppFileBrowser.state.collectAsStateWithLifecycle()
	val swipeUpMovesForward by settings.swipeUpMovesForward.state.collectAsStateWithLifecycle()
	val textScalePercent by settings.textScalePercent.state.collectAsStateWithLifecycle()
	val lineSpacing by settings.lineSpacing.state.collectAsStateWithLifecycle()
	val paragraphSpacing by settings.paragraphSpacing.state.collectAsStateWithLifecycle()
	val textAlignment by settings.textAlignment.state.collectAsStateWithLifecycle()
	val currentSpeechRate by viewModel.ttsManager.currentSpeechRate.collectAsStateWithLifecycle()
	val currentPitch by viewModel.ttsManager.currentPitch.collectAsStateWithLifecycle()
	val availableVoices by viewModel.ttsManager.availableVoices.collectAsStateWithLifecycle()
	val currentVoice by viewModel.ttsManager.currentVoice.collectAsStateWithLifecycle()
	val currentEngineName by viewModel.ttsManager.currentEngineName.collectAsStateWithLifecycle()
	val engines = viewModel.ttsManager.getAvailableEngines()
	val currentEngine = currentEngineName ?: viewModel.ttsManager.getDefaultEngine()

	var engineExpanded by remember { mutableStateOf(false) }
	var voiceExpanded by remember { mutableStateOf(false) }

	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = t("Settings") },
		color = MaterialTheme.colorScheme.surface
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = {
					// TRANSLATORS: Title of the app settings screen
					Text(t("Settings"))
				},
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						// TRANSLATORS: Accessibility label for the back button that leaves the settings screen
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				}
			)
			Column(
				modifier = Modifier
					.fillMaxSize()
					.verticalScroll(rememberScrollState())
					.padding(16.dp)
					// Scrolled to the end, Play Sample otherwise sits under the navigation bar. The
					// top bar pads itself, so the bottom is the only side that belongs here.
					.windowInsetsPadding(WindowInsets.safeDrawing.only(WindowInsetsSides.Bottom))
			) {
				// TRANSLATORS: Section heading for general (non-speech) settings
				Text(
					t("General"),
					style = MaterialTheme.typography.titleMedium,
					modifier = Modifier.padding(bottom = 8.dp).semantics { heading() }
				)

				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(
							value = restorePreviousDocuments,
							onValueChange = { settings.restorePreviousDocuments.set(it) },
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
							onValueChange = { settings.useInAppFileBrowser.set(it) },
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
							onValueChange = { settings.swipeUpMovesForward.set(it) },
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
				// TRANSLATORS: Explanation shown under the "Swipe up moves forward" switch
				Text(
					t("With TalkBack, swipe up on the play button to move forward, or down to move back."),
					style = MaterialTheme.typography.bodySmall,
					color = MaterialTheme.colorScheme.onSurfaceVariant
				)

				Spacer(modifier = Modifier.height(24.dp))
				// TRANSLATORS: Section header in Settings grouping controls for how document text is displayed
				Text(
					t("Readability"),
					style = MaterialTheme.typography.titleMedium,
					modifier = Modifier.padding(bottom = 8.dp).semantics { heading() }
				)

				// TRANSLATORS: Row label for the control that scales the size of document text
				val textSizeLabel = t("Text Size")
				Column(
					modifier = Modifier.clearAndSetSemantics {
						contentDescription = textSizeLabel
						stateDescription = t("{} percent", textScalePercent.toString())
						progressBarRangeInfo = ProgressBarRangeInfo(
							current = textScalePercent.toFloat(),
							range = MIN_SCALE..MAX_SCALE,
							steps = SCALE_STEPS
						)
						setProgress { targetValue ->
							settings.textScalePercent.set(snapScale(targetValue))
							true
						}
					}
				) {
					Text("$textSizeLabel: $textScalePercent%", style = MaterialTheme.typography.labelLarge)
					// The track draws one tick per step, which at this many steps reads as a dotted
					// line rather than a slider. snapScale already pins the value to whole steps, and
					// the step count TalkBack swipes by lives in the semantics block above, so the
					// visible slider gives up its ticks without either behaviour changing.
					Slider(
						value = textScalePercent.toFloat(),
						onValueChange = { settings.textScalePercent.set(snapScale(it)) },
						valueRange = MIN_SCALE..MAX_SCALE,
						steps = 0
					)
				}
				Spacer(modifier = Modifier.height(16.dp))
				ChoiceSetting(
					// TRANSLATORS: Label for the picker choosing how much space sits between lines of text
					label = t("Line Spacing"),
					options = listOf(
						// TRANSLATORS: Default spacing option, shown in the line and paragraph spacing pickers
						t("Normal"),
						// TRANSLATORS: 1.5x line spacing option
						t("1.5×"),
						// TRANSLATORS: Double line spacing option
						t("Double")
					),
					selectedIndex = lineSpacing,
					onSelect = { settings.lineSpacing.set(it) }
				)
				Spacer(modifier = Modifier.height(16.dp))
				ChoiceSetting(
					// TRANSLATORS: Label for the picker choosing how much space sits between paragraphs
					label = t("Paragraph Spacing"),
					options = listOf(
						t("Normal"),
						// TRANSLATORS: Relaxed paragraph spacing option
						t("Relaxed"),
						// TRANSLATORS: Wide paragraph spacing option
						t("Wide")
					),
					selectedIndex = paragraphSpacing,
					onSelect = { settings.paragraphSpacing.set(it) }
				)
				Spacer(modifier = Modifier.height(16.dp))
				ChoiceSetting(
					// TRANSLATORS: Label for the picker choosing how document text is aligned
					label = t("Alignment"),
					options = listOf(
						// TRANSLATORS: Left text alignment option
						t("Left"),
						// TRANSLATORS: Center text alignment option
						t("Center"),
						// TRANSLATORS: Right text alignment option
						t("Right"),
						// TRANSLATORS: Justified text alignment option
						t("Justify")
					),
					selectedIndex = textAlignment,
					onSelect = { settings.textAlignment.set(it) }
				)

				Spacer(modifier = Modifier.height(24.dp))
				// TRANSLATORS: Section heading for text-to-speech (read-aloud) settings
				Text(
					t("Text-to-Speech"),
					style = MaterialTheme.typography.titleMedium,
					modifier = Modifier.padding(bottom = 8.dp).semantics { heading() }
				)

				ExposedDropdownMenuBox(
					expanded = engineExpanded,
					onExpandedChange = { engineExpanded = it }
				) {
					OutlinedButton(
						onClick = { engineExpanded = true },
						colors = pickerAnchorColors(),
						modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth()
					) {
						// TRANSLATORS: Value shown when a setting is following the system/engine default
						val selectedName = engines.find { it.name == currentEngine }?.label ?: currentEngine ?: t("Default")
						// TRANSLATORS: Label for the dropdown choosing which text-to-speech engine to speak with
						Text("${t("Speech Engine")}: $selectedName", modifier = Modifier.weight(1f))
						ExposedDropdownMenuDefaults.TrailingIcon(expanded = engineExpanded)
					}
					ExposedDropdownMenu(
						expanded = engineExpanded,
						onDismissRequest = { engineExpanded = false }
					) {
						engines.forEach { engine ->
							PickerMenuItem(label = engine.label, selected = engine.name == currentEngine) {
								viewModel.ttsManager.setEngine(engine.name)
								engineExpanded = false
							}
						}
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				val isSystemDefault = currentEngine == dev.paperback.android.tts.TtsManager.SYSTEM_DEFAULT
				ExposedDropdownMenuBox(
					expanded = voiceExpanded,
					onExpandedChange = { if (!isSystemDefault) voiceExpanded = it }
				) {
					OutlinedButton(
						onClick = { voiceExpanded = true },
						colors = pickerAnchorColors(),
						modifier = Modifier.menuAnchor(type = ExposedDropdownMenuAnchorType.PrimaryNotEditable).fillMaxWidth(),
						enabled = !isSystemDefault
					) {
						val voiceName = currentVoice?.name ?: t("Default")
						// TRANSLATORS: Label for the dropdown choosing which text-to-speech voice to speak with
						Text("${t("Voice")}: $voiceName", modifier = Modifier.weight(1f))
						ExposedDropdownMenuDefaults.TrailingIcon(expanded = voiceExpanded)
					}
					ExposedDropdownMenu(
						expanded = voiceExpanded,
						onDismissRequest = { voiceExpanded = false }
					) {
						availableVoices.forEach { voice ->
							PickerMenuItem(label = voice.name, selected = voice.name == currentVoice?.name) {
								viewModel.ttsManager.setVoice(voice)
								voiceExpanded = false
							}
						}
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				PercentSetting(
					// TRANSLATORS: TalkBack label for the speech rate slider
					label = t("Speech Rate"),
					percentage = currentSpeechRate,
					isSystemDefault = isSystemDefault,
					onChange = { viewModel.ttsManager.setSpeechRate(it) }
				)
				Spacer(modifier = Modifier.height(16.dp))
				PercentSetting(
					// TRANSLATORS: TalkBack label for the speech pitch slider
					label = t("Pitch"),
					percentage = currentPitch,
					isSystemDefault = isSystemDefault,
					onChange = { viewModel.ttsManager.setPitch(it) }
				)
				Spacer(modifier = Modifier.height(16.dp))
				Button(
					onClick = {
						// TRANSLATORS: Sentence spoken aloud to preview the selected speech engine, voice, rate and pitch
						viewModel.ttsManager.speak(t("This is a sample of the selected speech engine."), isSample = true)
					},
					modifier = Modifier.fillMaxWidth()
				) {
					// TRANSLATORS: Button to speak a sample sentence using the currently selected TTS voice/rate/pitch
					Text(t("Play Sample"))
				}
			}
		}
	}
}
