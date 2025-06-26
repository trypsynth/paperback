#include <cctype>
#include <sstream>
#include <string_view>
#include "utils.hpp"

long find_case_insensitive(const wxString& haystack, const wxString& needle, long start, bool forward) {
	wxString haystack_lc = haystack.Lower();
	wxString needle_lc = needle.Lower();
	return forward ? haystack_lc.find(needle_lc, start) : haystack_lc.Left(start).rfind(needle_lc);
}

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case) {
	if (needle.empty()) return wxNOT_FOUND;
	if (match_case)
		if (forward)
			return haystack.find(needle, start);
		else
			return haystack.Left(start).rfind(needle);
	else
		return find_case_insensitive(haystack, needle, start, forward);
}

std::string collapse_whitespace(std::string_view input) {
	std::ostringstream oss;
	bool in_space = false;
	for (unsigned char ch : input) {
		if (std::isspace(ch)) {
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
