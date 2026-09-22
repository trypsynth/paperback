use super::rst_to_html;

#[test]
fn underlined_title_becomes_a_heading() {
	let html = rst_to_html("Chapter One\n===========\n\nBody text.\n");
	assert_eq!(html, r#"<h1 id="chapter-one">Chapter One</h1><p>Body text.</p>"#);
}

#[test]
fn overlined_title_is_also_a_heading() {
	let html = rst_to_html("=====\nTitle\n=====\n\nBody.\n");
	assert_eq!(html, r#"<h1 id="title">Title</h1><p>Body.</p>"#);
}

#[test]
fn adornment_characters_are_leveled_by_first_appearance() {
	let html = rst_to_html("One\n===\n\nTwo\n---\n\nThree\n=====\n\nFour\n----\n");
	// The second `===`/`---` pair reuses the characters already seen, so they repeat the same
	// levels rather than climbing further.
	assert_eq!(html, r#"<h1 id="one">One</h1><h2 id="two">Two</h2><h1 id="three">Three</h1><h2 id="four">Four</h2>"#);
}

#[test]
fn bullet_list_with_nested_paragraph() {
	let html = rst_to_html("- one\n- two\n  still two\n- three\n");
	assert_eq!(html, "<ul><li><p>one</p></li><li><p>two still two</p></li><li><p>three</p></li></ul>");
}

#[test]
fn enumerated_list_with_auto_numbering_marker() {
	let html = rst_to_html("#. first\n#. second\n");
	assert_eq!(html, "<ol><li><p>first</p></li><li><p>second</p></li></ol>");
}

#[test]
fn nested_bullet_list_inside_enumerated_item() {
	let html = rst_to_html("1. one\n   - nested\n   - bullets\n2. two\n");
	assert_eq!(
		html,
		"<ol><li><p>one</p><ul><li><p>nested</p></li><li><p>bullets</p></li></ul></li><li><p>two</p></li></ol>"
	);
}

#[test]
fn literal_block_after_double_colon() {
	let html = rst_to_html("Example::\n\n    code here\n    more code\n");
	assert_eq!(html, "<p>Example:</p><pre><code>code here\nmore code</code></pre>");
}

#[test]
fn lone_double_colon_paragraph_is_dropped() {
	let html = rst_to_html("::\n\n    raw block\n");
	assert_eq!(html, "<pre><code>raw block</code></pre>");
}

#[test]
fn code_block_directive_is_rendered_literally() {
	let html = rst_to_html(".. code-block:: python\n\n    print(1)\n");
	assert_eq!(html, "<pre><code>print(1)</code></pre>");
}

#[test]
fn note_directive_becomes_a_labeled_blockquote() {
	let html = rst_to_html(".. note::\n\n   Pay attention.\n");
	assert_eq!(html, "<blockquote><p><strong>Note</strong></p><p>Pay attention.</p></blockquote>");
}

#[test]
fn unknown_directive_still_shows_its_body() {
	let html = rst_to_html(".. only:: html\n\n   Visible content.\n");
	assert_eq!(html, "<p>Visible content.</p>");
}

#[test]
fn toctree_directive_is_skipped_entirely() {
	let html = rst_to_html("Before.\n\n.. toctree::\n   :maxdepth: 2\n\n   chapter-one\n   chapter-two\n\nAfter.\n");
	assert_eq!(html, "<p>Before.</p><p>After.</p>");
}

#[test]
fn image_directive_becomes_an_img_tag() {
	let html = rst_to_html(".. image:: cover.png\n   :alt: Cover art\n");
	assert_eq!(html, r#"<img alt="Cover art">"#);
}

#[test]
fn comment_is_dropped_along_with_its_indented_body() {
	let html = rst_to_html("Before.\n\n.. this is a comment\n   with a continuation\n\nAfter.\n");
	assert_eq!(html, "<p>Before.</p><p>After.</p>");
}

#[test]
fn transition_becomes_a_horizontal_rule() {
	let html = rst_to_html("First.\n\n----\n\nSecond.\n");
	assert_eq!(html, "<p>First.</p><hr><p>Second.</p>");
}

#[test]
fn definition_list_pairs_terms_with_their_indented_body() {
	let html = rst_to_html("term one\n    definition one\nterm two\n    definition two\n");
	assert_eq!(
		html,
		"<dl><dt>term one</dt><dd><p>definition one</p></dd><dt>term two</dt><dd><p>definition two</p></dd></dl>"
	);
}

#[test]
fn field_list_becomes_a_definition_list() {
	let html = rst_to_html(":Author: Jane Doe\n:Version: 1.0\n");
	assert_eq!(html, "<dl><dt>Author</dt><dd>Jane Doe</dd><dt>Version</dt><dd>1.0</dd></dl>");
}

#[test]
fn strong_and_emphasis_and_literal_markup() {
	let html = rst_to_html("**bold** and *italic* and ``code``.\n");
	assert_eq!(html, "<p><strong>bold</strong> and <em>italic</em> and <code>code</code>.</p>");
}

/// A real bug found against Flask's changelog: a double-backtick literal wrapping a dunder name,
/// followed by a `:pr:` role, was mis-scanned as a hyperlink reference starting at the second
/// backtick of the literal's own opening pair, eating the leading underscores of its content.
#[test]
fn double_backtick_literal_around_dunder_name_is_not_mistaken_for_a_reference() {
	let html = rst_to_html("Remove ``__version__``. See :pr:`5648`\n");
	assert_eq!(html, "<p>Remove <code>__version__</code>. See 5648</p>");
}

#[test]
fn embedded_uri_reference_becomes_a_link() {
	let html = rst_to_html("See `the docs <https://example.com/>`_ for more.\n");
	assert_eq!(html, r#"<p>See <a href="https://example.com/">the docs</a> for more.</p>"#);
}

/// A URL wrapped across a source line (docutils' convention for long links) must not gain a
/// space in the middle from the paragraph's line-join.
#[test]
fn embedded_uri_wrapped_across_lines_has_no_inserted_space() {
	let html = rst_to_html("`long link <https://example.com/a/very/long\n/path>`_.\n");
	assert_eq!(html, r#"<p><a href="https://example.com/a/very/long/path">long link</a>.</p>"#);
}

#[test]
fn reference_to_an_explicit_target_resolves() {
	let html = rst_to_html("See Example_ for details.\n\n.. _Example: https://example.com/\n");
	assert_eq!(html, r#"<p>See <a href="https://example.com/">Example</a> for details.</p>"#);
}

/// Every section title implicitly names a target; a reference to a section that defines no
/// explicit `.. _label:` target of its own should still resolve, as a genuine in-book jump link
/// to the heading (matching its own `id`) rather than falling back to plain text.
#[test]
fn reference_to_a_section_title_becomes_an_internal_link() {
	let html = rst_to_html("See Details_ below.\n\nDetails\n=======\n\nBody.\n");
	assert_eq!(html, r##"<p>See <a href="#details">Details</a> below.</p><h1 id="details">Details</h1><p>Body.</p>"##);
}

/// An explicit target with no URL of its own (`.. _label:`) binds to whichever block follows it,
/// same as docutils' "internal hyperlink target" - here, a paragraph rather than a heading.
#[test]
fn anchor_only_target_binds_to_the_following_block() {
	let html = rst_to_html("See Note_ below.\n\n.. _Note:\n\nA plain paragraph.\n");
	assert_eq!(html, r##"<p>See <a href="#note">Note</a> below.</p><p id="note">A plain paragraph.</p>"##);
}

#[test]
fn reference_to_an_undefined_target_is_left_as_plain_text() {
	let html = rst_to_html("See Nowhere_ for nothing.\n");
	assert_eq!(html, "<p>See Nowhere_ for nothing.</p>");
}

#[test]
fn footnote_reference_drops_the_trailing_underscore() {
	let html = rst_to_html("A claim [1]_ needs support.\n");
	assert_eq!(html, "<p>A claim [1] needs support.</p>");
}

/// A real bug found while manually testing a sample book: `.. [1] body text` (a footnote
/// definition) was falling into the generic comment branch and vanishing entirely.
#[test]
fn footnote_definition_body_is_not_dropped() {
	let html = rst_to_html("See the claim [1]_.\n\n.. [1] This is the footnote body.\n");
	assert_eq!(html, "<p>See the claim [1].</p><p>[1] This is the footnote body.</p>");
}

#[test]
fn citation_definition_body_is_not_dropped() {
	let html = rst_to_html(".. [CIT2002] A citation body,\n   continued on a second line.\n");
	assert_eq!(html, "<p>[CIT2002] A citation body, continued on a second line.</p>");
}

#[test]
fn substitution_reference_is_replaced() {
	let html = rst_to_html("Version |version|.\n\n.. |version| replace:: 2.0\n");
	assert_eq!(html, "<p>Version 2.0.</p>");
}

#[test]
fn undefined_substitution_is_left_as_is() {
	let html = rst_to_html("Version |version|.\n");
	assert_eq!(html, "<p>Version |version|.</p>");
}

#[test]
fn bare_url_is_autolinked() {
	let html = rst_to_html("Visit https://example.com/page for more.\n");
	assert_eq!(html, r#"<p>Visit <a href="https://example.com/page">https://example.com/page</a> for more.</p>"#);
}

#[test]
fn html_special_characters_are_escaped() {
	let html = rst_to_html("A <tag> & an ampersand.\n");
	assert_eq!(html, "<p>A &lt;tag&gt; &amp; an ampersand.</p>");
}

#[test]
fn backslash_escapes_suppress_markup() {
	let html = rst_to_html("Not \\*\\*bold\\*\\*.\n");
	assert_eq!(html, "<p>Not **bold**.</p>");
}

#[test]
fn block_quote_wraps_an_indented_paragraph() {
	let html = rst_to_html("Normal.\n\n    Quoted text.\n\nBack to normal.\n");
	assert_eq!(html, "<p>Normal.</p><blockquote><p>Quoted text.</p></blockquote><p>Back to normal.</p>");
}

/// Nothing here should ever panic, however malformed the input: this crate does not use a
/// battle-tested RST implementation, and a crash on an untrusted document would take the whole
/// reader down with it.
#[test]
fn never_panics_on_pathological_input() {
	let samples = [
		"",
		"   \n\n\t\n",
		"**unterminated bold\n",
		"``unterminated literal\n",
		"`unterminated ref\n",
		":role:`unterminated\n",
		".. \n",
		".. directive::\n",
		".. _label:\n",
		"===\n===\n===\n",
		"- \n- \n",
		"1.\n",
		":\n",
		"|\n",
		"[",
		"\\",
		&"=".repeat(5000),
		"Title\n=====\n\n   deeply\n      nested\n         quotes\n            forever\n",
		"**bold with ``literal`` nested inside it**\n",
		"1. one\n   - nested bullet\n     continued\n2. two\n",
		".. image::\n",
		".. |x|\n",
		"term\n    def1\nterm\n    def2\n",
		"* \n\n* \n",
		"`only opening backtick",
		"``\n``\n",
	];
	for sample in samples {
		let _ = rst_to_html(sample);
	}
}
