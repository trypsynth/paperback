#pragma once

#include "parser.hpp"
#include <vector>
#include <wx/string.h>

class parser_registry {
public:
	static void register_parser(parser* p);
	static parser* find_by_extension(const wxString& extension);
	static const std::vector<parser*>& all();
};

void register_parsers();
wxString get_supported_wildcards();
