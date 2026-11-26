use std::sync::{Arc, Mutex};

use super::*;

#[derive(Clone, Default)]
struct MemoryStore(Arc<Mutex<Option<String>>>);

impl TokenStore for MemoryStore {
	fn load(&self) -> Option<String> {
		self.0.lock().unwrap().clone()
	}

	fn save(&self, refresh_token: &str) {
		*self.0.lock().unwrap() = Some(refresh_token.to_string());
	}

	fn clear(&self) {
		*self.0.lock().unwrap() = None;
	}
}

/// Nothing listens on port 1, so a call that reaches the network fails at once.
fn offline(store: &MemoryStore) -> Scribe {
	Scribe::with_base_url(Box::new(store.clone()), "en", Url::parse("http://127.0.0.1:1").unwrap())
}

fn redirect(query: &str) -> Url {
	Url::parse(&format!("http://127.0.0.1:5000/oauth/scribe?{query}")).unwrap()
}

#[test]
fn a_saved_sign_in_is_restored() {
	let store = MemoryStore::default();
	assert!(!offline(&store).is_signed_in());
	store.save("refresh");
	assert!(offline(&store).is_signed_in());
}

#[test]
fn the_sign_in_page_asks_for_paperback_with_a_challenge_and_state() {
	let scribe = offline(&MemoryStore::default());
	let sign_in = scribe.begin_sign_in("http://127.0.0.1:5000/oauth/scribe");
	let page = sign_in.url.to_string();
	assert!(page.contains(CLIENT_ID), "{page}");
	assert!(page.contains("code_challenge"), "{page}");
	assert!(page.contains(sign_in.pkce.state()), "{page}");
}

#[tokio::test]
async fn an_answer_to_another_sign_in_is_refused() {
	let scribe = offline(&MemoryStore::default());
	let sign_in = scribe.begin_sign_in("http://127.0.0.1:5000/oauth/scribe");
	let result = scribe.finish_sign_in(sign_in, &redirect("code=abc&state=someone-elses")).await;
	assert!(matches!(result, Err(Error::WrongState)), "{result:?}");
}

#[tokio::test]
async fn a_sign_in_turned_down_in_the_browser_is_cancelled() {
	let scribe = offline(&MemoryStore::default());
	let sign_in = scribe.begin_sign_in("http://127.0.0.1:5000/oauth/scribe");
	let state = sign_in.pkce.state().to_string();
	let result = scribe.finish_sign_in(sign_in, &redirect(&format!("error=access_denied&state={state}"))).await;
	assert!(matches!(result, Err(Error::Cancelled)), "{result:?}");
	assert!(!scribe.is_signed_in());
}

#[tokio::test]
async fn an_answer_without_a_code_is_refused() {
	let scribe = offline(&MemoryStore::default());
	let sign_in = scribe.begin_sign_in("http://127.0.0.1:5000/oauth/scribe");
	let state = sign_in.pkce.state().to_string();
	let result = scribe.finish_sign_in(sign_in, &redirect(&format!("state={state}"))).await;
	assert!(matches!(result, Err(Error::NoCode)), "{result:?}");
}

#[tokio::test]
async fn a_call_before_signing_in_says_so_without_reaching_the_server() {
	let scribe = offline(&MemoryStore::default());
	assert!(matches!(scribe.account().await, Err(Error::NotSignedIn)));
}

#[tokio::test]
async fn signing_out_forgets_the_sign_in_even_when_the_server_cant_be_reached() {
	let store = MemoryStore::default();
	store.save("refresh");
	let scribe = offline(&store);
	scribe.sign_out().await;
	assert!(!scribe.is_signed_in());
	assert_eq!(store.load(), None);
}

/// wx-utils' `background` runs them on a Tokio thread, which needs `Send` futures.
#[test]
fn the_calls_can_run_on_another_thread() {
	fn sendable<T: Send>(_: &T) {}
	let scribe = offline(&MemoryStore::default());
	let sign_in = scribe.begin_sign_in("http://127.0.0.1:5000/oauth/scribe");
	let answer = redirect("code=abc");
	sendable(&scribe.finish_sign_in(sign_in, &answer));
	sendable(&scribe.account());
	sendable(&scribe.update_settings(SettingsUpdate::default()));
	sendable(&scribe.sign_out());
}

/// reqwest and tokio-tungstenite build their TLS configuration without naming a provider, which panics until one is installed.
#[test]
fn a_tls_connection_can_be_configured_without_naming_a_provider() {
	let _scribe = offline(&MemoryStore::default());
	let _config = rustls::ClientConfig::builder();
}
