//! One document's conversion: the upload, then its pages as Scribe streams them. Uploading starts the conversion, and it is charged: the stream is the whole document, not a preview.

use std::{collections::BTreeMap, path::Path, time::Duration};

use scribe_client::{ChannelEvent, DocumentChannel, DocumentSource, OutputFormat, ScribeError, Stage};

use super::{Error, Scribe};

/// Scribe drops a channel now and then, as when it restarts, and the conversion carries on without it.
const RECONNECTS: u32 = 5;

/// What changed since the last [`Conversion::next`].
#[derive(Clone, Debug, PartialEq)]
pub enum Update {
	/// What Scribe is doing, and how far through, from 0 to 1.
	Stage { stage: Stage, progress: f64 },
	/// More pages arrived, and [`Conversion::html`] has them.
	Pages,
	/// The document is locked, or the password given was wrong, so nothing more comes until [`Conversion::unlock`].
	PasswordRequired,
	/// Every page has arrived.
	Complete,
}

pub struct Conversion {
	document_id: String,
	title: String,
	/// By their index in the stream, since a reconnected channel sends them all again.
	chunks: BTreeMap<u64, String>,
	channel: Option<DocumentChannel>,
	reconnects: u32,
}

impl Scribe {
	/// Uploads the file at `path`, which starts converting it.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said, such as that the account is out of pages.
	pub async fn upload(&self, path: &Path) -> Result<Conversion, Error> {
		let source = DocumentSource::Path(path.to_path_buf());
		let created = self.call(async |client| client.create_document(source, None).await).await?;
		let title = path.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default();
		Ok(Conversion::new(created.document_id, title))
	}
}

impl Conversion {
	fn new(document_id: String, title: String) -> Self {
		Self { document_id, title, chunks: BTreeMap::new(), channel: None, reconnects: 0 }
	}

	/// Waits for the next change, reconnecting when the channel drops.
	///
	/// # Errors
	///
	/// [`Error::Failed`] with Scribe's reason when the conversion fails, or what the server said when the channel can't be opened again.
	pub async fn next(&mut self, scribe: &Scribe) -> Result<Update, Error> {
		loop {
			let channel = self.channel(scribe).await?;
			match channel.next_event().await {
				Ok(event) => {
					self.reconnects = 0;
					if let Some(update) = self.apply(event)? {
						return Ok(update);
					}
				}
				Err(error @ (ScribeError::WebSocket(_) | ScribeError::ChannelClosed)) => self.dropped(error).await?,
				Err(error) => return Err(error.into()),
			}
		}
	}

	/// Unlocks a password-protected document, which then carries on converting.
	///
	/// # Errors
	///
	/// What the server said. A wrong password comes back from [`Self::next`] as [`Update::PasswordRequired`].
	pub async fn unlock(&mut self, scribe: &Scribe, password: &str) -> Result<(), Error> {
		let channel = self.channel(scribe).await?;
		channel.start_conversion(OutputFormat::HtmlStream, Some(password)).await?;
		Ok(())
	}

	async fn channel(&mut self, scribe: &Scribe) -> Result<&mut DocumentChannel, Error> {
		while self.channel.is_none() {
			let id = self.document_id.clone();
			match scribe.call(async |client| client.open_document_channel(&id).await).await {
				Ok(channel) => self.channel = Some(channel),
				Err(Error::Scribe(error @ (ScribeError::WebSocket(_) | ScribeError::Http(_)))) => {
					self.dropped(error).await?;
				}
				Err(error) => return Err(error),
			}
		}
		Ok(self.channel.as_mut().expect("the channel was just opened"))
	}

	/// Waits longer after each drop in a row, and gives up after [`RECONNECTS`].
	async fn dropped(&mut self, error: ScribeError) -> Result<(), Error> {
		self.channel = None;
		if self.reconnects == RECONNECTS {
			return Err(error.into());
		}
		tracing::warn!(%error, reconnects = self.reconnects, "lost the Scribe conversion channel");
		tokio::time::sleep(Duration::from_secs(1 << self.reconnects)).await;
		self.reconnects += 1;
		Ok(())
	}

