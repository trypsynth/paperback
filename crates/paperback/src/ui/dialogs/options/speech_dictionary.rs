//! The speech dictionary, opened from the Read Aloud group: rules in the order they apply, each checked if it's on, and a dialog for one rule.

use std::{cell::RefCell, rc::Rc};

use paperback_core::speech::{MatchType, RuleProblem, RuleScope, SpeechRule, Voice, VoiceFilter};
use patois::t;
use wx_utils::{add_ok_cancel_footer, build_ok_cancel_buttons, dialog_padding, form::Form, show_error};
use wxdragon::prelude::*;

use super::audio::language_label;

/// `voices` are the engine's, offered to rules that fix one voice's or one language's pronunciation. `None` if cancelled.
pub(super) fn show_speech_dictionary(
	parent: &dyn WxWidget,
	rules: &[SpeechRule],
	voices: &[Voice],
) -> Option<Vec<SpeechRule>> {
	// TRANSLATORS: Title of the speech dictionary dialog, opened from the Read Aloud group on the Audio tab of Settings
	let dialog = Dialog::builder(parent, &t("Speech Dictionary")).build();
	let form = Form::new(dialog);
	let size = Size::new(dialog.get_char_width() * 60, dialog.get_char_height() * 12);
	// TRANSLATORS: Label of the list of speech dictionary rules, which are applied in order and can each be turned on or off
	let list = form.labeled_above(&t("&Rules:"), true, |parent| CheckListBox::builder(parent).with_size(size).build());
	let rules = Rc::new(RefCell::new(rules.to_vec()));
	let voices: Rc<[Voice]> = voices.into();
	let padding = dialog_padding(&dialog);
	// TRANSLATORS: Button in the speech dictionary dialog that adds a rule
	let add = Button::builder(&dialog).with_label(&t("&Add...")).build();
	// TRANSLATORS: Button in the speech dictionary dialog that edits the current rule
	let edit = Button::builder(&dialog).with_label(&t("&Edit...")).build();
	// TRANSLATORS: Button in the speech dictionary dialog that removes the current rule
	let remove = Button::builder(&dialog).with_label(&t("Re&move")).build();
	let buttons = BoxSizer::builder(Orientation::Horizontal).build();
	buttons.add(&add, 0, SizerFlag::Right, padding);
	buttons.add(&edit, 0, SizerFlag::Right, padding);
	buttons.add(&remove, 0, SizerFlag::empty(), 0);
	form.sizer().add_sizer(&buttons, 0, SizerFlag::Left | SizerFlag::Right | SizerFlag::Top, padding);
	let (ok, cancel) = build_ok_cancel_buttons(&dialog, &t("OK"));
	add_ok_cancel_footer(form.sizer(), ok, cancel);
	let current = move || list.get_selection().map(|index| index as usize);
	let show = {
		let (rules, voices) = (Rc::clone(&rules), Rc::clone(&voices));
		Rc::new(move |selected: usize| {
			list.clear();
			let rules = rules.borrow();
			for (index, rule) in rules.iter().enumerate() {
				list.append(&row(rule, &voices));
				list.check(u32::try_from(index).unwrap_or_default(), rule.enabled);
			}
			if !rules.is_empty() {
				list.set_selection(u32::try_from(selected.min(rules.len() - 1)).unwrap_or_default(), true);
			}
			edit.enable(!rules.is_empty());
			remove.enable(!rules.is_empty());
		})
	};
	show(0);
	{
		let rules = Rc::clone(&rules);
		list.on_toggled(move |event| {
			let Some(index) = event.get_selection() else { return };
			if let Some(rule) = rules.borrow_mut().get_mut(index as usize) {
				rule.enabled = list.is_checked(index);
			}
		});
	}
	{
		let (rules, voices, show) = (Rc::clone(&rules), Rc::clone(&voices), Rc::clone(&show));
		add.on_click(move |_| {
			if let Some(rule) = edit_rule(dialog, &SpeechRule::default(), &voices, true) {
				let index = current().map_or(rules.borrow().len(), |current| current + 1);
				rules.borrow_mut().insert(index, rule);
				show(index);
			}
			list.set_focus();
		});
	}
	let edit_current = {
		let (rules, voices, show) = (Rc::clone(&rules), Rc::clone(&voices), Rc::clone(&show));
		Rc::new(move || {
			let Some(index) = current() else { return };
			let Some(existing) = rules.borrow().get(index).cloned() else { return };
			if let Some(rule) = edit_rule(dialog, &existing, &voices, false) {
				rules.borrow_mut()[index] = rule;
				show(index);
			}
			list.set_focus();
		})
	};
	{
		let edit_current = Rc::clone(&edit_current);
		edit.on_click(move |_| edit_current());
	}
	list.on_double_clicked(move |_| edit_current());
	let remove_current = {
		let (rules, show) = (Rc::clone(&rules), Rc::clone(&show));
		Rc::new(move || {
			let Some(index) = current() else { return };
			rules.borrow_mut().remove(index);
			show(index);
			list.set_focus();
		})
	};
	{
		let remove_current = Rc::clone(&remove_current);
		remove.on_click(move |_| remove_current());
	}
	{
		let rules = Rc::clone(&rules);
		dialog.on_char_hook(move |event| {
			if let WindowEventData::Keyboard(key) = &event
				&& list.has_focus()
			{
				let handled = match key.get_key_code() {
					Some(WXK_DELETE | WXK_NUMPAD_DELETE) => {
						remove_current();
						true
					}
					Some(code @ (WXK_UP | WXK_DOWN)) if key.control_down() => {
						if let Some(from) = current() {
							let to = if code == WXK_DOWN { from + 1 } else { from.wrapping_sub(1) };
							if to < rules.borrow().len() {
								rules.borrow_mut().swap(from, to);
								show(to);
							}
						}
						true
					}
					_ => false,
				};
				if handled {
					key.event.skip(false);
					return;
				}
			}
			event.skip(true);
		});
	}
	dialog.set_sizer_and_fit(form.sizer(), true);
	dialog.centre();
	list.set_focus();
	let accepted = dialog.show_modal() == ID_OK;
	dialog.destroy();
	accepted.then(|| rules.take())
}

