//! Reading text that a Windows console program wrote in the console's code page.

/// `bytes` decoded with the console's input code page, which is what cmd's own commands write into
/// a pipe. `None` without a console, and on other platforms.
#[must_use]
pub fn text(bytes: &[u8]) -> Option<String> {
	#[cfg(windows)]
	{
		// SAFETY: GetConsoleCP takes no arguments and returns 0 when there is no console.
		let code_page = unsafe { windows_sys::Win32::System::Console::GetConsoleCP() };
		if code_page == 0 {
			return None;
		}
		decode_code_page(code_page, bytes)
	}
	#[cfg(not(windows))]
	{
		let _ = bytes;
		None
	}
}

/// `bytes` decoded with the system's ANSI code page, which a text file saved by a Windows program is in when it is not UTF-8. It differs from the console's code page on most systems, so a list file decoded with that one comes out wrong without failing. `None` on other platforms.
#[must_use]
pub fn ansi_text(bytes: &[u8]) -> Option<String> {
	#[cfg(windows)]
	{
		// SAFETY: GetACP takes no arguments and cannot fail.
		decode_code_page(unsafe { windows_sys::Win32::Globalization::GetACP() }, bytes)
	}
	#[cfg(not(windows))]
	{
		let _ = bytes;
		None
	}
}

/// `bytes` decoded with the Windows code page `code_page`; `None` for empty input or bytes that
/// are not valid in that code page.
#[cfg(windows)]
fn decode_code_page(code_page: u32, bytes: &[u8]) -> Option<String> {
	use windows_sys::Win32::Globalization::{MB_ERR_INVALID_CHARS, MultiByteToWideChar};
	let length = i32::try_from(bytes.len()).ok().filter(|&length| length > 0)?;
	// SAFETY: the input pointer and length describe `bytes`; a null output buffer with size 0 asks
	// only for the size needed.
	let needed = unsafe {
		MultiByteToWideChar(code_page, MB_ERR_INVALID_CHARS, bytes.as_ptr(), length, std::ptr::null_mut(), 0)
	};
	let mut wide = vec![0u16; usize::try_from(needed).ok().filter(|&needed| needed > 0)?];
	// SAFETY: `wide` holds exactly `needed` units, the size the first call asked for.
	let written = unsafe {
		MultiByteToWideChar(code_page, MB_ERR_INVALID_CHARS, bytes.as_ptr(), length, wide.as_mut_ptr(), needed)
	};
	let written = usize::try_from(written).ok().filter(|&written| written > 0)?;
	String::from_utf16(&wide[..written]).ok()
}

#[cfg(all(test, windows))]
mod tests {
	use super::*;

	#[test]
	fn a_line_in_an_oem_code_page_is_decoded() {
		assert_eq!(decode_code_page(850, b"caf\x82.txt").as_deref(), Some("caf\u{e9}.txt"));
	}

	/// The same é is a different byte in the ANSI code page a list file is saved in, which is why a file is not decoded with the console's.
	#[test]
	fn a_line_in_an_ansi_code_page_is_decoded() {
		assert_eq!(decode_code_page(1252, b"caf\xe9.txt").as_deref(), Some("caf\u{e9}.txt"));
	}

	#[test]
	fn an_empty_line_decodes_to_nothing() {
		assert_eq!(decode_code_page(850, b""), None);
	}
}
