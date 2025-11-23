/* parser.hpp - base parser interface.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document_data.hpp"
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>
#include <wx/string.h>

enum class error_severity {
	error,
	warning
};

enum class parser_error_code {
	generic,
	password_required
};

class parser_exception : public std::runtime_error {
public:
	parser_exception(const wxString& msg, error_severity sev = error_severity::error, parser_error_code code = parser_error_code::generic) : std::runtime_error(msg.ToStdString()), message{msg}, severity{sev}, error_code{code} {
	}
	parser_exception(const wxString& msg, const wxString& fp, error_severity sev = error_severity::error, parser_error_code code = parser_error_code::generic) : std::runtime_error(msg.ToStdString()), message{msg}, file_path{fp}, severity{sev}, error_code{code} {
	}

	[[nodiscard]] error_severity get_severity() const noexcept {
		return severity;
	}

	[[nodiscard]] const wxString& get_file_path() const noexcept {
		return file_path;
	}

	[[nodiscard]] const wxString& get_message() const noexcept {
		return message;
	}

	[[nodiscard]] wxString get_display_message() const {
		if (file_path.IsEmpty()) {
			return message;
		}
		return wxString::Format("%s: %s", file_path, message);
	}

	[[nodiscard]] parser_error_code get_error_code() const noexcept {
		return error_code;
	}

private:
	wxString message;
	wxString file_path;
	error_severity severity;
	parser_error_code error_code;
};

enum class parser_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
	supports_pages = 1 << 2,
	supports_lists = 1 << 3,
};

inline constexpr parser_flags operator|(parser_flags a, parser_flags b) noexcept {
	return static_cast<parser_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline constexpr parser_flags operator&(parser_flags a, parser_flags b) noexcept {
	return static_cast<parser_flags>(static_cast<int>(a) & static_cast<int>(b));
}

inline constexpr parser_flags& operator|=(parser_flags& a, parser_flags b) noexcept {
	return a = a | b;
}

inline constexpr bool parser_supports(parser_flags flags, parser_flags flag) noexcept {
	return (flags & flag) == flag;
}

struct parser_info {
	wxString name;
	std::vector<wxString> extensions;
	parser_flags flags{parser_flags::none};
};

bool initialize_parser_registry();
[[nodiscard]] const parser_info* find_parser_by_extension(const wxString& extension);
[[nodiscard]] wxString get_supported_wildcards();
[[nodiscard]] std::unique_ptr<document> load_document_from_rust(const wxString& path, const std::optional<std::string>& password = std::nullopt);
