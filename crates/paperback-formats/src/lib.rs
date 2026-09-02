//! Single source of truth for which document formats Paperback supports.
//!
//! Parsers in `paperback-core` are registered against the statics here, and packaging
//! scripts (Windows installer, macOS bundle) read the same statics to generate file
//! associations. Add a format to the `formats!` table below and every consumer picks
//! it up automatically.

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
		const SUPPORTS_AUDIO = 1 << 6;
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
	/// Offered unchecked, but becomes the default handler when the user opts in.
	pub const OPT_IN_HANDLER: Self = Self { default_checked: false, default_handler: true };
	/// Offered pre-checked, added to "Open with" only.
	pub const CHECKED: Self = Self { default_checked: true, default_handler: false };
	/// Offered pre-checked and becomes the default handler.
	pub const CHECKED_HANDLER: Self = Self { default_checked: true, default_handler: true };
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

/// Declares the format table: one static per entry, plus `ALL` listing them in declaration order.
///
/// `installer` defaults to [`InstallerMeta::OPT_IN`] and names an associated constant of
/// [`InstallerMeta`] when given; `flags` names [`ParserFlags`] constants joined with `|`.
macro_rules! formats {
	(@installer) => { InstallerMeta::OPT_IN };
	(@installer $installer:ident) => { InstallerMeta::$installer };
	($(
		$(#[$attr:meta])*
		$konst:ident {
			name: $name:literal,
			extensions: [$($ext:literal),+ $(,)?],
			flags: $($flag:ident)|+
			$(, installer: $installer:ident)?
			$(,)?
		}
	),+ $(,)?) => {
		$(
			$(#[$attr])*
			pub static $konst: FormatMeta = FormatMeta {
				name: $name,
				extensions: &[$($ext),+],
				flags: ParserFlags::NONE $(.union(ParserFlags::$flag))+,
				installer: formats!(@installer $($installer)?),
			};
		)+

		/// Every registered format, in the order parsers are registered and packaging
		/// entries are emitted.
		pub static ALL: &[&FormatMeta] = &[$(&$konst),+];
	};
}

formats! {
	CHM {
		name: "Compiled HTML Help files",
		extensions: ["chm"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_SECTIONS | SUPPORTS_IMAGES | SUPPORTS_FIGURES,
	},
	/// Declared ahead of [`WORD`] so that it gets first crack at the `.zip` both claim.
	DAISY {
		name: "DAISY Books",
		extensions: ["opf", "zip"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_PAGES,
	},
	WORD {
		name: "Word Documents",
		extensions: ["docx", "docm", "doc", "zip"],
		flags: SUPPORTS_TOC | SUPPORTS_SECTIONS,
	},
	EPUB {
		name: "EPUB Books",
		extensions: ["epub"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_PAGES | SUPPORTS_IMAGES | SUPPORTS_FIGURES,
		installer: CHECKED_HANDLER,
	},
	FB2 {
		name: "FictionBook Documents",
		extensions: ["fb2"],
		flags: SUPPORTS_TOC | SUPPORTS_SECTIONS,
		installer: OPT_IN_HANDLER,
	},
	HTML {
		name: "HTML Files",
		extensions: ["htm", "html", "xhtml"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	PDF {
		name: "PDF Documents",
		extensions: ["pdf"],
		flags: SUPPORTS_PAGES | SUPPORTS_TOC | SUPPORTS_LISTS,
		installer: CHECKED,
	},
	MARKDOWN {
		name: "Markdown Files",
		extensions: ["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	M4B {
		name: "M4B Audiobooks",
		extensions: ["m4b"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_AUDIO,
	},
	MOBI {
		name: "MOBI Books",
		extensions: ["mobi", "azw", "azw3"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	FODP {
		name: "Flat OpenDocument Presentations",
		extensions: ["fodp"],
		flags: NONE,
	},
	ODP {
		name: "OpenDocument Presentations",
		extensions: ["odp"],
		flags: NONE,
	},
	FODT {
		name: "Flat OpenDocument Text Files",
		extensions: ["fodt"],
		flags: SUPPORTS_TOC,
	},
	ODT {
		name: "OpenDocument Text Files",
		extensions: ["odt"],
		flags: SUPPORTS_TOC,
	},
	POWERPOINT {
		name: "PowerPoint Presentations",
		extensions: ["pptx", "pptm", "ppt"],
		flags: SUPPORTS_TOC,
	},
	RTF {
		name: "RTF Documents",
		extensions: ["rtf"],
		flags: SUPPORTS_PAGES,
		installer: CHECKED,
	},
	TEXT {
		name: "Text Files",
		extensions: ["txt", "log"],
		flags: NONE,
	},
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_lists_every_declared_format_once() {
		let mut names: Vec<&str> = ALL.iter().map(|format| format.name).collect();
		let count = names.len();
		names.sort_unstable();
		names.dedup();
		assert_eq!(names.len(), count, "format names must be unique");
	}

	#[test]
	fn extensions_are_lowercase_and_dotless() {
		for format in ALL {
			for ext in format.extensions {
				assert!(!ext.starts_with('.'), "{}: extension '{ext}' must not include the dot", format.name);
				assert_eq!(**ext, ext.to_ascii_lowercase(), "{}: extension '{ext}' must be lowercase", format.name);
			}
		}
	}

	#[test]
	fn declared_flags_and_installer_settings_survive_the_macro() {
		assert_eq!(RTF.flags, ParserFlags::SUPPORTS_PAGES);
		assert_eq!(TEXT.flags, ParserFlags::NONE);
		assert!(HTML.flags.contains(ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_LISTS));
		assert_eq!(M4B.flags, ParserFlags::SUPPORTS_SECTIONS | ParserFlags::SUPPORTS_TOC | ParserFlags::SUPPORTS_AUDIO);
		assert!(!HTML.flags.contains(ParserFlags::SUPPORTS_PAGES));
		assert!(!TEXT.installer.default_checked, "unspecified installer settings default to OPT_IN");
		assert!(EPUB.installer.default_checked && EPUB.installer.default_handler);
		assert!(!FB2.installer.default_checked && FB2.installer.default_handler);
	}
}
