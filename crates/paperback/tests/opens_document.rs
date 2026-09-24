//! Opening a document from the command line.

#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

use common::fixture;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn opening_a_document_focuses_its_text_and_names_it_in_the_title() {
	let app = common::launch("opens-document");
	let log = app.config_dir().join("paperback.log");
	common::wait_until(
		Duration::from_secs(30),
		|| app.logs().contains(fixture::HTML_NAME),
		|| format!("{} does not mention the fixture; exists: {}", log.display(), log.exists()),
	);

	common::wait_for_reader_focus(app.pid);
	common::wait_for_title(app.pid, &format!("{} - Paperback", fixture::TITLE));
}
