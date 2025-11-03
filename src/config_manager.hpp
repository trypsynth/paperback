/* config_manager.hpp - config management header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include <functional>
#include <memory>
#include <vector>
#include <wx/fileconf.h>
#include <wx/string.h>

template <typename T>
struct app_setting {
	const char* key;
	T default_value;

	constexpr app_setting(const char* k, const T& def) : key{k}, default_value{def} {
	}
};

struct bookmark {
	long start;
	long end;
	wxString note;

	bookmark(long s, long e, const wxString& n = wxEmptyString) : start{s}, end{e}, note{n} {
	}

	bookmark() : start{0}, end{0}, note{wxEmptyString} {
	}

	bool is_whole_line() const {
		return start == end;
	}

	bool has_note() const {
		return !note.IsEmpty();
	}

	bool operator==(const bookmark& other) const {
		return start == other.start && end == other.end;
	}
};

class config_manager {
public:
	static constexpr app_setting<int> recent_documents_to_show{"recent_documents_to_show", 25};
	static constexpr app_setting<bool> restore_previous_documents{"restore_previous_documents", true};
	static constexpr app_setting<bool> word_wrap{"word_wrap", false};
	static constexpr app_setting<bool> minimize_to_tray{"minimize_to_tray", false};
	static constexpr app_setting<bool> compact_go_menu{"compact_go_menu", true};
	static constexpr app_setting<bool> navigation_wrap{"navigation_wrap", false};
	static constexpr app_setting<bool> check_for_updates_on_startup{"check_for_updates_on_startup", true};
	static constexpr app_setting<int> sleep_timer_duration{"sleep_timer_duration", 30};
	static constexpr app_setting<int> config_version{"version", 0};
	static inline const app_setting<wxString> language{"language", wxString("")};
	static inline const app_setting<wxString> active_document{"active_document", wxString("")};

	config_manager() = default;
	~config_manager();
	config_manager(const config_manager&) = delete;
	config_manager& operator=(const config_manager&) = delete;
	config_manager(config_manager&&) = default;
	config_manager& operator=(config_manager&&) = default;
	bool initialize();
	void flush();
	void shutdown();
	wxString get_string(const wxString& key, const wxString& default_value = "") const;
	bool get_bool(const wxString& key, bool default_value = false) const;
	int get_int(const wxString& key, int default_value = 0) const;
	void set_string(const wxString& key, const wxString& value);
	void set_bool(const wxString& key, bool value);
	void set_int(const wxString& key, int value);

	wxFileConfig* get_config() const {
		return config.get();
	}

	bool is_initialized() const {
		return config != nullptr;
	}

	template <typename T>
	T get(const app_setting<T>& setting) const {
		return get_app_setting(wxString(setting.key), setting.default_value);
	}

	template <typename T>
	void set(const app_setting<T>& setting, const T& value) {
		set_app_setting(wxString(setting.key), value);
	}

	void add_recent_document(const wxString& path);
	wxArrayString get_recent_documents() const;
	void clear_recent_documents();
	void rebuild_recent_documents();
	void add_opened_document(const wxString& path);
	void remove_opened_document(const wxString& path);
	wxArrayString get_opened_documents() const;
	void clear_opened_documents();
	void set_document_position(const wxString& path, long position);
	long get_document_position(const wxString& path) const;
	void set_document_opened(const wxString& path, bool opened);
	void remove_document_history(const wxString& path);
	bool get_document_opened(const wxString& path) const;
	wxArrayString get_all_opened_documents() const;
	wxArrayString get_all_documents() const;
	void add_bookmark(const wxString& path, long start, long end, const wxString& note = wxEmptyString);
	void remove_bookmark(const wxString& path, long start, long end);
	void toggle_bookmark(const wxString& path, long start, long end, const wxString& note = wxEmptyString);
	void update_bookmark_note(const wxString& path, long start, long end, const wxString& note);
	std::vector<bookmark> get_bookmarks(const wxString& path) const;
	void clear_bookmarks(const wxString& path);
	bookmark get_next_bookmark(const wxString& path, long current_position) const;
	bookmark get_previous_bookmark(const wxString& path, long current_position) const;
	bookmark get_closest_bookmark(const wxString& path, long current_position) const;
	void set_document_format(const wxString& path, const wxString& format);
	wxString get_document_format(const wxString& path) const;
	bool needs_migration() const;
	bool migrate_config();
	    void export_document_settings(const wxString& doc_path, const wxString& export_path);
	    void import_document_settings(const wxString& path);
	    void import_settings_from_file(const wxString& doc_path, const wxString& import_path);
	
	private:	std::unique_ptr<wxFileConfig> config;
	bool owns_global_config{false};

	template <typename T>
	T get_app_setting(const wxString& key, const T& default_value) const;
	template <typename T>
	void set_app_setting(const wxString& key, const T& value);
	template <typename T>
	T get_document_setting(const wxString& path, const wxString& key, const T& default_value) const;
	template <typename T>
	void set_document_setting(const wxString& path, const wxString& key, const T& value);
	static wxString get_config_path();
	static bool is_directory_writable(const wxString& dir);
	void load_defaults();
	static wxString get_document_section(const wxString& path);
	static wxString escape_document_path(const wxString& path);
	void with_document_section(const wxString& path, const std::function<void()>& func) const;
	void with_app_section(const std::function<void()>& func) const;
	static wxString encode_note(const wxString& note);
	static wxString decode_note(const wxString& encoded);
};
