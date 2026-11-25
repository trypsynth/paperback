//! The speech dictionary: rules that change how text is pronounced, without changing the document.
//!
//! Ported from Paperback for iOS, which had it first, so its rules behave the same here and can move to core unchanged.

use std::{borrow::Cow, ops::Range};

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{TextMap, Voice};

/// How much of the text a rule considers. Paragraph rules run first, so a paragraph rule can't undo what a word rule produced.
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleScope {
	#[default]
	Word,
	Paragraph,
}

/// Only paragraph rules can be regular expressions.
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
	#[default]
	Literal,
	Regex,
}

/// Which voices a rule applies to, since a rule that fixes one voice's pronunciation can break another's.
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum VoiceFilter {
	#[default]
	All,
	/// Voices whose [`Voice::language`] is exactly this.
	Language(String),
	/// These [`Voice::id`]s.
	Voices(Vec<String>),
}

impl VoiceFilter {
	fn matches(&self, voice: Option<&Voice>) -> bool {
		match self {
			Self::All => true,
			Self::Language(language) => voice.is_some_and(|voice| voice.language == *language),
			Self::Voices(ids) => voice.is_some_and(|voice| ids.contains(&voice.id)),
		}
	}
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SpeechRule {
	pub enabled: bool,
	pub scope: RuleScope,
	pub match_type: MatchType,
	/// Matched case-sensitively.
	pub pattern: String,
	/// For a regular expression, `\1` stands for the first captured group, as in NVDA's speech dictionary.
	pub replacement: String,
	/// Only for word rules: the pattern must not be part of a longer word.
	pub whole_word: bool,
	pub voice_filter: VoiceFilter,
}

impl Default for SpeechRule {
	fn default() -> Self {
		Self {
			enabled: true,
			scope: RuleScope::Word,
			match_type: MatchType::Literal,
			pattern: String::new(),
			replacement: String::new(),
			whole_word: false,
			voice_filter: VoiceFilter::All,
		}
	}
}

/// Why a rule can't be saved.
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleProblem {
	EmptyPattern,
	/// The regular expression doesn't parse, for this reason.
	InvalidRegex(String),
}

impl SpeechRule {
	/// Whether the rule can be saved: it needs a pattern, and a regular expression that parses.
	///
	/// # Errors
	///
	/// Returns what's wrong with the rule.
	pub fn check(&self) -> Result<(), RuleProblem> {
		if self.pattern.is_empty() {
			return Err(RuleProblem::EmptyPattern);
		}
		if self.scope == RuleScope::Paragraph && self.match_type == MatchType::Regex {
			Regex::new(&self.pattern).map_err(|error| RuleProblem::InvalidRegex(error.to_string()))?;
		}
		Ok(())
	}

	/// Applies the rule to `text` and records what it replaced in `map`. `None` when it changes nothing: a rule that's off, empty, for another voice, or an invalid regular expression, or one that matches nowhere.
	fn apply(&self, text: &str, voice: Option<&Voice>, map: &mut TextMap) -> Option<String> {
		if !self.enabled || self.pattern.is_empty() || !self.voice_filter.matches(voice) {
			return None;
		}
		match (self.scope, self.match_type) {
			(RuleScope::Word, _) if self.whole_word => {
				let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(&self.pattern))).ok()?;
				replace(
					text,
					pattern.captures_iter(text).map(with_range),
					|_, out| out.push_str(&self.replacement),
					map,
				)
			}
			(RuleScope::Paragraph, MatchType::Regex) => {
				let pattern = Regex::new(&self.pattern).ok()?;
				let template = nvda_template(&self.replacement);
				replace(
					text,
					pattern.captures_iter(text).map(with_range),
					|captures, out| captures.expand(&template, out),
					map,
				)
			}
			// Searched for as it is: most rules are this kind, and each would otherwise build a regular expression for every paragraph spoken.
			_ => {
				let found =
					text.match_indices(self.pattern.as_str()).map(|(start, found)| (start..start + found.len(), ()));
				replace(text, found, |(), out| out.push_str(&self.replacement), map)
			}
		}
	}
}

