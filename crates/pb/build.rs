//! Most conversions never touch OCR, downloads or the shell, and loading their libraries anyway was about a third of pb's start on Windows, a cost paid once per document in a batch.

use embed_manifest::{embed_manifest, empty_manifest, manifest::HeapType};

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
	if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
		// The desktop app's manifest already asks for the segment heap. Without it, a big PDF read on every core spent as long in the kernel waiting on the heap as it did converting, and ran almost three times slower.
		embed_manifest(empty_manifest().heap_type(HeapType::SegmentHeap)).expect("embedding pb's manifest");
	}
}
