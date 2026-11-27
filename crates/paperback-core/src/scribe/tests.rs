use std::{
	io::{Read, Write},
	net::TcpStream,
	sync::{Arc, Mutex},
};

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

/// Sends `target` to `uri`'s port as the browser would, and gives back the response.
fn visit(uri: &str, target: &str) -> String {
	let port = Url::parse(uri).unwrap().port().unwrap();
	let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
	write!(
		stream,
		"GET {target} HTTP/1.1
Host: 127.0.0.1:{port}

"
	)
	.unwrap();
	let mut response = String::new();
	stream.read_to_string(&mut response).unwrap();
	response
}

fn wait_in_background(loopback: Loopback) -> std::thread::JoinHandle<Option<Url>> {
	std::thread::spawn(move || loopback.wait(&AtomicBool::new(false), Duration::from_secs(10), "de", "Fertig & zurück"))
}

#[test]
fn the_browser_coming_back_ends_the_wait() {
	let loopback = Loopback::bind().unwrap();
	let uri = loopback.redirect_uri();
	assert!(uri.starts_with("http://127.0.0.1:") && uri.ends_with("/oauth/scribe"), "{uri}");
	let waiting = wait_in_background(loopback);
	let response = visit(&uri, "/oauth/scribe?code=abc&state=xyz");
	assert!(response.starts_with("HTTP/1.1 200"), "{response}");
	assert!(response.contains(r#"lang="de""#) && response.contains("Fertig &amp; zurück"), "{response}");
	let redirect = waiting.join().unwrap().unwrap();
	assert_eq!(redirect.query(), Some("code=abc&state=xyz"));
}

#[test]
fn other_requests_are_turned_away_while_waiting() {
	let loopback = Loopback::bind().unwrap();
	let uri = loopback.redirect_uri();
	let waiting = wait_in_background(loopback);
	assert!(visit(&uri, "/favicon.ico").starts_with("HTTP/1.1 404"));
	visit(&uri, "/oauth/scribe?code=abc");
	assert!(waiting.join().unwrap().is_some());
}

#[test]
fn a_cancelled_or_timed_out_wait_gives_nothing() {
	assert!(Loopback::bind().unwrap().wait(&AtomicBool::new(true), Duration::from_secs(10), "en", "").is_none());
	assert!(Loopback::bind().unwrap().wait(&AtomicBool::new(false), Duration::ZERO, "en", "").is_none());
}

fn settings(language: &str, dialects: &serde_json::Value, voices: &serde_json::Value) -> Settings {
	serde_json::from_value(serde_json::json!({
		"language": language, "dialects": dialects, "voices": voices, "tts_gender": "", "tts_rate": 1.0,
		"braille_translation_table": "en-us-g2.ctb", "braille_cells_per_line": 40, "braille_split_into_pages": true,
		"braille_lines_per_page": 25, "large_print": false, "add_image_descriptions": true, "math": false,
		"notify_when_complete": false
	}))
	.unwrap()
}

#[test]
fn the_voice_comes_from_the_dialect_chosen_for_the_language() {
	let choice = VoiceChoice::from_settings(&settings(
		"en",
		&serde_json::json!({"en": "en-GB", "fr": "fr-CA"}),
		&serde_json::json!({"en-GB": "en-GB-RyanNeural", "en-US": "en-US-AriaNeural"}),
	));
	assert_eq!(choice.dialect(), Some("en-GB"));
	assert_eq!(choice.voice(), Some("en-GB-RyanNeural"));
}

#[test]
fn going_back_to_a_language_finds_its_dialect_and_voice() {
	let mut choice = VoiceChoice::from_settings(&settings("en", &serde_json::json!({}), &serde_json::json!({})));
	choice.set_dialect(Some("en-GB".to_string()));
	choice.set_voice(Some("en-GB-RyanNeural".to_string()));
	choice.language = Some("fr".to_string());
	assert_eq!(choice.dialect(), None);
	choice.language = Some("en".to_string());
	assert_eq!(choice.voice(), Some("en-GB-RyanNeural"));
}

#[test]
fn automatic_is_sent_as_an_empty_language_and_no_dialect() {
	let mut choice =
		VoiceChoice::from_settings(&settings("en", &serde_json::json!("{\"en\": \"en-GB\"}"), &serde_json::json!({})));
	assert_eq!(choice.dialect(), Some("en-GB"), "a string of JSON is read too");
	choice.set_dialect(None);
	choice.language = None;
	let mut update = SettingsUpdate::default();
	choice.apply_to(&mut update);
	assert_eq!(update.language.as_deref(), Some(""));
	assert_eq!(update.dialects, Some(serde_json::json!({})));
}

#[test]
fn a_language_without_dialects_offers_none() {
	let catalog = Catalog::default();
	assert!(catalog.dialects_of(Some("xx")).is_empty());
	assert!(catalog.voices_of(None).is_empty());
}
