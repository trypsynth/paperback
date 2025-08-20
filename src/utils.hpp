#pragma once
#include <Poco/Zip/ZipArchive.h>
#include <string>
#include <string_view>
#include <wx/string.h>

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
[[nodiscard]] bool should_open_as_txt(const wxString& path);
void speak(const wxString& message);
[[nodiscard]] std::string url_decode(std::string_view encoded);
[[nodiscard]] Poco::Zip::ZipArchive::FileHeaders::const_iterator find_file_in_archive(std::string_view filename, const std::unique_ptr<Poco::Zip::ZipArchive>& archive);
