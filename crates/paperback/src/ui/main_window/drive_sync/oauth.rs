use std::{
	fs,
	io::{BufRead, BufReader, ErrorKind, Read, Write},
	net::{Ipv4Addr, TcpListener, TcpStream},
	path::{Path, PathBuf},
	thread,
	time::{Duration, Instant},
};

use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use paperback_core::book_sync::DRIVE_SCOPES;
use patois::t;
use serde_json::Value;
use sha2::{Digest, Sha256};
use url::{Url, form_urlencoded};
use windows::{
	Win32::{
		Foundation::{HLOCAL, LocalFree},
		Security::Cryptography::{CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData},
	},
	core::PCWSTR,
};

const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub(super) struct Client {
	id: String,
	secret: String,
}

impl Client {
	pub(super) fn bundled() -> Option<Self> {
		let id = option_env!("PAPERBACK_GOOGLE_DRIVE_CLIENT_ID")?.trim();
		let secret = option_env!("PAPERBACK_GOOGLE_DRIVE_CLIENT_SECRET")?.trim();
		(!id.is_empty() && !secret.is_empty()).then(|| Self { id: id.into(), secret: secret.into() })
	}

	pub(super) fn refresh(&self, credentials: &Path) -> Result<AccessToken> {
		let refresh = String::from_utf8(crypt(&fs::read(credentials)?, false)?)?;
		let data = self.exchange(&[("grant_type", "refresh_token"), ("refresh_token", &refresh)])?;
		AccessToken::read(&data)
	}

	fn exchange(&self, fields: &[(&str, &str)]) -> Result<Value> {
		let body = form_urlencoded::Serializer::new(String::new())
			.append_pair("client_id", &self.id)
			.append_pair("client_secret", &self.secret)
			.extend_pairs(fields.iter().copied())
			.finish();
		let agent: ureq::Agent =
			ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(20))).max_redirects(0).build().into();
		let mut response =
			agent.post(TOKEN_URL).content_type("application/x-www-form-urlencoded").send(body.as_str())?;
		let bytes = response.body_mut().with_config().limit(128 * 1024).read_to_vec()?;
		Ok(serde_json::from_slice(&bytes)?)
	}
}

#[derive(Clone)]
pub(super) struct AccessToken {
	pub value: String,
	expires: Instant,
}

impl AccessToken {
	fn read(data: &Value) -> Result<Self> {
		let expires = data["expires_in"].as_u64().unwrap_or(3_600).clamp(1, 86_400);
		Ok(Self {
			value: data["access_token"].as_str().context("Google did not return an access token")?.to_owned(),
			expires: Instant::now() + Duration::from_secs(expires),
		})
	}

	pub(super) fn fresh(&self) -> bool {
		self.expires.saturating_duration_since(Instant::now()) > Duration::from_secs(45)
	}
}

pub(super) struct Authorization {
	client: Client,
	listener: TcpListener,
	redirect: String,
	verifier: String,
	state: String,
	pub url: String,
}

impl Authorization {
	pub(super) fn start(client: Client) -> Result<Self> {
		let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
		listener.set_nonblocking(true)?;
		let redirect = format!("http://127.0.0.1:{}/", listener.local_addr()?.port());
		let verifier = random_url_value()?;
		let state = random_url_value()?;
		let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
		let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
		url.query_pairs_mut()
			.append_pair("client_id", &client.id)
			.append_pair("redirect_uri", &redirect)
			.append_pair("response_type", "code")
			.append_pair("access_type", "offline")
			.append_pair("prompt", "consent")
			.append_pair("scope", &DRIVE_SCOPES.join(" "))
			.append_pair("code_challenge", &challenge)
			.append_pair("code_challenge_method", "S256")
			.append_pair("state", &state);
		Ok(Self { client, listener, redirect, verifier, state, url: url.into() })
	}

	pub(super) fn complete(self, credentials: PathBuf) -> Result<AccessToken> {
		let deadline = Instant::now() + Duration::from_secs(180);
		while Instant::now() < deadline {
			match self.listener.accept() {
				Ok((mut stream, _)) => match self.read_callback(&mut stream) {
					Ok(Some(code)) => {
						let data = self.client.exchange(&[
							("grant_type", "authorization_code"),
							("code", &code),
							("redirect_uri", &self.redirect),
							("code_verifier", &self.verifier),
						])?;
						let token = AccessToken::read(&data)?;
						let refresh =
							data["refresh_token"].as_str().context("Google did not return a refresh token")?;
						let pending = credentials.with_extension("tmp");
						fs::write(&pending, crypt(refresh.as_bytes(), true)?)?;
						fs::rename(pending, credentials)?;
						return Ok(token);
					}
					Ok(None) => {}
					Err(error) => return Err(error),
				},
				Err(error) if error.kind() == ErrorKind::WouldBlock => thread::sleep(Duration::from_millis(50)),
				Err(error) => return Err(error.into()),
			}
		}
		bail!("Google Drive authorization timed out")
	}

	fn read_callback(&self, stream: &mut TcpStream) -> Result<Option<String>> {
		stream.set_read_timeout(Some(Duration::from_secs(3)))?;
		stream.set_write_timeout(Some(Duration::from_secs(3)))?;
		let mut line = String::new();
		BufReader::new((&mut *stream).take(8192)).read_line(&mut line)?;
		let target = line.split_ascii_whitespace().nth(1).unwrap_or_default();
		let url = Url::parse(&self.redirect)?.join(target)?;
		let fields: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
		if url.path() != "/" || fields.get("state") != Some(&self.state) {
			respond(stream, "400 Bad Request", &t("Invalid Google Drive authorization request."))?;
			return Ok(None);
		}
		if fields.contains_key("error") {
			respond(stream, "200 OK", &t("Authorization was canceled. You can return to Paperback."))?;
			bail!("Google Drive authorization was canceled");
		}
		let code = fields.get("code").context("Missing authorization code")?.clone();
		respond(stream, "200 OK", &t("You can close this page and return to Paperback."))?;
		Ok(Some(code))
	}
}

fn random_url_value() -> Result<String> {
	let mut bytes = [0u8; 32];
	getrandom::fill(&mut bytes).map_err(|error| anyhow::anyhow!("{error}"))?;
	Ok(URL_SAFE_NO_PAD.encode(bytes))
}

fn respond(stream: &mut TcpStream, status: &str, message: &str) -> Result<()> {
	write!(
		stream,
		"HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{message}",
		message.len()
	)?;
	Ok(())
}

fn crypt(bytes: &[u8], protect: bool) -> Result<Vec<u8>> {
	let input = CRYPT_INTEGER_BLOB { cbData: u32::try_from(bytes.len())?, pbData: bytes.as_ptr().cast_mut() };
	let mut output = CRYPT_INTEGER_BLOB::default();
	// DPAPI returns an allocation owned by LocalFree, and the input bytes remain borrowed for the call.
	unsafe {
		if protect {
			CryptProtectData(
				&raw const input,
				PCWSTR::null(),
				None,
				None,
				None,
				CRYPTPROTECT_UI_FORBIDDEN,
				&raw mut output,
			)?;
		} else {
			CryptUnprotectData(&raw const input, None, None, None, None, CRYPTPROTECT_UI_FORBIDDEN, &raw mut output)?;
		}
		ensure!(!output.pbData.is_null(), "DPAPI returned no credential");
		let result = std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec();
		let _ = LocalFree(Some(HLOCAL(output.pbData.cast())));
		Ok(result)
	}
}
