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

search_result find_text_with_wrap(const wxString& haystack, const wxString& needle, long start, find_options options) {
	search_result result{};
	if (needle.empty()) return result;
	const bool forward = has_option(options, find_options::forward);
	const bool match_case = has_option(options, find_options::match_case);
	const bool match_whole_word = has_option(options, find_options::match_whole_word);
	const bool use_regex = has_option(options, find_options::use_regex);
	const std::string hay = std::string(haystack.ToUTF8());
	const std::string ned = std::string(needle.ToUTF8());
	const auto search = reader_search_with_wrap(hay, ned, start, forward, match_case, match_whole_word, use_regex);
	result.found = search.found;
	result.wrapped = search.wrapped;
	result.position = search.found ? static_cast<long>(search.position) : wxNOT_FOUND;
	return result;
}

bool ensure_parser_for_unknown_file(const wxString& path, config_manager& config) {
	const wxString saved_format = config.get_document_format(path);
	if (!saved_format.IsEmpty() && is_parser_supported(saved_format)) return true;
	open_as_dialog dlg(nullptr, path);
	if (dlg.ShowModal() != wxID_OK) return false;
	const wxString format = dlg.get_selected_format();
	if (!is_parser_supported(format)) {
		wxMessageBox(_("Unsupported format selected."), _("Error"), wxICON_ERROR);
		return false;
	}
	config.set_document_format(path, format);
	return true;
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
	return is_heading_marker_type(static_cast<int>(type));
}
