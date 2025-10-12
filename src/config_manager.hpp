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
#include <wx/fileconf.h>
#include <wx/string.h>

class config_manager {
public:
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
	wxFileConfig* get_config() const { return config.get(); }
	bool is_initialized() const { return config != nullptr; }
	void add_recent_document(const wxString& path);
	wxArrayString get_recent_documents() const;
	void clear_recent_documents();
	int get_recent_documents_to_show() const;
	void set_recent_documents_to_show(int count);
	bool get_restore_previous_documents() const;
	void set_restore_previous_documents(bool restore);
	bool get_word_wrap() const;
	void set_word_wrap(bool word_wrap);
	void set_active_document(const wxString& path);
	wxString get_active_document() const;
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
	int get_config_version() const;
	void set_config_version(int version);
	void add_bookmark(const wxString& path, long position);
	void remove_bookmark(const wxString& path, long position);
	void toggle_bookmark(const wxString& path, long position);
	wxArrayLong get_bookmarks(const wxString& path) const;
	void clear_bookmarks(const wxString& path);
	long get_next_bookmark(const wxString& path, long current_position) const;
	long get_previous_bookmark(const wxString& path, long current_position) const;
	long get_closest_bookmark(const wxString& path, long current_position) const;
	void set_document_format(const wxString& path, const wxString& format);
	wxString get_document_format(const wxString& path) const;
	bool needs_migration() const;
	bool migrate_config();

private:
	std::unique_ptr<wxFileConfig> config;
	bool owns_global_config{false};

	wxString get_config_path() const;
	bool is_directory_writable(const wxString& dir) const;
	void load_defaults();
	wxString get_document_section(const wxString& path) const;
	wxString escape_document_path(const wxString& path) const;
	void with_document_section(const wxString& path, std::function<void()> func) const;
	void with_app_section(std::function<void()> func) const;
};
