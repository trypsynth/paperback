use super::*;

fn titles(nodes: &[ContentsNode]) -> Vec<String> {
	nodes.iter().map(|node| format!("{}{:?}", node.title, titles(&node.children))).collect()
}

#[test]
fn reads_entries_as_writetree_streams_them() {
	let data = write_contents(&[(0, "index.html", "Qt Core"), (1, "qobject.html#details", "QObject — détails")]);
	assert_eq!(
		read_contents(&data),
		vec![
			ContentsEntry { depth: 0, link: "index.html".into(), title: "Qt Core".into() },
			ContentsEntry { depth: 1, link: "qobject.html#details".into(), title: "QObject — détails".into() },
		]
	);
}

#[test]
fn reads_characters_outside_the_basic_plane() {
	let data = write_contents(&[(0, "a.html", "Music 𝄞")]);
	assert_eq!(read_contents(&data)[0].title, "Music 𝄞");
}

#[test]
fn a_null_link_is_empty() {
	let mut data = 0i32.to_be_bytes().to_vec();
	data.extend_from_slice(&NULL_STRING.to_be_bytes());
	data.extend_from_slice(&write_contents(&[(0, "", "T")])[8..]);
	assert_eq!(read_contents(&data), vec![ContentsEntry { depth: 0, link: String::new(), title: "T".into() }]);
}

#[test]
fn reads_an_extended_length() {
	let mut data = 0i32.to_be_bytes().to_vec();
	for text in ["a.html", "Title"] {
		let units: Vec<u16> = text.encode_utf16().collect();
		data.extend_from_slice(&EXTENDED_SIZE.to_be_bytes());
		data.extend_from_slice(&(units.len() as u64 * 2).to_be_bytes());
		for unit in units {
			data.extend_from_slice(&unit.to_be_bytes());
		}
	}
	assert_eq!(read_contents(&data)[0].title, "Title");
}

#[test]
fn stops_at_an_entry_with_no_title() {
	let data = write_contents(&[(0, "a.html", "A"), (0, "b.html", ""), (0, "c.html", "C")]);
	assert_eq!(read_contents(&data).len(), 1);
}

#[test]
fn stops_at_truncated_or_corrupt_data() {
	let data = write_contents(&[(0, "a.html", "A"), (0, "b.html", "B")]);
	assert_eq!(read_contents(&data[..data.len() - 1]).len(), 1);
	let mut odd = write_contents(&[(0, "a.html", "A")]);
	odd[7] = 3;
	assert!(read_contents(&odd).is_empty());
	assert!(read_contents(&[0, 0]).is_empty());
}

#[test]
fn builds_the_tree_from_depths() {
	let entries = read_contents(&write_contents(&[
		(0, "a.html", "A"),
		(1, "b.html", "B"),
		(2, "c.html", "C"),
		(1, "d.html", "D"),
		(0, "e.html", "E"),
	]));
	assert_eq!(titles(&build_tree(entries)), vec![r#"A["B[\"C[]\"]", "D[]"]"#, "E[]"]);
}

/// The case `qhelpenginecore.cpp` describes: 0, 2, 2, 1 makes the last three children of the first.
#[test]
fn a_gap_in_depths_nests_one_level() {
	let entries = read_contents(&write_contents(&[(0, "", "A"), (2, "", "B"), (2, "", "C"), (1, "", "D")]));
	assert_eq!(titles(&build_tree(entries)), vec![r#"A["B[]", "C[]", "D[]"]"#]);
}

#[test]
fn a_negative_or_late_first_depth_is_a_root() {
	let entries = read_contents(&write_contents(&[(3, "", "A"), (-5, "", "B"), (4, "", "C")]));
	assert_eq!(titles(&build_tree(entries)), vec!["A[]", r#"B["C[]"]"#]);
}
