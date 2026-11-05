//! Most conversions never touch OCR, downloads or the shell, and loading their libraries anyway was about a third of pb's start on Windows, a cost paid once per document in a batch.

/// Only libraries Windows lists as known DLLs, which it always loads from the system folder: a delayed load of any other could pick up a planted copy from the folder pb is run in.
const DELAYED: &[&str] = &["ole32.dll", "combase.dll", "oleaut32.dll", "shell32.dll", "advapi32.dll", "ws2_32.dll"];

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
		for dll in DELAYED {
			println!("cargo:rustc-link-arg-bins=/DELAYLOAD:{dll}");
		}
		println!("cargo:rustc-link-arg-bins=delayimp.lib");
	}
}