fn row(rule: &SpeechRule, voices: &[Voice]) -> String {
	let replacement = if rule.replacement.is_empty() {
		// TRANSLATORS: In the speech dictionary's list, the replacement of a rule that removes what it finds
		t("removed")
	} else {
		rule.replacement.clone()
	};
	[rule.pattern.clone(), replacement, applies_to(rule), voices_text(&rule.voice_filter, voices)].join(", ")
}

fn applies_to(rule: &SpeechRule) -> String {
	match (rule.scope, rule.match_type, rule.whole_word) {
		// TRANSLATORS: In the speech dictionary's list, a rule that matches whole words only
		(RuleScope::Word, _, true) => t("whole words"),
		// TRANSLATORS: In the speech dictionary's list, a rule that matches inside words too
		(RuleScope::Word, _, false) => t("words"),
		// TRANSLATORS: In the speech dictionary's list, a rule that matches a regular expression across a paragraph
		(RuleScope::Paragraph, MatchType::Regex, _) => t("paragraphs, regular expression"),
		// TRANSLATORS: In the speech dictionary's list, a rule that matches text across a paragraph
		(RuleScope::Paragraph, MatchType::Literal, _) => t("paragraphs"),
	}
}

fn voices_text(filter: &VoiceFilter, voices: &[Voice]) -> String {
	match filter {
		// TRANSLATORS: In the speech dictionary, a rule that applies to every voice
		VoiceFilter::All => t("all voices"),
		VoiceFilter::Language(tag) => language_label(tag),
		VoiceFilter::Voices(ids) => ids
			.iter()
			.map(|id| {
				voices.iter().find(|voice| &voice.id == id).map_or_else(|| id.clone(), |voice| voice.name.clone())
			})
			.collect::<Vec<_>>()
			.join(", "),
	}
}

