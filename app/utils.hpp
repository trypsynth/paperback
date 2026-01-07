#pragma once
#include "config_manager.hpp"
#include "document_data.hpp"
#include "parser.hpp"
#include <map>
#include <string>
#include <string_view>
#include <wx/string.h>

// Forward declarations for FFI types
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

struct search_result {
	bool found{false};
	bool wrapped{false};
	long position{wxNOT_FOUND};
};
[[nodiscard]] search_result find_text_with_wrap(const wxString& haystack, const wxString& needle, long start, find_options options = find_options::forward);
[[nodiscard]] bool ensure_parser_for_unknown_file(const wxString& path, config_manager& config);
void speak(const wxString& message);

// FFI helper functions
[[nodiscard]] wxString to_wxstring(const rust::String& rust_str);
[[nodiscard]] bool is_heading_marker(marker_type type);
