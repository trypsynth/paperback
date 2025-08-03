#include "utils.hpp"
#include <cctype>
#include <sstream>
#define WIN32_LEAN_AND_MEAN
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#include <wx/msgdlg.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case) {
	if (needle.empty()) return wxNOT_FOUND;
	if (match_case) return forward ? haystack.find(needle, start) : haystack.Left(start).rfind(needle);
	const wxString haystack_lc = haystack.Lower();
	const wxString needle_lc = needle.Lower();
	return forward ? haystack_lc.find(needle_lc, start) : haystack_lc.Left(start).rfind(needle_lc);
}

std::string collapse_whitespace(std::string_view input) {
	std::ostringstream result;
	bool prev_was_space = false;
	for (unsigned char ch : input) {
		if (std::isspace(ch)) {
			if (!prev_was_space) {
				result << ' ';
				prev_was_space = true;
			}
		} else {
			result << ch;
			prev_was_space = false;
		}
	}
	return result.str();
}

bool should_open_as_txt(const wxString& path) {
	const wxString message = wxString::Format("No suitable parser was found for %s. Would you like to treat it as plain text?", path);
	return wxMessageBox(message, "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
}

void speak(const wxString& message) {
	speechSayA(message, 1);
}
