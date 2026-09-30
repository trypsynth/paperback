//! BSD manual pages, written with the `mdoc` macro package.
//!
//! `mdoc` differs from `man` in kind rather than in spelling. A `man` page says how a line is
//! set: `.B` for bold, `.TP` for a tagged paragraph. An `mdoc` page says what a line *is*:
//! `.Nm` the name of the thing documented, `.Fl` a flag, `.Ar` an argument, `.Op` something
//! optional. How it looks follows from that.
//!
//! Two rules shape everything here. A macro's arguments can be macros themselves, so
//! `.Op Fl f Ar file` reads `[-f file]` and has to be evaluated rather than looked up. And
//! punctuation is an argument of its own, set against the word beside it rather than spaced
//! away from it, so `.Xr ls 1 ,` reads `ls(1),`.

use super::{
	escape::{Font, Piece},
	render::Renderer,
};

/// How a macro sets the text it covers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Style {
	Bold,
	Italic,
	Roman,
}

impl Style {
	const fn font(self) -> Font {
		match self {
			Self::Bold => Font::Bold,
			Self::Italic => Font::Italic,
			Self::Roman => Font::Roman,
		}
	}
}

/// The pair of marks an enclosing macro puts around what it covers.
const fn enclosure(name: &str) -> Option<(&'static str, &'static str)> {
	Some(match name.as_bytes() {
		b"Op" | b"Bq" | b"Bo" => ("[", "]"),
		b"Pq" | b"Po" => ("(", ")"),
		b"Aq" | b"Ao" => ("\u{27E8}", "\u{27E9}"),
		b"Brq" | b"Bro" => ("{", "}"),
		b"Dq" | b"Do" => ("\u{201C}", "\u{201D}"),
		b"Sq" | b"So" | b"Ql" => ("\u{2018}", "\u{2019}"),
		b"Qq" | b"Qo" => ("\"", "\""),
		_ => return None,
	})
}

/// The font a macro sets its arguments in, for the macros whose whole job is to name a kind of
/// thing. The distinctions `mdoc` draws are finer than a reader can hear, so they collapse onto
/// the three fonts roff has.
const fn style_of(name: &str) -> Option<Style> {
	Some(match name.as_bytes() {
		// The name of the thing, its flags, its commands: what a reader types.
		b"Nm" | b"Fl" | b"Cm" | b"Ic" | b"Sy" | b"Fn" | b"Fd" | b"In" | b"Cd" | b"Ms" => Style::Bold,
		// What stands for something else: arguments, variables, emphasis.
		b"Ar" | b"Em" | b"Va" | b"Fa" | b"Ft" | b"Vt" => Style::Italic,
		b"Pa" | b"Ev" | b"Er" | b"Dv" | b"Li" | b"No" | b"Sx" | b"Tn" | b"Ux" | b"At" | b"Bx" | b"Nx" | b"Ox"
		| b"Fx" | b"Dx" => Style::Roman,
		_ => return None,
	})
}

/// Whether a word is punctuation that sits against the word beside it rather than apart from
/// it. `mdoc` passes trailing punctuation as an argument of its own precisely so that it can be
/// set this way.
fn closing_punctuation(word: &str) -> bool {
	matches!(word, "." | "," | ";" | ":" | "?" | "!" | ")" | "]" | "}")
}

fn opening_punctuation(word: &str) -> bool {
	matches!(word, "(" | "[" | "{")
}

/// What the text of a page needs from the macros that name the page itself.
#[derive(Default)]
struct Names {
	/// The name and section from `.Dt`, which is how the page is titled.
	name: String,
	section: String,
	/// The name from the first `.Nm`, which is what a later bare `.Nm` stands for.
	subject: String,
}

/// The kind of list a `.Bl` opened, which decides what each `.It` in it looks like.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ListKind {
	/// Each item is headed by a tag of its own: the options of a page.
	Tag,
	Bullet,
	Dash,
	/// Numbered, counting as it goes.
	Enum,
	/// No mark of any kind.
	Plain,
}

struct List {
	kind: ListKind,
	number: usize,
}

pub(super) struct Reader<'a> {
	renderer: Renderer,
	names: Names,
	lists: Vec<List>,
	/// Whether the section being read is the synopsis, which is set one line per declaration.
	in_synopsis: bool,
	/// The sections read so far, with subsections nested under the section they fall in.
	pub(super) toc: Vec<crate::document::TocItem>,
	lines: Vec<&'a str>,
	index: usize,
	render_tables_inline: bool,
}

