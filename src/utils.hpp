#pragma once

#include <wx/string.h>

long find_case_insensitive(const wxString& haystack, const wxString& needle, long start, bool forward = true);
