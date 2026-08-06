//! Single source of truth for which document formats Paperback supports.
//!
//! Parsers in `paperback-core` are registered against the constants here, and
//! packaging scripts (Windows installer, macOS bundle) read the same constants
//! to generate file associations. Add a format here once and every consumer
//! picks it up automatically.

use bitflags::bitflags;

bitflags! {
	/// Navigation and structure features a format's parser can produce.
	///
	/// The reader uses these to decide which navigation commands to offer, so a flag
	/// belongs here when the format itself can carry the feature, whether or not a
	/// particular document uses it.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct ParserFlags: u32 {
		const NONE = 0;
		const SUPPORTS_SECTIONS = 1 << 0;
		const SUPPORTS_TOC = 1 << 1;
		const SUPPORTS_PAGES = 1 << 2;
		const SUPPORTS_LISTS = 1 << 3;
		const SUPPORTS_IMAGES = 1 << 4;
		const SUPPORTS_FIGURES = 1 << 5;
	}
}

/// How the Windows installer should treat a format's file associations.
pub struct InstallerMeta {
	/// Whether the installer should offer this association pre-checked.
	pub default_checked: bool,
	/// Whether the installer should register Paperback as the extension's default
	/// handler outright, rather than just adding it to "Open with". Reserve this for
	/// formats unlikely to already have another default app on Windows.
	pub default_handler: bool,
}

impl InstallerMeta {
	/// Offered unchecked, and only ever added to "Open with" — the right default for
	/// formats that other Windows apps already claim.
	pub const OPT_IN: Self = Self { default_checked: false, default_handler: false };
}

/// Metadata describing one user-facing document format.
pub struct FormatMeta {
	/// Display name, shown in the app's open-file dialog and in packaging UI (installer checkboxes).
	pub name: &'static str,
	/// Lowercase extensions (without the leading dot) this format is known by.
	pub extensions: &'static [&'static str],
	/// Navigation features the format's parser can produce.
	pub flags: ParserFlags,
	/// Packaging-only settings; ignored outside the Windows installer.
	pub installer: InstallerMeta,
}

pub const CHM: FormatMeta = FormatMeta {
	name: "Compiled HTML Help files",
	extensions: &["chm"],
	flags: ParserFlags::SUPPORTS_TOC
		.union(ParserFlags::SUPPORTS_LISTS)
		.union(ParserFlags::SUPPORTS_SECTIONS)
		.union(ParserFlags::SUPPORTS_IMAGES)
		.union(ParserFlags::SUPPORTS_FIGURES),
	installer: InstallerMeta::OPT_IN,
};
pub const DAISY: FormatMeta = FormatMeta {
	name: "DAISY Books",
	extensions: &["opf", "zip"],
	flags: ParserFlags::SUPPORTS_SECTIONS
		.union(ParserFlags::SUPPORTS_TOC)
		.union(ParserFlags::SUPPORTS_LISTS)
		.union(ParserFlags::SUPPORTS_PAGES),
	installer: InstallerMeta::OPT_IN,
};
pub const WORD: FormatMeta = FormatMeta {
	name: "Word Documents",
	extensions: &["docx", "docm", "doc", "zip"],
	flags: ParserFlags::SUPPORTS_TOC.union(ParserFlags::SUPPORTS_SECTIONS),
	installer: InstallerMeta::OPT_IN,
};
pub const EPUB: FormatMeta = FormatMeta {
	name: "EPUB Books",
	extensions: &["epub"],
	flags: ParserFlags::SUPPORTS_SECTIONS
		.union(ParserFlags::SUPPORTS_TOC)
		.union(ParserFlags::SUPPORTS_LISTS)
		.union(ParserFlags::SUPPORTS_PAGES)
		.union(ParserFlags::SUPPORTS_IMAGES)
		.union(ParserFlags::SUPPORTS_FIGURES),
	installer: InstallerMeta { default_checked: true, default_handler: true },
};
pub const FB2: FormatMeta = FormatMeta {
	name: "FictionBook Documents",
	extensions: &["fb2"],
	flags: ParserFlags::SUPPORTS_TOC.union(ParserFlags::SUPPORTS_SECTIONS),
	installer: InstallerMeta { default_checked: false, default_handler: true },
};
pub const HTML: FormatMeta = FormatMeta {
	name: "HTML Files",
	extensions: &["htm", "html", "xhtml"],
	flags: ParserFlags::SUPPORTS_TOC.union(ParserFlags::SUPPORTS_LISTS),
	installer: InstallerMeta::OPT_IN,
};
pub const MARKDOWN: FormatMeta = FormatMeta {
	name: "Markdown Files",
	extensions: &["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"],
	flags: ParserFlags::SUPPORTS_TOC.union(ParserFlags::SUPPORTS_LISTS),
	installer: InstallerMeta::OPT_IN,
};
pub const MOBI: FormatMeta = FormatMeta {
	name: "MOBI Books",
	extensions: &["mobi", "azw", "azw3"],
	flags: ParserFlags::SUPPORTS_TOC.union(ParserFlags::SUPPORTS_LISTS),
	installer: InstallerMeta::OPT_IN,
};
pub const ODP: FormatMeta = FormatMeta {
	name: "OpenDocument Presentations",
	extensions: &["odp"],
	flags: ParserFlags::NONE,
	installer: InstallerMeta::OPT_IN,
};
pub const FODP: FormatMeta = FormatMeta {
	name: "Flat OpenDocument Presentations",
	extensions: &["fodp"],
	flags: ParserFlags::NONE,
	installer: InstallerMeta::OPT_IN,
};
pub const ODT: FormatMeta = FormatMeta {
	name: "OpenDocument Text Files",
	extensions: &["odt"],
	flags: ParserFlags::SUPPORTS_TOC,
	installer: InstallerMeta::OPT_IN,
};
pub const FODT: FormatMeta = FormatMeta {
	name: "Flat OpenDocument Text Files",
	extensions: &["fodt"],
	flags: ParserFlags::SUPPORTS_TOC,
	installer: InstallerMeta::OPT_IN,
};
pub const PDF: FormatMeta = FormatMeta {
	name: "PDF Documents",
	extensions: &["pdf"],
	flags: ParserFlags::SUPPORTS_PAGES.union(ParserFlags::SUPPORTS_TOC).union(ParserFlags::SUPPORTS_LISTS),
	installer: InstallerMeta { default_checked: true, default_handler: false },
};
pub const POWERPOINT: FormatMeta = FormatMeta {
	name: "PowerPoint Presentations",
	extensions: &["pptx", "pptm", "ppt"],
	flags: ParserFlags::SUPPORTS_TOC,
	installer: InstallerMeta::OPT_IN,
};
pub const RTF: FormatMeta = FormatMeta {
	name: "RTF Documents",
	extensions: &["rtf"],
	flags: ParserFlags::SUPPORTS_PAGES,
	installer: InstallerMeta { default_checked: true, default_handler: false },
};
pub const TEXT: FormatMeta = FormatMeta {
	name: "Text Files",
	extensions: &["txt", "log"],
	flags: ParserFlags::NONE,
	installer: InstallerMeta::OPT_IN,
};

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
