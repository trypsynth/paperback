use super::{find_token, int_value, name_value, rebuild, ref_value, scan_objects};

/// A one-page tagged PDF written the way Apple Pages writes one: the structure elements are all
/// there, the tree root names a `/ParentTree` object the file never writes, and the page carries
/// no `/StructParents`. The offsets in the cross-reference table are not real; nothing here reads
/// them.
fn pages_style_pdf() -> String {
	concat!(
		"%PDF-1.3\n",
		"1 0 obj\n<< /Type /Page /Parent 2 0 R /Contents 3 0 R /MediaBox [0 0 595 842] >>\nendobj\n",
		"2 0 obj\n<< /Type /Pages /Kids [1 0 R] /Count 1 >>\nendobj\n",
		"3 0 obj\n<< /Length 25 /Filter /FlateDecode >>\nstream\n1 0 obj endstream endobj \nendstream\nendobj\n",
		"4 0 obj\n<< /Type /StructTreeRoot /K 5 0 R /ParentTree 20 0 R >>\nendobj\n",
		"5 0 obj\n<< /Type /StructElem /S /Document /P 4 0 R /K [6 0 R 7 0 R] >>\nendobj\n",
		"6 0 obj\n<< /Type /StructElem /S /H1 /P 5 0 R /Pg 1 0 R /K 1 >>\nendobj\n",
		"7 0 obj\n<< /Type /StructElem /S /P /P 5 0 R /Pg 1 0 R /K 2 >>\nendobj\n",
		"8 0 obj\n<< /Type /Catalog /Pages 2 0 R /StructTreeRoot 4 0 R >>\nendobj\n",
		"xref\n0 1\n0000000000 65535 f \n",
		"trailer\n<< /Size 9 /Root 8 0 R >>\nstartxref\n1234\n%%EOF\n",
	)
	.to_string()
}

#[test]
fn scan_steps_over_object_headers_inside_streams() {
	let pdf = pages_style_pdf();
	let numbers: Vec<u32> = scan_objects(pdf.as_bytes()).iter().map(|object| object.number).collect();
	// Object 3's stream data spells "1 0 obj", "endstream" and "endobj"; its stated /Length is
	// what has to carry the scan past all three.
	assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn scan_keeps_the_last_definition_of_a_number() {
	let pdf = format!("{}1 0 obj\n<< /Type /Page /Rewritten true >>\nendobj\n", pages_style_pdf());
	let objects = scan_objects(pdf.as_bytes());
	let page = objects.iter().find(|object| object.number == 1).expect("object 1");
	assert!(find_token(&pdf.as_bytes()[page.start..page.end], b"/Rewritten").is_some(), "the newest definition wins");
	assert_eq!(objects.iter().filter(|object| object.number == 1).count(), 1);
}

#[test]
fn name_and_ref_values_read_whole_tokens_only() {
	let page = b"<< /Type /Page /Parent 2 0 R /StructParents 4 >>".as_slice();
	assert_eq!(name_value(page, b"/Type"), Some(b"Page".as_slice()));
	assert_eq!(ref_value(page, b"/Parent"), Some(2));
	assert_eq!(int_value(page, b"/StructParents"), Some(4));
	// "/Page" must not be found inside "/Pages", nor "/P" inside "/Parent".
	let pages = b"<< /Type /Pages /Kids [1 0 R] >>".as_slice();
	assert_eq!(name_value(pages, b"/Type"), Some(b"Pages".as_slice()));
	assert_eq!(ref_value(pages, b"/P"), None);
}

#[test]
fn repair_fills_in_the_parent_tree_the_file_names_but_never_writes() {
	let pdf = pages_style_pdf();
	let repaired = rebuild(pdf.as_bytes()).expect("a pdf whose parent tree object is missing is repaired");
	let text = String::from_utf8_lossy(&repaired).into_owned();
	assert!(text.starts_with(&pdf), "the original bytes stay where they are");
	// The tree root already points at object 20, so it is object 20 that gets written.
	assert!(text.contains("20 0 obj\n<< /Nums [ 0 [ 6 0 R 7 0 R ] ] >>"), "{text}");
	assert!(!text.contains("4 0 obj\n<< /ParentTree"), "the tree root needs no rewrite");
	assert!(text.contains("<< /StructParents 0 /Type /Page /Parent 2 0 R"), "{text}");
	assert!(text.contains("/Size 21 /Root 8 0 R /Prev 1234"), "{text}");
	assert_xref_points_at_its_objects(&repaired, &[1, 20]);
}

#[test]
fn repair_adds_the_key_when_the_tree_root_has_none() {
	let pdf = pages_style_pdf().replace(" /ParentTree 20 0 R", "");
	let repaired = rebuild(pdf.as_bytes()).expect("a pdf with no parent tree key is repaired");
	let text = String::from_utf8_lossy(&repaired).into_owned();
	// Nothing names a tree, so it is written past the highest number the file uses.
	assert!(text.contains("4 0 obj\n<< /ParentTree 9 0 R /Type /StructTreeRoot"), "{text}");
	assert!(text.contains("9 0 obj\n<< /Nums [ 0 [ 6 0 R 7 0 R ] ] >>"), "{text}");
	assert_xref_points_at_its_objects(&repaired, &[1, 4, 9]);
}

#[test]
fn a_pdf_whose_parent_tree_object_is_there_is_left_alone() {
	let pdf = format!("{}20 0 obj\n<< /Nums [0 [6 0 R]] >>\nendobj\n", pages_style_pdf());
	assert!(rebuild(pdf.as_bytes()).is_none());
}

#[test]
fn an_encrypted_pdf_is_left_alone() {
	let pdf = pages_style_pdf().replace("/Size 9 /Root", "/Encrypt 10 0 R /Size 9 /Root");
	assert!(rebuild(pdf.as_bytes()).is_none());
}

#[test]
fn a_pdf_with_object_streams_is_left_alone() {
	let pdf = pages_style_pdf().replace("/Filter /FlateDecode", "/Type /ObjStm /N 4");
	assert!(rebuild(pdf.as_bytes()).is_none());
}

#[test]
fn a_page_that_already_has_a_key_keeps_it_and_is_not_rewritten() {
	let pdf = pages_style_pdf().replace("<< /Type /Page /Parent", "<< /StructParents 7 /Type /Page /Parent");
	let repaired = rebuild(pdf.as_bytes()).expect("still repaired");
	let text = String::from_utf8_lossy(&repaired).into_owned();
	assert!(text.contains("20 0 obj\n<< /Nums [ 7 [ 6 0 R 7 0 R ] ] >>"), "{text}");
	assert!(!text[pdf.len()..].contains("1 0 obj"), "no rewritten page object in the update");
}

/// Every entry of the update's cross-reference section has to be twenty bytes long and has to
/// name the offset the object it stands for was written at.
fn assert_xref_points_at_its_objects(repaired: &[u8], numbers: &[u32]) {
	let text = String::from_utf8_lossy(repaired).into_owned();
	let section = &text[text.rfind("\nxref\n").expect("the update's own xref") + 1..];
	for entry in section.lines().filter(|line| line.ends_with(" n ") || line.ends_with(" f ")) {
		assert_eq!(entry.len() + 1, 20, "a twenty byte entry: {entry:?}");
	}
	for number in numbers {
		let entry = section.split(&format!("\n{number} 1\n")).nth(1).expect("an xref entry for the object");
		let offset: usize = entry[..10].parse().expect("a ten digit offset");
		assert!(repaired[offset..].starts_with(format!("{number} 0 obj").as_bytes()), "object {number} at {offset}");
	}
}
