use std::{env, error::Error};

mod android;
mod ios;
mod pot;
mod pot_lint;
mod release;
mod translate;
mod workspace;

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release::release()?,
		Some("android") => android::android()?,
		Some("ios") => ios::ios()?,
		Some("ios-release") => ios::ios_release()?,
		Some("gen-pot") => pot::gen_pot()?,
		Some("translate") => translate::translate()?,
		_ => print_help(),
	}
	Ok(())
}

pub(crate) fn print_help() {
	println!("Tasks:");
	println!("	release       Build release binaries and package them");
	println!("	gen-pot       Regenerate po/paperback.pot from all translatable crates");
	println!("	android       Generate Kotlin bindings and build native Android libraries");
	println!("	  --release          Build APK using gradlew assembleRelease");
	println!("	  --debug            Build APK using gradlew assembleDebug");
	println!("	  --install-release  Install release APK using gradlew installRelease");
	println!("	  --install-debug    Install debug APK using gradlew installDebug");
	println!("	  --build-aab        Build a release App Bundle (.aab) for Play Store upload");
	println!("	ios           Generate Swift bindings and build XCFramework for iOS");
	println!("	  --release          Build in release mode (default is debug)");
	println!("	ios-release   Archive and export a release IPA for App Store Connect");
	println!("	  --upload           Upload directly to App Store Connect via altool");
	println!("	translate     Regenerate the pot, sync po/*.po via msgmerge, and fill blank/fuzzy");
	println!("	              entries via the Claude API (needs ANTHROPIC_API_KEY)");
	println!("	  --dry-run          Report what would change; no API calls, no writes");
	println!("	  --repair           Also re-translate entries whose existing translation dropped");
	println!("	                     a placeholder, an accelerator or a shortcut suffix");
}
