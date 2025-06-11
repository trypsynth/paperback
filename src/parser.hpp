#pragma once

#include "document.hpp"
#include <memory>
#include <vector>
#include <wx/string.h>

enum class parser_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
};

inline parser_flags operator|(parser_flags a, parser_flags b) {
	return static_cast<parser_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline parser_flags operator&(parser_flags a, parser_flags b) {
	return static_cast<parser_flags>(static_cast<int>(a) & static_cast<int>(b));
}

class parser {
public:
	virtual ~parser() = default;
	virtual wxString name() const = 0;
	virtual const std::vector<wxString>& extensions() const = 0;
	virtual parser_flags flags() const = 0;
	virtual std::unique_ptr<document> load(const wxString& path) = 0;

	bool has_flag(parser_flags flag) const {
		return (flags() & flag) == flag;
	}
};

const std::vector<parser*>& get_all_parsers();
parser* find_parser_by_extension(const wxString& extension);
wxString get_supported_wildcards();
