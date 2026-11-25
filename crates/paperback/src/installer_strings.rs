//! The Windows installer's own strings. They live beside the app's so `cargo xtask pot` takes them into the catalogs; the build script includes this file and writes them into the installer script in every language, so it is not part of the app.

/// What marks a string for the catalogs; the build script translates it from them.
const fn t(text: &'static str) -> &'static str {
	text
}

/// Each string's name in the installer script, which a task's own name is for its checkbox.
pub const MESSAGES: &[(&str, &str)] = &[
	// TRANSLATORS: Checkbox in the Windows installer for a Paperback shortcut in the Start Menu, beside Inno Setup's own "Create a desktop shortcut". The & marks the key that checks it.
	("StartMenuIcon", t("Create a &Start Menu shortcut")),
	// TRANSLATORS: Checkbox in the Windows installer: EPUB books open in Paperback when opened from File Explorer. Every other format Paperback reads is offered in Explorer's "Open with" without asking.
	("default_epub_books", t("Make Paperback the default app for EPUB books (*.epub)")),
	// TRANSLATORS: Checkbox in the Windows installer: PDF files open in Paperback when opened from File Explorer, instead of in a browser or another PDF reader.
	("default_pdf_documents", t("Make Paperback the default app for PDF files (*.pdf)")),
	// TRANSLATORS: Checkbox in the Windows installer adding Paperback's folder to the PATH environment variable, so pb, Paperback's command-line converter, runs from any command prompt.
	("AddToPath", t("Add Paperback's folder to PATH, for the pb command")),
	// TRANSLATORS: The file type File Explorer shows for a document set to open in Paperback, as in its Type column.
	("DocumentType", t("Paperback Document")),
];