/// The voice choices after "All voices": each language, then each voice.
fn voice_filters(voices: &[Voice], current: &VoiceFilter) -> Vec<VoiceFilter> {
	let mut languages: Vec<String> = voices.iter().map(|voice| voice.language.clone()).collect();
	languages.sort_by_cached_key(|tag| language_label(tag));
	languages.dedup();
	let mut sorted: Vec<&Voice> = voices.iter().collect();
	sorted.sort_by_cached_key(|voice| (language_label(&voice.language), voice.name.clone()));
	let mut filters: Vec<VoiceFilter> = languages
		.into_iter()
		.map(VoiceFilter::Language)
		.chain(sorted.into_iter().map(|voice| VoiceFilter::Voices(vec![voice.id.clone()])))
		.collect();
	// A filter for voices this engine doesn't have is kept, so editing the rule doesn't lose it.
	if *current != VoiceFilter::All && !filters.contains(current) {
		filters.push(current.clone());
	}
	filters
}

fn edit_rule(parent: Dialog, rule: &SpeechRule, voices: &[Voice], adding: bool) -> Option<SpeechRule> {
	let title = if adding {
		// TRANSLATORS: Title of the dialog that adds a speech dictionary rule
		t("Add Rule")
	} else {
		// TRANSLATORS: Title of the dialog that edits a speech dictionary rule
		t("Edit Rule")
	};
	let dialog = Dialog::builder(&parent, &title).build();
	let form = Form::new(dialog);
	// TRANSLATORS: Label of the text a speech dictionary rule matches, or its regular expression
	let pattern = form.labeled(&t("&Pattern:"), |parent| TextCtrl::builder(parent).with_value(&rule.pattern).build());
	// TRANSLATORS: Label of the text a speech dictionary rule says instead; it can be empty
	let replacement =
		form.labeled(&t("&Replacement:"), |parent| TextCtrl::builder(parent).with_value(&rule.replacement).build());
	// TRANSLATORS: Label of the choice between a rule matching words and one matching across a whole paragraph
	let scope = form.labeled(&t("&Applies to:"), |parent| Choice::builder(parent).build());
	// TRANSLATORS: Speech dictionary rule scope: the rule matches words
	scope.append(&t("Words"));
	// TRANSLATORS: Speech dictionary rule scope: the rule matches across a whole paragraph, and can be a regular expression
	scope.append(&t("Whole paragraphs"));
	scope.set_selection(u32::from(rule.scope == RuleScope::Paragraph));
	// TRANSLATORS: Label of the choice of voices a speech dictionary rule applies to
	let voice = form.labeled(&t("&Voices:"), |parent| Choice::builder(parent).build());
	let filters = voice_filters(voices, &rule.voice_filter);
	// TRANSLATORS: First choice of voices for a speech dictionary rule
	voice.append(&t("All voices"));
	for filter in &filters {
		let label = match filter {
			// TRANSLATORS: A choice of voices for a speech dictionary rule: every voice of one language; %s is the language
			VoiceFilter::Language(tag) => t("Voices in %s").replacen("%s", &language_label(tag), 1),
			other => voices_text(other, voices),
		};
		voice.append(&label);
	}
	let voice_index = filters.iter().position(|filter| *filter == rule.voice_filter).map_or(0, |index| index + 1);
	voice.set_selection(u32::try_from(voice_index).unwrap_or(0));
	// TRANSLATORS: Check box: a word rule matches only whole words, not part of a longer word
	let whole_word = form.check_box(&t("Match &whole words only"));
	whole_word.set_value(rule.whole_word);
	// TRANSLATORS: Check box: a paragraph rule's pattern is a regular expression, whose groups the replacement can use as \1
	let regex = form.check_box(&t("Regular e&xpression"));
	regex.set_value(rule.match_type == MatchType::Regex);
	let (ok, cancel) = build_ok_cancel_buttons(&dialog, &t("OK"));
	add_ok_cancel_footer(form.sizer(), ok, cancel);
	let match_scope = move || {
		let words = scope.get_selection() != Some(1);
		whole_word.enable(words);
		regex.enable(!words);
	};
	match_scope();
	scope.on_selection_changed(move |_| match_scope());
	let enabled = rule.enabled;
	let read = Rc::new(move || SpeechRule {
		enabled,
		scope: if scope.get_selection() == Some(1) { RuleScope::Paragraph } else { RuleScope::Word },
		match_type: if scope.get_selection() == Some(1) && regex.is_checked() {
			MatchType::Regex
		} else {
			MatchType::Literal
		},
		pattern: pattern.get_value(),
		replacement: replacement.get_value(),
		whole_word: scope.get_selection() != Some(1) && whole_word.is_checked(),
		voice_filter: voice
			.get_selection()
			.and_then(|index| usize::try_from(index).ok()?.checked_sub(1))
			.and_then(|index| filters.get(index).cloned())
			.unwrap_or_default(),
	});
	{
		let read = Rc::clone(&read);
		ok.on_click(move |_| match read().check() {
			Ok(()) => dialog.end_modal(ID_OK),
			Err(problem) => {
				let message = match problem {
					// TRANSLATORS: Shown when a speech dictionary rule is saved without a pattern
					RuleProblem::EmptyPattern => t("Type a pattern for the rule."),
					// TRANSLATORS: Shown when a speech dictionary rule's regular expression doesn't parse; %s is why
					RuleProblem::InvalidRegex(reason) => {
						t("The regular expression isn't valid: %s").replacen("%s", &reason, 1)
					}
				};
				show_error(&dialog, message, &title);
				pattern.set_focus();
			}
		});
	}
	dialog.set_sizer_and_fit(form.sizer(), true);
	dialog.centre();
	pattern.set_focus();
	let accepted = dialog.show_modal() == ID_OK;
	let edited = accepted.then(|| read());
	dialog.destroy();
	edited
}

