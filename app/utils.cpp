/* utils.cpp - miscellaneous helpers shared across Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "utils.hpp"
#include "app.hpp"
#include "config_manager.hpp"
#include "constants.hpp"
#include "dialogs.hpp"
#include "document_data.hpp"
#include "libpaperback/src/bridge.rs.h"
#include "live_region.hpp"
#include "main_window.hpp"
#include "parser.hpp"
#include <cctype>
#include <cstddef>
#include <iterator>
#include <optional>
#include <regex>
#include <sstream>
#include <string>
#include <string_view>
#include <wx/defs.h>
#include <wx/strconv.h>
#include <wx/string.h>

long find_text(const wxString& haystack, const wxString& needle, long start, find_options options) {
	if (needle.empty()) return wxNOT_FOUND;
	const bool forward = has_option(options, find_options::forward);
	const bool match_case = has_option(options, find_options::match_case);
	const bool match_whole_word = has_option(options, find_options::match_whole_word);
	const bool use_regex = has_option(options, find_options::use_regex);
	const std::string hay = std::string(haystack.ToUTF8());
	const std::string ned = std::string(needle.ToUTF8());
	const auto result = reader_search(hay, ned, start, forward, match_case, match_whole_word, use_regex);
	return result < 0 ? wxNOT_FOUND : static_cast<long>(result);
}

const parser_info* get_parser_for_unknown_file(const wxString& path, config_manager& config) {
	const wxString saved_format = config.get_document_format(path);
	if (!saved_format.IsEmpty()) {
		const auto* par = find_parser_by_extension(saved_format);
		if (par != nullptr) return par;
	}
	open_as_dialog dlg(nullptr, path);
	if (dlg.ShowModal() != wxID_OK) return nullptr;
	const wxString format = dlg.get_selected_format();
	config.set_document_format(path, format);
	return find_parser_by_extension(format);
}

void speak(const wxString& message) {
	auto* main_win = dynamic_cast<main_window*>(wxGetApp().GetTopWindow());
	if (main_win == nullptr) return;
	auto* label = main_win->get_live_region_label();
	if (label == nullptr) return;
	label->SetLabel(message);
	notify_live_region_changed(label);
}

// FFI helper functions
wxString to_wxstring(const rust::String& rust_str) {
	const std::string utf8 = std::string(rust_str);
	return wxString::FromUTF8(utf8.c_str());
}

marker to_marker(const FfiMarker& ffi_marker) {
	return marker{
		ffi_marker.position,
		static_cast<marker_type>(ffi_marker.marker_type),
		to_wxstring(ffi_marker.text),
		to_wxstring(ffi_marker.reference),
		ffi_marker.level,
		ffi_marker.length,
	};
}

bool is_heading_marker(marker_type type) {
	switch (type) {
		case marker_type::Heading1:
		case marker_type::Heading2:
		case marker_type::Heading3:
		case marker_type::Heading4:
		case marker_type::Heading5:
		case marker_type::Heading6:
			return true;
		default:
			return false;
	}
}
