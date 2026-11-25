package dev.paperback.android.ui.dialogs

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Checkbox
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.error
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.unit.dp
import dev.paperback.android.t
import dev.paperback.android.ui.screens.ChoiceSetting
import dev.paperback.android.ui.state.DictionaryVoice
import dev.paperback.android.ui.state.normalized
import dev.paperback.android.ui.state.ruleProblemMessage
import dev.paperback.android.ui.state.voiceFilterChoices
import dev.paperback.android.ui.state.voiceFilterLabel
import uniffi.paperback.MatchType
import uniffi.paperback.RuleProblem
import uniffi.paperback.RuleScope
import uniffi.paperback.SpeechRule

/** Checked by core on save, since core's regular expressions are what apply the rule. */
@Composable
fun SpeechRuleDialog(
	initial: SpeechRule,
	adding: Boolean,
	voices: List<DictionaryVoice>,
	problemOf: (SpeechRule) -> RuleProblem?,
	onSave: (SpeechRule) -> Unit,
	onDismiss: () -> Unit
) {
	var pattern by remember { mutableStateOf(initial.pattern) }
	var replacement by remember { mutableStateOf(initial.replacement) }
	var paragraph by remember { mutableStateOf(initial.scope == RuleScope.PARAGRAPH) }
	var wholeWord by remember { mutableStateOf(initial.wholeWord) }
	var regex by remember { mutableStateOf(initial.matchType == MatchType.REGEX) }
	var enabled by remember { mutableStateOf(initial.enabled) }
	var voiceFilter by remember { mutableStateOf(initial.voiceFilter) }
	var problem by remember { mutableStateOf<String?>(null) }
	val filters = remember(voices) { voiceFilterChoices(voices, initial.voiceFilter) }
	val patternFocus = remember { FocusRequester() }
	LaunchedEffect(Unit) { runCatching { patternFocus.requestFocus() } }
	val save: () -> Unit = {
		val rule = SpeechRule(
			enabled = enabled,
			scope = if (paragraph) RuleScope.PARAGRAPH else RuleScope.WORD,
			matchType = if (regex) MatchType.REGEX else MatchType.LITERAL,
			pattern = pattern,
			replacement = replacement,
			wholeWord = wholeWord,
			voiceFilter = voiceFilter
		).normalized()
		val found = problemOf(rule)
		if (found == null) {
			onSave(rule)
		} else {
			problem = ruleProblemMessage(found)
			// The error is read as part of the field once focus lands there, which is also where it gets fixed.
			runCatching { patternFocus.requestFocus() }
		}
	}
	val title = if (adding) {
		// TRANSLATORS: Title of the dialog that adds a speech dictionary rule
		t("Add Rule")
	} else {
		// TRANSLATORS: Title of the dialog that edits a speech dictionary rule
		t("Edit Rule")
	}
	val plainText = KeyboardOptions(capitalization = KeyboardCapitalization.None, autoCorrectEnabled = false)
	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = title },
		title = { Text(title) },
		text = {
			Column(modifier = Modifier.verticalScroll(rememberScrollState())) {
				val shownProblem = problem
				OutlinedTextField(
					value = pattern,
					onValueChange = {
						pattern = it
						problem = null
					},
					// TRANSLATORS: Placeholder for the text field where the user enters the text (or regex) to search for
					label = { Text(t("Pattern")) },
					singleLine = true,
					isError = shownProblem != null,
					// Spoken through the field's error instead, so it isn't read twice.
					supportingText = shownProblem?.let { { Text(it, modifier = Modifier.clearAndSetSemantics { }) } },
					keyboardOptions = plainText.copy(imeAction = ImeAction.Next),
					modifier = Modifier
						.fillMaxWidth()
						.focusRequester(patternFocus)
						.semantics { if (shownProblem != null) error(shownProblem) }
				)
				OutlinedTextField(
					value = replacement,
					onValueChange = { replacement = it },
					// TRANSLATORS: Placeholder for the text field where the user enters the replacement text
					label = { Text(t("Replacement")) },
					singleLine = true,
					keyboardOptions = plainText.copy(imeAction = ImeAction.Done),
					keyboardActions = KeyboardActions(onDone = { save() }),
					modifier = Modifier.fillMaxWidth()
				)
				Spacer(modifier = Modifier.height(16.dp))
				ChoiceSetting(
					// TRANSLATORS: Label of the choice between a rule matching words and one matching across a whole paragraph
					label = t("Applies to"),
					options = listOf(
						// TRANSLATORS: Speech dictionary rule scope: the rule matches words
						t("Words"),
						// TRANSLATORS: Speech dictionary rule scope: the rule matches across a whole paragraph, and can be a regular expression
						t("Whole paragraphs")
					),
					selectedIndex = if (paragraph) 1 else 0,
					onSelect = {
						paragraph = it == 1
						problem = null
					}
				)
				if (paragraph) {
					CheckRow(
						// TRANSLATORS: Toggle enabling regex matching for a paragraph-scope rule; "\1" is a regex backreference syntax and must stay untranslated
						label = t("Regular expression (\\1 = first capture group)"),
						checked = regex,
						onChange = {
							regex = it
							problem = null
						}
					)
				} else {
					// TRANSLATORS: Toggle limiting a word-scope rule to whole-word matches only (no partial-word matches)
					CheckRow(label = t("Whole word only"), checked = wholeWord, onChange = { wholeWord = it })
				}
				Spacer(modifier = Modifier.height(8.dp))
				ChoiceSetting(
					// TRANSLATORS: Row label leading to the voice-filter picker for this rule
					label = t("Voices"),
					options = filters.map(::voiceFilterLabel),
					selectedIndex = filters.indexOf(voiceFilter).coerceAtLeast(0),
					onSelect = { voiceFilter = filters[it] }
				)
				Row(
					modifier = Modifier
						.fillMaxWidth()
						.toggleable(value = enabled, onValueChange = { enabled = it }, role = Role.Switch)
						.padding(vertical = 8.dp),
					verticalAlignment = Alignment.CenterVertically,
					horizontalArrangement = Arrangement.SpaceBetween
				) {
					// TRANSLATORS: Toggle for whether this text-substitution rule is currently active
					Text(t("Enabled"), modifier = Modifier.weight(1f))
					Switch(checked = enabled, onCheckedChange = null)
				}
			}
		},
		confirmButton = {
			TextButton(onClick = save) {
				// TRANSLATORS: Button that saves what was typed and closes the editor, such as a speech rule or a bookmark's note
				Text(t("Save"))
			}
		},
		dismissButton = { TextButton(onClick = onDismiss) { Text(t("Cancel")) } }
	)
}

@Composable
private fun CheckRow(label: String, checked: Boolean, onChange: (Boolean) -> Unit) {
	Row(
		modifier = Modifier
			.fillMaxWidth()
			.toggleable(value = checked, onValueChange = onChange, role = Role.Checkbox)
			.padding(vertical = 8.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Checkbox(checked = checked, onCheckedChange = null)
		Text(label, modifier = Modifier.padding(start = 8.dp))
	}
}
