package dev.paperback.android.ui.screens

import android.view.View
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.WindowInsetsSides
import androidx.compose.foundation.layout.asPaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.only
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusProperties
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.KeyEventType
import androidx.compose.ui.input.key.isCtrlPressed
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.onPreviewKeyEvent
import androidx.compose.ui.input.key.type
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.semantics.CustomAccessibilityAction
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.customActions
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.paneTitle
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.stateDescription
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import dev.paperback.android.t
import dev.paperback.android.ui.dialogs.SpeechRuleDialog
import dev.paperback.android.ui.state.DictionaryEntry
import dev.paperback.android.ui.state.DictionaryVoice
import dev.paperback.android.ui.state.MainScreenViewModel
import dev.paperback.android.ui.state.SpeechDictionary
import dev.paperback.android.ui.state.appliesTo
import dev.paperback.android.ui.state.newSpeechRule
import dev.paperback.android.ui.state.replacementText
import dev.paperback.android.ui.state.ruleSummary
import dev.paperback.android.ui.state.voicesText
import uniffi.paperback.SpeechRule

/** Which rule the editor is open on; a null id is a new rule. */
private data class RuleEdit(val id: Long?, val rule: SpeechRule)

private data class RuleAction(val label: String, val run: () -> Unit)

@Suppress("DEPRECATION")
private fun View.announce(message: String) = announceForAccessibility(message)

/** A real destination like Bookmarks, so the rules get the whole window and the system back gesture leaves them. */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SpeechDictionaryScreen(viewModel: MainScreenViewModel = viewModel(), onDismiss: () -> Unit) {
	// Read afresh on each visit, so rules brought in by a settings import show up.
	val dictionary = remember { SpeechDictionary(viewModel.configManager) }
	val entries by dictionary.entries.collectAsStateWithLifecycle()
	val voices = remember {
		viewModel.ttsManager.voicesForRules().map { DictionaryVoice(it.name, it.locale.toLanguageTag()) }
	}
	var editing by remember { mutableStateOf<RuleEdit?>(null) }
	var focusId by remember { mutableStateOf<Long?>(null) }
	var focusAdd by remember { mutableStateOf(false) }
	val addFocus = remember { FocusRequester() }
	val listState = rememberLazyListState()
	val view = LocalView.current
	LaunchedEffect(focusId) {
		// The flow, not the collected state, which can be a frame behind a rule just added.
		val index = dictionary.entries.value.indexOfFirst { it.id == focusId }
		if (index >= 0) listState.scrollToItem(index)
	}
	LaunchedEffect(focusAdd) {
		if (focusAdd) {
			focusAdd = false
			runCatching { addFocus.requestFocus() }
		}
	}
	// TRANSLATORS: Title of the screen listing the speech dictionary's rules, opened from the Text-to-Speech section of Settings
	val title = t("Speech Dictionary")
	Surface(
		modifier = Modifier.fillMaxSize().semantics { paneTitle = title },
		color = MaterialTheme.colorScheme.background
	) {
		Column(modifier = Modifier.fillMaxSize()) {
			TopAppBar(
				title = { Text(title) },
				navigationIcon = {
					IconButton(onClick = onDismiss) {
						Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = t("Back"))
					}
				},
				actions = {
					IconButton(
						onClick = { editing = RuleEdit(null, newSpeechRule()) },
						modifier = Modifier.focusRequester(addFocus)
					) {
						Icon(Icons.Filled.Add, contentDescription = t("Add Rule"))
					}
				}
			)
			if (entries.isEmpty()) {
				EmptyRules()
				return@Column
			}
			// Otherwise the navigation bar half covers the last row; the top bar pads itself.
			LazyColumn(
				state = listState,
				modifier = Modifier.fillMaxSize(),
				contentPadding = WindowInsets.safeDrawing.only(WindowInsetsSides.Bottom).asPaddingValues()
			) {
				itemsIndexed(entries, key = { _, entry -> entry.id }) { index, entry ->
					RuleRow(
						entry = entry,
						isFirst = index == 0,
						isLast = index == entries.lastIndex,
						focusNow = focusId == entry.id,
						onFocused = { focusId = null },
						onEdit = { editing = RuleEdit(entry.id, entry.rule) },
						onToggle = { dictionary.toggle(entry.id) },
						onMove = { by ->
							dictionary.move(entry.id, by)?.let { to ->
								// The row keeps focus as it moves, so only where it went is news.
								// TRANSLATORS: Announced after a speech dictionary rule is moved up or down; the first {} is its new position, the second how many rules there are
								view.announce(t("Moved to {} of {}", (to + 1).toString(), entries.size.toString()))
							}
						},
						onDelete = {
							val next = dictionary.delete(entry.id)
							// TRANSLATORS: Announced after a speech dictionary rule is deleted
							view.announce(t("Rule deleted"))
							if (next != null) focusId = next else focusAdd = true
						}
					)
				}
			}
		}
	}
	editing?.let { edit ->
		SpeechRuleDialog(
			initial = edit.rule,
			adding = edit.id == null,
			voices = voices,
			problemOf = dictionary::problemOf,
			onSave = { rule ->
				editing = null
				focusId = if (edit.id == null) dictionary.add(rule) else edit.id.also { dictionary.replace(it, rule) }
			},
			onDismiss = {
				editing = null
				if (edit.id == null) focusAdd = true else focusId = edit.id
			}
		)
	}
}