/// Everything a page turned into.
pub(super) struct Page {
	pub(super) buffer: crate::document::DocumentBuffer,
	pub(super) title: Option<String>,
	pub(super) toc: Vec<crate::document::TocItem>,
}

/// Whether a page is written in `mdoc` rather than `man`, which its first macro says: an `mdoc`
/// page opens with `.Dd`, and a `man` page with `.TH`.
pub(super) fn is_mdoc(source: &str) -> bool {
	for line in source.lines().take(super::SNIFF_LINES) {
		if super::is_comment(line) {
			continue;
		}
		let Some((name, _)) = super::control_line(line) else { continue };
		match name {
			"Dd" | "Dt" | "Os" => return true,
			"TH" => return false,
			_ => {}
		}
	}
	false
}

pub(super) fn read(source: &str, render_tables_inline: bool) -> Page {
	let mut reader = Reader {
		renderer: Renderer::new(),
		names: Names::default(),
		lists: Vec::new(),
		in_synopsis: false,
		toc: Vec::new(),
		lines: source.lines().collect(),
		index: 0,
		render_tables_inline,
	};
	reader.run();
	let title = (!reader.names.name.is_empty()).then(|| {
		if reader.names.section.is_empty() {
			reader.names.name.clone()
		} else {
			format!("{}({})", reader.names.name, reader.names.section)
		}
	});
	Page { buffer: reader.renderer.finish(), title, toc: reader.toc }
}

