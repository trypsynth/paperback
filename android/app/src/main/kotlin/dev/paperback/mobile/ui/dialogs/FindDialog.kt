package dev.paperback.mobile.ui.dialogs

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.*
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.SearchOptionsFfi
import androidx.compose.ui.input.key.onKeyEvent
import androidx.compose.ui.input.key.Key
import androidx.compose.ui.input.key.key
import androidx.compose.ui.input.key.KeyEventType
import androidx.compose.ui.input.key.type
import dev.paperback.mobile.t

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FindDialog(
	configManager: ConfigManagerFfi,
	initialQuery: String,
	onDismiss: () -> Unit,
	onSearch: (String, SearchOptionsFfi) -> Unit
) {
	var query by remember { mutableStateOf(initialQuery) }

	var matchCase by remember { mutableStateOf(configManager.getAppBool("find_match_case", false)) }
	var wholeWord by remember { mutableStateOf(configManager.getAppBool("find_whole_word", false)) }
	var useRegex by remember { mutableStateOf(configManager.getAppBool("find_use_regex", false)) }

	val searchHistory = remember { configManager.getFindHistory() }

	val submitSearch = {
		if (query.isNotBlank()) {
			configManager.setAppBool("find_match_case", matchCase)
			configManager.setAppBool("find_whole_word", wholeWord)
			configManager.setAppBool("find_use_regex", useRegex)
			configManager.addFindHistory(query, 10)

			onSearch(
				query,
				SearchOptionsFfi(
					matchCase = matchCase,
					wholeWord = wholeWord,
					regex = useRegex,
					forward = true
				)
			)
			onDismiss()
		}
	}

	AlertDialog(
		onDismissRequest = onDismiss,
		modifier = Modifier.semantics { paneTitle = "Find" },
		// TRANSLATORS: Title of the Find dialog
		title = { Text(t("Find")) },
		text = {
			Column {
				var historyExpanded by remember { mutableStateOf(false) }
				TextField(
					value = query,
					onValueChange = { query = it },
					// TRANSLATORS: Label for the text field where the user types what to search for
					label = { Text(t("Search Term")) },
					keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
					keyboardActions = KeyboardActions(onSearch = { submitSearch() }),
					singleLine = true,
					modifier = Modifier
						.fillMaxWidth()
						.onKeyEvent { event ->
							if (event.type == KeyEventType.KeyUp && (event.key == Key.Enter || event.key == Key.NumPadEnter)) {
								submitSearch()
								true
							} else {
								false
							}
						}
				)
				if (searchHistory.isNotEmpty()) {
					Box(modifier = Modifier.fillMaxWidth().padding(top = 8.dp)) {
						TextButton(
							onClick = { historyExpanded = true },
							modifier = Modifier.semantics {
								customActions = searchHistory.map { historyTerm ->
									CustomAccessibilityAction(historyTerm) {
										query = historyTerm
										true
									}
								}
							}
						) {
							// TRANSLATORS: Button that opens a dropdown of previously used search terms
							Text(t("Search History"))
						}
						DropdownMenu(
							expanded = historyExpanded,
							onDismissRequest = { historyExpanded = false }
						) {
							searchHistory.forEach { historyTerm ->
								DropdownMenuItem(
									text = { Text(historyTerm) },
									onClick = {
										query = historyTerm
										historyExpanded = false
									}
								)
							}
						}
					}
				}
				Spacer(modifier = Modifier.height(16.dp))
				Row(
					verticalAlignment = Alignment.CenterVertically,
					modifier = Modifier
						.fillMaxWidth()
						.semantics(mergeDescendants = true) {
							stateDescription = if (matchCase) "ticked" else "not ticked"
						}.toggleable(
							value = matchCase,
							onValueChange = { matchCase = it },
							role = Role.Checkbox
						).padding(vertical = 8.dp)
				) {
					Checkbox(
						checked = matchCase,
						onCheckedChange = null // Handled by Row toggleable
					)
					// TRANSLATORS: Checkbox label to make Find treat uppercase/lowercase letters as distinct
					Text(t("Match Case"), modifier = Modifier.padding(start = 8.dp))
				}
				Row(
					verticalAlignment = Alignment.CenterVertically,
					modifier = Modifier
						.fillMaxWidth()
						.semantics(mergeDescendants = true) {
							stateDescription = if (wholeWord) "ticked" else "not ticked"
						}.toggleable(
							value = wholeWord,
							onValueChange = { wholeWord = it },
							role = Role.Checkbox
						).padding(vertical = 8.dp)
				) {
					Checkbox(
						checked = wholeWord,
						onCheckedChange = null // Handled by Row toggleable
					)
					// TRANSLATORS: Checkbox label to make Find only match whole words, not substrings within other words
					Text(t("Whole Word"), modifier = Modifier.padding(start = 8.dp))
				}
				Row(
					verticalAlignment = Alignment.CenterVertically,
					modifier = Modifier
						.fillMaxWidth()
						.semantics(mergeDescendants = true) {
							stateDescription = if (useRegex) "ticked" else "not ticked"
						}.toggleable(
							value = useRegex,
							onValueChange = { useRegex = it },
							role = Role.Checkbox
						).padding(vertical = 8.dp)
				) {
					Checkbox(
						checked = useRegex,
						onCheckedChange = null // Handled by Row toggleable
					)
					// TRANSLATORS: Checkbox label to interpret the search term as a regular expression
					Text(t("Regular Expression"), modifier = Modifier.padding(start = 8.dp))
				}
			}
		},
		confirmButton = {
			TextButton(onClick = submitSearch) {
				// TRANSLATORS: Button to run the search and jump to the next match
				Text(t("Find Next"))
			}
		},
		dismissButton = {
			TextButton(onClick = onDismiss) {
				// TRANSLATORS: Button to close the Find dialog without searching
				Text(t("Cancel"))
			}
		}
	)
}
