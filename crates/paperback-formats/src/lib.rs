//! Packaging scripts read these statics too, to generate file associations.

use bitflags::bitflags;

bitflags! {
	/// The reader offers navigation by these, so a flag belongs here when the format can carry the feature, whether or not a given document uses it.
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

pub struct InstallerMeta {
	pub default_checked: bool,
	/// Reserve this for formats unlikely to already have another default app on Windows.
	pub default_handler: bool,
}

impl InstallerMeta {
	/// The right default for formats that other Windows apps already claim.
	pub const OPT_IN: Self = Self { default_checked: false, default_handler: false };
	pub const OPT_IN_HANDLER: Self = Self { default_checked: false, default_handler: true };
	pub const CHECKED: Self = Self { default_checked: true, default_handler: false };
	pub const CHECKED_HANDLER: Self = Self { default_checked: true, default_handler: true };
}

pub struct FormatMeta {
	pub name: &'static str,
	pub extensions: &'static [&'static str],
	/// Never `application/zip`: several formats share `.zip`, which gets its own standalone association, like the Windows installer's `assoc_zip` task.
	pub mime_types: &'static [&'static str],
	pub flags: ParserFlags,
	pub installer: InstallerMeta,
}

macro_rules! formats {
	(@installer) => { InstallerMeta::OPT_IN };
	(@installer $installer:ident) => { InstallerMeta::$installer };
	($(
		$(#[$attr:meta])*
		$konst:ident {
			name: $name:literal,
			extensions: [$($ext:literal),+ $(,)?],
			mime_types: [$($mime:literal),+ $(,)?],
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
				mime_types: &[$($mime),+],
				flags: ParserFlags::NONE $(.union(ParserFlags::$flag))+,
				installer: formats!(@installer $($installer)?),
			};
		)+

		/// Order matters: parsers are registered in it.
		pub static ALL: &[&FormatMeta] = &[$(&$konst),+];
	};
}

formats! {
	/// Not named like [`CBZ`], since the installer builds a task name from it; no brackets, since the open dialog adds the extensions in brackets.
	CBR {
		name: "RAR comic book archives",
		extensions: ["cbr"],
		mime_types: ["application/vnd.comicbook-rar"],
		flags: SUPPORTS_PAGES | SUPPORTS_IMAGES,
	},
	CBZ {
		name: "Comic book archives",
		extensions: ["cbz"],
		mime_types: ["application/vnd.comicbook+zip"],
		flags: SUPPORTS_PAGES | SUPPORTS_IMAGES | SUPPORTS_TOC,
	},
	CHM {
		name: "Compiled HTML Help files",
		extensions: ["chm"],
		mime_types: ["application/x-chm"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_SECTIONS | SUPPORTS_IMAGES | SUPPORTS_FIGURES,
	},
	HLP {
		name: "WinHelp files",
		extensions: ["hlp"],
		mime_types: ["application/winhlp"],
		flags: SUPPORTS_TOC | SUPPORTS_SECTIONS | SUPPORTS_IMAGES,
	},
	/// Declared ahead of [`WORD`] so it gets first crack at the `.zip` both claim.
	/// `application/x-daisy-opf` isn't a registered freedesktop.org type, so the Linux association dialog installs a shared-mime-info entry for it.
	DAISY {
		name: "DAISY Books",
		extensions: ["opf", "zip"],
		mime_types: ["application/x-daisy-opf"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_PAGES,
	},
	WORD {
		name: "Word Documents",
		extensions: ["docx", "docm", "doc", "zip"],
		mime_types: ["application/vnd.openxmlformats-officedocument.wordprocessingml.document", "application/msword"],
		flags: SUPPORTS_TOC | SUPPORTS_SECTIONS | SUPPORTS_LISTS | SUPPORTS_PAGES | SUPPORTS_IMAGES,
	},
	EPUB {
		name: "EPUB Books",
		extensions: ["epub"],
		mime_types: ["application/epub+zip"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_PAGES | SUPPORTS_IMAGES | SUPPORTS_FIGURES,
		installer: CHECKED_HANDLER,
	},
	FB2 {
		name: "FictionBook Documents",
		extensions: ["fb2", "zip"],
		mime_types: ["application/x-fictionbook+xml"],
		flags: SUPPORTS_TOC | SUPPORTS_SECTIONS,
		installer: OPT_IN_HANDLER,
	},
	HTML {
		name: "HTML Files",
		extensions: ["htm", "html", "xhtml"],
		mime_types: ["text/html", "application/xhtml+xml"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	PDF {
		name: "PDF Documents",
		extensions: ["pdf"],
		mime_types: ["application/pdf"],
		flags: SUPPORTS_PAGES | SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_IMAGES | SUPPORTS_FIGURES,
		installer: CHECKED,
	},
	/// `gz` is claimed for installed pages like `printf.3.gz`; the parser checks what is inside, so other gzipped files are turned away.
	MAN {
		name: "Manual pages",
		extensions: ["man", "roff", "gz", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
		mime_types: ["application/x-troff-man", "text/troff"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	MARKDOWN {
		name: "Markdown Files",
		extensions: ["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"],
		mime_types: ["text/markdown"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	M4B {
		name: "M4B Audiobooks",
		extensions: ["m4b"],
		mime_types: ["audio/x-m4b"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_AUDIO,
	},
	MP3 {
		name: "MP3 Audiobooks",
		extensions: ["mp3"],
		mime_types: ["audio/mpeg"],
		flags: SUPPORTS_SECTIONS | SUPPORTS_TOC | SUPPORTS_AUDIO,
	},
	MOBI {
		name: "MOBI Books",
		extensions: ["mobi", "azw", "azw3"],
		mime_types: ["application/x-mobipocket-ebook", "application/vnd.amazon.ebook"],
		// Only KF8 books carry the EPUB file boundaries that sections come from; older Mobipocket ones produce none.
		flags: SUPPORTS_TOC | SUPPORTS_LISTS | SUPPORTS_SECTIONS,
	},
	FODP {
		name: "Flat OpenDocument Presentations",
		extensions: ["fodp"],
		mime_types: ["application/vnd.oasis.opendocument.presentation-flat-xml"],
		flags: SUPPORTS_IMAGES,
	},
	ODP {
		name: "OpenDocument Presentations",
		extensions: ["odp"],
		mime_types: ["application/vnd.oasis.opendocument.presentation"],
		flags: SUPPORTS_IMAGES,
	},
	FODT {
		name: "Flat OpenDocument Text Files",
		extensions: ["fodt"],
		mime_types: ["application/vnd.oasis.opendocument.text-flat-xml"],
		flags: SUPPORTS_TOC | SUPPORTS_IMAGES,
	},
	ODT {
		name: "OpenDocument Text Files",
		extensions: ["odt"],
		mime_types: ["application/vnd.oasis.opendocument.text"],
		flags: SUPPORTS_TOC | SUPPORTS_IMAGES,
	},
	POWERPOINT {
		name: "PowerPoint Presentations",
		extensions: ["pptx", "pptm", "ppt"],
		mime_types: ["application/vnd.openxmlformats-officedocument.presentationml.presentation", "application/vnd.ms-powerpoint"],
		flags: SUPPORTS_TOC,
	},
	RST {
		name: "reStructuredText Files",
		extensions: ["rst", "rest"],
		mime_types: ["text/x-rst"],
		flags: SUPPORTS_TOC | SUPPORTS_LISTS,
	},
	RTF {
		name: "RTF Documents",
		extensions: ["rtf"],
		mime_types: ["application/rtf", "text/rtf"],
		flags: SUPPORTS_PAGES,
		installer: CHECKED,
	},
	TEXT {
		name: "Text Files",
		extensions: ["txt", "log"],
		mime_types: ["text/plain"],
		flags: NONE,
	},
	/// Real `.wri` files are mostly RTF or plain text that the parser routes elsewhere, so the flags stay `NONE` rather than over-promise.
	WRI {
		name: "Windows Write Documents",
		extensions: ["wri"],
		mime_types: ["application/x-mswrite"],
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
	fn mime_types_are_present_and_never_the_shared_zip_type() {
		for format in ALL {
			assert!(!format.mime_types.is_empty(), "{}: must declare at least one MIME type", format.name);
			for mime in format.mime_types {
				assert!(mime.contains('/'), "{}: '{mime}' doesn't look like a MIME type", format.name);
				assert_ne!(
					*mime, "application/zip",
					"{}: application/zip belongs to the standalone zip task",
					format.name
				);
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

	/// The Word parser marks lists, their items and pages, and the mobile apps only offer those to step through for a format that claims them.
	#[test]
	fn word_documents_offer_list_and_page_navigation() {
		assert!(WORD.flags.contains(ParserFlags::SUPPORTS_LISTS | ParserFlags::SUPPORTS_PAGES));
	}
}
