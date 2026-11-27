//! One document's conversion: the upload, then its pages as Scribe streams them. Uploading starts the conversion, and it is charged: the stream is the whole document, not a preview. A document reopened from the account comes back the same way where the server sends the pages again, and otherwise as HTML Scribe makes of it, which costs nothing more.

use std::{collections::BTreeMap, path::Path, time::Duration};

use scribe_client::{ChannelEvent, DocumentChannel, DocumentSource, DocumentSummary, OutputFormat, ScribeError, Stage};

use super::{Error, Scribe};

/// Scribe drops a channel now and then, as when it restarts, and the conversion carries on without it.
const RECONNECTS: u32 = 5;
/// A server that sends a finished document's pages again sends them as the channel opens.
const REPLAY_WAIT: Duration = Duration::from_secs(2);

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
	/// Opened from the account rather than uploaded, so the pages before now may not come again.
	reopened: bool,
	/// Scribe has the document as HTML of its own, to download rather than stream.
	html_ready: bool,
	/// Scribe was asked for that HTML, as the stream came back without every page.
	html_requested: bool,
	html: Option<String>,
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

	/// The documents in the reader's account, newest first.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn documents(&self) -> Result<Vec<DocumentSummary>, Error> {
		Ok(self.call(async |client| client.list_documents().await).await?.documents)
	}

	/// Scribe deletes a trashed document for good after 7 days, and the website can recover it until then.
	///
	/// # Errors
	///
	/// [`Error::NotSignedIn`], or what the server said.
	pub async fn trash(&self, document_id: &str) -> Result<(), Error> {
		self.call(async |client| client.trash_document(document_id).await).await
	}
}

impl Conversion {
	fn new(document_id: String, title: String) -> Self {
		Self {
			document_id,
			title,
			chunks: BTreeMap::new(),
			channel: None,
			reconnects: 0,
			reopened: false,
			html_ready: false,
			html_requested: false,
			html: None,
		}
	}

	/// A document already in the reader's account, whether Scribe has finished it or not.
	#[must_use]
	pub fn reopen(document: &DocumentSummary) -> Self {
		let mut conversion = Self::new(document.id.clone(), document.title.clone().unwrap_or_default());
		conversion.reopened = true;
		conversion.html_ready =
			document.outputs.iter().any(|output| output.format == OutputFormat::Html && output.stage.is_complete());
		conversion
	}

