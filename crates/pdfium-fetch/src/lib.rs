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
	tag: "chromium/8100",
	name: "pdfium-win-x64.tgz",
	sha256: "51621bcc1d9be9ea448f4b84b0516862b928e3968d3fca899af1a6d6e2848d6d",
};
pub const WIN_X86: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-win-x86.tgz",
	sha256: "bdab604368d0116adf84eaab452521a5f200dfa3f97001193d63edb35e75378e",
};
pub const WIN_ARM64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-win-arm64.tgz",
	sha256: "cca9b28d406ae2a902b0b710a92ab699c4c2931564ab6efe44630014d492a9ca",
};
pub const MAC_X64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-mac-x64.tgz",
	sha256: "d513a0530a0f0c1ca9929ded06660e27edb9a108a7d18d9df55a5fb060b091c8",
};
pub const MAC_ARM64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-mac-arm64.tgz",
	sha256: "7ea647c7a7a9c1169be3f947000bc071ece2fade2cd6bd7e401e3675c6560cce",
};
pub const LINUX_X64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-linux-x64.tgz",
	sha256: "1c1894a12d4d96ee01400c3874444126254180c5c7d2ab628576b67f8120320a",
};
pub const LINUX_ARM64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-linux-arm64.tgz",
	sha256: "58c87a95a40a24c914a9c755fae798843f46f06adf9e5e9f2c893302d4bdb040",
};
pub const ANDROID_ARM64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-android-arm64.tgz",
	sha256: "e9bab2929443d3d881166af9ec1d191c76ecdd37effb07a58743a7d7b507e149",
};
pub const ANDROID_ARM: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-android-arm.tgz",
	sha256: "876a8c44bb8d10b19ce816af11f1c527f101c4f714ef5670d885457b451846bb",
};
pub const ANDROID_X64: Archive = Archive {
	tag: "chromium/8100",
	name: "pdfium-android-x64.tgz",
	sha256: "b5cd96bc81c3414a84ce1f3eb928d6fe6981b13fe555f21bfa931bc478f51876",
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
	use super::{Archive, WIN_X64};

	#[test]
	fn a_tag_with_a_slash_is_escaped_in_the_url() {
		assert_eq!(
			WIN_X64.url(),
			"https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F8100/pdfium-win-x64.tgz"
		);
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
