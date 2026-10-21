//! `PDFium`'s prebuilt libraries from `bblanchon/pdfium-binaries`, each pinned to a release and the SHA-256 of its archive, so a build gets the library it was tested with and a tampered or truncated download fails instead of shipping.

use std::{
	fmt::Write as _,
	fs,
	io::{self, Cursor, Read},
	path::Path,
};

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};

/// One release archive. Renovate moves every one below to the newest release together and rehashes it, matching on `tag`, `name` and `sha256` in that order, so keep them that way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Archive {
	pub tag: &'static str,
	pub name: &'static str,
	pub sha256: &'static str,
}

pub const WIN_X64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-win-x64.tgz",
	sha256: "3452d11031050ec65e57f180215b57b9e6d465e3fbedafa8a77ad90885c7d23e",
};
pub const WIN_X86: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-win-x86.tgz",
	sha256: "ef298314d6e71b08e38dc90724111e37e1d29d668f739ad656e2329d491e533a",
};
pub const WIN_ARM64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-win-arm64.tgz",
	sha256: "99ec8b76d5ac30602a8488da745397393ff6cddc824ef36f398146f6b4f76d4e",
};
pub const MAC_X64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-mac-x64.tgz",
	sha256: "e7fece4693f65a7e47f84fa38099068d573d4dcbd46b1a3c8d2c26581c308466",
};
pub const MAC_ARM64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-mac-arm64.tgz",
	sha256: "46c674298c60123f1195c2a03c65ee08c4783c84874ce4f5b3a054abb3ecaf30",
};
pub const LINUX_X64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-linux-x64.tgz",
	sha256: "bdc60126399f39fe768566a18d4be055a7b3681bc9e248e8a6656b4f6cae0aa2",
};
pub const LINUX_ARM64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-linux-arm64.tgz",
	sha256: "042de247138f96341534cccf24ed21c2550a7245b936bb4a60d2340120000f05",
};
pub const ANDROID_ARM64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-android-arm64.tgz",
	sha256: "727cb2645ece75d28d0b72f2ec05b82858ca3bfb6712cd333ffc248807016e2c",
};
pub const ANDROID_ARM: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-android-arm.tgz",
	sha256: "fc6a54e286a46a52e45d5bd52803f59f3b86dfcff73ba8f458939872f5466de3",
};
pub const ANDROID_X64: Archive = Archive {
	tag: "chromium/8111",
	name: "pdfium-android-x64.tgz",
	sha256: "e2c31cffe42ee3a34e2b9d6c6c254ddf5da1541ce2d82216171e9354c86f0a7c",
};

// Held back by hand, and kept out of Renovate's pattern by its name: later releases crash on real iOS devices (not the Simulator) with `[FATAL:partition_address_space.cc(81)] Check failed: false.` inside libpdfium.dylib the moment PDFium is initialized, even though they set `pdf_use_partition_alloc = false`. chromium/7920 is confirmed on a device not to. Re-verify on a real device before moving it.
pub const IOS_ARM64: Archive = Archive {
	tag: "chromium/7920",
	name: "pdfium-ios-device-arm64.tgz",
	sha256: "03b0f149d38ff879574a1f58aeecda158c797ef8a3c7cf31c5eef393c46f6b2f",
};

/// The desktop archive for a target, by Cargo's `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH` values.
#[must_use]
pub fn desktop(os: &str, arch: &str) -> Option<Archive> {
	match (os, arch) {
		("windows", "x86_64") => Some(WIN_X64),
		("windows", "x86") => Some(WIN_X86),
		("windows", "aarch64") => Some(WIN_ARM64),
		("macos", "x86_64") => Some(MAC_X64),
		("macos", "aarch64") => Some(MAC_ARM64),
		("linux", "x86_64") => Some(LINUX_X64),
		("linux", "aarch64") => Some(LINUX_ARM64),
		_ => None,
	}
}

impl Archive {
	#[must_use]
	pub fn url(&self) -> String {
		format!(
			"https://github.com/bblanchon/pdfium-binaries/releases/download/{}/{}",
			self.tag.replace('/', "%2F"),
			self.name
		)
	}

	/// Downloads the archive, checks it against its pinned SHA-256, and unpacks the entry named `file_name` to `dest`. The library lands under a temporary name first, so a failed run never leaves half a library where a build would load it.
	pub fn fetch(&self, file_name: &str, dest: &Path) -> io::Result<()> {
		let response = ureq::get(&self.url())
			.call()
			.map_err(|err| io::Error::other(format!("downloading {}: {err}", self.url())))?;
		let mut bytes = Vec::new();
		response.into_body().as_reader().read_to_end(&mut bytes)?;
		self.verify(&bytes)?;
		extract(&bytes, file_name, dest)
	}

	fn verify(&self, bytes: &[u8]) -> io::Result<()> {
		let actual = Sha256::digest(bytes).iter().fold(String::new(), |mut hex, byte| {
			let _ = write!(hex, "{byte:02x}");
			hex
		});
		if actual == self.sha256 {
			Ok(())
		} else {
			Err(io::Error::other(format!(
				"{} from {} has SHA-256 {actual}, expected {}",
				self.name, self.tag, self.sha256
			)))
		}
	}
}

fn extract(archive_bytes: &[u8], file_name: &str, dest: &Path) -> io::Result<()> {
	if let Some(parent) = dest.parent() {
		fs::create_dir_all(parent)?;
	}
	let mut archive = tar::Archive::new(GzDecoder::new(Cursor::new(archive_bytes)));
	for entry in archive.entries()? {
		let mut entry = entry?;
		if entry.path()?.file_name().and_then(|name| name.to_str()) == Some(file_name) {
			let mut temp_name = dest.file_name().unwrap_or_default().to_os_string();
			temp_name.push(".tmp");
			let temp = dest.with_file_name(temp_name);
			entry.unpack(&temp)?;
			if dest.exists() {
				fs::remove_file(dest)?;
			}
			return fs::rename(&temp, dest);
		}
	}
	Err(io::Error::other(format!("{file_name} not found in the archive")))
}

#[cfg(test)]
mod tests {
	use super::Archive;

	#[test]
	fn a_tag_with_a_slash_is_escaped_in_the_url() {
		let archive = Archive { tag: "chromium/1", name: "x.tgz", sha256: "" };
		assert_eq!(archive.url(), "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F1/x.tgz");
	}

	#[test]
	fn an_archive_that_does_not_match_its_pin_is_refused() {
		let archive = Archive {
			tag: "chromium/1",
			name: "x.tgz",
			sha256: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
		};
		assert!(archive.verify(b"hello").is_ok());
		assert!(archive.verify(b"hellp").is_err());
	}
}
