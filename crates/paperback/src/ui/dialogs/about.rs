use std::ffi::CString;

use wxdragon::{ffi, prelude::*, translations::translate as t};

pub fn show_about_dialog(parent: &Frame) {
	let name = CString::new("Paperback").unwrap_or_else(|_| CString::new("").unwrap());
	let version = CString::new(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| CString::new("").unwrap());
	let description = CString::new(t("An accessible, lightweight, fast ebook and document reader"))
		.unwrap_or_else(|_| CString::new("").unwrap());
	let copyright = CString::new("Copyright (C) 2025-2026 Quin Gillespie. All rights reserved.")
		.unwrap_or_else(|_| CString::new("").unwrap());
	let website = CString::new("https://paperback.dev").unwrap_or_else(|_| CString::new("").unwrap());
	unsafe {
		let info = ffi::wxd_AboutDialogInfo_Create();
		if info.is_null() {
			return;
		}
		ffi::wxd_AboutDialogInfo_SetName(info, name.as_ptr());
		ffi::wxd_AboutDialogInfo_SetVersion(info, version.as_ptr());
		ffi::wxd_AboutDialogInfo_SetDescription(info, description.as_ptr());
		ffi::wxd_AboutDialogInfo_SetCopyright(info, copyright.as_ptr());
		ffi::wxd_AboutDialogInfo_SetWebSite(info, website.as_ptr());
		ffi::wxd_AboutBox(info, parent.handle_ptr());
		ffi::wxd_AboutDialogInfo_Destroy(info);
	}
}
