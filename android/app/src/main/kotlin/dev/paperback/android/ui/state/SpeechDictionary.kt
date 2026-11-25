package dev.paperback.android.ui.state

import dev.paperback.android.saveSoon
import dev.paperback.android.t
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.MatchType
import uniffi.paperback.RuleProblem
import uniffi.paperback.RuleScope
import uniffi.paperback.SpeechRule
import uniffi.paperback.VoiceFilter
import java.util.Locale

/** An Android voice's name is its id, and [language] is its locale's BCP 47 tag, as the reader hands them to the rules. */
data class DictionaryVoice(val name: String, val language: String)

/** Rules can repeat, so each gets an id for the list to key on, letting focus stay on a rule as it moves. */
data class DictionaryEntry(val id: Long, val rule: SpeechRule)

class SpeechDictionary(private val config: ConfigManagerFfi) {
	private var nextId = 0L
	private val _entries = MutableStateFlow(config.getSpeechRules().map { DictionaryEntry(nextId++, it) })
	val entries: StateFlow<List<DictionaryEntry>> = _entries.asStateFlow()

	fun problemOf(rule: SpeechRule): RuleProblem? = config.speechRuleProblem(rule)

	fun add(rule: SpeechRule): Long {
		val entry = DictionaryEntry(nextId++, rule)
		save(_entries.value + entry)
		return entry.id
	}

	fun replace(id: Long, rule: SpeechRule) {
		save(_entries.value.map { if (it.id == id) it.copy(rule = rule) else it })
	}

	fun toggle(id: Long) {
		save(_entries.value.map { if (it.id == id) it.copy(rule = it.rule.copy(enabled = !it.rule.enabled)) else it })
	}

	/** The rule's new index, or null when it is already at that end. */
	fun move(id: Long, by: Int): Int? {
		val list = _entries.value
		val from = list.indexOfFirst { it.id == id }
		val to = from + by
		if (from < 0 || to !in list.indices) return null
		save(list.toMutableList().apply { add(to, removeAt(from)) })
		return to
	}

	/** The rule now in its place, so focus has somewhere to go, or null when the list is empty. */
	fun delete(id: Long): Long? {
		val rest = _entries.value.toMutableList()
		val index = rest.indexOfFirst { it.id == id }
		if (index < 0) return null
		rest.removeAt(index)
		save(rest)
		return rest.getOrNull(index.coerceAtMost(rest.lastIndex))?.id
	}

	private fun save(list: List<DictionaryEntry>) {
		_entries.value = list
		config.setSpeechRules(list.map { it.rule })
		config.saveSoon()
	}
}

fun newSpeechRule(): SpeechRule = SpeechRule(
	enabled = true,
	scope = RuleScope.WORD,
	matchType = MatchType.LITERAL,
	pattern = "",
	replacement = "",
	wholeWord = false,
	voiceFilter = VoiceFilter.All
)

/** The editor keeps both check boxes while the scope changes, so what the scope can't use is dropped on save, as on desktop. */
fun SpeechRule.normalized(): SpeechRule = copy(
	matchType = if (scope == RuleScope.PARAGRAPH) matchType else MatchType.LITERAL,
	wholeWord = scope == RuleScope.WORD && wholeWord
)

fun languageLabel(tag: String): String {
	val locale = Locale.getDefault()
	val name = Locale.forLanguageTag(tag).getDisplayLanguage(locale).replaceFirstChar { it.titlecase(locale) }
	return if (name.isEmpty() || name == tag) tag else "$name ($tag)"
}

fun appliesTo(rule: SpeechRule): String = when {
	// TRANSLATORS: In the speech dictionary's list, a rule that matches whole words only
	rule.scope == RuleScope.WORD && rule.wholeWord -> t("whole words")
	// TRANSLATORS: In the speech dictionary's list, a rule that matches inside words too
	rule.scope == RuleScope.WORD -> t("words")
	// TRANSLATORS: In the speech dictionary's list, a rule that matches a regular expression across a paragraph
	rule.matchType == MatchType.REGEX -> t("paragraphs, regular expression")
	// TRANSLATORS: In the speech dictionary's list, a rule that matches text across a paragraph
	else -> t("paragraphs")
}

fun voicesText(filter: VoiceFilter): String = when (filter) {
	// TRANSLATORS: In the speech dictionary, a rule that applies to every voice
	VoiceFilter.All -> t("all voices")
	is VoiceFilter.Language -> languageLabel(filter.v1)
	is VoiceFilter.Voices -> filter.v1.joinToString(", ")
}

// TRANSLATORS: In the speech dictionary's list, the replacement of a rule that removes what it finds
fun replacementText(rule: SpeechRule): String = rule.replacement.ifEmpty { t("removed") }

fun ruleSummary(rule: SpeechRule): String =
	listOf(rule.pattern, replacementText(rule), appliesTo(rule), voicesText(rule.voiceFilter)).joinToString(", ")

/** All voices, then each language, then each voice, as on desktop. */
fun voiceFilterChoices(voices: List<DictionaryVoice>, current: VoiceFilter): List<VoiceFilter> {
	val languages = voices.map { it.language }.distinct().sortedBy(::languageLabel)
	val named = voices
		.sortedWith(compareBy({ languageLabel(it.language) }, { it.name }))
		.map { it.name }
		.distinct()
		.map { VoiceFilter.Voices(listOf(it)) }
	val filters = listOf(VoiceFilter.All) + languages.map { VoiceFilter.Language(it) } + named
	// A filter for voices this engine doesn't have is kept, so editing the rule doesn't lose it.
	return if (current in filters) filters else filters + current
}

fun voiceFilterLabel(filter: VoiceFilter): String = when (filter) {
	// TRANSLATORS: First choice of voices for a speech dictionary rule
	VoiceFilter.All -> t("All voices")
	// TRANSLATORS: A choice of voices for a speech dictionary rule: every voice of one language; %s is the language
	is VoiceFilter.Language -> t("Voices in %s").replaceFirst("%s", languageLabel(filter.v1))
	is VoiceFilter.Voices -> voicesText(filter)
}

fun ruleProblemMessage(problem: RuleProblem): String = when (problem) {
	// TRANSLATORS: Shown when a speech dictionary rule is saved without a pattern
	RuleProblem.EmptyPattern -> t("Type a pattern for the rule.")
	// TRANSLATORS: Shown when a speech dictionary rule's regular expression doesn't parse; %s is why
	is RuleProblem.InvalidRegex -> t("The regular expression isn't valid: %s").replaceFirst("%s", problem.v1)
}
