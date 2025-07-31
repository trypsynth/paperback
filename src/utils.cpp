#include "utils.hpp"
#include <cctype>
#include <sstream>
#include <string_view>
#define WIN32_LEAN_AND_MEAN
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#include <wx/msgdlg.h>
#include <wx/tokenzr.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case) {
	if (needle.empty()) return wxNOT_FOUND;
	if (match_case) return forward ? haystack.find(needle, start) : haystack.Left(start).rfind(needle);
	wxString haystack_lc = haystack.Lower();
	wxString needle_lc = needle.Lower();
	return forward ? haystack_lc.find(needle_lc, start) : haystack_lc.Left(start).rfind(needle_lc);
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

bool should_open_as_txt(const wxString& path) {
	return wxMessageBox("No suitable parser was found for " + path + ". Would you like to treat it as plain text?", "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
}

void speak(const wxString& message) {
	speechSayA(message, 1);
}

int get_word_count(const wxString& text) {
	if (text.IsEmpty()) return 0;
	wxStringTokenizer tokenizer(text, " \t\r\n", wxTOKEN_STRTOK);
	int count = 0;
	while (tokenizer.HasMoreTokens()) {
		tokenizer.GetNextToken();
		++count;
	}
	return count;
}

int get_line_count(const wxString& text) {
	if (text.IsEmpty()) return 0;
	int count = 1;
	for (size_t i = 0; i < text.Length(); ++i) {
		if (text[i] == '\n') ++count;
	}
	return count;
}

int get_char_count_no_whitespace(const wxString& text) {
	int count = 0;
	for (size_t i = 0; i < text.Length(); ++i) {
		wxChar ch = text[i];
		if (ch != ' ' && ch != '\t' && ch != '\r' && ch != '\n') ++count;
	}
	return count;
}