/// Applies every rule that fits `voice` to `text`: paragraph rules, then word rules, each in list order. Returns the text to speak, and the map from it back to `text`.
#[must_use]
pub fn apply_rules(rules: &[SpeechRule], text: &str, voice: Option<&Voice>) -> (String, TextMap) {
	let mut map = TextMap::default();
	let mut result = Cow::Borrowed(text);
	for scope in [RuleScope::Paragraph, RuleScope::Word] {
		for rule in rules.iter().filter(|rule| rule.scope == scope) {
			if let Some(changed) = rule.apply(&result, voice, &mut map) {
				result = Cow::Owned(changed);
			}
		}
	}
	(result.into_owned(), map)
}

fn with_range(captures: regex::Captures<'_>) -> (Range<usize>, regex::Captures<'_>) {
	(captures.get(0).expect("group 0 is the whole match").range(), captures)
}

/// Replaces each of `matches`, which are in order and don't overlap, writing its replacement with `write`, and records the replacements in `map`. `None` when there are none, so the text isn't copied for a rule that doesn't touch it.
fn replace<M>(
	text: &str,
	matches: impl Iterator<Item = (Range<usize>, M)>,
	write: impl Fn(&M, &mut String),
	map: &mut TextMap,
) -> Option<String> {
	let mut matches = matches.peekable();
	matches.peek()?;
	let mut out = String::with_capacity(text.len());
	let mut replacements = Vec::new();
	let mut copied_to = 0;
	let mut copied_chars = 0;
	for (range, found) in matches {
		let before = &text[copied_to..range.start];
		out.push_str(before);
		copied_chars += before.chars().count();
		let replaced_from = out.len();
		write(&found, &mut out);
		let replaced_chars = text[range.clone()].chars().count();
		replacements.push((copied_chars, replaced_chars, out[replaced_from..].chars().count()));
		copied_chars += replaced_chars;
		copied_to = range.end;
	}
	out.push_str(&text[copied_to..]);
	map.push_step(&replacements);
	Some(out)
}

/// The editor promises `\1` for a captured group, as NVDA does, but the regex crate writes it `${1}`. A literal `$` is escaped so it isn't read as a group.
fn nvda_template(replacement: &str) -> String {
	let mut template = String::with_capacity(replacement.len());
	let mut chars = replacement.chars().peekable();
	while let Some(c) = chars.next() {
		match c {
			'$' => template.push_str("$$"),
			'\\' if chars.peek().is_some_and(char::is_ascii_digit) => {
				template.push_str("${");
				template.push(chars.next().expect("peeked"));
				template.push('}');
			}
			_ => template.push(c),
		}
	}
	template
}

#[cfg(test)]
mod tests {
	use super::*;

	fn voice(id: &str, language: &str) -> Voice {
		Voice { id: id.into(), name: id.into(), language: language.into() }
	}

	fn word(pattern: &str, replacement: &str) -> SpeechRule {
		SpeechRule { pattern: pattern.into(), replacement: replacement.into(), ..SpeechRule::default() }
	}

	fn speak(rules: &[SpeechRule], text: &str) -> String {
		apply_rules(rules, text, None).0
	}

	#[test]
	fn a_word_rule_replaces_every_occurrence_case_sensitively() {
		assert_eq!(speak(&[word("Dr", "Doctor")], "Dr Who and dr Dre"), "Doctor Who and dr Doctore");
	}

	#[test]
	fn a_whole_word_rule_leaves_longer_words_alone() {
		let rule = SpeechRule { whole_word: true, ..word("Dr", "Doctor") };
		assert_eq!(speak(&[rule], "Dr Who and Dre"), "Doctor Who and Dre");
	}

