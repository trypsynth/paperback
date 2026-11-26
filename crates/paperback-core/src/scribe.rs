//! Scribe for Documents, Pneuma Solutions' conversion service. The calls are async to run on a Tokio runtime, such as wx-utils' `background`, since scribe-client is built on reqwest.

use std::{
	collections::HashMap,
	sync::{Arc, Mutex, PoisonError},
};

pub use scribe_client::{AccountInfo, BrailleTable, Dialect, Language, Settings, SettingsUpdate, Voice};
use scribe_client::{
	AuthClient, DELETE_ACCOUNT_PATH, PkceChallenge, ScribeClient, ScribeError, TokenSet, Url, reqwest,
};

/// Scribe registers Paperback as a public client, with no secret, so the ID isn't sensitive.
pub const CLIENT_ID: &str = "caaff386e53250d3130b246aee5b4a271b65bb5095cf448fdab2e7780a2a94df";
const BASE_URL: &str = "https://scribeit.io";
const ACCOUNT_PATH: &str = "/settings";

/// Where an app keeps the refresh token between runs, such as Credential Manager on Windows.
pub trait TokenStore: Send + Sync {
	fn load(&self) -> Option<String>;
	fn save(&self, refresh_token: &str);
	fn clear(&self);
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("not signed in to Scribe")]
	NotSignedIn,
	#[error("the sign-in was cancelled in the browser")]
	Cancelled,
	/// The redirect's `state` isn't the one this sign-in sent, so it answers some other request.
	#[error("the sign-in answer belongs to another request")]
	WrongState,
	#[error("Scribe sent back no sign-in code")]
	NoCode,
	/// Its `Display` is the server's own sentence, in the locale the client was made with.
	#[error(transparent)]
	Scribe(#[from] ScribeError),
}

/// A sign-in between opening `url` in the browser and the redirect back to Paperback.
pub struct SignIn {
	pub url: Url,
	pkce: PkceChallenge,
	redirect_uri: String,
}

pub struct Scribe {
	http: reqwest::Client,
	base_url: Url,
	locale: String,
	store: Box<dyn TokenStore>,
	client: Mutex<Option<Arc<ScribeClient>>>,
	/// The refresh token last given to `store`, so it's only written when the server rotates it.
	saved: Mutex<Option<String>>,
}

impl Scribe {
	/// `locale` is a language tag such as `de` or `pt-BR`, for the sign-in page and the server's messages.
	#[must_use]
	pub fn new(store: Box<dyn TokenStore>, locale: &str) -> Self {
		Self::with_base_url(store, locale, Url::parse(BASE_URL).expect("the base URL is a valid URL"))
	}

	#[must_use]
	pub fn with_base_url(store: Box<dyn TokenStore>, locale: &str, base_url: Url) -> Self {
		// scribe-client's reqwest and WebSocket take rustls's process-wide provider, which is ring, as ureq already uses; it is already set when this runs twice.
		let _ = rustls::crypto::ring::default_provider().install_default();
		let saved = store.load();
		let scribe = Self {
			http: reqwest::Client::new(),
			base_url,
			locale: locale.to_string(),
			store,
			client: Mutex::new(None),
			saved: Mutex::new(saved.clone()),
		};
		// Without an access token, the first call refreshes it with the saved refresh token.
		if let Some(refresh_token) = saved {
			scribe.set_client(Some(TokenSet {
				access_token: String::new(),
				refresh_token: Some(refresh_token),
				expires_at: None,
			}));
		}
		scribe
	}

	fn auth(&self) -> AuthClient {
		AuthClient::new(self.http.clone(), self.base_url.clone(), CLIENT_ID).with_locale(&self.locale)
	}

	fn set_client(&self, tokens: Option<TokenSet>) {
		let client = tokens.map(|tokens| {
			Arc::new(
				ScribeClient::new(self.http.clone(), self.base_url.clone(), CLIENT_ID, tokens)
					.with_locale(&self.locale),
			)
		});
		*self.client.lock().unwrap_or_else(PoisonError::into_inner) = client;
	}

	fn current(&self) -> Option<Arc<ScribeClient>> {
		self.client.lock().unwrap_or_else(PoisonError::into_inner).clone()
	}

	#[must_use]
	pub fn is_signed_in(&self) -> bool {
		self.current().is_some()
	}

	/// `redirect_uri` must be one Scribe registered for Paperback; a loopback one may name any port.
	#[must_use]
	pub fn begin_sign_in(&self, redirect_uri: &str) -> SignIn {
		let pkce = PkceChallenge::generate();
		SignIn { url: self.auth().authorization_url(redirect_uri, &pkce), pkce, redirect_uri: redirect_uri.to_string() }
	}

