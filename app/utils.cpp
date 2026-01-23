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

// open_as_dialog has been ported to Rust - see src/ui/dialogs.rs::show_open_as_dialog
// This function is now unused as document opening is handled by src/ui/utils.rs::ensure_parser_for_unknown_file
bool ensure_parser_for_unknown_file(const wxString& path, config_manager& config) {
	// Legacy C++ implementation removed
	(void)path;
	(void)config;
	return false;
}

void speak(const wxString& message) {
	auto* main_win = dynamic_cast<main_window*>(wxGetApp().GetTopWindow());
	if (main_win == nullptr) return;
	auto* label = main_win->get_live_region_label();
	if (label == nullptr) return;
	label->SetLabel(message);
	notify_live_region_changed(label);
}
