#pragma once
#include "libpaperback/src/bridge.rs.h"
#include <optional>
#include <string>
#include <vector>
#include <wx/arrstr.h>
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
	static constexpr app_setting<bool> start_maximized{"start_maximized", false};
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
	config_manager(config_manager&&) noexcept = default;
	config_manager& operator=(config_manager&&) noexcept = default;
	bool initialize();
	void flush();
	void shutdown();
	wxString get_string(const wxString& key, const wxString& default_value = "") const;
	bool get_bool(const wxString& key, bool default_value = false) const;
	int get_int(const wxString& key, int default_value = 0) const;
	void set_string(const wxString& key, const wxString& value);
	void set_bool(const wxString& key, bool value);
	void set_int(const wxString& key, int value);

	template <typename T>
	T get(const app_setting<T>& setting) const {
		return get_app_setting(wxString(setting.key), setting.default_value);
	}

	template <typename T>
	void set(const app_setting<T>& setting, const T& value) {
		set_app_setting(wxString(setting.key), value);
	}

	void add_recent_document(const wxString& path);
	void set_document_position(const wxString& path, long position);
	long get_document_position(const wxString& path) const;
	void set_navigation_history(const wxString& path, const std::vector<long>& history, size_t history_index);
	void get_navigation_history(const wxString& path, std::vector<long>& history, size_t& history_index) const;
	void set_document_opened(const wxString& path, bool opened);
	void remove_document_history(const wxString& path);
	void remove_navigation_history(const wxString& path);
	bool get_document_opened(const wxString& path) const;
	wxArrayString get_all_opened_documents() const;
	wxArrayString get_all_documents() const;
	void add_bookmark(const wxString& path, long start, long end, const wxString& note = wxEmptyString);
	void remove_bookmark(const wxString& path, long start, long end);
	void toggle_bookmark(const wxString& path, long start, long end, const wxString& note = wxEmptyString);
	void update_bookmark_note(const wxString& path, long start, long end, const wxString& note);
	void set_document_format(const wxString& path, const wxString& format);
	wxString get_document_format(const wxString& path) const;
	void set_document_password(const wxString& path, const wxString& password);
	wxString get_document_password(const wxString& path) const;
	bool needs_migration() const;
	bool migrate_config();
	void export_document_settings(const wxString& doc_path, const wxString& export_path);
	void import_document_settings(const wxString& path);
	void import_settings_from_file(const wxString& doc_path, const wxString& import_path);
	// Exposes the Rust backend for FFI helpers that need a const reference.
	const ConfigManager& backend_for_ffi() const;

private:
	std::optional<rust::Box<ConfigManager>> backend;
	bool is_initialized() const;
	ConfigManager& backend_mut();
	const ConfigManager& backend_ref() const;

	template <typename T>
	T get_app_setting(const wxString& key, const T& default_value) const;
	template <typename T>
	void set_app_setting(const wxString& key, const T& value);
	template <typename T>
	T get_document_setting(const wxString& path, const wxString& key, const T& default_value) const;
	template <typename T>
	void set_document_setting(const wxString& path, const wxString& key, const T& value);
};