	#[test]
	fn special_characters_are_taken_literally_outside_regular_expressions() {
		assert_eq!(speak(&[word("a.b", "x$1")], "a.b acb"), "x$1 acb");
		let whole = SpeechRule { whole_word: true, ..word("C++", "C plus plus") };
		assert_eq!(speak(&[whole], "C++ and C"), "C++ and C");
		let regex_word = SpeechRule { match_type: MatchType::Regex, ..word("a+", "x") };
		assert_eq!(speak(&[regex_word], "aaa a+"), "aaa x", "word rules ignore the match type");
	}

	#[test]
	fn a_paragraph_regex_uses_nvda_style_groups() {
		let rule = SpeechRule {
			scope: RuleScope::Paragraph,
			match_type: MatchType::Regex,
			pattern: r"(\d+)%".into(),
			replacement: r"\1 percent, $5".into(),
			..SpeechRule::default()
		};
		assert_eq!(speak(&[rule], "Down 40% today"), "Down 40 percent, $5 today");
	}

	#[test]
	fn paragraph_rules_run_before_word_rules() {
		let word_rule = word("cat", "dog");
		let paragraph_rule = SpeechRule { scope: RuleScope::Paragraph, ..word("dog", "wolf") };
		assert_eq!(speak(&[word_rule, paragraph_rule], "cat dog"), "dog wolf");
	}

	#[test]
	fn rules_that_are_off_empty_invalid_or_for_another_voice_change_nothing() {
		let off = SpeechRule { enabled: false, ..word("a", "b") };
		let empty = word("", "b");
		let invalid = SpeechRule {
			scope: RuleScope::Paragraph,
			match_type: MatchType::Regex,
			pattern: "(".into(),
			..word("", "b")
		};
		let finnish = SpeechRule { voice_filter: VoiceFilter::Language("fi-FI".into()), ..word("a", "b") };
		assert_eq!(speak(&[off, empty, invalid, finnish.clone()], "a"), "a");
		let petteri = voice("petteri", "fi-FI");
		assert_eq!(apply_rules(&[finnish], "a", Some(&petteri)).0, "b");
		let only_petteri = SpeechRule { voice_filter: VoiceFilter::Voices(vec!["petteri".into()]), ..word("a", "c") };
		assert_eq!(apply_rules(std::slice::from_ref(&only_petteri), "a", Some(&petteri)).0, "c");
		assert_eq!(apply_rules(&[only_petteri], "a", Some(&voice("anna", "fi-FI"))).0, "a");
	}

	#[test]
	fn a_spoken_word_maps_back_to_its_place_in_the_original() {
		let (spoken, map) = apply_rules(&[word("Dr", "Doctor")], "Ask Dr Smith", None);
		assert_eq!(spoken, "Ask Doctor Smith");
		assert_eq!(map.to_original(spoken.find("Smith").unwrap()), "Ask Dr ".len());
		assert_eq!(map.to_original(spoken.find("Doctor").unwrap()), "Ask ".len());
	}

	#[test]
	fn a_rule_needs_a_pattern_and_a_valid_regular_expression() {
		assert_eq!(word("", "x").check(), Err(RuleProblem::EmptyPattern));
		assert_eq!(word("(", "x").check(), Ok(()), "a word rule's pattern is literal");
		let regex = SpeechRule { scope: RuleScope::Paragraph, match_type: MatchType::Regex, ..word("(", "x") };
		assert!(matches!(regex.check(), Err(RuleProblem::InvalidRegex(_))));
		assert_eq!(SpeechRule { pattern: r"\d+".into(), ..regex }.check(), Ok(()));
	}

	#[test]
	fn rules_round_trip_through_settings() {
		let rule = SpeechRule {
			scope: RuleScope::Paragraph,
			voice_filter: VoiceFilter::Voices(vec!["petteri".into()]),
			..word("a", "b")
		};
		let saved = toml::to_string(&rule).unwrap();
		assert_eq!(toml::from_str::<SpeechRule>(&saved).unwrap(), rule);
	}
}
