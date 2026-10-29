use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use super::ConfigManager;
use crate::{
	config::settings::{DocumentSpeech, StoredBookmark},
	document::DocPos,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(super) struct SidecarData {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	last_position: Option<DocPos>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	format: Option<String>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	bookmarks: Vec<StoredBookmark>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	temporary_bookmark: Option<DocPos>,
	#[serde(default, skip_serializing_if = "DocumentSpeech::is_empty")]
	speech: DocumentSpeech,
}

impl ConfigManager {
	pub fn import_document_settings(&self, path: &str) {
		let import_path = Path::new(path).with_extension("paperback");
		if let Some(import_path_str) = import_path.to_str()
			&& import_path.exists()
		{
			self.import_settings_from_file(path, import_path_str);
		}
	}

	pub fn import_settings_from_file(&self, doc_path: &str, import_path: &str) {
		if !self.initialized || !Path::new(import_path).exists() {
			return;
		}
		let Ok(content) = fs::read_to_string(import_path) else { return };
		let Ok(sidecar) = toml::from_str::<SidecarData>(&content) else { return };
		if let Some(pos) = sidecar.last_position {
			self.set_document_position(doc_path, pos);
		}
		if let Some(format) = sidecar.format {
			self.set_document_format(doc_path, &format);
		}
		if let Some(position) = sidecar.temporary_bookmark {
			self.set_temporary_bookmark(doc_path, Some(position));
		}
		if !sidecar.speech.is_empty() {
			self.set_document_speech(doc_path, &sidecar.speech);
		}
		if !sidecar.bookmarks.is_empty() {
			let key = self.get_doc_key(doc_path);
			let mut data = self.data.borrow_mut();
			Self::doc_entry_mut(&mut data, key, doc_path).bookmarks = sidecar.bookmarks;
			self.dirty.set(true);
		}
	}

	pub fn export_document_settings(&self, doc_path: &str, export_path: &str) {
		if !self.initialized {
			return;
		}
		let key = self.get_doc_key(doc_path);
		let data = self.data.borrow();
		let doc = data.documents.get(&key);
		let sidecar = SidecarData {
			last_position: doc.map(|d| d.last_position).filter(|&p| p > DocPos::ZERO),
			format: doc.and_then(|d| if d.format.is_empty() { None } else { Some(d.format.clone()) }),
			bookmarks: doc.map(|d| d.bookmarks.clone()).unwrap_or_default(),
			temporary_bookmark: doc.and_then(|d| d.temporary_bookmark),
			speech: doc.map(|d| d.speech.clone()).unwrap_or_default(),
		};
		if let Ok(s) = toml::to_string_pretty(&sidecar) {
			let _ = fs::write(export_path, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::test_support::TempDir;

	fn config() -> ConfigManager {
		let mut config = ConfigManager::new();
		config.initialized = true;
		config
	}

	#[test]
	fn a_documents_own_speech_travels_in_its_exported_settings() {
		let dir = TempDir::new("sidecar-speech");
		let file = dir.join_str("book.paperback");
		let speech = DocumentSpeech { voice: Some("voice".into()), rate_percent: Some(40), pitch_percent: Some(70) };
		let from = config();
		from.set_document_speech("book.epub", &speech);
		from.export_document_settings("book.epub", &file);
		let to = config();
		to.import_settings_from_file("book.epub", &file);
		assert_eq!(to.get_document_speech("book.epub"), speech);
	}
}
