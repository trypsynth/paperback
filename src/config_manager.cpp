/* config_manager.cpp - manages reading from and writing to our INI-based config file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "config_manager.hpp"
#include "constants.hpp"
#include <Poco/Base64Encoder.h>
#include <Poco/DigestEngine.h>
#include <Poco/SHA1Engine.h>
#include <algorithm>
#include <sstream>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/tokenzr.h>

config_manager::~config_manager() {
	if (config) {
		shutdown();
	}
}

bool config_manager::initialize() {
	const wxString config_path = get_config_path();
	config = std::make_unique<wxFileConfig>(APP_NAME, "", config_path);
	if (!config) {
		return false;
	}
	if (!wxConfigBase::Get()) {
		wxConfigBase::Set(config.get());
		owns_global_config = true;
	}
	load_defaults();
	return true;
}

void config_manager::flush() {
	if (config) {
		config->Flush();
	}
}

void config_manager::shutdown() {
	if (config) {
		config->Flush();
		if (owns_global_config) {
			wxConfigBase::Set(nullptr);
			owns_global_config = false;
		}
		config.reset();
	}
}

wxString config_manager::get_string(const wxString& key, const wxString& default_value) const {
	return config ? config->Read(key, default_value) : default_value;
}

bool config_manager::get_bool(const wxString& key, bool default_value) const {
	return config ? config->ReadBool(key, default_value) : default_value;
}

int config_manager::get_int(const wxString& key, int default_value) const {
	return config ? config->ReadLong(key, default_value) : default_value;
}

void config_manager::set_string(const wxString& key, const wxString& value) {
	if (config) {
		config->Write(key, value);
	}
}

void config_manager::set_bool(const wxString& key, bool value) {
	if (config) {
		config->Write(key, value);
	}
}

void config_manager::set_int(const wxString& key, int value) {
	if (config) {
		config->Write(key, value);
	}
}

void config_manager::add_recent_document(const wxString& path) {
	if (!config) {
		return;
	}
	wxString doc_id = escape_document_path(path);
	with_document_section(path, [this, path]() {
		if (!config->HasEntry("path")) {
			config->Write("path", path);
		}
	});
	wxArrayString recent_paths = get_recent_documents();
	int existing_index = recent_paths.Index(path);
	if (existing_index != wxNOT_FOUND) {
		recent_paths.RemoveAt(existing_index);
	}
	recent_paths.Insert(path, 0);
	const int max_recent = 100;
	while (recent_paths.GetCount() > max_recent) {
		recent_paths.RemoveAt(recent_paths.GetCount() - 1);
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < recent_paths.GetCount(); ++i) {
		wxString path_doc_id = escape_document_path(recent_paths[i]);
		config->Write(wxString::Format("doc%zu", i), path_doc_id);
	}
	config->SetPath("/");
}

wxArrayString config_manager::get_recent_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < 100; ++i) {
		wxString key = wxString::Format("doc%zu", i);
		wxString doc_id = config->Read(key, "");
		if (doc_id.IsEmpty()) {
			break;
		}
		config->SetPath("/" + doc_id);
		wxString path = config->Read("path", "");
		if (!path.IsEmpty()) {
			result.Add(path);
		}
		config->SetPath("/recent_documents");
	}
	config->SetPath("/");
	return result;
}

void config_manager::clear_recent_documents() {
	if (config) {
		config->DeleteGroup("recent_documents");
	}
}

void config_manager::rebuild_recent_documents() {
	if (!config) {
		return;
	}
	wxArrayString current_recent = get_recent_documents();
	wxArrayString all_docs = get_all_documents();
	for (const auto& doc : all_docs) {
		if (current_recent.Index(doc) == wxNOT_FOUND) {
			current_recent.Add(doc);
		}
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < current_recent.GetCount(); ++i) {
		wxString path_doc_id = escape_document_path(current_recent[i]);
		config->Write(wxString::Format("doc%zu", i), path_doc_id);
	}
	config->SetPath("/");
}

int config_manager::get_recent_documents_to_show() const {
	int result = 25;
	with_app_section([this, &result]() {
		result = config->ReadLong("recent_documents_to_show", 25);
	});
	return result;
}

void config_manager::set_recent_documents_to_show(int count) {
	with_app_section([this, count]() {
		config->Write("recent_documents_to_show", count);
	});
}

bool config_manager::get_restore_previous_documents() const {
	bool result = true;
	with_app_section([this, &result]() {
		result = config->ReadBool("restore_previous_documents", true);
	});
	return result;
}

void config_manager::set_restore_previous_documents(bool restore) {
	with_app_section([this, restore]() {
		config->Write("restore_previous_documents", restore);
	});
}

bool config_manager::get_word_wrap() const {
	bool result = false;
	with_app_section([this, &result]() {
		result = config->ReadBool("word_wrap", false);
	});
	return result;
}

void config_manager::set_word_wrap(bool word_wrap) {
	with_app_section([this, word_wrap]() {
		config->Write("word_wrap", word_wrap);
	});
}

bool config_manager::get_minimize_to_tray() const {
	bool result = false;
	with_app_section([this, &result]() {
		result = config->ReadBool("minimize_to_tray", false);
	});
	return result;
}

void config_manager::set_minimize_to_tray(bool minimize) {
	with_app_section([this, minimize]() {
		config->Write("minimize_to_tray", minimize);
	});
}

wxString config_manager::get_language() const {
	wxString result = "";
	with_app_section([this, &result]() {
		result = config->Read("language", "");
	});
	return result;
}

void config_manager::set_language(const wxString& language) {
	with_app_section([this, language]() {
		config->Write("language", language);
	});
}

int config_manager::get_config_version() const {
	int version = CONFIG_VERSION_LEGACY;
	with_app_section([this, &version]() {
		version = config->ReadLong("version", CONFIG_VERSION_LEGACY);
	});
	return version;
}

void config_manager::set_config_version(int version) {
	with_app_section([this, version]() {
		config->Write("version", version);
	});
}

void config_manager::set_active_document(const wxString& path) {
	with_app_section([this, path]() {
		config->Write("active_document", path);
	});
}

wxString config_manager::get_active_document() const {
	wxString active_doc = "";
	with_app_section([this, &active_doc]() {
		active_doc = config->Read("active_document", "");
	});
	return active_doc;
}

void config_manager::add_opened_document(const wxString& path) {
	if (!config) {
		return;
	}
	wxArrayString opened = get_opened_documents();
	int existing_index = opened.Index(path);
	if (existing_index == wxNOT_FOUND) {
		opened.Add(path);
		config->DeleteGroup("opened_documents");
		config->SetPath("/opened_documents");
		for (size_t i = 0; i < opened.GetCount(); ++i) {
			config->Write(wxString::Format("File%zu", i), opened[i]);
		}
		config->SetPath("/");
	}
}

void config_manager::remove_opened_document(const wxString& path) {
	if (!config) {
		return;
	}
	wxArrayString opened = get_opened_documents();
	int existing_index = opened.Index(path);
	if (existing_index != wxNOT_FOUND) {
		opened.RemoveAt(existing_index);
		config->DeleteGroup("opened_documents");
		config->SetPath("/opened_documents");
		for (size_t i = 0; i < opened.GetCount(); ++i) {
			config->Write(wxString::Format("File%zu", i), opened[i]);
		}
		config->SetPath("/");
	}
}

wxArrayString config_manager::get_opened_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/opened_documents");
	wxString key;
	long index;
	bool cont = config->GetFirstEntry(key, index);
	while (cont) {
		wxString path = config->Read(key, "");
		if (!path.IsEmpty()) {
			result.Add(path);
		}
		cont = config->GetNextEntry(key, index);
	}
	config->SetPath("/");
	return result;
}

void config_manager::clear_opened_documents() {
	if (config) {
		config->DeleteGroup("opened_documents");
	}
}

void config_manager::set_document_position(const wxString& path, long position) {
	with_document_section(path, [this, path, position]() {
		config->Write("path", path);
		config->Write("last_position", position);
	});
}

long config_manager::get_document_position(const wxString& path) const {
	long position = 0;
	with_document_section(path, [this, &position]() {
		position = config->ReadLong("last_position", 0);
	});
	return position;
}

void config_manager::set_document_opened(const wxString& path, bool opened) {
	with_document_section(path, [this, path, opened]() {
		config->Write("path", path);
		config->Write("opened", opened);
	});
}

bool config_manager::get_document_opened(const wxString& path) const {
	bool opened = false;
	with_document_section(path, [this, &opened]() {
		opened = config->ReadBool("opened", false);
	});
	return opened;
}

wxArrayString config_manager::get_all_opened_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/");
	wxString group;
	long index;
	bool cont = config->GetFirstGroup(group, index);
	while (cont) {
		if (group.StartsWith("doc_")) {
			config->SetPath("/" + group);
			if (config->ReadBool("opened", false)) {
				wxString path = config->Read("path", "");
				if (!path.IsEmpty()) {
					result.Add(path);
				}
			}
			config->SetPath("/");
		}
		cont = config->GetNextGroup(group, index);
	}
	config->SetPath("/");
	return result;
}

void config_manager::remove_document_history(const wxString& path) {
	if (!config) {
		return;
	}
	wxArrayString recent_paths = get_recent_documents();
	int existing_index = recent_paths.Index(path);
	if (existing_index != wxNOT_FOUND) {
		recent_paths.RemoveAt(existing_index);
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < recent_paths.GetCount(); ++i) {
		wxString path_doc_id = escape_document_path(recent_paths[i]);
		config->Write(wxString::Format("doc%zu", i), path_doc_id);
	}
	config->SetPath("/");
	wxString doc_id_to_remove = escape_document_path(path);
	config->DeleteGroup(doc_id_to_remove);
}

wxArrayString config_manager::get_all_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/");
	wxString group;
	long index;
	bool cont = config->GetFirstGroup(group, index);
	while (cont) {
		if (group.StartsWith("doc_")) {
			config->SetPath("/" + group);
			wxString path = config->Read("path", "");
			if (!path.IsEmpty()) {
				result.Add(path);
			}
			config->SetPath("/");
		}
		cont = config->GetNextGroup(group, index);
	}
	config->SetPath("/");
	return result;
}

void config_manager::add_bookmark(const wxString& path, long position) {
	if (!config) {
		return;
	}
	wxArrayLong bookmarks = get_bookmarks(path);
	if (bookmarks.Index(position) == wxNOT_FOUND) {
		bookmarks.Add(position);
		bookmarks.Sort([](long a, long b) { return a < b ? -1 : (a > b ? 1 : 0); });
		wxString bookmark_string = "";
		for (size_t i = 0; i < bookmarks.GetCount(); ++i) {
			if (i > 0) {
				bookmark_string += ",";
			}
			bookmark_string += wxString::Format("%ld", bookmarks[i]);
		}
		with_document_section(path, [this, path, bookmark_string]() {
			config->Write("path", path);
			config->Write("bookmarks", bookmark_string);
		});
	}
}

void config_manager::remove_bookmark(const wxString& path, long position) {
	if (!config) {
		return;
	}
	wxArrayLong bookmarks = get_bookmarks(path);
	int index = bookmarks.Index(position);
	if (index != wxNOT_FOUND) {
		bookmarks.RemoveAt(index);
		wxString bookmark_string = "";
		for (size_t i = 0; i < bookmarks.GetCount(); ++i) {
			if (i > 0) {
				bookmark_string += ",";
			}
			bookmark_string += wxString::Format("%ld", bookmarks[i]);
		}
		with_document_section(path, [this, path, bookmark_string]() {
			config->Write("path", path);
			if (bookmark_string.IsEmpty()) {
				config->DeleteEntry("bookmarks");
			} else {
				config->Write("bookmarks", bookmark_string);
			}
		});
	}
}

void config_manager::toggle_bookmark(const wxString& path, long position) {
	wxArrayLong bookmarks = get_bookmarks(path);
	if (bookmarks.Index(position) != wxNOT_FOUND) {
		remove_bookmark(path, position);
	} else {
		add_bookmark(path, position);
	}
}

wxArrayLong config_manager::get_bookmarks(const wxString& path) const {
	wxArrayLong result;
	if (!config) {
		return result;
	}
	wxString bookmark_string = "";
	with_document_section(path, [this, &bookmark_string]() {
		bookmark_string = config->Read("bookmarks", "");
	});
	if (!bookmark_string.IsEmpty()) {
		wxStringTokenizer tokenizer(bookmark_string, ",");
		while (tokenizer.HasMoreTokens()) {
			wxString token = tokenizer.GetNextToken().Trim().Trim(false);
			long position;
			if (token.ToLong(&position)) {
				result.Add(position);
			}
		}
		result.Sort([](long a, long b) { return a < b ? -1 : (a > b ? 1 : 0); });
	}
	return result;
}

void config_manager::clear_bookmarks(const wxString& path) {
	if (!config) {
		return;
	}
	with_document_section(path, [this]() {
		config->DeleteEntry("bookmarks");
	});
}

long config_manager::get_next_bookmark(const wxString& path, long current_position) const {
	wxArrayLong bookmarks = get_bookmarks(path);
	for (size_t i = 0; i < bookmarks.GetCount(); ++i) {
		if (bookmarks[i] > current_position) {
			return bookmarks[i];
		}
	}
	return -1;
}

long config_manager::get_previous_bookmark(const wxString& path, long current_position) const {
	wxArrayLong bookmarks = get_bookmarks(path);
	for (int i = bookmarks.GetCount() - 1; i >= 0; --i) {
		if (bookmarks[i] < current_position) {
			return bookmarks[i];
		}
	}
	return -1;
}

long config_manager::get_closest_bookmark(const wxString& path, long current_position) const {
	wxArrayLong bookmarks = get_bookmarks(path);
	if (bookmarks.IsEmpty()) {
		return -1;
	}
	long closest = bookmarks[0];
	long min_distance = std::abs(closest - current_position);
	for (size_t i = 1; i < bookmarks.GetCount(); ++i) {
		long distance = std::abs(bookmarks[i] - current_position);
		if (distance < min_distance) {
			min_distance = distance;
			closest = bookmarks[i];
		}
	}
	return closest;
}

void config_manager::set_document_format(const wxString& path, const wxString& format) {
	with_document_section(path, [this, path, format]() {
		config->Write("path", path);
		config->Write("format", format);
	});
}

wxString config_manager::get_document_format(const wxString& path) const {
	wxString format = "";
	with_document_section(path, [this, &format]() {
		format = config->Read("format", "");
	});
	return format;
}

bool config_manager::needs_migration() const {
	if (!config) {
		return false;
	}
	if (get_config_version() == CONFIG_VERSION_CURRENT) {
		return false;
	}
	config->SetPath("/positions");
	wxString key;
	long index;
	bool has_old_positions = config->GetFirstEntry(key, index);
	config->SetPath("/");
	bool has_old_globals = config->HasEntry("restore_previous_documents") || config->HasEntry("word_wrap");
	bool has_old_opened = config->HasGroup("opened_documents");
	return has_old_positions || has_old_globals || has_old_opened;
}

bool config_manager::migrate_config() {
	if (!config) {
		return false;
	}
	config->SetPath("/");
	bool restore_docs = config->ReadBool("restore_previous_documents", true);
	bool word_wrap = config->ReadBool("word_wrap", false);
	config->SetPath("/app");
	if (!config->HasEntry("restore_previous_documents")) {
		config->Write("restore_previous_documents", restore_docs);
	}
	if (!config->HasEntry("word_wrap")) {
		config->Write("word_wrap", word_wrap);
	}
	config->SetPath("/positions");
	wxString key;
	long index;
	bool cont = config->GetFirstEntry(key, index);
	while (cont) {
		long position = config->ReadLong(key, 0);
		if (position > 0) {
			set_document_position(key, position);
		}
		cont = config->GetNextEntry(key, index);
	}
	config->SetPath("/recent_documents");
	wxString recent_key;
	long recent_index;
	bool recent_cont = config->GetFirstEntry(recent_key, recent_index);
	wxArrayString old_recent_paths;
	while (recent_cont) {
		wxString path = config->Read(recent_key, "");
		if (!path.IsEmpty()) {
			old_recent_paths.Add(path);
		}
		recent_cont = config->GetNextEntry(recent_key, recent_index);
	}
	config->SetPath("/");
	config->DeleteGroup("recent_documents");
	for (const auto& path : old_recent_paths) {
		add_recent_document(path);
	}
	config->SetPath("/opened_documents");
	wxString opened_key;
	long opened_index;
	bool opened_cont = config->GetFirstEntry(opened_key, opened_index);
	wxArrayString old_opened_paths;
	while (opened_cont) {
		wxString path = config->Read(opened_key, "");
		if (!path.IsEmpty()) {
			old_opened_paths.Add(path);
		}
		opened_cont = config->GetNextEntry(opened_key, opened_index);
	}
	config->SetPath("/");
	for (const auto& path : old_opened_paths) {
		set_document_opened(path, true);
	}
	config->SetPath("/");
	config->DeleteGroup("positions");
	config->DeleteEntry("restore_previous_documents");
	config->DeleteEntry("word_wrap");
	config->DeleteGroup("opened_documents");
	return true;
}

wxString config_manager::get_config_path() const {
	const wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	const wxString exe_dir = wxFileName(exe_path).GetPath();
	if (is_directory_writable(exe_dir)) {
		return exe_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
	}
	wxString appdata_dir = wxStandardPaths::Get().GetUserDataDir();
	if (!wxFileName::DirExists(appdata_dir)) {
		wxFileName::Mkdir(appdata_dir, wxS_DIR_DEFAULT, wxPATH_MKDIR_FULL);
	}
	return appdata_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
}

bool config_manager::is_directory_writable(const wxString& dir) const {
	wxFileName fn(dir, wxEmptyString);
	return fn.IsDirWritable();
}

void config_manager::load_defaults() {
	if (needs_migration()) {
		migrate_config();
	}
	config->SetPath("/app");
	if (!config->HasEntry("restore_previous_documents")) {
		config->Write("restore_previous_documents", true);
	}
	if (!config->HasEntry("word_wrap")) {
		config->Write("word_wrap", false);
	}
	if (!config->HasEntry("minimize_to_tray")) {
		config->Write("minimize_to_tray", false);
	}
	if (!config->HasEntry("recent_documents_to_show")) {
		config->Write("recent_documents_to_show", 25);
	}
	if (get_config_version() != CONFIG_VERSION_CURRENT) {
		set_config_version(CONFIG_VERSION_CURRENT);
	}
	config->SetPath("/");
	rebuild_recent_documents();
}

wxString config_manager::get_document_section(const wxString& path) const {
	return "/" + escape_document_path(path);
}

wxString config_manager::escape_document_path(const wxString& path) const {
	Poco::SHA1Engine sha1;
	sha1.update(path.ToStdString());
	const Poco::DigestEngine::Digest& digest = sha1.digest();
	std::ostringstream b64_stream;
	Poco::Base64Encoder encoder(b64_stream, Poco::BASE64_URL_ENCODING | Poco::BASE64_NO_PADDING);
	encoder.write(reinterpret_cast<const char*>(digest.data()), digest.size());
	encoder.close();
	return wxString::Format("doc_%s", b64_stream.str());
}

void config_manager::with_document_section(const wxString& path, std::function<void()> func) const {
	if (!config) {
		return;
	}
	wxString section = get_document_section(path);
	config->SetPath(section);
	func();
	config->SetPath("/");
}

void config_manager::with_app_section(std::function<void()> func) const {
	if (!config) {
		return;
	}
	config->SetPath("/app");
	func();
	config->SetPath("/");
}
