use std::{error::Error, fmt::Write as _, fs, path::Path};

use patois::language_name;
use patois_build::po::PoDocument;
use shipfitter::docs::{Page, Readme, bcp47, convert, markdown_to_html, readmes};

use crate::workspace::{crate_version, project_root};

const TITLE: &str = "Paperback Documentation";
const SITE: &str = "https://paperback.dev";

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

/// Runs after cobalt has built the rest of the site, writing into it.
pub fn site_docs() -> Result<(), Box<dyn Error>> {
	let out_dir = std::env::args().nth(2).map_or_else(|| project_root().join("web/_site"), Into::into);
	fs::create_dir_all(&out_dir)?;
	let readmes = readmes(&project_root().join("doc"))?;
	let released =
		version_parts(&crate_version(&project_root(), "paperback")?).ok_or("the workspace version isn't a version")?;
	let head = site_head(&readmes);
	for readme in &readmes {
		let lang = bcp47(&readme.code);
		let before_body = "<p><a href=\"/\">&larr; paperback.dev</a></p>\n";
		let page = Page { lang: &lang, title: TITLE, head: &head, before_body, ..Page::default() };
		let markdown = without_unreleased(&fs::read_to_string(&readme.path)?, &released);
		let html = markdown_to_html(&markdown, &page);
		// The page template has no place after the document, so the picker goes in before the body closes.
		let end = html.rfind("</body>").ok_or("the guide page has no </body>")?;
		let html = format!("{}{}{}", &html[..end], language_picker(&readmes, &readme.code), &html[end..]);
		fs::write(out_dir.join(readme.html_name()), html)?;
		println!("built {} from {}", readme.html_name(), readme.path.file_name().unwrap_or_default().to_string_lossy());
	}
	Ok(())
}

/// Leaves out the changelog of any version newer than the workspace's, so master's draft stays off the site until the release bumps the version.
fn without_unreleased(markdown: &str, released: &[u32]) -> String {
	let mut kept = String::with_capacity(markdown.len());
	let mut skipping = false;
	for line in markdown.split_inclusive('\n') {
		let level = line.bytes().take_while(|&byte| byte == b'#').count();
		if (1..=3).contains(&level) && line.as_bytes().get(level) == Some(&b' ') {
			skipping = level == 3 && heading_version(line).is_some_and(|version| version.as_slice() > released);
		}
		if !skipping {
			kept.push_str(line);
		}
	}
	kept
}

/// Translations put the number anywhere in the heading: "Version 1.2", "1.2-es verzió", "バージョン1.2".
fn heading_version(heading: &str) -> Option<Vec<u32>> {
	heading.split(|c: char| !(c.is_ascii_digit() || c == '.')).find_map(version_parts)
}

fn version_parts(text: &str) -> Option<Vec<u32>> {
	let parts: Vec<u32> = text.trim_matches('.').split('.').map(str::parse).collect::<Result<_, _>>().ok()?;
	(parts.len() >= 2).then_some(parts)
}

/// `x-default` is English, which is also where a language with no manual should land.
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

/// Closed at the foot of the page, as scribeit.io has it, so the list doesn't stand between the reader and the guide. Each name carries its own `lang` so a screen reader says "日本語" in a Japanese voice rather than spelling it out in the page's.
fn language_picker(readmes: &[Readme], current: &str) -> String {
	let label = word_for_language(current);
	let mut bar = format!(
		"<footer class=\"language-picker\">\n<details>\n<summary>{label}</summary>\n<nav aria-label=\"{label}\">\n<ul>\n"
	);
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
	bar.push_str("</ul>\n</nav>\n</details>\n</footer>\n");
	bar
}

/// "Language" in the page's own language, from the app's translation of it, so the picker is found by someone who can read the page.
fn word_for_language(code: &str) -> String {
	fs::read_to_string(project_root().join("po").join(format!("{code}.po")))
		.ok()
		.and_then(|po| {
			PoDocument::parse(&po).entries.into_iter().find(|entry| entry.msgid == "Language").map(|entry| entry.msgstr)
		})
		.filter(|word| !word.is_empty())
		.unwrap_or_else(|| "Language".to_string())
}

#[cfg(test)]
mod tests {
	use super::without_unreleased;

	const README: &str = "# Paperback\n\n## Changelog\n\n### Version 1.2\n\n#### Added\n* Read aloud.\n\n### Version 1.1\n\n* OCR on Linux.\n";

	#[test]
	fn the_changelog_of_an_unreleased_version_is_left_out() {
		assert_eq!(
			without_unreleased(README, &[1, 1, 0]),
			"# Paperback\n\n## Changelog\n\n### Version 1.1\n\n* OCR on Linux.\n"
		);
	}

	#[test]
	fn a_released_version_shows_however_a_translation_writes_it() {
		assert_eq!(without_unreleased(README, &[1, 2, 0]), README);
		let hungarian = "## Változásnapló\n\n### 1.2-es verzió\n\n* Új.\n\n### 1.1-es verzió\n\n* Régi.\n";
		assert_eq!(without_unreleased(hungarian, &[1, 1, 0]), "## Változásnapló\n\n### 1.1-es verzió\n\n* Régi.\n");
		assert_eq!(without_unreleased("### バージョン1.2\n\n* 新規\n", &[1, 1, 0]), "");
	}
}
