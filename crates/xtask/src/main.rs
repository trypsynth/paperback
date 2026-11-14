use std::{env, process::ExitCode};

mod android;
mod docs;
mod ios;
mod pot;
mod release;
mod translate;
mod workspace;

fn main() -> ExitCode {
	let task = env::args().nth(1);
	let result = match task.as_deref() {
		Some("release") => release::release(),
		Some("android") => android::android(),
		Some("ios") => ios::ios(),
		Some("ios-release") => ios::ios_release(),
		Some("gen-pot") => pot::gen_pot(),
		Some("translate") => translate::translate(),
		Some("site-docs") => docs::site_docs(),
		_ => {
			print_help();
			Ok(())
		}
	};
	// Printed here instead of returned from main, which would print it with Debug: in quotes, with every line break
	// shown as \n.
	if let Err(error) = result {
		eprintln!("Error: {error}");
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub(crate) fn print_help() {
	println!("Tasks:");
	println!("	release       Build release binaries and package them");
	println!("	gen-pot       Regenerate po/paperback.pot from all translatable crates");
	println!("	site-docs     Build the manual for paperback.dev into web/_site, or the directory given");
	println!("	android       Generate Kotlin bindings and build native Android libraries");
	println!("	  --release          Build APK using gradlew assembleRelease");
	println!("	  --debug            Build APK using gradlew assembleDebug");
	println!("	  --install-release  Install release APK using gradlew installRelease");
	println!("	  --install-debug    Install debug APK using gradlew installDebug");
	println!("	  --build-aab        Build a release App Bundle (.aab) for Play Store upload");
	println!("	ios           Generate Swift bindings and build XCFramework for iOS");
	println!("	  --release          Build in release mode (default is debug)");
	println!("	ios run       Build the iOS app and start it on a simulator or device");
	println!("	  --simulator [NAME] Simulator name or UDID (default: the booted one, else the newest iPhone)");
	println!("	  --device [NAME]    Run on a paired iPhone or iPad; sets up signing on first use");
	println!("	  --team ID          With --device, sign with this team instead of the one signed in to Xcode");
	println!("	  --console          Stay attached and print the app's output until it exits");
	println!("	  --release          Build in release mode");
	println!("	ios-release   Archive and export a release IPA for App Store Connect");
	println!("	  --upload           Upload directly to App Store Connect via altool");
	println!("	translate     Regenerate the pot, sync po/*.po via msgmerge, and fill blank/fuzzy");
	println!("	              entries via the Claude API (needs ANTHROPIC_API_KEY)");
	println!("	  --dry-run          Report what would change; no API calls, no writes");
	println!("	  --repair           Also re-translate entries whose existing translation dropped");
	println!("	                     a placeholder, an accelerator or a shortcut suffix");
	println!("	  --repair-copies    One-off: re-translate machine translations shared with a different string");
}
