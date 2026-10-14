use super::*;

#[test]
fn a_new_scratch_pad_is_an_empty_titled_document_with_no_file() {
	let session = DocumentSession::new_scratch("scratch1");
	assert_eq!(session.title(), "scratch1");
	assert_eq!(session.content(), "");
	assert_eq!(session.document_len(), 0);
	assert_eq!(session.file_path, "");
	// Nothing was parsed, so there is nothing to navigate to by heading, page or link.
	assert_eq!(session.parser_flags, ParserFlags::NONE);
}

/// What the reader's own commands ask a session after the text has been typed. All of these read
/// the buffer rather than figures computed once at parse time, which is what lets a scratch pad
/// keep Go to Line, Go to Percent and the status bar working as the text changes.
#[test]
fn the_position_helpers_follow_the_text_typed_into_it() {
	let mut session = DocumentSession::new_scratch("scratch1");
	let typed = "first line\nsecond line\nthird line";
	session.replace_range(0, 0, typed);
	assert_eq!(session.content(), typed);
	assert_eq!(session.line_count(), 3);
	assert_eq!(session.position_from_line(2), 11);
	assert_eq!(session.get_line_text(session.position_from_line(2)), "second line");
	assert_eq!(session.get_status_info(0).line_number, 1);
	assert_eq!(session.get_status_info(session.position_from_line(3)).line_number, 3);
	assert_eq!(session.position_from_percent(100), i64::try_from(typed.chars().count()).unwrap());
}

/// The whole text is replaced rather than appended to on every keystroke, so a later line is
/// reached only if the edit that shortened the text took the earlier one with it.
#[test]
fn replacing_the_whole_text_replaces_what_the_helpers_report() {
	let mut session = DocumentSession::new_scratch("scratch1");
	session.replace_range(0, 0, "one\ntwo\nthree");
	let old_len = session.document_len();
	session.replace_range(0, old_len, "only one line");
	assert_eq!(session.content(), "only one line");
	assert_eq!(session.line_count(), 1);
	assert_eq!(session.get_line_text(0), "only one line");
}

/// The figures Word Count and Document Info read, which an edit leaves behind until something asks
/// for them again.
#[test]
fn statistics_are_stale_until_they_are_recomputed() {
	let mut session = DocumentSession::new_scratch("scratch1");
	assert_eq!(session.stats().word_count, 0);
	session.replace_range(0, 0, "four words are here");
	assert_eq!(session.stats().word_count, 0, "the buffer changed but nothing has recounted");
	session.recompute_stats();
	assert_eq!(session.stats().word_count, 4);
}
