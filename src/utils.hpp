#pragma once
#include <string>
#include <wx/string.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case);
std::string collapse_whitespace(std::string_view input);
bool should_open_as_txt(const wxString& path);
void speak(const wxString& message);
int get_word_count(const wxString& text);
int get_line_count(const wxString& text);
int get_char_count_no_whitespace(const wxString& text);
