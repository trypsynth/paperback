//! The user guide as HTML: the pages the iOS and Android apps show for Help, and the manual on paperback.dev in every language.

use std::{error::Error, fmt::Write as _, fs, path::Path};

use patois::language_name;
use shipfitter::docs::{Page, Readme, bcp47, convert, readmes};

use crate::workspace::project_root;

const TITLE: &str = "Paperback Documentation";
const SITE: &str = "https://paperback.dev";

/// Every `doc/readme*.md` as `readme.html` and `readme-<code>.html` in `out_dir`, for an app's Help.
pub fn write_app_readmes(out_dir: &Path) -> Result<(), Box<dyn Error>> {
	fs::create_dir_all(out_dir)?;
	for readme in readmes(&project_root().join("doc"))? {
		let lang = bcp47(&readme.code);
		convert(
			&readme.path,
			&out_dir.join(readme.html_name()),
			&Page { lang: &lang, title: TITLE, ..Page::default() },
		)?;
	}
	Ok(())
}

/// The manual for paperback.dev, written into the built site (`web/_site` unless a directory is given) after cobalt has built the rest of it, with the site's stylesheet, a way back to the home page, and a bar linking every language.
pub fn site_docs() -> Result<(), Box<dyn Error>> {
	let out_dir = std::env::args().nth(2).map_or_else(|| project_root().join("web/_site"), Into::into);
	fs::create_dir_all(&out_dir)?;
	let readmes = readmes(&project_root().join("doc"))?;
	let head = site_head(&readmes);
	for readme in &readmes {
		let lang = bcp47(&readme.code);
		let before_body =
			format!("<p><a href=\"/\">&larr; paperback.dev</a></p>\n{}", language_bar(&readmes, &readme.code));
		let page = Page { lang: &lang, title: TITLE, head: &head, before_body: &before_body, ..Page::default() };
		convert(&readme.path, &out_dir.join(readme.html_name()), &page)?;
		println!("built {} from {}", readme.html_name(), readme.path.file_name().unwrap_or_default().to_string_lossy());
	}
	Ok(())
}

/// The `hreflang` alternates, the same on every page, and the site's stylesheet. `x-default` is English, which is also where a language with no manual should land.
fn site_head(readmes: &[Readme]) -> String {
	let mut head = String::new();
	for readme in readmes {
		let _ = writeln!(
			head,
			"<link rel=\"alternate\" hreflang=\"{}\" href=\"{SITE}/{}\">",
			bcp47(&readme.code),
			readme.html_name()
		);
	}
	let _ = writeln!(head, "<link rel=\"alternate\" hreflang=\"x-default\" href=\"{SITE}/readme.html\">");
	head.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"color-scheme\" content=\"light dark\">\n<link rel=\"stylesheet\" href=\"/css/site.css\">\n");
	head
}

/// Every language's name in that language, each carrying its own `lang` so a screen reader says "日本語" in a Japanese voice rather than spelling it out in the page's.
fn language_bar(readmes: &[Readme], current: &str) -> String {
	let mut bar = String::from("<nav class=\"language-bar\" aria-label=\"Language\">\n<ul>\n");
	for readme in readmes {
		let tag = bcp47(&readme.code);
		let name = language_name(&readme.code);
		if readme.code == current {
			let _ = writeln!(bar, "<li><span lang=\"{tag}\" aria-current=\"true\">{name}</span></li>");
		} else {
			let _ = writeln!(
				bar,
				"<li><a href=\"/{}\" lang=\"{tag}\" hreflang=\"{tag}\">{name}</a></li>",
				readme.html_name()
			);
		}
	}
	bar.push_str("</ul>\n</nav>\n");
	bar
}
