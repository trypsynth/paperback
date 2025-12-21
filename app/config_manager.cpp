/* config_manager.cpp - thin C++ wrapper delegating to the Rust config manager.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "config_manager.hpp"
#include <algorithm>
#include <cstdint>
#include <type_traits>

namespace {
std::string to_utf8(const wxString& value) {
	const auto buf = value.ToUTF8();
	return std::string(buf.data(), buf.length());
}

wxString to_wx_string(const rust::String& value) {
	const std::string utf8 = std::string(value);
	return wxString::FromUTF8(utf8.c_str());
}

wxArrayString to_wx_array(const rust::Vec<rust::String>& rust_vec) {
	wxArrayString result;
	for (const auto& entry : rust_vec) {
		result.Add(to_wx_string(entry));
	}
	return result;
}

std::vector<long> to_long_vector(const rust::Vec<long long>& values) {
	std::vector<long> result(values.size());
	std::transform(values.begin(), values.end(), result.begin(), [](long long value) {
		return static_cast<long>(value);
	});
	return result;
}

bookmark to_bookmark(const FfiBookmark& fb) {
	return bookmark(static_cast<long>(fb.start), static_cast<long>(fb.end), to_wx_string(fb.note));
}
} // namespace

config_manager::~config_manager() {
	if (is_initialized()) {
		shutdown();
	}
}

bool config_manager::initialize() {
	if (!backend) {
		backend = config_manager_new();
	}
	if (!backend) {
		return false;
	}
	return config_manager_initialize(backend_mut());
}

void config_manager::flush() {
	if (is_initialized()) {
		config_manager_flush(backend_ref());
	}
}

void config_manager::shutdown() {
	if (!is_initialized()) {
		return;
	}
	config_manager_shutdown(backend_mut());
	backend.reset();
}

wxString config_manager::get_string(const wxString& key, const wxString& default_value) const {
	if (!is_initialized()) {
		return default_value;
	}
	const std::string key_utf8 = to_utf8(key);
	const std::string def_utf8 = to_utf8(default_value);
	return to_wx_string(config_manager_get_string(backend_ref(), key_utf8, def_utf8));
}

bool config_manager::get_bool(const wxString& key, bool default_value) const {
	return is_initialized() ? config_manager_get_bool(backend_ref(), to_utf8(key), default_value) : default_value;
}

int config_manager::get_int(const wxString& key, int default_value) const {
	return is_initialized() ? config_manager_get_int(backend_ref(), to_utf8(key), default_value) : default_value;
}

void config_manager::set_string(const wxString& key, const wxString& value) {
	if (is_initialized()) {
		config_manager_set_string(backend_mut(), to_utf8(key), to_utf8(value));
	}
}

void config_manager::set_bool(const wxString& key, bool value) {
	if (is_initialized()) {
		config_manager_set_bool(backend_mut(), to_utf8(key), value);
	}
}

void config_manager::set_int(const wxString& key, int value) {
	if (is_initialized()) {
		config_manager_set_int(backend_mut(), to_utf8(key), value);
	}
}

void config_manager::add_recent_document(const wxString& path) {
	if (is_initialized()) {
		config_manager_add_recent_document(backend_mut(), to_utf8(path));
	}
}

wxArrayString config_manager::get_recent_documents() const {
	if (!is_initialized()) {
		return {};
	}
	return to_wx_array(config_manager_get_recent_documents(backend_ref()));
}

void config_manager::clear_recent_documents() {
	if (is_initialized()) {
		config_manager_clear_recent_documents(backend_mut());
	}
}

void config_manager::add_opened_document(const wxString& path) {
	if (is_initialized()) {
		config_manager_add_opened_document(backend_mut(), to_utf8(path));
	}
}

void config_manager::remove_opened_document(const wxString& path) {
	if (is_initialized()) {
		config_manager_remove_opened_document(backend_mut(), to_utf8(path));
	}
}

void config_manager::clear_opened_documents() {
	if (is_initialized()) {
		config_manager_clear_opened_documents(backend_mut());
	}
}

void config_manager::set_document_position(const wxString& path, long position) {
	if (is_initialized()) {
		config_manager_set_document_position(backend_mut(), to_utf8(path), static_cast<std::int64_t>(position));
	}
}

long config_manager::get_document_position(const wxString& path) const {
	if (!is_initialized()) {
		return 0;
	}
	return static_cast<long>(config_manager_get_document_position(backend_ref(), to_utf8(path)));
}

void config_manager::set_navigation_history(const wxString& path, const std::vector<long>& history, size_t history_index) {
	if (!is_initialized()) {
		return;
	}
	rust::Vec<long long> rust_history;
	rust_history.reserve(history.size());
	std::transform(history.begin(), history.end(), std::back_inserter(rust_history), [](long entry) {
		return static_cast<long long>(entry);
	});
	rust::Slice<const std::int64_t> history_slice(rust_history.data(), rust_history.size());
	config_manager_set_navigation_history(backend_mut(), to_utf8(path), history_slice, history_index);
}

void config_manager::get_navigation_history(const wxString& path, std::vector<long>& history, size_t& history_index) const {
	history.clear();
	history_index = 0;
	if (!is_initialized()) {
		return;
	}
	const auto nav = config_manager_get_navigation_history(backend_ref(), to_utf8(path));
	history = to_long_vector(nav.positions);
	history_index = nav.index;
}

void config_manager::set_document_opened(const wxString& path, bool opened) {
	if (is_initialized()) {
		config_manager_set_document_opened(backend_mut(), to_utf8(path), opened);
	}
}

void config_manager::remove_document_history(const wxString& path) {
	if (is_initialized()) {
		config_manager_remove_document_history(backend_mut(), to_utf8(path));
	}
}

void config_manager::remove_navigation_history(const wxString& path) {
	if (is_initialized()) {
		config_manager_remove_navigation_history(backend_mut(), to_utf8(path));
	}
}

bool config_manager::get_document_opened(const wxString& path) const {
	return is_initialized() ? config_manager_get_document_opened(backend_ref(), to_utf8(path)) : false;
}

wxArrayString config_manager::get_all_opened_documents() const {
	if (!is_initialized()) {
		return {};
	}
	return to_wx_array(config_manager_get_all_opened_documents(backend_ref()));
}

wxArrayString config_manager::get_all_documents() const {
	if (!is_initialized()) {
		return {};
	}
	return to_wx_array(config_manager_get_all_documents(backend_ref()));
}

void config_manager::add_bookmark(const wxString& path, long start, long end, const wxString& note) {
	if (is_initialized()) {
		config_manager_add_bookmark(backend_mut(), to_utf8(path), static_cast<std::int64_t>(start), static_cast<std::int64_t>(end), to_utf8(note));
	}
}

void config_manager::remove_bookmark(const wxString& path, long start, long end) {
	if (is_initialized()) {
		config_manager_remove_bookmark(backend_mut(), to_utf8(path), static_cast<std::int64_t>(start), static_cast<std::int64_t>(end));
	}
}

void config_manager::toggle_bookmark(const wxString& path, long start, long end, const wxString& note) {
	if (is_initialized()) {
		config_manager_toggle_bookmark(backend_mut(), to_utf8(path), static_cast<std::int64_t>(start), static_cast<std::int64_t>(end), to_utf8(note));
	}
}

void config_manager::update_bookmark_note(const wxString& path, long start, long end, const wxString& note) {
	if (is_initialized()) {
		config_manager_update_bookmark_note(backend_mut(), to_utf8(path), static_cast<std::int64_t>(start), static_cast<std::int64_t>(end), to_utf8(note));
	}
}

std::vector<bookmark> config_manager::get_bookmarks(const wxString& path) const {
	if (!is_initialized()) {
		return {};
	}
	const auto rust_bookmarks = config_manager_get_bookmarks(backend_ref(), to_utf8(path));
	std::vector<bookmark> result;
	result.reserve(rust_bookmarks.size());
	std::transform(rust_bookmarks.begin(), rust_bookmarks.end(), std::back_inserter(result), [](const FfiBookmark& bm) {
		return to_bookmark(bm);
	});
	return result;
}

void config_manager::clear_bookmarks(const wxString& path) {
	if (is_initialized()) {
		config_manager_clear_bookmarks(backend_mut(), to_utf8(path));
	}
}

bookmark config_manager::get_next_bookmark(const wxString& path, long current_position) const {
	if (!is_initialized()) {
		return bookmark(-1, -1);
	}
	return to_bookmark(config_manager_get_next_bookmark(backend_ref(), to_utf8(path), static_cast<std::int64_t>(current_position)));
}

bookmark config_manager::get_previous_bookmark(const wxString& path, long current_position) const {
	if (!is_initialized()) {
		return bookmark(-1, -1);
	}
	return to_bookmark(config_manager_get_previous_bookmark(backend_ref(), to_utf8(path), static_cast<std::int64_t>(current_position)));
}

void config_manager::set_document_format(const wxString& path, const wxString& format) {
	if (is_initialized()) {
		config_manager_set_document_format(backend_mut(), to_utf8(path), to_utf8(format));
	}
}

wxString config_manager::get_document_format(const wxString& path) const {
	if (!is_initialized()) {
		return {};
	}
	return to_wx_string(config_manager_get_document_format(backend_ref(), to_utf8(path)));
}

void config_manager::set_document_password(const wxString& path, const wxString& password) {
	if (is_initialized()) {
		config_manager_set_document_password(backend_mut(), to_utf8(path), to_utf8(password));
	}
}

wxString config_manager::get_document_password(const wxString& path) const {
	if (!is_initialized()) {
		return {};
	}
	return to_wx_string(config_manager_get_document_password(backend_ref(), to_utf8(path)));
}

bool config_manager::needs_migration() const {
	return is_initialized() && config_manager_needs_migration(backend_ref());
}

bool config_manager::migrate_config() {
	return is_initialized() && config_manager_migrate_config(backend_mut());
}

void config_manager::export_document_settings(const wxString& doc_path, const wxString& export_path) {
	if (is_initialized()) {
		config_manager_export_document_settings(backend_ref(), to_utf8(doc_path), to_utf8(export_path));
	}
}

void config_manager::import_document_settings(const wxString& path) {
	if (is_initialized()) {
		config_manager_import_document_settings(backend_mut(), to_utf8(path));
	}
}

void config_manager::import_settings_from_file(const wxString& doc_path, const wxString& import_path) {
	if (is_initialized()) {
		config_manager_import_settings_from_file(backend_mut(), to_utf8(doc_path), to_utf8(import_path));
	}
}

bool config_manager::is_initialized() const {
	return backend.has_value();
}

ConfigManager& config_manager::backend_mut() {
	return **backend;
}

const ConfigManager& config_manager::backend_ref() const {
	return **backend;
}

const ConfigManager& config_manager::backend_for_ffi() const {
	return backend_ref();
}

template <typename T>
T config_manager::get_document_setting(const wxString& path, const wxString& key, const T& default_value) const {
	if (!is_initialized()) {
		return default_value;
	}
	const std::string key_utf8 = to_utf8(key);
	const std::string path_utf8 = to_utf8(path);
	if constexpr (std::is_same_v<T, bool>) {
		return config_manager_get_doc_bool(backend_ref(), path_utf8, key_utf8, default_value);
	} else if constexpr (std::is_same_v<T, int> || std::is_same_v<T, long>) {
		const auto value = config_manager_get_doc_int(backend_ref(), path_utf8, key_utf8, default_value);
		return static_cast<T>(value);
	} else if constexpr (std::is_same_v<T, wxString>) {
		const std::string def_utf8 = to_utf8(default_value);
		return to_wx_string(config_manager_get_doc_string(backend_ref(), path_utf8, key_utf8, def_utf8));
	} else {
		static_assert(!sizeof(T), "Unsupported document setting type");
	}
}

template <typename T>
void config_manager::set_document_setting(const wxString& path, const wxString& key, const T& value) {
	if (!is_initialized()) {
		return;
	}
	const std::string key_utf8 = to_utf8(key);
	const std::string path_utf8 = to_utf8(path);
	if constexpr (std::is_same_v<T, bool>) {
		config_manager_set_doc_bool(backend_mut(), path_utf8, key_utf8, value);
	} else if constexpr (std::is_same_v<T, int> || std::is_same_v<T, long>) {
		config_manager_set_doc_int(backend_mut(), path_utf8, key_utf8, static_cast<std::int64_t>(value));
	} else if constexpr (std::is_same_v<T, wxString>) {
		config_manager_set_doc_string(backend_mut(), path_utf8, key_utf8, to_utf8(value));
	} else {
		static_assert(!sizeof(T), "Unsupported document setting type");
	}
}

template <typename T>
T config_manager::get_app_setting(const wxString& key, const T& default_value) const {
	if (!is_initialized()) {
		return default_value;
	}
	const std::string key_utf8 = to_utf8(key);
	if constexpr (std::is_same_v<T, bool>) {
		return config_manager_get_app_bool(backend_ref(), key_utf8, default_value);
	} else if constexpr (std::is_same_v<T, int>) {
		return config_manager_get_app_int(backend_ref(), key_utf8, default_value);
	} else if constexpr (std::is_same_v<T, wxString>) {
		const std::string def_utf8 = to_utf8(default_value);
		return to_wx_string(config_manager_get_app_string(backend_ref(), key_utf8, def_utf8));
	} else {
		static_assert(!sizeof(T), "Unsupported app setting type");
	}
}

template <typename T>
void config_manager::set_app_setting(const wxString& key, const T& value) {
	if (!is_initialized()) {
		return;
	}
	const std::string key_utf8 = to_utf8(key);
	if constexpr (std::is_same_v<T, bool>) {
		config_manager_set_app_bool(backend_mut(), key_utf8, value);
	} else if constexpr (std::is_same_v<T, int>) {
		config_manager_set_app_int(backend_mut(), key_utf8, value);
	} else if constexpr (std::is_same_v<T, wxString>) {
		config_manager_set_app_string(backend_mut(), key_utf8, to_utf8(value));
	} else {
		static_assert(!sizeof(T), "Unsupported app setting type");
	}
}

template bool config_manager::get_app_setting<bool>(const wxString&, const bool&) const;
template int config_manager::get_app_setting<int>(const wxString&, const int&) const;
template wxString config_manager::get_app_setting<wxString>(const wxString&, const wxString&) const;
template void config_manager::set_app_setting<bool>(const wxString&, const bool&);
template void config_manager::set_app_setting<int>(const wxString&, const int&);
template void config_manager::set_app_setting<wxString>(const wxString&, const wxString&);
template bool config_manager::get_document_setting<bool>(const wxString&, const wxString&, const bool&) const;
template int config_manager::get_document_setting<int>(const wxString&, const wxString&, const int&) const;
template long config_manager::get_document_setting<long>(const wxString&, const wxString&, const long&) const;
template wxString config_manager::get_document_setting<wxString>(const wxString&, const wxString&, const wxString&) const;
template void config_manager::set_document_setting<bool>(const wxString&, const wxString&, const bool&);
template void config_manager::set_document_setting<int>(const wxString&, const wxString&, const int&);
template void config_manager::set_document_setting<long>(const wxString&, const wxString&, const long&);
template void config_manager::set_document_setting<wxString>(const wxString&, const wxString&, const wxString&);
