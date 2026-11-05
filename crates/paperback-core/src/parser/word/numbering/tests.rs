use roxmltree::Document as XmlDocument;

use super::{ListLabel, Numbering, format_number};

const W: &str = r#"xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main""#;

fn numbering_xml() -> String {
	format!(
		r#"<w:numbering {W}>
			<w:abstractNum w:abstractNumId="0">
				<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>
				<w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1.%2."/></w:lvl>
				<w:lvl w:ilvl="2"><w:start w:val="1"/><w:numFmt w:val="lowerLetter"/><w:lvlText w:val="%3)"/></w:lvl>
			</w:abstractNum>
			<w:abstractNum w:abstractNumId="1">
				<w:lvl w:ilvl="0"><w:numFmt w:val="bullet"/><w:lvlText w:val="&#xF0B7;"/></w:lvl>
			</w:abstractNum>
			<w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num>
			<w:num w:numId="2"><w:abstractNumId w:val="1"/></w:num>
			<w:num w:numId="3"><w:abstractNumId w:val="0"/>
				<w:lvlOverride w:ilvl="0"><w:startOverride w:val="1"/></w:lvlOverride>
			</w:num>
		</w:numbering>"#
	)
}

fn styles_xml() -> String {
	format!(
		r#"<w:styles {W}>
			<w:style w:type="paragraph" w:styleId="ListNumber"><w:pPr><w:numPr><w:numId w:val="1"/></w:numPr></w:pPr></w:style>
			<w:style w:type="paragraph" w:styleId="Steps"><w:basedOn w:val="ListNumber"/></w:style>
		</w:styles>"#
	)
}

fn label(numbering: &mut Numbering, ppr_inner: &str) -> Option<String> {
	let xml = format!("<w:pPr {W}>{ppr_inner}</w:pPr>");
	let doc = XmlDocument::parse(&xml).expect("test pPr parses");
	numbering.label_for(Some(doc.root_element())).map(|l| l.text)
}

fn item(num_id: u32, ilvl: u32) -> String {
	format!(r#"<w:numPr><w:ilvl w:val="{ilvl}"/><w:numId w:val="{num_id}"/></w:numPr>"#)
}

#[test]
fn numbered_paragraphs_count_on_and_nested_levels_restart_under_their_parent() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), None);
	let labels: Vec<_> = [(1, 0), (1, 1), (1, 1), (1, 2), (1, 0), (1, 1)]
		.iter()
		.map(|&(n, l)| label(&mut numbering, &item(n, l)))
		.collect();
	assert_eq!(
		labels,
		[Some("1."), Some("1.1."), Some("1.2."), Some("a)"), Some("2."), Some("2.1.")].map(|s| s.map(String::from))
	);
}

#[test]
fn a_restarting_instance_begins_the_shared_count_again() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), None);
	assert_eq!(label(&mut numbering, &item(1, 0)).as_deref(), Some("1."));
	assert_eq!(label(&mut numbering, &item(1, 0)).as_deref(), Some("2."));
	// numId 3 shares abstract list 0 but restarts level 0.
	assert_eq!(label(&mut numbering, &item(3, 0)).as_deref(), Some("1."));
	assert_eq!(label(&mut numbering, &item(3, 0)).as_deref(), Some("2."));
}

#[test]
fn restarting_two_levels_at_once_numbers_the_same_every_time() {
	let xml = format!(
		r#"<w:numbering {W}>
			<w:abstractNum w:abstractNumId="0">
				<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>
				<w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1.%2."/></w:lvl>
			</w:abstractNum>
			<w:num w:numId="1"><w:abstractNumId w:val="0"/>
				<w:lvlOverride w:ilvl="1"><w:startOverride w:val="5"/></w:lvlOverride>
				<w:lvlOverride w:ilvl="0"><w:startOverride w:val="3"/></w:lvlOverride>
			</w:num>
		</w:numbering>"#
	);
	// Each load hashes differently, so a hash-ordered restart disagreed with itself within a few tries.
	let labels: std::collections::HashSet<Option<String>> = (0..32)
		.map(|_| {
			let mut numbering = Numbering::load(Some(&xml), None);
			label(&mut numbering, &item(1, 1))
		})
		.collect();
	assert_eq!(labels.len(), 1, "{labels:?}");
	assert_eq!(
		labels.into_iter().next().flatten().as_deref(),
		Some("3.5."),
		"the outer level shows its restart, not one below it"
	);
}

#[test]
fn a_restarted_level_counts_on_from_its_restart() {
	let xml = format!(
		r#"<w:numbering {W}>
			<w:abstractNum w:abstractNumId="0">
				<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl>
				<w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1.%2."/></w:lvl>
			</w:abstractNum>
			<w:num w:numId="1"><w:abstractNumId w:val="0"/>
				<w:lvlOverride w:ilvl="0"><w:startOverride w:val="3"/></w:lvlOverride>
			</w:num>
		</w:numbering>"#
	);
	let mut numbering = Numbering::load(Some(&xml), None);
	let labels: Vec<_> =
		[(1, 1), (1, 0), (1, 1), (1, 0)].iter().map(|&(n, l)| label(&mut numbering, &item(n, l))).collect();
	assert_eq!(labels, [Some("3.1."), Some("3."), Some("3.1."), Some("4.")].map(|s| s.map(String::from)));
}

#[test]
fn symbol_font_bullets_become_a_plain_bullet() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), None);
	assert_eq!(label(&mut numbering, &item(2, 0)).as_deref(), Some("\u{2022}"));
}

#[test]
fn paragraph_styles_number_their_paragraphs_through_based_on() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), Some(&styles_xml()));
	assert_eq!(label(&mut numbering, r#"<w:pStyle w:val="ListNumber"/>"#).as_deref(), Some("1."));
	assert_eq!(label(&mut numbering, r#"<w:pStyle w:val="Steps"/>"#).as_deref(), Some("2."));
}

#[test]
fn num_id_zero_takes_a_paragraph_out_of_its_styles_list() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), Some(&styles_xml()));
	let ppr = r#"<w:pStyle w:val="ListNumber"/><w:numPr><w:numId w:val="0"/></w:numPr>"#;
	assert_eq!(label(&mut numbering, ppr), None);
}

#[test]
fn unnumbered_paragraphs_and_documents_without_numbering_get_no_label() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), None);
	assert_eq!(label(&mut numbering, ""), None);
	let mut empty = Numbering::load(None, None);
	assert_eq!(label(&mut empty, &item(1, 0)), None);
}

#[test]
fn levels_are_reported_from_one() {
	let mut numbering = Numbering::load(Some(&numbering_xml()), None);
	let xml = format!("<w:pPr {W}>{}</w:pPr>", item(1, 1));
	let doc = XmlDocument::parse(&xml).expect("test pPr parses");
	assert_eq!(numbering.label_for(Some(doc.root_element())), Some(ListLabel { text: "1.1.".to_string(), level: 2 }));
}

#[test]
fn number_formats_match_words() {
	assert_eq!(format_number(4, "lowerRoman"), "iv");
	assert_eq!(format_number(1994, "upperRoman"), "MCMXCIV");
	assert_eq!(format_number(3, "upperLetter"), "C");
	assert_eq!(format_number(28, "lowerLetter"), "bb");
	assert_eq!(format_number(7, "decimalZero"), "07");
	assert_eq!(format_number(12, "decimal"), "12");
	assert_eq!(format_number(5, "none"), "");
}
