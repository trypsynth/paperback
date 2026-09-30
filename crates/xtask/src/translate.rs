use std::{env, error::Error, fs, path::Path};

use patois_translate::{App, Options, Project};

use crate::workspace::project_root;

/// Regenerates `po/paperback.pot`, then has patois-translate fill the blank and fuzzy entries of
/// every `po/<lang>.po` and translate `doc/readme.md` into `doc/readme-<lang>.md`.
///
/// Locales listed in `po/human-maintained-locales.txt` are skipped. See that file and
/// <https://github.com/trypsynth/paperback/issues/638>.
pub fn translate() -> Result<(), Box<dyn Error>> {
	let mut options = Options::default();
	for arg in env::args().skip(2) {
		match arg.as_str() {
			"--dry-run" => options.dry_run = true,
			"--repair" => options.repair = true,
			"--repair-copies" => options.repair_copies = true,
			_ => {
				crate::print_help();
				return Err(format!("Unknown argument for translate: {arg}").into());
			}
		}
	}
	let root = project_root();
	let pot_path = root.join("po/paperback.pot");
	// gen_pot() always writes a fresh POT-Creation-Date, which alone would leave every run with a
	// modified pot, and a dry run must touch nothing on disk.
	let original_pot = fs::read_to_string(&pot_path).ok();
	crate::pot::gen_pot()?;
	if let Some(original) = &original_pot {
		let regenerated = fs::read_to_string(&pot_path)?;
		if options.dry_run || without_creation_date(original) == without_creation_date(&regenerated) {
			fs::write(&pot_path, original)?;
		}
	}
	let project = Project {
		root: &root,
		po_dir: Path::new("po"),
		pot: Path::new("po/paperback.pot"),
		readme: Some(Path::new("doc/readme.md")),
		app: App {
			name: "Paperback".to_string(),
			description: "a desktop ebook and document reader used heavily with screen readers".to_string(),
			proper_nouns: ["EPUB", "PDF", "DAISY", "HTML", "Markdown"].map(str::to_string).to_vec(),
		},
	};
	patois_translate::translate(&project, options)
}

fn without_creation_date(pot: &str) -> String {
	pot.lines().filter(|line| !line.trim().starts_with("\"POT-Creation-Date:")).collect::<Vec<_>>().join("\n")
}
