#include <cctype>
#include <sstream>
#include <string_view>
#include "utils.hpp"

long find_case_insensitive(const wxString& haystack, const wxString& needle, long start, bool forward) {
	wxString needle_lc = needle.Lower();
	const long hlen = haystack.Length();
	const long nlen = needle_lc.Length();
	if (nlen > hlen) return wxNOT_FOUND;
	if (forward)
		for (long i = start; i <= hlen - nlen; ++i)
			if (haystack.SubString(i, i + nlen - 1).Lower() == needle_lc) return i;
	else
		for (long i = start - nlen; i >= 0; --i)
			if (haystack.SubString(i, i + nlen - 1).Lower() == needle_lc) return i;
	return wxNOT_FOUND;
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