impl Reader<'_> {
	fn run(&mut self) {
		while self.index < self.lines.len() {
			let line = self.lines[self.index].to_string();
			self.index += 1;
			if super::is_comment(&line) {
				continue;
			}
			match super::control_line(&line) {
				Some((name, rest)) => {
					let (name, rest) = (name.to_string(), rest.to_string());
					self.macro_line(&name, &rest);
				}
				// A line that is not a macro call is text, whatever words are in it. Only a
				// control line carries macros, which is what lets a page open a sentence with
				// "No" without it being read as the macro of that name.
				None => self.renderer.write_line(&super::escape::pieces(&line)),
			}
		}
	}

	fn macro_line(&mut self, name: &str, rest: &str) {
		let words = split_words(rest);
		match name {
			// The date and the operating system, which name the edition rather than the page.
			"Dd" | "Os" | "Ud" | "Bt" => {}
			"Dt" => self.title_header(&words),
			// The first `.Nm` with a name in it is what a later bare `.Nm` stands for. The
			// title is not: `.Dt` writes the name in capitals, which is how a page is referred
			// to rather than how it is spelled.
			"Nm" if self.names.subject.is_empty() && words.first().is_some_and(|word| !word.text().is_empty()) => {
				self.names.subject = words[0].text();
				let pieces = self.evaluate(&words);
				self.renderer.write_line(&pieces);
			}
			"Ex" => self.exit_status(&words),
			"In" => self.include(&words),
			"Fn" => self.function(&words),
			"Ft" => {
				self.synopsis_break();
				let pieces = self.evaluate(&words);
				self.renderer.write_line(&pieces);
			}
			"Rv" => self.return_value(&words),
			"Sh" | "Ss" => self.heading(name, &words),
			"Nd" => {
				// The one-line description of the page, which follows its name on the same line.
				let mut pieces = vec![Piece::Text("- ".to_string())];
				pieces.extend(self.evaluate(&words));
				self.renderer.write_line(&pieces);
			}
			"Pp" | "Lp" | "sp" => self.renderer.end_paragraph(),
			"br" => self.renderer.break_line(),
			"Bl" => self.begin_list(&words),
			"El" => self.end_list(),
			"It" => self.item(&words),
			"Bd" => {
				self.renderer.break_line();
				self.renderer.set_filling(false);
			}
			"Ed" => {
				self.renderer.break_line();
				self.renderer.set_filling(true);
			}
			// A one-line literal display, which is a whole `.Bd`/`.Ed` in a single macro.
			"Dl" | "D1" => {
				self.renderer.break_line();
				let pieces = self.evaluate(&words);
				self.renderer.write_line(&pieces);
				self.renderer.break_line();
			}
			"TS" => self.table(),
			"Rs" | "Re" | "%A" | "%B" | "%D" | "%I" | "%J" | "%N" | "%O" | "%P" | "%Q" | "%R" | "%T" | "%U" | "%V" => {
				// A reference to something outside the page. Its parts are written one to a
				// line, which reads well enough left as they are.
				if let Some(rest) = name.strip_prefix('%') {
					let _ = rest;
					let pieces = self.evaluate(&words);
					self.renderer.write_line(&pieces);
					self.renderer.break_line();
				}
			}
			_ => {
				// Everything else is either an in-line macro, which lays down text, or a
				// typesetting instruction with nothing to say to a reader.
				let mut words = words;
				words.insert(0, Word::Bare(name.to_string()));
				let pieces = self.evaluate(&words);
				if !pieces.is_empty() {
					self.renderer.write_line(&pieces);
				}
			}
		}
	}

	/// `.Dt NAME SECTION`, which names the page.
	fn title_header(&mut self, words: &[Word]) {
		if let Some(name) = words.first().map(Word::text) {
			self.names.name = name;
		}
		if let Some(section) = words.get(1).map(Word::text) {
			self.names.section = section;
		}
	}

	fn heading(&mut self, name: &str, words: &[Word]) {
		let level = i32::from(name == "Ss") + 1;
		let text = super::escape::plain_pieces(&self.evaluate(words));
		let text = text.trim().to_string();
		self.in_synopsis = text.eq_ignore_ascii_case("SYNOPSIS");
		let Some(position) = self.renderer.heading(&text, level) else {
			return;
		};
		let item = crate::document::TocItem::new(text, String::new(), position);
		match self.toc.last_mut() {
			Some(section) if level > 1 => section.children.push(item),
			_ => self.toc.push(item),
		}
	}

	/// `.Bl` opens a list, and its first flag says what kind.
	fn begin_list(&mut self, words: &[Word]) {
		let kind = words
			.iter()
			.find_map(|word| match word.text().as_str() {
				"-tag" | "-hang" | "-ohang" | "-inset" | "-diag" => Some(ListKind::Tag),
				"-bullet" => Some(ListKind::Bullet),
				"-dash" | "-hyphen" => Some(ListKind::Dash),
				"-enum" => Some(ListKind::Enum),
				"-item" | "-column" => Some(ListKind::Plain),
				_ => None,
			})
			.unwrap_or(ListKind::Plain);
		self.renderer.end_paragraph();
		self.lists.push(List { kind, number: 0 });
	}

	fn end_list(&mut self) {
		self.lists.pop();
		self.renderer.end_list();
		self.renderer.end_paragraph();
	}

	/// `.It` starts an item, whose look depends on the list it is in. A tagged item carries its
	/// tag on the macro line, which is the same shape as a `man` page's `.TP`.
	fn item(&mut self, words: &[Word]) {
		let Some(list) = self.lists.last_mut() else {
			self.renderer.end_paragraph();
			return;
		};
		let kind = list.kind;
		list.number += 1;
		let number = list.number;
		let tag = match kind {
			ListKind::Tag => self.evaluate(words),
			ListKind::Bullet => vec![Piece::Text("\u{2022}".to_string())],
			ListKind::Dash => vec![Piece::Text("-".to_string())],
			ListKind::Enum => vec![Piece::Text(format!("{number}."))],
			ListKind::Plain => Vec::new(),
		};
		if tag.is_empty() {
			self.renderer.end_paragraph();
			if !words.is_empty() {
				let pieces = self.evaluate(words);
				self.renderer.write_line(&pieces);
			}
			return;
		}
		self.renderer.start_item(&tag);
		// A bulleted or numbered item carries its text on the same line as its mark.
		if kind != ListKind::Tag && !words.is_empty() {
			let pieces = self.evaluate(words);
			self.renderer.write_line(&pieces);
		}
	}

	/// A `tbl` table, which an `mdoc` page carries the same way a `man` page does.
	fn table(&mut self) {
		let mut table = super::table::Table::new();
		let mut rows: Vec<Vec<String>> = Vec::new();
		while self.index < self.lines.len() {
			let line = self.lines[self.index].to_string();
			self.index += 1;
			let mut index = self.index;
			let lines = &self.lines;
			let row = table.read_line(&line, || {
				let next = lines.get(index).map(|line| (*line).to_string());
				index += 1;
				next
			});
			self.index = index;
			match row {
				super::table::Row::Cells(cells) => rows.push(cells),
				super::table::Row::Nothing => {}
				super::table::Row::End => break,
			}
		}
		self.renderer.table(&rows, self.render_tables_inline);
	}

	/// `.In` is a header file, which the synopsis of a function page writes as the line a
	/// caller would have to put in their source.
	fn include(&mut self, words: &[Word]) {
		let Some(header) = words.first().map(Word::text) else { return };
		self.synopsis_break();
		self.renderer.write_line(&[
			Piece::Font(Font::Bold),
			Piece::Text(format!("#include <{header}>")),
			Piece::Font(Font::Roman),
		]);
		self.synopsis_break();
	}

	/// `.Fn name "arg" "arg"` is a function, and reads as the call a caller would write.
	fn function(&mut self, words: &[Word]) {
		let Some(name) = words.first().map(Word::text) else { return };
		self.synopsis_break();
		let mut pieces = vec![Piece::Font(Font::Bold), Piece::Text(name), Piece::Font(Font::Roman)];
		pieces.push(Piece::Text("(".to_string()));
		for (index, argument) in words[1..].iter().enumerate() {
			let text = argument.text();
			if closing_punctuation(argument.raw()) {
				continue;
			}
			if index > 0 {
				pieces.push(Piece::Font(Font::Roman));
				pieces.push(Piece::Text(", ".to_string()));
			}
			pieces.push(Piece::Font(Font::Italic));
			pieces.push(Piece::Text(text));
		}
		pieces.push(Piece::Font(Font::Roman));
		pieces.push(Piece::Text(")".to_string()));
		// Trailing punctuation belongs to the sentence the function was named in.
		for word in words[1..].iter().filter(|word| closing_punctuation(word.raw())) {
			pieces.push(Piece::Text(word.text()));
		}
		self.renderer.write_line(&pieces);
		self.synopsis_break();
	}

	/// A synopsis is written one declaration to a line, where the rest of a page fills.
	fn synopsis_break(&mut self) {
		if self.in_synopsis {
			self.renderer.break_line();
		}
	}

	/// `.Ex -std` stands for the sentence every page would otherwise write out by hand.
	fn exit_status(&mut self, words: &[Word]) {
		let name = self.named_subject(words);
		let pieces = vec![
			Piece::Text("The ".to_string()),
			Piece::Font(Font::Bold),
			Piece::Text(name),
			Piece::Font(Font::Roman),
			Piece::Text(" utility exits 0 on success, and >0 if an error occurs.".to_string()),
		];
		self.renderer.write_line(&pieces);
	}

	/// `.Rv -std` is the same thing for a function rather than a utility.
	fn return_value(&mut self, words: &[Word]) {
		let name = self.named_subject(words);
		let pieces = vec![
			Piece::Text("The ".to_string()),
			Piece::Font(Font::Bold),
			Piece::Text(name),
			Piece::Font(Font::Roman),
			Piece::Text(
				"() function returns 0 on success. On error, -1 is returned and errno is set to indicate the error."
					.to_string(),
			),
		];
		self.renderer.write_line(&pieces);
	}

	/// The name these stand for: the one they were given, or the page's own.
	fn named_subject(&self, words: &[Word]) -> String {
		words
			.iter()
			.map(Word::text)
			.find(|word| !word.starts_with('-') && !word.is_empty())
			.unwrap_or_else(|| self.names.subject.clone())
	}

	/// Turns a macro's arguments into text, evaluating any that are macros themselves.
	///
	/// This is the whole of `mdoc`: `.Op Fl f Ar file` is `Op` covering `Fl f` and `Ar file`,
	/// and reads `[-f file]`. The words are walked left to right, a macro taking the words after
	/// it until one closes it or the line ends.
	fn evaluate(&self, words: &[Word]) -> Vec<Piece> {
		let mut out = Vec::new();
		let mut index = 0;
		let mut spaced = false;
		self.evaluate_run(words, &mut index, &mut out, &mut spaced, None);
		// A font set by a macro ends with the line the macro is on. Roff carries a font over
		// to the next line and `man` pages rely on that; `mdoc` never does, because a macro
		// covers its own arguments and nothing else.
		if !out.is_empty() {
			out.push(Piece::Font(Font::Roman));
		}
		out
	}

	/// Evaluates words until `closer` is met or the words run out.
	fn evaluate_run(
		&self,
		words: &[Word],
		index: &mut usize,
		out: &mut Vec<Piece>,
		spaced: &mut bool,
		closer: Option<&str>,
	) {
		while *index < words.len() {
			let word = &words[*index];
			let text = word.text();
			if closer.is_some_and(|close| text == close) && word.is_bare() {
				*index += 1;
				return;
			}
			*index += 1;
			if !word.is_bare() {
				self.push_word(out, spaced, &text, Font::Roman);
				continue;
			}
			if let Some((open, close)) = enclosure(&text) {
				// Punctuation at the end of the arguments belongs outside the marks: `.Ql ^I .`
				// reads as the quoted text and then the full stop.
				let trailing = trailing_punctuation(words, *index);
				let inner_end = words.len() - trailing.len();
				self.push_word(out, spaced, open, Font::Roman);
				// Nothing is spaced away from an opening mark.
				*spaced = true;
				let closing = closing_name(&text);
				self.evaluate_run(&words[..inner_end], index, out, spaced, closing);
				out.push(Piece::Font(Font::Roman));
				out.push(Piece::Text(close.to_string()));
				for word in trailing {
					self.push_word(out, spaced, &word, Font::Roman);
					*index += 1;
				}
				continue;
			}
			match text.as_str() {
				// The name of the page, which stands alone once it has been given.
				"Nm" if self.next_is_macro_or_end(words, *index) => {
					let subject = if self.names.subject.is_empty() {
						self.names.name.clone()
					} else {
						self.names.subject.clone()
					};
					self.push_word(out, spaced, &subject, Font::Bold);
				}
				// A cross-reference to another page: its name and section, set together.
				"Xr" => {
					let name = words.get(*index).map(Word::text).unwrap_or_default();
					let section = words.get(*index + 1).map(Word::text).unwrap_or_default();
					*index += usize::from(!name.is_empty()) + usize::from(!section.is_empty());
					let reference =
						if section.is_empty() { name } else { format!("{name}({})", section.trim_matches('"')) };
					self.push_word(out, spaced, &reference, Font::Bold);
				}
				// An argument with nothing after it stands for the arguments in general.
				"Ar" if self.next_is_macro_or_end(words, *index) => {
					self.push_word(out, spaced, "file ...", Font::Italic);
				}
				// Written against what comes next, with no space between.
				"Ns" | "Pf" | "Ap" => *spaced = true,
				// A flag with nothing after it is the lone dash a page writes for standard input.
				"Fl" if self.next_is_macro_or_end(words, *index) => {
					self.push_word(out, spaced, "-", Font::Bold);
				}
				// The rest name a kind of thing, and set every word after them until a macro
				// that does something else.
				_ => match style_of(&text) {
					Some(style) => self.styled_run(words, index, out, spaced, style, &text),
					None => self.push_word(out, spaced, &text, Font::Roman),
				},
			}
		}
	}

	/// Lays down the words after a macro in its font, stopping at the next macro.
	///
	/// `.Fl` writes the dash a reader would type, once for each of its arguments, which is why
	/// a page writes `.Fl b` rather than `.Fl -b`.
	fn styled_run(
		&self,
		words: &[Word],
		index: &mut usize,
		out: &mut Vec<Piece>,
		spaced: &mut bool,
		style: Style,
		macro_name: &str,
	) {
		let flag = macro_name == "Fl";
		while *index < words.len() {
			let word = &words[*index];
			let text = word.text();
			if word.is_bare() && is_macro_name(&text) {
				break;
			}
			*index += 1;
			if closing_punctuation(word.raw()) {
				self.push_word(out, spaced, &text, Font::Roman);
				continue;
			}
			let text = if flag { format!("-{text}") } else { text };
			self.push_word(out, spaced, &text, style.font());
		}
	}

	/// Whether the word at `index` is a macro, or there is nothing there at all.
	fn next_is_macro_or_end(&self, words: &[Word], index: usize) -> bool {
		words.get(index).is_none_or(|word| word.is_bare() && is_macro_name(&word.text()))
	}

	/// Writes one word, spacing it from the one before unless punctuation says otherwise.
	fn push_word(&self, out: &mut Vec<Piece>, spaced: &mut bool, text: &str, font: Font) {
		if text.is_empty() {
			return;
		}
		let needs_space = !*spaced
			&& !out.is_empty()
			&& !closing_punctuation(text)
			&& !out.iter().rev().find_map(last_text).is_some_and(|last| opening_punctuation(&last));
		if needs_space {
			out.push(Piece::Font(Font::Roman));
			out.push(Piece::Text(" ".to_string()));
		}
		out.push(Piece::Font(font));
		out.push(Piece::Text(text.to_string()));
		*spaced = false;
	}
}

