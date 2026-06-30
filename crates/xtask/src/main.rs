use std::{
	env,
	error::Error,
	path::{Path, PathBuf},
};

mod android;
mod ios;
mod release;

fn main() -> Result<(), Box<dyn Error>> {
	let task = env::args().nth(1);
	match task.as_deref() {
		Some("release") => release::release()?,
		Some("android") => android::android()?,
		Some("ios") => ios::ios()?,
		Some("ios-release") => ios::ios_release()?,
		Some("gen-pot") => gen_pot()?,
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
	println!("	ios           Generate Swift bindings and build XCFramework for iOS");
	println!("	  --release          Build in release mode (default is debug)");
	println!("	ios-release   Archive and export a release IPA for App Store Connect");
	println!("	  --upload           Upload directly to App Store Connect via altool");
}

pub(crate) fn project_root() -> PathBuf {
	Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(2).unwrap().to_path_buf()
}

fn gen_pot() -> Result<(), Box<dyn Error>> {
	let root = project_root();
	let po_dir = root.join("po");
	let pot_file = po_dir.join("paperback.pot");

	// Step 1: generate from Rust crates tagged with translatable = true
	patois_build::gen_pot(&root, &po_dir, "paperback")?;

	// Step 2: extend with iOS Swift sources (t() calls in Swift files)
	let ios_src = root.join("ios/Paperback");
	if ios_src.is_dir() {
		patois_build::extend_pot_from_source_dirs(&[&ios_src], "swift", &pot_file)?;
	}

	// Step 3: extend with Android Kotlin sources (excluding uniffi-generated bindings)
	let kt_src = root.join("android/app/src/main/kotlin/dev/paperback/mobile");
	if kt_src.is_dir() {
		patois_build::extend_pot_from_source_dirs(&[&kt_src], "kt", &pot_file)?;
	}

	Ok(())
}
