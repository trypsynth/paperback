#pragma once

#include <string>
#include <wx/string.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case);
std::string collapse_whitespace(std::string_view input);
bool should_open_as_txt(const wxString& path);