fn last_text(piece: &Piece) -> Option<String> {
	match piece {
		Piece::Text(text) if !text.is_empty() => Some(text.clone()),
		_ => None,
	}
}

/// The macro that closes an enclosure opened by the block form: `.Bo` is closed by `.Bc`.
const fn closing_name(open: &str) -> Option<&'static str> {
	Some(match open.as_bytes() {
		b"Bo" => "Bc",
		b"Po" => "Pc",
		b"Ao" => "Ac",
		b"Bro" => "Brc",
		b"Do" => "Dc",
		b"So" => "Sc",
		b"Qo" => "Qc",
		_ => return None,
	})
}

/// Whether a word names a macro. Every `mdoc` macro is two or three characters, a capital
/// followed by lower case, which is exactly why the language can be written this way.
fn is_macro_name(word: &str) -> bool {
	let mut chars = word.chars();
	let Some(first) = chars.next() else { return false };
	if !first.is_ascii_uppercase() {
		return false;
	}
	let rest: Vec<char> = chars.collect();
	if rest.is_empty() || rest.len() > 2 {
		return false;
	}
	rest.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
		&& (style_of(word).is_some() || enclosure(word).is_some() || OTHER_MACROS.contains(&word))
}

/// The macros that lay down text without naming a font of their own, which the argument walker
/// still has to recognise so that it stops the macro before them.
const OTHER_MACROS: &[&str] = &[
	"Nm", "Ar", "Xr", "Ns", "Pf", "Ap", "It", "Sx", "Fn", "Fa", "Ft", "Vt", "Va", "Pa", "Ev", "Er", "Dv", "Li", "No",
	"Cm", "Ic", "Sy", "Em", "Fl", "Tn", "Ux", "At", "Bx", "Nx", "Ox", "Fx", "Dx", "St", "Bc", "Pc", "Ac", "Brc", "Dc",
	"Sc", "Qc",
];

