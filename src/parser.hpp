/* parser.hpp - base parser interface.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include <memory>
#include <span>
#include <string_view>
#include <vector>
#include <wx/string.h>

enum class parser_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
	supports_pages = 1 << 2,
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

class parser {
public:
	virtual ~parser() = default;
	[[nodiscard]] virtual wxString name() const = 0;
	[[nodiscard]] virtual std::span<const wxString> extensions() const = 0;
	[[nodiscard]] virtual std::unique_ptr<document> load(const wxString& path) const = 0;
	[[nodiscard]] virtual parser_flags supported_flags() const = 0;
	[[nodiscard]] inline bool has_flag(parser_flags flag) const noexcept {
		return (supported_flags() & flag) == flag;
	}
};

class parser_registry {
public:
	static void register_parser(const parser& p) { get_parsers().push_back(&p); }
	[[nodiscard]] static std::span<const parser* const> get_all() noexcept { return get_parsers(); }

private:
	static std::vector<const parser*>& get_parsers() {
		static std::vector<const parser*> parsers;
		return parsers;
	}
};

template <typename ParserType>
class parser_registrar {
public:
	parser_registrar() { parser_registry::register_parser(instance); }

private:
	static inline ParserType instance{};
};

#define REGISTER_PARSER(parser_type) static parser_registrar<parser_type> parser_type##_registrar;

[[nodiscard]] const parser* find_parser_by_extension(const wxString& extension) noexcept;
[[nodiscard]] wxString get_supported_wildcards();
