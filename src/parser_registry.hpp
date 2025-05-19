#pragma once

#include "parser.hpp"
#include <vector>
#include <wx/string.h>

const std::vector<parser*>& get_all_parsers();
parser* find_parser_by_extension(const wxString& extension);
wxString get_supported_wildcards();
