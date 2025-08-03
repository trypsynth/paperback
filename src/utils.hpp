#pragma once
#include <string>
#include <string_view>
#include <wx/string.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward = true, bool match_case = false);
std::string collapse_whitespace(std::string_view input);
bool should_open_as_txt(const wxString& path);
void speak(const wxString& message);
