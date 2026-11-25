package dev.paperback.android.ui.state

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.paperback.ConfigManagerFfi
import uniffi.paperback.MatchType
import uniffi.paperback.NoHandle
import uniffi.paperback.RuleProblem
import uniffi.paperback.RuleScope
import uniffi.paperback.SpeechRule
import uniffi.paperback.VoiceFilter

private class RulesConfig(var rules: List<SpeechRule>) : ConfigManagerFfi(NoHandle) {
	override fun getSpeechRules(): List<SpeechRule> = rules

	override fun setSpeechRules(rules: List<SpeechRule>) {
		this.rules = rules
	}

	override fun flush() {}
}

private fun rule(pattern: String) = newSpeechRule().copy(pattern = pattern)

class SpeechDictionaryTest {
	// Order is how rules apply, so a move the reader asked for that didn't reach the config would read differently next time.
	@Test
	fun `moving a rule saves the new order and stops at the ends`() {
		val config = RulesConfig(listOf(rule("a"), rule("b"), rule("c")))
		val dictionary = SpeechDictionary(config)
		val first = dictionary.entries.value.first().id
		assertEquals(1, dictionary.move(first, 1))
		assertEquals(listOf("b", "a", "c"), config.rules.map { it.pattern })
		assertEquals(0, dictionary.move(first, -1))
		assertNull(dictionary.move(first, -1))
		assertEquals(listOf("a", "b", "c"), config.rules.map { it.pattern })
	}

	// Focus has to land somewhere after a delete, or TalkBack is left on nothing.
	@Test
	fun `deleting a rule hands focus to the one now in its place, or the one before at the end`() {
		val config = RulesConfig(listOf(rule("a"), rule("b"), rule("c")))
		val dictionary = SpeechDictionary(config)
		val (a, b, c) = dictionary.entries.value.map { it.id }
		assertEquals(c, dictionary.delete(b))
		assertEquals(a, dictionary.delete(c))
		assertNull(dictionary.delete(a))
		assertEquals(emptyList<SpeechRule>(), config.rules)
	}

	@Test
	fun `toggling and editing change only that rule, and an added rule goes last`() {
		val config = RulesConfig(listOf(rule("a"), rule("a")))
		val dictionary = SpeechDictionary(config)
		val (first, second) = dictionary.entries.value.map { it.id }
		dictionary.toggle(second)
		assertEquals(listOf(true, false), config.rules.map { it.enabled })
		dictionary.replace(first, rule("x"))
		val added = dictionary.add(rule("z"))
		assertEquals(listOf("x", "a", "z"), config.rules.map { it.pattern })
		assertEquals(added, dictionary.entries.value.last().id)
	}

	@Test
	fun `saving drops what the scope can't use`() {
		val word = newSpeechRule().copy(matchType = MatchType.REGEX, wholeWord = true).normalized()
		assertEquals(MatchType.LITERAL, word.matchType)
		assertEquals(true, word.wholeWord)
		val regex = newSpeechRule().copy(scope = RuleScope.PARAGRAPH, matchType = MatchType.REGEX, wholeWord = true)
		val paragraph = regex.normalized()
		assertEquals(MatchType.REGEX, paragraph.matchType)
		assertFalse(paragraph.wholeWord)
	}

	@Test
	fun `a row reads pattern, replacement, scope and voices`() {
		val rule = rule("Dr").copy(wholeWord = true, voiceFilter = VoiceFilter.Voices(listOf("en-us-x-sfg-local")))
		assertEquals("Dr, removed, whole words, en-us-x-sfg-local", ruleSummary(rule))
		val regex = rule("(\\d+)").copy(scope = RuleScope.PARAGRAPH, matchType = MatchType.REGEX, replacement = "\\1")
		assertEquals("(\\d+), \\1, paragraphs, regular expression, all voices", ruleSummary(regex))
	}

	@Test
	fun `voice choices are all voices, then languages, then voices, keeping a filter for a missing voice`() {
		val voices = listOf(DictionaryVoice("b", "en-US"), DictionaryVoice("a", "fi-FI"), DictionaryVoice("c", "en-US"))
		val missing = VoiceFilter.Voices(listOf("gone"))
		val filters = voiceFilterChoices(voices, missing)
		assertEquals(
			listOf(
				VoiceFilter.All,
				VoiceFilter.Language("en-US"),
				VoiceFilter.Language("fi-FI"),
				VoiceFilter.Voices(listOf("b")),
				VoiceFilter.Voices(listOf("c")),
				VoiceFilter.Voices(listOf("a")),
				missing
			),
			filters
		)
		assertEquals(6, voiceFilterChoices(voices, VoiceFilter.All).size)
	}

	@Test
	fun `each problem core finds is said in words`() {
		assertEquals("Type a pattern for the rule.", ruleProblemMessage(RuleProblem.EmptyPattern))
		val invalid = RuleProblem.InvalidRegex("unclosed group")
		assertEquals("The regular expression isn't valid: unclosed group", ruleProblemMessage(invalid))
	}
}