	fn apply(&mut self, event: ChannelEvent) -> Result<Option<Update>, Error> {
		Ok(match event {
			ChannelEvent::Status { format: OutputFormat::HtmlStream, stage: Stage::Complete, .. }
			| ChannelEvent::ConversionComplete { format: OutputFormat::HtmlStream, .. } => Some(Update::Complete),
			ChannelEvent::Status { format: OutputFormat::HtmlStream, stage, progress } => {
				Some(Update::Stage { stage, progress })
			}
			ChannelEvent::Chunk { index, content } => {
				// Servers from before chunks had an index send each once, in order.
				let index = index.unwrap_or_else(|| self.chunks.last_key_value().map_or(0, |(last, _)| last + 1));
				self.chunks.insert(index, content).is_none().then_some(Update::Pages)
			}
			ChannelEvent::PasswordRequired => Some(Update::PasswordRequired),
			ChannelEvent::Error { reason, .. } if reason == "invalid_password" => Some(Update::PasswordRequired),
			ChannelEvent::Error { message, .. } => return Err(Error::Failed(message)),
			_ => None,
		})
	}

	#[must_use]
	pub fn has_pages(&self) -> bool {
		!self.chunks.is_empty()
	}

	/// The pages so far as one page of HTML. The chunks are fragments of its body, as Scribe's own reading view joins them.
	#[must_use]
	pub fn html(&self) -> String {
		let body: String = self.chunks.values().map(String::as_str).collect();
		page(&self.title, &body)
	}

	#[must_use]
	pub fn title(&self) -> &str {
		&self.title
	}
}

/// A page of HTML titled `title` with `message` as its one paragraph, to show before any pages arrive.
#[must_use]
pub fn status_page(title: &str, message: &str) -> String {
	page(title, &format!("<h1>{}</h1><p>{}</p>", escape(title), escape(message)))
}

fn page(title: &str, body: &str) -> String {
	format!(
		"<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body>{body}</body></html>",
		escape(title)
	)
}

fn escape(text: &str) -> String {
	text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
	use super::*;

	fn chunk(index: Option<u64>, content: &str) -> ChannelEvent {
		ChannelEvent::Chunk { index, content: content.to_string() }
	}

	#[test]
	fn pages_sent_again_after_a_reconnect_are_kept_once_in_order() {
		let mut conversion = Conversion::new("id".to_string(), "Book".to_string());
		assert_eq!(conversion.apply(chunk(Some(1), "<p>two</p>")).unwrap(), Some(Update::Pages));
		assert_eq!(conversion.apply(chunk(Some(0), "<p>one</p>")).unwrap(), Some(Update::Pages));
		assert_eq!(conversion.apply(chunk(Some(0), "<p>one</p>")).unwrap(), None);
		assert!(conversion.html().contains("<body><p>one</p><p>two</p></body>"), "{}", conversion.html());
	}

	#[test]
	fn pages_without_an_index_follow_each_other() {
		let mut conversion = Conversion::new("id".to_string(), "Book".to_string());
		conversion.apply(chunk(None, "a")).unwrap();
		conversion.apply(chunk(None, "b")).unwrap();
		assert!(conversion.html().contains("<body>ab</body>"));
	}

	#[test]
	fn only_the_streamed_format_reports_progress() {
		let mut conversion = Conversion::new("id".to_string(), "Book".to_string());
		let epub = ChannelEvent::Status { format: OutputFormat::Epub, stage: Stage::Convert, progress: 0.5 };
		assert_eq!(conversion.apply(epub).unwrap(), None);
		let stream = ChannelEvent::Status { format: OutputFormat::HtmlStream, stage: Stage::Convert, progress: 0.5 };
		assert_eq!(conversion.apply(stream).unwrap(), Some(Update::Stage { stage: Stage::Convert, progress: 0.5 }));
		let done = ChannelEvent::ConversionComplete { format: OutputFormat::HtmlStream, output_id: "o".to_string() };
		assert_eq!(conversion.apply(done).unwrap(), Some(Update::Complete));
	}

	#[test]
	fn a_wrong_password_asks_again_and_a_failure_gives_scribes_reason() {
		let mut conversion = Conversion::new("id".to_string(), "Book".to_string());
		let wrong = ChannelEvent::Error { reason: "invalid_password".to_string(), message: None };
		assert_eq!(conversion.apply(wrong).unwrap(), Some(Update::PasswordRequired));
		let failed =
			ChannelEvent::Error { reason: "failed".to_string(), message: Some("The file is damaged.".to_string()) };
		assert!(
			matches!(conversion.apply(failed), Err(Error::Failed(Some(message))) if message == "The file is damaged.")
		);
	}

	#[test]
	fn the_status_page_escapes_the_title() {
		let page = status_page("Tom & Jerry", "Uploading...");
		assert!(page.contains("<title>Tom &amp; Jerry</title>") && page.contains("<p>Uploading...</p>"), "{page}");
	}
}