@Composable
private fun RuleRow(
	entry: DictionaryEntry,
	isFirst: Boolean,
	isLast: Boolean,
	focusNow: Boolean,
	onFocused: () -> Unit,
	onEdit: () -> Unit,
	onToggle: () -> Unit,
	onMove: (Int) -> Unit,
	onDelete: () -> Unit
) {
	val rule = entry.rule
	var menuExpanded by remember { mutableStateOf(false) }
	val focusRequester = remember { FocusRequester() }
	LaunchedEffect(focusNow) {
		if (focusNow) {
			runCatching { focusRequester.requestFocus() }
			onFocused()
		}
	}
	// TRANSLATORS: Menu item and TalkBack action moving a speech dictionary rule one place earlier, so it applies sooner
	val moveUp = RuleAction(t("Move Up")) { onMove(-1) }.takeUnless { isFirst }
	// TRANSLATORS: Menu item and TalkBack action moving a speech dictionary rule one place later, so it applies later
	val moveDown = RuleAction(t("Move Down")) { onMove(1) }.takeUnless { isLast }
	// TRANSLATORS: Menu item / TalkBack action deleting a speech dictionary rule
	val delete = RuleAction(t("Delete"), onDelete)
	val toggle = if (rule.enabled) {
		// TRANSLATORS: TalkBack action turning a speech dictionary rule off, so it no longer changes what is read aloud
		RuleAction(t("Turn Off"), onToggle)
	} else {
		// TRANSLATORS: TalkBack action turning a speech dictionary rule back on
		RuleAction(t("Turn On"), onToggle)
	}
	// Edit is the row's own click, and on or off is the switch beside it, so the menu holds the rest.
	val menuActions = listOfNotNull(moveUp, moveDown, delete)
	// TRANSLATORS: TalkBack state of a speech dictionary rule that is on
	val on = t("On")
	// TRANSLATORS: TalkBack state of a speech dictionary rule that is off, so it changes nothing
	val off = t("Off")
	Row(modifier = Modifier.fillMaxWidth().padding(start = 16.dp), verticalAlignment = Alignment.CenterVertically) {
		Column(
			modifier = Modifier
				.weight(1f)
				.focusRequester(focusRequester)
				.onPreviewKeyEvent { event ->
					if (event.type != KeyEventType.KeyDown) return@onPreviewKeyEvent false
					// The same keys as the desktop list: Control with an arrow moves, Delete deletes, Space switches.
					when {
						event.isCtrlPressed && event.key == Key.DirectionUp -> onMove(-1)
						event.isCtrlPressed && event.key == Key.DirectionDown -> onMove(1)
						event.key == Key.Delete -> onDelete()
						event.key == Key.Spacebar -> onToggle()
						else -> return@onPreviewKeyEvent false
					}
					true
				}.clickable(
					// TRANSLATORS: TalkBack action label for a rule in the Speech Dictionary, read as "double tap to edit rule"
					onClickLabel = t("edit rule"),
					onClick = onEdit
				).semantics(mergeDescendants = true) {
					contentDescription = ruleSummary(rule)
					stateDescription = if (rule.enabled) on else off
					customActions = (listOf(toggle) + menuActions).map { action ->
						CustomAccessibilityAction(action.label) {
							action.run()
							true
						}
					}
				}.padding(vertical = 12.dp)
		) {
			// Drawn here and spoken once, from the description above.
			Column(modifier = Modifier.clearAndSetSemantics { }) {
				Text("${rule.pattern} → ${replacementText(rule)}", maxLines = 3, overflow = TextOverflow.Ellipsis)
				Text(
					"${appliesTo(rule)}, ${voicesText(rule.voiceFilter)}",
					style = MaterialTheme.typography.bodySmall,
					color = MaterialTheme.colorScheme.onSurfaceVariant,
					maxLines = 2,
					overflow = TextOverflow.Ellipsis
				)
			}
		}
		// The row says on or off and offers both actions, so these are for touch and a keyboard's Tab and stay out of the screen reader's way.
		Switch(
			checked = rule.enabled,
			onCheckedChange = { onToggle() },
			modifier = Modifier.focusProperties { canFocus = false }.clearAndSetSemantics { }
		)
		Box {
			IconButton(onClick = { menuExpanded = true }, modifier = Modifier.clearAndSetSemantics { }) {
				Icon(Icons.Filled.MoreVert, contentDescription = null)
			}
			DropdownMenu(expanded = menuExpanded, onDismissRequest = { menuExpanded = false }) {
				menuActions.forEach { action ->
					DropdownMenuItem(text = { Text(action.label) }, onClick = {
						menuExpanded = false
						action.run()
					})
				}
			}
		}
	}
}

@Composable
private fun EmptyRules() {
	Column(
		modifier = Modifier.fillMaxSize().padding(24.dp),
		horizontalAlignment = Alignment.CenterHorizontally,
		verticalArrangement = Arrangement.Center
	) {
		// TRANSLATORS: Message shown when no speech dictionary rules have been created yet
		Text(t("No Rules"), style = MaterialTheme.typography.titleMedium, modifier = Modifier.semantics { heading() })
		// TRANSLATORS: Shown under "No Rules" in the Speech Dictionary, saying how to make the first rule
		Text(
			t("Use Add Rule to change how a word or pattern is read aloud."),
			style = MaterialTheme.typography.bodyMedium,
			color = MaterialTheme.colorScheme.onSurfaceVariant,
			modifier = Modifier.padding(top = 8.dp)
		)
	}
}
