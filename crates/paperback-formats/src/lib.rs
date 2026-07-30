//! Single source of truth for which document formats Paperback supports.
//!
//! Parsers in `paperback-core` read their `name()`/`extensions()` from the
//! constants here, and packaging scripts (Windows installer, macOS bundle)
//! read the same constants to generate file associations. Add a format here
//! once and every consumer picks it up automatically.

/// Metadata describing one user-facing document format.
pub struct FormatMeta {
	/// Display name, shown in the app's open-file dialog and in packaging UI (installer checkboxes).
	pub name: &'static str,
	/// Lowercase extensions (without the leading dot) this format is known by.
	pub extensions: &'static [&'static str],
	/// Whether the Windows installer should offer this association pre-checked.
	pub default_checked: bool,
	/// Whether the Windows installer should register Paperback as the extension's
	/// default handler outright, rather than just adding it to "Open with".
	/// Reserve this for formats unlikely to already have another default app on Windows.
	pub default_handler: bool,
}

pub const CHM: FormatMeta = FormatMeta {
	name: "Compiled HTML Help files",
	extensions: &["chm"],
	default_checked: false,
	default_handler: false,
};
pub const DAISY: FormatMeta =
	FormatMeta { name: "DAISY Books", extensions: &["opf", "zip"], default_checked: false, default_handler: false };
pub const WORD: FormatMeta = FormatMeta {
	name: "Word Documents",
	extensions: &["docx", "docm", "doc", "zip"],
	default_checked: false,
	default_handler: false,
};
pub const EPUB: FormatMeta =
	FormatMeta { name: "EPUB Books", extensions: &["epub"], default_checked: true, default_handler: true };
pub const FB2: FormatMeta =
	FormatMeta { name: "FictionBook Documents", extensions: &["fb2"], default_checked: false, default_handler: true };
pub const HTML: FormatMeta = FormatMeta {
	name: "HTML Files",
	extensions: &["htm", "html", "xhtml"],
	default_checked: false,
	default_handler: false,
};
pub const MARKDOWN: FormatMeta = FormatMeta {
	name: "Markdown Files",
	extensions: &["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"],
	default_checked: false,
	default_handler: false,
};
pub const MOBI: FormatMeta = FormatMeta {
	name: "MOBI Books",
	extensions: &["mobi", "azw", "azw3"],
	default_checked: false,
	default_handler: false,
};
pub const ODP: FormatMeta = FormatMeta {
	name: "OpenDocument Presentations",
	extensions: &["odp"],
	default_checked: false,
	default_handler: false,
};
pub const FODP: FormatMeta = FormatMeta {
	name: "Flat OpenDocument Presentations",
	extensions: &["fodp"],
	default_checked: false,
	default_handler: false,
};
pub const ODT: FormatMeta = FormatMeta {
	name: "OpenDocument Text Files",
	extensions: &["odt"],
	default_checked: false,
	default_handler: false,
};
pub const FODT: FormatMeta = FormatMeta {
	name: "Flat OpenDocument Text Files",
	extensions: &["fodt"],
	default_checked: false,
	default_handler: false,
};
pub const PDF: FormatMeta =
	FormatMeta { name: "PDF Documents", extensions: &["pdf"], default_checked: true, default_handler: false };
pub const POWERPOINT: FormatMeta = FormatMeta {
	name: "PowerPoint Presentations",
	extensions: &["pptx", "pptm", "ppt"],
	default_checked: false,
	default_handler: false,
};
pub const RTF: FormatMeta =
	FormatMeta { name: "RTF Documents", extensions: &["rtf"], default_checked: true, default_handler: false };
pub const TEXT: FormatMeta =
	FormatMeta { name: "Text Files", extensions: &["txt", "log"], default_checked: false, default_handler: false };

/// Every registered format, in the order parsers should be registered / packaging entries emitted.
pub const ALL: &[&FormatMeta] = &[
	&CHM,
	&WORD,
	&DAISY,
	&EPUB,
	&FB2,
	&HTML,
	&MARKDOWN,
	&MOBI,
	&ODP,
	&FODP,
	&ODT,
	&FODT,
	&PDF,
	&POWERPOINT,
	&RTF,
	&TEXT,
];
