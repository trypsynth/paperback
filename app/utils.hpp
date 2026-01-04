/* utils.hpp - miscellaneous helpers shared across Paperback.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "config_manager.hpp"
#include "document_data.hpp"
#include "parser.hpp"
#include <map>
#include <string>
#include <string_view>
#include <wx/string.h>

// Forward declarations for FFI types
struct FfiMarker;
namespace rust {
inline namespace cxxbridge1 {
class String;
}
} // namespace rust

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
[[nodiscard]] const parser_info* get_parser_for_unknown_file(const wxString& path, config_manager& config);
void speak(const wxString& message);

// FFI helper functions
[[nodiscard]] wxString to_wxstring(const rust::String& rust_str);
[[nodiscard]] marker to_marker(const FfiMarker& ffi_marker);
[[nodiscard]] bool is_heading_marker(marker_type type);
