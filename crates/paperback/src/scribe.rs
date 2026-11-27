//! Scribe for Documents in the desktop app: one sign-in for the whole app, kept in Windows Credential Manager.

use std::{
	iter, ptr, slice,
	sync::{Arc, LazyLock},
};

use paperback_core::scribe::{Scribe, TokenStore};
use windows::{
	Win32::Security::Credentials::{
		CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC, CREDENTIALW, CredDeleteW, CredFree, CredReadW, CredWriteW,
	},
	core::{HSTRING, PWSTR},
};

use crate::translation_manager::TranslationManager;

const CREDENTIAL: &str = "Paperback/Scribe";

static SCRIBE: LazyLock<Arc<Scribe>> = LazyLock::new(|| {
	let language = TranslationManager::instance().lock().unwrap().current_language().replace('_', "-");
	Arc::new(Scribe::new(Box::new(CredentialStore), &language))
});

pub fn shared() -> Arc<Scribe> {
	Arc::clone(&SCRIBE)
}

/// The refresh token, as a generic credential that only this Windows user can read.
struct CredentialStore;

impl TokenStore for CredentialStore {
	fn load(&self) -> Option<String> {
		let mut credential = ptr::null_mut::<CREDENTIALW>();
		// SAFETY: CredReadW fills `credential` with a buffer it allocated, which is freed below once read.
		unsafe { CredReadW(&HSTRING::from(CREDENTIAL), CRED_TYPE_GENERIC, None, &raw mut credential) }.ok()?;
		// SAFETY: a successful CredReadW leaves `credential` pointing at a credential whose blob is `CredentialBlobSize` bytes long.
		let token = unsafe {
			let credential = &*credential;
			let blob = slice::from_raw_parts(credential.CredentialBlob, credential.CredentialBlobSize as usize);
			String::from_utf8(blob.to_vec()).ok()
		};
		// SAFETY: `credential` came from CredReadW and is freed only here.
		unsafe { CredFree(credential.cast()) };
		token
	}

	fn save(&self, refresh_token: &str) {
		let mut name: Vec<u16> = CREDENTIAL.encode_utf16().chain(iter::once(0)).collect();
		let mut blob = refresh_token.as_bytes().to_vec();
		let credential = CREDENTIALW {
			Type: CRED_TYPE_GENERIC,
			TargetName: PWSTR(name.as_mut_ptr()),
			CredentialBlobSize: u32::try_from(blob.len()).unwrap_or(0),
			CredentialBlob: blob.as_mut_ptr(),
			Persist: CRED_PERSIST_LOCAL_MACHINE,
			..Default::default()
		};
		// SAFETY: `name` and `blob` outlive the call, which copies them.
		if let Err(error) = unsafe { CredWriteW(&raw const credential, 0) } {
			tracing::warn!(%error, "couldn't keep the Scribe sign-in in Credential Manager");
		}
	}

	fn clear(&self) {
		// SAFETY: the name is a valid string for the call's length.
		let _ = unsafe { CredDeleteW(&HSTRING::from(CREDENTIAL), CRED_TYPE_GENERIC, None) };
	}
}