	/// Waits for the next change, reconnecting when the channel drops.
	///
	/// # Errors
	///
	/// [`Error::Failed`] with Scribe's reason when the conversion fails, or what the server said when the channel can't be opened again.
	pub async fn next(&mut self, scribe: &Scribe) -> Result<Update, Error> {
		loop {
			if self.html_ready && self.html.is_none() {
				self.download(scribe).await?;
				return Ok(Update::Complete);
			}
			let channel = self.channel(scribe).await?;
			match channel.next_event().await {
				Ok(event) => {
					self.reconnects = 0;
					match self.apply(event)? {
						Some(Update::Complete) if self.reopened && !self.has_every_page() => {
							if !self.html_requested {
								self.await_replay().await?;
								if self.has_every_page() {
									return Ok(Update::Complete);
								}
								self.request_html(scribe).await?;
								return Ok(Update::Stage { stage: Stage::Start, progress: 0.0 });
							}
						}
						Some(update) => return Ok(update),
						None => {}
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

	/// Takes the pages a server sends again as the channel opens, until it goes quiet.
	async fn await_replay(&mut self) -> Result<(), Error> {
		let Some(mut channel) = self.channel.take() else { return Ok(()) };
		while let Ok(event) = tokio::time::timeout(REPLAY_WAIT, channel.next_event()).await {
			// A dropped channel is opened again by the next call that needs it.
			let Ok(event) = event else { return Ok(()) };
			self.apply(event)?;
		}
		self.channel = Some(channel);
		Ok(())
	}

	/// Asks Scribe to make HTML of the whole document, which [`Self::next`] downloads when it's done. A conversion already running, such as one asked for when the document was last opened, is waited for instead.
	async fn request_html(&mut self, scribe: &Scribe) -> Result<(), Error> {
		let channel = self.channel(scribe).await?;
		match channel.start_conversion(OutputFormat::Html, None).await {
			Ok(_) | Err(ScribeError::ConversionInProgress { .. }) => {}
			Err(error) => return Err(error.into()),
		}
		self.html_requested = true;
		Ok(())
	}

	async fn download(&mut self, scribe: &Scribe) -> Result<(), Error> {
		let id = self.document_id.clone();
		let download = scribe.call(async |client| client.download_output(&id, OutputFormat::Html).await).await?;
		self.html = Some(String::from_utf8_lossy(&download.data).into_owned());
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
			ChannelEvent::Status { format: OutputFormat::Html, stage: Stage::Complete, .. }
			| ChannelEvent::ConversionComplete { format: OutputFormat::Html, .. } => {
				self.html_ready = true;
				None
			}
			ChannelEvent::Status { format: OutputFormat::Html, stage, progress } if self.html_requested => {
				Some(Update::Stage { stage, progress })
			}
			ChannelEvent::PasswordRequired => Some(Update::PasswordRequired),
			ChannelEvent::Error { reason, .. } if reason == "invalid_password" => Some(Update::PasswordRequired),
			ChannelEvent::Error { message, .. } => return Err(Error::Failed(message)),
			_ => None,
		})
	}

	#[must_use]
	pub fn has_pages(&self) -> bool {
		!self.chunks.is_empty() || self.html.is_some()
	}

	/// Every page from the first, with none missing, as a reopened document has when the server sent them all again.
	fn has_every_page(&self) -> bool {
		!self.chunks.is_empty() && self.chunks.keys().copied().eq(0..self.chunks.len() as u64)
	}

	/// The pages so far as one page of HTML. The chunks are fragments of its body, as Scribe's own reading view joins them.
	#[must_use]
	pub fn html(&self) -> String {
		if let Some(html) = &self.html {
			return html.clone();
		}
		let body: String = self.chunks.values().map(String::as_str).collect();
		page(&self.title, &body)
	}

	#[must_use]
	pub fn title(&self) -> &str {
		&self.title
	}
}

/// How many of the reader's pages converting `path` takes, where Paperback can tell before uploading, as Scribe counts them: a PDF's pages, or one for an image of one page. Scribe counts other files, such as Word documents or a TIFF of many pages, once it lays them out.
#[must_use]
pub fn pages_to_convert(path: &Path) -> Option<usize> {
	let extension = path.extension()?.to_str()?.to_ascii_lowercase();
	match extension.as_str() {
		"pdf" => crate::pdf_engine::Document::open(path, None).ok().map(|document| document.page_count()),
		"jpg" | "jpeg" | "gif" | "png" | "bmp" | "pcx" | "jp2" | "jpx" | "jpf" | "j2k" | "j2c" | "jpc" => Some(1),
		_ => None,
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

	fn two_page_pdf() -> Vec<u8> {
		use std::fmt::Write;
		let objects = [
			"<< /Type /Catalog /Pages 2 0 R >>",
			"<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >>",
			"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>",
			"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>",
		];
		let mut pdf = String::from("%PDF-1.4\n");
		let mut offsets = Vec::new();
		for (index, object) in objects.iter().enumerate() {
			offsets.push(pdf.len());
			write!(pdf, "{} 0 obj\n{object}\nendobj\n", index + 1).unwrap();
		}
		let (xref, size) = (pdf.len(), objects.len() + 1);
		write!(pdf, "xref\n0 {size}\n0000000000 65535 f \n").unwrap();
		for offset in offsets {
			writeln!(pdf, "{offset:010} 00000 n ").unwrap();
		}
		write!(pdf, "trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").unwrap();
		pdf.into_bytes()
	}

	#[test]
	fn a_pdf_costs_its_pages_and_an_image_one() {
		let pdf = std::env::temp_dir().join(format!("paperback-scribe-pages-{}.PDF", std::process::id()));
		std::fs::write(&pdf, two_page_pdf()).unwrap();
		let pages = pages_to_convert(&pdf);
		let _ = std::fs::remove_file(&pdf);
		assert_eq!(pages, Some(2));
		assert_eq!(pages_to_convert(Path::new("scan.jpeg")), Some(1));
	}

	#[test]
	fn what_scribe_lays_out_itself_has_no_count_yet() {
		assert_eq!(pages_to_convert(Path::new("Report.docx")), None);
		assert_eq!(pages_to_convert(Path::new("Fax.tiff")), None);
		assert_eq!(pages_to_convert(Path::new("Missing.pdf")), None);
	}

	fn summary(outputs: &serde_json::Value) -> DocumentSummary {
		serde_json::from_value(serde_json::json!({
			"id": "doc", "title": "Book", "page_count": 2, "inserted_at": "2026-11-27T03:53:52Z", "outputs": outputs
		}))
		.unwrap()
	}

	fn output(format: &str, stage: &str) -> serde_json::Value {
		serde_json::json!({"format": format, "stage": stage, "progress": 1.0, "estimated_time_remaining": null, "is_preview": false})
	}

	#[test]
	fn a_reopened_document_with_html_is_downloaded_rather_than_streamed() {
		assert!(Conversion::reopen(&summary(&serde_json::json!([output("html", "complete")]))).html_ready);
		assert!(!Conversion::reopen(&summary(&serde_json::json!([output("html_stream", "complete")]))).html_ready);
		assert!(!Conversion::reopen(&summary(&serde_json::json!([output("html", "convert")]))).html_ready);
	}

	#[test]
	fn pages_sent_again_are_whole_only_from_the_first_with_none_missing() {
		let mut conversion = Conversion::reopen(&summary(&serde_json::json!([])));
		assert!(!conversion.has_every_page());
		conversion.apply(chunk(Some(1), "b")).unwrap();
		assert!(!conversion.has_every_page(), "the first page is missing");
		conversion.apply(chunk(Some(0), "a")).unwrap();
		assert!(conversion.has_every_page());
	}

	#[test]
	fn finished_html_is_noted_for_downloading() {
		let mut conversion = Conversion::reopen(&summary(&serde_json::json!([])));
		let done = ChannelEvent::ConversionComplete { format: OutputFormat::Html, output_id: "o".to_string() };
		assert_eq!(conversion.apply(done).unwrap(), None);
		assert!(conversion.html_ready);
	}

	#[test]
	fn the_status_page_escapes_the_title() {
		let page = status_page("Tom & Jerry", "Uploading...");
		assert!(page.contains("<title>Tom &amp; Jerry</title>") && page.contains("<p>Uploading...</p>"), "{page}");
	}
}
