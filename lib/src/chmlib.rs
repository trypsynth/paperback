use std::{
	ffi::{CStr, CString},
	mem,
	os::raw::{c_char, c_int, c_longlong, c_uchar, c_void},
	path::Path,
};

use anyhow::{Context, Result};

unsafe extern "C" {
	fn chm_open(filename: *const c_char) -> *mut ChmFile;
	fn chm_close(file: *mut ChmFile);
	fn chm_enumerate(file: *mut ChmFile, what: c_int, callback: ChmEnumerateCallback, context: *mut c_void) -> c_int;
	fn chm_resolve_object(file: *mut ChmFile, path: *const c_char, ui: *mut ChmUnitInfo) -> c_int;
	fn chm_retrieve_object(
		file: *mut ChmFile,
		ui: *const ChmUnitInfo,
		buf: *mut c_uchar,
		addr: c_longlong,
		len: c_longlong,
	) -> c_longlong;
}

#[repr(C)]
pub struct ChmFile {
	_private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ChmUnitInfo {
	pub start: c_longlong,
	pub length: c_longlong,
	pub space: c_int,
	pub flags: c_int,
	pub path: [c_char; 512],
}

pub type ChmEnumerateCallback = extern "C" fn(*mut ChmFile, *mut ChmUnitInfo, *mut c_void) -> c_int;

pub const CHM_ENUMERATE_ALL: c_int = 3;
pub const CHM_ENUMERATOR_CONTINUE: c_int = 1;
pub const CHM_ENUMERATOR_SUCCESS: c_int = 0;
pub const CHM_RESOLVE_SUCCESS: c_int = 0;

pub struct ChmHandle {
	handle: *mut ChmFile,
}

impl ChmHandle {
	pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		let path_str = path.as_ref().to_string_lossy().to_string();
		let c_path =
			CString::new(path_str.as_str()).with_context(|| format!("Invalid path for CHM file: {path_str}"))?;
		unsafe {
			let handle = chm_open(c_path.as_ptr());
			if handle.is_null() {
				anyhow::bail!("Failed to open CHM file: {path_str}");
			}
			Ok(Self { handle })
		}
	}

	pub fn enumerate<F>(&mut self, what: c_int, mut callback: F) -> Result<()>
	where
		F: FnMut(&ChmUnitInfo) -> bool,
	{
		extern "C" fn trampoline<F>(_file: *mut ChmFile, ui: *mut ChmUnitInfo, context: *mut c_void) -> c_int
		where
			F: FnMut(&ChmUnitInfo) -> bool,
		{
			unsafe {
				let callback: &mut F = &mut *context.cast::<F>();
				let should_continue = callback(&*ui);
				if should_continue { CHM_ENUMERATOR_CONTINUE } else { CHM_ENUMERATOR_SUCCESS }
			}
		}
		unsafe {
			let context: *mut c_void = (&raw mut callback).cast::<c_void>();
			let result = chm_enumerate(self.handle, what, trampoline::<F>, context);
			if result != 0 { Ok(()) } else { anyhow::bail!("CHM enumeration failed") }
		}
	}

	pub fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
		let c_path = CString::new(path).context("Invalid file path")?;
		unsafe {
			let mut ui: ChmUnitInfo = mem::zeroed();
			let resolve_result = chm_resolve_object(self.handle, c_path.as_ptr(), &raw mut ui);
			if resolve_result != CHM_RESOLVE_SUCCESS {
				anyhow::bail!("Failed to resolve CHM object: {path}");
			}
			if ui.length == 0 {
				return Ok(Vec::new());
			}
			let mut buffer = vec![0u8; usize::try_from(ui.length)?];
			let bytes_read = chm_retrieve_object(self.handle, &raw const ui, buffer.as_mut_ptr(), 0, ui.length);
			if bytes_read != ui.length {
				anyhow::bail!("Failed to read complete CHM file (expected {} bytes, got {})", ui.length, bytes_read);
			}
			Ok(buffer)
		}
	}
}

impl Drop for ChmHandle {
	fn drop(&mut self) {
		if !self.handle.is_null() {
			unsafe {
				chm_close(self.handle);
			}
		}
	}
}

// SAFETY: this is safe because we're the only ones with access to the handle, and CHM operations are thread-safe at the file level.
unsafe impl Send for ChmHandle {}
unsafe impl Sync for ChmHandle {}

pub fn unit_info_path(ui: &ChmUnitInfo) -> String {
	unsafe { CStr::from_ptr(ui.path.as_ptr()).to_string_lossy().into_owned() }
}