#[cfg(test)]
mod tests {
	use super::*;

	fn voice(id: &str, name: &str, language: &str) -> Voice {
		Voice { id: id.into(), name: name.into(), language: language.into() }
	}

	#[test]
	fn a_row_reads_pattern_replacement_scope_and_voices() {
		let voices = [voice("v1", "Satu", "fi-FI")];
		let rule = SpeechRule {
			pattern: "Dr".into(),
			whole_word: true,
			voice_filter: VoiceFilter::Voices(vec!["v1".into()]),
			enabled: false,
			..SpeechRule::default()
		};
		assert_eq!(row(&rule, &voices), "Dr, removed, whole words, Satu");
	}

	#[test]
	fn languages_come_before_voices_and_a_filter_for_a_missing_voice_is_kept() {
		let voices = [voice("b", "Bea", "en-US"), voice("a", "Aino", "fi-FI"), voice("c", "Cal", "en-US")];
		let missing = VoiceFilter::Voices(vec!["gone".into()]);
		let filters = voice_filters(&voices, &missing);
		assert_eq!(filters[..2], [VoiceFilter::Language("en-US".into()), VoiceFilter::Language("fi-FI".into())]);
		assert_eq!(
			filters[2..5],
			[
				VoiceFilter::Voices(vec!["b".into()]),
				VoiceFilter::Voices(vec!["c".into()]),
				VoiceFilter::Voices(vec!["a".into()])
			]
		);
		assert_eq!(filters.last(), Some(&missing));
		assert!(!voice_filters(&voices, &VoiceFilter::All).contains(&VoiceFilter::All));
	}
}