/// One argument of a macro line: a bare word, which may name a macro, or a quoted one, which
/// never does.
#[derive(Debug, Clone)]
pub(super) enum Word {
	Bare(String),
	Quoted(String),
}

impl Word {
	fn text(&self) -> String {
		super::escape::plain(self.raw())
	}

	/// The argument as it was written, escapes and all.
	///
	/// Whether a word is punctuation is asked of this rather than of the text: a page writes
	/// `\&.` for a full stop that belongs to the text, and the escape is what says so.
	const fn raw(&self) -> &String {
		match self {
			Self::Bare(text) | Self::Quoted(text) => text,
		}
	}

	const fn is_bare(&self) -> bool {
		matches!(self, Self::Bare(_))
	}
}

/// Splits a macro line into its arguments. A quoted argument is one word however many spaces it
/// holds, and is never read as a macro.
fn split_words(rest: &str) -> Vec<Word> {
	let mut words = Vec::new();
	let mut current = String::new();
	let mut quoted = false;
	let mut started = false;
	for c in rest.chars() {
		match c {
			'"' if quoted => {
				words.push(Word::Quoted(std::mem::take(&mut current)));
				quoted = false;
				started = false;
			}
			'"' if !started => {
				quoted = true;
				started = true;
			}
			c if c.is_whitespace() && !quoted => {
				if started {
					words.push(Word::Bare(std::mem::take(&mut current)));
					started = false;
				}
			}
			c => {
				current.push(c);
				started = true;
			}
		}
	}
	if started {
		words.push(if quoted { Word::Quoted(current) } else { Word::Bare(current) });
	}
	words
}

/// The punctuation at the end of a macro's arguments, which is set outside whatever the
/// macro encloses rather than inside it.
fn trailing_punctuation(words: &[Word], from: usize) -> Vec<String> {
	let mut trailing = Vec::new();
	for word in words[from.min(words.len())..].iter().rev() {
		let text = word.text();
		if word.is_bare() && closing_punctuation(word.raw()) {
			trailing.push(text);
		} else {
			break;
		}
	}
	trailing.reverse();
	trailing
}
