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
#include <Poco/Base64Decoder.h>
#include <Poco/Base64Encoder.h>
#include <Poco/DigestEngine.h>
#include <Poco/SHA1Engine.h>
#include <algorithm>
#include <cmath>
#include <functional>
#include <ios>
#include <sstream>
#include <wx/filefn.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>
#include <wx/string.h>
#include <wx/tokenzr.h>
#include <wx/utils.h>

namespace {
inline bool read_config_value(wxFileConfig* cfg, const wxString& key, bool default_val) {
	return cfg->ReadBool(key, default_val);
}

inline long read_config_value(wxFileConfig* cfg, const wxString& key, long default_val) {
	return cfg->ReadLong(key, default_val);
}

inline int read_config_value(wxFileConfig* cfg, const wxString& key, int default_val) {
	return static_cast<int>(cfg->ReadLong(key, default_val));
}

inline wxString read_config_value(wxFileConfig* cfg, const wxString& key, const wxString& default_val) {
	return cfg->Read(key, default_val);
}
} // namespace

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
	if (wxConfigBase::Get() == nullptr) {
		wxConfigBase::Set(config.get());
		owns_global_config = true;
	}
	load_defaults();
	return true;
}

void config_manager::flush() {
	if (!config) {
		return;
	}
	config->Flush();
}

void config_manager::shutdown() {
	if (!config) {
		return;
	}
	config->Flush();
	if (owns_global_config) {
		wxConfigBase::Set(nullptr);
		owns_global_config = false;
	}
	config.reset();
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
	if (!config) {
		return;
	}
	config->Write(key, value);
}

void config_manager::set_bool(const wxString& key, bool value) {
	if (!config) {
		return;
	}
	config->Write(key, value);
}

void config_manager::set_int(const wxString& key, int value) {
	if (!config) {
		return;
	}
	config->Write(key, value);
}

template <typename T>
T config_manager::get_app_setting(const wxString& key, const T& default_value) const {
	T result = default_value;
	with_app_section([this, &key, &default_value, &result]() {
		result = read_config_value(config.get(), key, default_value);
	});
	return result;
}

template <typename T>
void config_manager::set_app_setting(const wxString& key, const T& value) {
	with_app_section([this, &key, &value]() {
		config->Write(key, value);
	});
}

template <typename T>
T config_manager::get_document_setting(const wxString& path, const wxString& key, const T& default_value) const {
	T result = default_value;
	with_document_section(path, [this, &key, &default_value, &result]() {
		result = read_config_value(config.get(), key, default_value);
	});
	return result;
}

template <typename T>
void config_manager::set_document_setting(const wxString& path, const wxString& key, const T& value) {
	with_document_section(path, [this, &path, &key, &value]() {
		config->Write("path", path);
		config->Write(key, value);
	});
}

