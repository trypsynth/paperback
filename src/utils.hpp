/* utils.hpp - various helper functions that didn't belong anywhere else.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include <map>
#include <string>
#include <string_view>
#include <wx/string.h>
#include <wx/zipstrm.h>

class config_manager;
class parser;

enum class find_options {
	none = 0,
	forward = 1 << 0,
	match_case = 1 << 1,
	match_whole_word = 1 << 2,
	use_regex = 1 << 3
};

inline constexpr find_options operator|(find_options a, find_options b) noexcept {
	return static_cast<find_options>(static_cast<int>(a) | static_cast<int>(b));
}

inline constexpr find_options operator&(find_options a, find_options b) noexcept {
	return static_cast<find_options>(static_cast<int>(a) & static_cast<int>(b));
}

inline constexpr find_options& operator|=(find_options& a, find_options b) noexcept {
	return a = a | b;
}

inline constexpr bool has_option(find_options options, find_options flag) noexcept {
	return (options & flag) != find_options::none;
}

[[nodiscard]] long find_text(const wxString& haystack, const wxString& needle, long start, find_options options = find_options::forward);
[[nodiscard]] std::string collapse_whitespace(std::string_view input);
[[nodiscard]] std::string trim_string(const std::string& str);
[[nodiscard]] std::string remove_soft_hyphens(std::string_view input);
[[nodiscard]] const parser* get_parser_for_unknown_file(const wxString& path, config_manager& config);
void speak(const wxString& message);
[[nodiscard]] std::string url_decode(std::string_view encoded);
[[nodiscard]] std::string convert_to_utf8(const std::string& input);
void cleanup_toc(std::vector<std::unique_ptr<toc_item>>& items);
[[nodiscard]] std::vector<std::unique_ptr<toc_item>> build_toc_from_headings(const document_buffer& buffer);
[[nodiscard]] std::string read_zip_entry(wxZipInputStream& zip);
[[nodiscard]] wxZipEntry* find_zip_entry(const std::string& filename, const std::map<std::string, wxZipEntry*>& entries);
