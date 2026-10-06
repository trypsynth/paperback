//! The commit hash and dev/release flag baked into the binary for the about dialog.

pub fn embed_commit_hash() {
	shipfitter::build::embed_commit_info("PAPERBACK");
}