void config_manager::add_recent_document(const wxString& path) {
	if (!config) {
		return;
	}
	const wxString doc_id = escape_document_path(path);
	with_document_section(path, [this, path]() {
		if (!config->HasEntry("path")) {
			config->Write("path", path);
		}
	});
	wxArrayString recent_paths = get_recent_documents();
	const int existing_index = recent_paths.Index(path);
	if (existing_index != wxNOT_FOUND) {
		recent_paths.RemoveAt(existing_index);
	}
	recent_paths.Insert(path, 0);
	while (recent_paths.GetCount() > MAX_RECENT_DOCUMENTS_TO_SHOW) {
		recent_paths.RemoveAt(recent_paths.GetCount() - 1);
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < recent_paths.GetCount(); ++i) {
		const wxString path_doc_id = escape_document_path(recent_paths[i]);
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
	for (size_t i = 0; i < MAX_RECENT_DOCUMENTS_TO_SHOW; ++i) {
		const wxString key = wxString::Format("doc%zu", i);
		const wxString doc_id = config->Read(key, "");
		if (doc_id.IsEmpty()) {
			break;
		}
		config->SetPath("/" + doc_id);
		const wxString path = config->Read("path", "");
		if (!path.IsEmpty()) {
			result.Add(path);
		}
		config->SetPath("/recent_documents");
	}
	config->SetPath("/");
	return result;
}

void config_manager::clear_recent_documents() {
	if (!config) {
		return;
	}
	config->DeleteGroup("recent_documents");
}

void config_manager::rebuild_recent_documents() {
	if (!config) {
		return;
	}
	wxArrayString current_recent = get_recent_documents();
	const wxArrayString all_docs = get_all_documents();
	for (const auto& doc : all_docs) {
		if (current_recent.Index(doc) == wxNOT_FOUND) {
			current_recent.Add(doc);
		}
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < current_recent.GetCount(); ++i) {
		const wxString path_doc_id = escape_document_path(current_recent[i]);
		config->Write(wxString::Format("doc%zu", i), path_doc_id);
	}
	config->SetPath("/");
}

void config_manager::add_opened_document(const wxString& path) {
	if (!config) {
		return;
	}
	wxArrayString opened = get_opened_documents();
	const int existing_index = opened.Index(path);
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
	const int existing_index = opened.Index(path);
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
	long index{0};
	bool cont = config->GetFirstEntry(key, index);
	while (cont) {
		const wxString path = config->Read(key, "");
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
	set_document_setting(path, "last_position", position);
}

long config_manager::get_document_position(const wxString& path) const {
	return get_document_setting(path, "last_position", 0L);
}

void config_manager::set_document_opened(const wxString& path, bool opened) {
	set_document_setting(path, "opened", opened);
}

bool config_manager::get_document_opened(const wxString& path) const {
	return get_document_setting(path, "opened", false);
}

wxArrayString config_manager::get_all_opened_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/");
	wxString group;
	long index{0};
	bool cont = config->GetFirstGroup(group, index);
	while (cont) {
		if (group.StartsWith("doc_")) {
			config->SetPath("/" + group);
			if (config->ReadBool("opened", false)) {
				const wxString path = config->Read("path", "");
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
	const int existing_index = recent_paths.Index(path);
	if (existing_index != wxNOT_FOUND) {
		recent_paths.RemoveAt(existing_index);
	}
	config->DeleteGroup("recent_documents");
	config->SetPath("/recent_documents");
	for (size_t i = 0; i < recent_paths.GetCount(); ++i) {
		const wxString path_doc_id = escape_document_path(recent_paths[i]);
		config->Write(wxString::Format("doc%zu", i), path_doc_id);
	}
	config->SetPath("/");
	const wxString doc_id_to_remove = escape_document_path(path);
	config->DeleteGroup(doc_id_to_remove);
}

wxArrayString config_manager::get_all_documents() const {
	wxArrayString result;
	if (!config) {
		return result;
	}
	config->SetPath("/");
	wxString group;
	long index{0};
	bool cont = config->GetFirstGroup(group, index);
	while (cont) {
		if (group.StartsWith("doc_")) {
			config->SetPath("/" + group);
			const wxString path = config->Read("path", "");
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

void config_manager::add_bookmark(const wxString& path, long start, long end, const wxString& note) {
	if (!config) {
		return;
	}
	std::vector<bookmark> bookmarks = get_bookmarks(path);
	bookmark new_bookmark(start, end, note);
	bool exists = false;
	for (const auto& bm : bookmarks) {
		if (bm == new_bookmark) {
			exists = true;
			break;
		}
	}
	if (exists) {
		return;
	}
	bookmarks.push_back(new_bookmark);
	std::sort(bookmarks.begin(), bookmarks.end(), [](const bookmark& a, const bookmark& b) {
		return a.start < b.start;
	});
	wxString bookmark_string;
	for (size_t i = 0; i < bookmarks.size(); ++i) {
		if (i > 0) {
			bookmark_string += ",";
		}
		const wxString encoded_note = encode_note(bookmarks[i].note);
		bookmark_string += wxString::Format("%ld:%ld:%s", bookmarks[i].start, bookmarks[i].end, encoded_note);
	}
	with_document_section(path, [this, path, bookmark_string]() {
		config->Write("path", path);
		config->Write("bookmarks", bookmark_string);
	});
}

void config_manager::remove_bookmark(const wxString& path, long start, long end) {
	if (!config) {
		return;
	}
	std::vector<bookmark> bookmarks = get_bookmarks(path);
	bookmark to_remove(start, end);
	auto it = std::find(bookmarks.begin(), bookmarks.end(), to_remove);
	if (it == bookmarks.end()) {
		return;
	}
	bookmarks.erase(it);
	wxString bookmark_string;
	for (size_t i = 0; i < bookmarks.size(); ++i) {
		if (i > 0) {
			bookmark_string += ",";
		}
		const wxString encoded_note = encode_note(bookmarks[i].note);
		bookmark_string += wxString::Format("%ld:%ld:%s", bookmarks[i].start, bookmarks[i].end, encoded_note);
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

void config_manager::toggle_bookmark(const wxString& path, long start, long end, const wxString& note) {
	std::vector<bookmark> bookmarks = get_bookmarks(path);
	bookmark to_toggle(start, end);
	bool exists = false;
	for (const auto& bm : bookmarks) {
		if (bm == to_toggle) {
			exists = true;
			break;
		}
	}
	if (exists) {
		remove_bookmark(path, start, end);
	} else {
		add_bookmark(path, start, end, note);
	}
}

void config_manager::update_bookmark_note(const wxString& path, long start, long end, const wxString& note) {
	if (!config) {
		return;
	}
	std::vector<bookmark> bookmarks = get_bookmarks(path);
	bool found = false;
	for (auto& bm : bookmarks) {
		if (bm.start == start && bm.end == end) {
			bm.note = note;
			found = true;
			break;
		}
	}
	if (!found) {
		return;
	}
	wxString bookmark_string;
	for (size_t i = 0; i < bookmarks.size(); ++i) {
		if (i > 0) {
			bookmark_string += ",";
		}
		const wxString encoded_note = encode_note(bookmarks[i].note);
		bookmark_string += wxString::Format("%d:%d:%s", bookmarks[i].start, bookmarks[i].end, encoded_note);
	}
	with_document_section(path, [this, path, bookmark_string]() {
		config->Write("path", path);
		config->Write("bookmarks", bookmark_string);
	});
}

std::vector<bookmark> config_manager::get_bookmarks(const wxString& path) const {
	std::vector<bookmark> result;
	if (!config) {
		return result;
	}
	const wxString bookmark_string = [this, &path]() {
		wxString str;
		with_document_section(path, [this, &str]() {
			str = config->Read("bookmarks", "");
		});
		return str;
	}();
	if (bookmark_string.IsEmpty()) {
		return result;
	}
	wxStringTokenizer tokenizer(bookmark_string, ",");
	while (tokenizer.HasMoreTokens()) {
		const wxString token = tokenizer.GetNextToken().Trim().Trim(false);
		if (token.Contains(":")) {
			wxStringTokenizer pair_tokenizer(token, ":");
			if (pair_tokenizer.HasMoreTokens()) {
				wxString start_str = pair_tokenizer.GetNextToken();
				if (pair_tokenizer.HasMoreTokens()) {
					wxString end_str = pair_tokenizer.GetNextToken();
					wxString note_str;
					if (pair_tokenizer.HasMoreTokens()) {
						note_str = pair_tokenizer.GetNextToken();
					}
					int start{0};
					int end{0};
					if (start_str.ToInt(&start) && end_str.ToInt(&end)) {
						wxString decoded_note = decode_note(note_str);
						result.push_back(bookmark(start, end, decoded_note));
					}
				}
			}
		} else {
			// Backward compatibility. This shouldn't happen after migration, but handle it gracefully anyway.
			int position{0};
			if (token.ToInt(&position)) {
				result.push_back(bookmark(position, position, wxEmptyString));
			}
		}
	}
	std::sort(result.begin(), result.end(), [](const bookmark& a, const bookmark& b) {
		return a.start < b.start;
	});
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

bookmark config_manager::get_next_bookmark(const wxString& path, long current_position) const {
	const auto& bookmarks = get_bookmarks(path);
	for (const auto& bm : bookmarks) {
		if (bm.start > current_position) {
			return bm;
		}
	}
	return {-1, -1};
}

bookmark config_manager::get_previous_bookmark(const wxString& path, long current_position) const {
	const auto& bookmarks = get_bookmarks(path);
	for (auto it = bookmarks.rbegin(); it != bookmarks.rend(); ++it) {
		if (it->start < current_position) {
			return *it;
		}
	}
	return {-1, -1};
}

bookmark config_manager::get_closest_bookmark(const wxString& path, long current_position) const {
	const auto& bookmarks = get_bookmarks(path);
	if (bookmarks.empty()) {
		return {-1, -1};
	}
	const auto* closest = &bookmarks.front();
	long min_distance = std::abs(closest->start - current_position);
	for (const auto& bm : bookmarks) {
		const long distance = std::abs(bm.start - current_position);
		if (distance < min_distance) {
			min_distance = distance;
			closest = &bm;
		}
	}
	return *closest;
}

void config_manager::set_document_format(const wxString& path, const wxString& format) {
	set_document_setting(path, "format", format);
}

wxString config_manager::get_document_format(const wxString& path) const {
	return get_document_setting(path, "format", wxString(""));
}

bool config_manager::needs_migration() const {
	if (!config) {
		return false;
	}
	const int version = get(config_version);
	if (version == CONFIG_VERSION_CURRENT) {
		return false;
	}
	config->SetPath("/positions");
	wxString key;
	long index{0};
	const bool has_old_positions = config->GetFirstEntry(key, index);
	config->SetPath("/");
	const bool has_old_globals = config->HasEntry("restore_previous_documents") || config->HasEntry("word_wrap");
	const bool has_old_opened = config->HasGroup("opened_documents");
	const bool needs_v1_to_v2 = (version == CONFIG_VERSION_1);
	return has_old_positions || has_old_globals || has_old_opened || needs_v1_to_v2;
}

bool config_manager::migrate_config() {
	if (!config) {
		return false;
	}
	const int version = get(config_version);
	if (version == CONFIG_VERSION_LEGACY) {
		config->SetPath("/");
		const bool restore_docs = config->ReadBool("restore_previous_documents", true);
		const bool wordwrap = config->ReadBool("word_wrap", false);
		config->SetPath("/app");
		if (!config->HasEntry("restore_previous_documents")) {
			config->Write("restore_previous_documents", restore_docs);
		}
		if (!config->HasEntry("word_wrap")) {
			config->Write("word_wrap", wordwrap);
		}
		config->SetPath("/positions");
		wxString key;
		long index{0};
		bool cont = config->GetFirstEntry(key, index);
		while (cont) {
			const long position = config->ReadLong(key, 0);
			if (position > 0) {
				set_document_position(key, position);
			}
			cont = config->GetNextEntry(key, index);
		}
		config->SetPath("/recent_documents");
		wxString recent_key;
		long recent_index{0};
		bool recent_cont = config->GetFirstEntry(recent_key, recent_index);
		wxArrayString old_recent_paths;
		while (recent_cont) {
			const wxString path = config->Read(recent_key, "");
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
		long opened_index{0};
		bool opened_cont = config->GetFirstEntry(opened_key, opened_index);
		wxArrayString old_opened_paths;
		while (opened_cont) {
			const wxString path = config->Read(opened_key, "");
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
	} else if (version == CONFIG_VERSION_1) {
		config->SetPath("/");
		wxString group;
		long group_index{0};
		bool cont = config->GetFirstGroup(group, group_index);
		while (cont) {
			if (group.StartsWith("doc_")) {
				config->SetPath("/" + group);
				const wxString old_bookmarks = config->Read("bookmarks", "");
				if (!old_bookmarks.IsEmpty()) {
					wxString new_bookmarks;
					wxStringTokenizer tokenizer(old_bookmarks, ",");
					bool first = true;
					while (tokenizer.HasMoreTokens()) {
						const wxString token = tokenizer.GetNextToken().Trim().Trim(false);
						if (!token.Contains(":")) {
							long position{0};
							if (token.ToLong(&position)) {
								if (!first) {
									new_bookmarks += ",";
								}
								new_bookmarks += wxString::Format("%ld:%ld:", position, position);
								first = false;
							}
						} else {
							int colon_count{token.Freq(':')};
							if (!first) {
								new_bookmarks += ",";
							}
							if (colon_count == 1) {
								new_bookmarks += token + ":";
							} else {
								new_bookmarks += token;
							}
							first = false;
						}
					}
					if (!new_bookmarks.IsEmpty()) {
						config->Write("bookmarks", new_bookmarks);
					}
				}
				config->SetPath("/");
			}
			cont = config->GetNextGroup(group, group_index);
		}
	}
	set(config_version, static_cast<int>(CONFIG_VERSION_CURRENT));
	return true;
}

void config_manager::export_document_settings(const wxString& doc_path, const wxString& export_path) {
	if (!config) {
		return;
	}
	const wxString doc_section = get_document_section(doc_path);
	wxFileConfig export_config(APP_NAME, "", export_path, wxEmptyString, wxCONFIG_USE_LOCAL_FILE);
	config->SetPath(doc_section);
	export_config.DeleteGroup("/");
	long index{0};
	wxString key;
	bool cont = config->GetFirstEntry(key, index);
	while (cont) {
		if (key != "path") {
			export_config.Write(key, config->Read(key, ""));
		}
		cont = config->GetNextEntry(key, index);
	}
	config->SetPath("/");
	export_config.Flush();
}

void config_manager::import_document_settings(const wxString& path) {
	if (!config) {
		return;
	}
	const wxString import_path = path + ".paperback";
	if (!wxFileName::FileExists(import_path)) {
		return;
	}
	import_settings_from_file(path, import_path);
}

void config_manager::import_settings_from_file(const wxString& doc_path, const wxString& import_path) {
	if (!config || !wxFileName::FileExists(import_path)) {
		return;
	}
	wxFileConfig import_config(APP_NAME, "", import_path, wxEmptyString, wxCONFIG_USE_LOCAL_FILE);
	const wxString doc_section = get_document_section(doc_path);
	config->SetPath(doc_section);
	long index{0};
	wxString key;
	bool cont = import_config.GetFirstEntry(key, index);
	while (cont) {
		config->Write(key, import_config.Read(key, ""));
		cont = import_config.GetNextEntry(key, index);
	}
	config->Write("path", doc_path);
	config->SetPath("/");
	config->Flush();
}

wxString config_manager::get_config_path() {
	const wxString exe_path = wxStandardPaths::Get().GetExecutablePath();
	const wxString exe_dir = wxFileName(exe_path).GetPath();
#ifdef __WXMSW__
	bool force_appdata = false;
	wxString program_files_path;
	wxGetEnv("ProgramFiles", &program_files_path);
	wxString program_files_x86_path;
	wxGetEnv("ProgramFiles(x86)", &program_files_x86_path);
	if ((!program_files_path.IsEmpty() && exe_path.StartsWith(program_files_path)) || (!program_files_x86_path.IsEmpty() && exe_path.StartsWith(program_files_x86_path))) {
		force_appdata = true;
	}
	if (!force_appdata && is_directory_writable(exe_dir)) {
		return exe_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
	}
#else
	if (is_directory_writable(exe_dir)) {
		return exe_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
	}
#endif
	const wxString appdata_dir = wxStandardPaths::Get().GetUserDataDir();
	if (!wxFileName::DirExists(appdata_dir)) {
		wxFileName::Mkdir(appdata_dir, wxS_DIR_DEFAULT, wxPATH_MKDIR_FULL);
	}
	return appdata_dir + wxFileName::GetPathSeparator() + APP_NAME + ".ini";
}

bool config_manager::is_directory_writable(const wxString& dir) {
	const wxFileName fn(dir, wxEmptyString);
	return fn.IsDirWritable();
}

void config_manager::load_defaults() {
	if (needs_migration()) {
		migrate_config();
	}
	auto set_default_if_missing = [this](const auto& setting) {
		config->SetPath("/app");
		if (!config->HasEntry(setting.key)) {
			config->Write(setting.key, setting.default_value);
		}
		config->SetPath("/");
	};
	set_default_if_missing(restore_previous_documents);
	set_default_if_missing(word_wrap);
	set_default_if_missing(minimize_to_tray);
	set_default_if_missing(compact_go_menu);
	set_default_if_missing(navigation_wrap);
	set_default_if_missing(check_for_updates_on_startup);
	set_default_if_missing(recent_documents_to_show);
	set_default_if_missing(sleep_timer_duration);
	if (get(config_version) != CONFIG_VERSION_CURRENT) {
		set(config_version, static_cast<int>(CONFIG_VERSION_CURRENT));
	}
	rebuild_recent_documents();
}

wxString config_manager::get_document_section(const wxString& path) {
	return "/" + escape_document_path(path);
}

wxString config_manager::escape_document_path(const wxString& path) {
	Poco::SHA1Engine sha1;
	sha1.update(path.ToStdString());
	const Poco::DigestEngine::Digest& digest = sha1.digest();
	std::ostringstream b64_stream;
	Poco::Base64Encoder encoder(b64_stream, static_cast<unsigned>(Poco::BASE64_URL_ENCODING) | static_cast<unsigned>(Poco::BASE64_NO_PADDING));
	encoder.write(reinterpret_cast<const char*>(digest.data()), static_cast<std::streamsize>(digest.size()));
	encoder.close();
	return wxString::Format("doc_%s", b64_stream.str());
}

void config_manager::with_document_section(const wxString& path, const std::function<void()>& func) const {
	if (!config) {
		return;
	}
	const wxString section = get_document_section(path);
	config->SetPath(section);
	func();
	config->SetPath("/");
}

void config_manager::with_app_section(const std::function<void()>& func) const {
	if (!config) {
		return;
	}
	config->SetPath("/app");
	func();
	config->SetPath("/");
}

wxString config_manager::encode_note(const wxString& note) {
	if (note.IsEmpty()) {
		return wxEmptyString;
	}
	std::ostringstream b64_stream;
	Poco::Base64Encoder encoder(b64_stream);
	const std::string note_str = note.ToStdString();
	encoder.write(note_str.data(), static_cast<std::streamsize>(note_str.size()));
	encoder.close();
	return wxString(b64_stream.str());
}

wxString config_manager::decode_note(const wxString& encoded) {
	if (encoded.IsEmpty()) {
		return wxEmptyString;
	}
	std::istringstream b64_stream(encoded.ToStdString());
	Poco::Base64Decoder decoder(b64_stream);
	std::string decoded_str;
	std::getline(decoder, decoded_str, '\0');
	return wxString::FromUTF8(decoded_str.c_str());
}

template bool config_manager::get_app_setting<bool>(const wxString&, const bool&) const;
template int config_manager::get_app_setting<int>(const wxString&, const int&) const;
template wxString config_manager::get_app_setting<wxString>(const wxString&, const wxString&) const;
template void config_manager::set_app_setting<bool>(const wxString&, const bool&);
template void config_manager::set_app_setting<int>(const wxString&, const int&);
template void config_manager::set_app_setting<wxString>(const wxString&, const wxString&);