	/// `redirect` is the URL the browser came back to, with its `code` and `state`.
	///
	/// # Errors
	///
	/// [`Error::WrongState`] when `redirect` answers another request, [`Error::Cancelled`] when the reader turned the sign-in down, and otherwise what the server said.
	pub async fn finish_sign_in(&self, sign_in: SignIn, redirect: &Url) -> Result<AccountInfo, Error> {
		let query: HashMap<_, _> = redirect.query_pairs().collect();
		if query.get("state").map(AsRef::as_ref) != Some(sign_in.pkce.state()) {
			return Err(Error::WrongState);
		}
		if query.contains_key("error") {
			return Err(Error::Cancelled);
		}
		let code = query.get("code").ok_or(Error::NoCode)?;
		let tokens = self.auth().exchange_code(&sign_in.redirect_uri, code, sign_in.pkce.verifier()).await?;
		self.set_client(Some(tokens));
		self.account().await
	}

	/// Signs out here even when the server can't be reached to revoke the token, so the reader is never left signed in against their wishes.
	pub async fn sign_out(&self) {
		let Some(client) = self.current() else { return };
		self.forget();
		if let Some(refresh_token) = client.current_tokens().await.refresh_token
			&& let Err(error) = self.auth().revoke(&refresh_token).await
		{
			tracing::warn!(%error, "couldn't revoke the Scribe sign-in on the server");
		}
	}

	fn forget(&self) {
		self.set_client(None);
		*self.saved.lock().unwrap_or_else(PoisonError::into_inner) = None;
		self.store.clear();
	}

	/// Runs `call` on the signed-in client, then keeps the refresh token, which the server replaces on every refresh.
	async fn call<T>(&self, call: impl AsyncFnOnce(&ScribeClient) -> Result<T, ScribeError>) -> Result<T, Error> {
		let client = self.current().ok_or(Error::NotSignedIn)?;
		let result = call(&client).await;
		if let Err(ScribeError::InvalidGrant { .. }) = result {
			// The sign-in was revoked or ran out, so the reader is signed out rather than shown the same error on every call.
			self.forget();
			return Err(Error::NotSignedIn);
		}
		if let Some(refresh_token) = client.current_tokens().await.refresh_token {
			let mut saved = self.saved.lock().unwrap_or_else(PoisonError::into_inner);
			if saved.as_deref() != Some(&refresh_token) {
				self.store.save(&refresh_token);
				*saved = Some(refresh_token);
			}
		}
		Ok(result?)
	}

	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn account(&self) -> Result<AccountInfo, Error> {
		self.call(async |client| client.get_account_info().await).await
	}

	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn pages_remaining(&self) -> Result<i64, Error> {
		self.call(async |client| Ok(client.billing().await?.pages_remaining)).await
	}

	/// The account's defaults for new conversions.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn settings(&self) -> Result<Settings, Error> {
		self.call(async |client| client.default_settings().await).await
	}

	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn update_settings(&self, update: SettingsUpdate) -> Result<Settings, Error> {
		self.call(async |client| client.update_default_settings(&update).await).await
	}

	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn languages(&self) -> Result<Vec<Language>, Error> {
		self.call(async |client| client.languages().await).await
	}

	/// By language code.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn dialects(&self) -> Result<HashMap<String, Vec<Dialect>>, Error> {
		self.call(async |client| client.dialects().await).await
	}

	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn braille_tables(&self) -> Result<Vec<BrailleTable>, Error> {
		self.call(async |client| client.braille_tables().await).await
	}

	/// By dialect locale.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn voices(&self) -> Result<HashMap<String, Vec<Voice>>, Error> {
		self.call(async |client| client.voices().await).await
	}

	/// The voice's sample as audio file bytes.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn voice_sample(&self, short_name: &str) -> Result<Vec<u8>, Error> {
		self.call(async |client| client.voice_sample(short_name).await).await
	}

	/// The account's page on the web, opening already signed in.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn account_page_url(&self) -> Result<String, Error> {
		self.call(async |client| client.browser_entry_url(ACCOUNT_PATH).await).await
	}

	/// The page that deletes the account, opening already signed in.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn delete_account_url(&self) -> Result<String, Error> {
		self.call(async |client| client.browser_entry_url(DELETE_ACCOUNT_PATH).await).await
	}
}

#[cfg(test)]
mod tests;
