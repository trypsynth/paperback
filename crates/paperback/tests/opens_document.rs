//! Opening a document from the command line.

#![cfg(target_os = "windows")]

mod common;

use std::time::Duration;

#[test]
#[ignore = "drives the real GUI via UI Automation; needs an interactive desktop"]
fn opening_a_document_logs_to_the_configured_dir() {
	let app = common::launch("opens-document");
	let log = app.config_dir().join("paperback.log");
	common::wait_until(
		Duration::from_secs(30),
		|| app.logs().contains("ui-test.html"),
		|| format!("{} does not mention the fixture; exists: {}", log.display(), log.exists()),
	);
}
