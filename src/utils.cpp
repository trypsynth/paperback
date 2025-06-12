#include <cctype>
#include <sstream>
#include "utils.hpp"

long find_case_insensitive(const wxString& haystack, const wxString& needle, long start, bool forward) {
	if (needle.empty()) return wxNOT_FOUND;
	wxString haystack_lower = haystack.Lower();
	wxString needle_lower = needle.Lower();
	const long haystack_len = haystack_lower.Length();
	const long needle_len = needle_lower.Length();
	if (needle_len > haystack_len) return wxNOT_FOUND;
	if (forward) {
		for (long i = start; i <= haystack_len - needle_len; ++i)
			if (haystack_lower.Mid(i, needle_len) == needle_lower) return i;
	} else {
		for (long i = start - needle_len; i >= 0; --i)
			if (haystack_lower.Mid(i, needle_len) == needle_lower) return i;
	}
	return wxNOT_FOUND;
}

std::string collapse_whitespace(const std::string& input) {
	std::ostringstream oss;
	bool in_space = false;
	for (char ch : input) {
		if (std::isspace(static_cast<unsigned char>(ch))) {
			if (!in_space) {
				oss << ' ';
				in_space = true;
			}
		} else {
			oss << ch;
			in_space = false;
		}
	}
	return oss.str();
}
