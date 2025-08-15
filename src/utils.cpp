#include "utils.hpp"
#include <Poco/Exception.h>
#include <Poco/RegularExpression.h>
#include <Poco/URI.h>
#include <cctype>
#include <sstream>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#endif
#include <wx/msgdlg.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward, bool match_case, bool match_whole_word, bool use_regex) {
	if (needle.empty()) return wxNOT_FOUND;
	if (use_regex) {
		try {
			std::string pattern = needle.ToStdString();
			std::string text = haystack.ToStdString();
			if (match_whole_word) pattern = "\\b" + pattern + "\\b";
			int options = 0;
			if (!match_case) options |= Poco::RegularExpression::RE_CASELESS;
			Poco::RegularExpression regex(pattern, options);
			Poco::RegularExpression::Match match;
			if (forward)
				if (regex.match(text, start, match))	
					return match.offset;
			else {
				std::string search_text = text.substr(0, start);
				long last_match = wxNOT_FOUND;
				size_t pos = 0;
				while (regex.match(search_text, pos, match)) {
					last_match = match.offset;
					pos = match.offset + 1;
				}
				return last_match;
			}
		} catch (const Poco::Exception&) {
			return wxNOT_FOUND;
		}
		return wxNOT_FOUND;
	}
	const wxString& search_haystack = match_case ? haystack : haystack.Lower();
	const wxString& search_needle = match_case ? needle : needle.Lower();
	if (!match_whole_word)
		return forward ? search_haystack.find(search_needle, start) : search_haystack.Left(start).rfind(search_needle);
	long pos = start;
	while (true) {
		pos = forward ? search_haystack.find(search_needle, pos) : search_haystack.Left(pos).rfind(search_needle);
		if (pos == wxNOT_FOUND) break;
		bool word_start = (pos == 0) || !wxIsalnum(haystack[pos - 1]);
		bool word_end = (pos + needle.length() >= haystack.length()) || !wxIsalnum(haystack[pos + needle.length()]);
		if (word_start && word_end) return pos;
		pos = forward ? pos + 1 : pos - 1;
		if (forward && pos >= haystack.length()) break;
		if (!forward && pos < 0) break;
	}
	return wxNOT_FOUND;
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

std::string trim_string(const std::string& str) {
	auto start = str.begin();
	auto end = str.end();
	start = std::find_if(start, end, [](unsigned char ch) {
		return !std::isspace(ch);
	});
	end = std::find_if(str.rbegin(), std::string::const_reverse_iterator(start), [](unsigned char ch) {
			  return !std::isspace(ch);
		  }).base();
	return std::string(start, end);
}

bool should_open_as_txt(const wxString& path) {
	const wxString message = wxString::Format("No suitable parser was found for %s. Would you like to treat it as plain text?", path);
	return wxMessageBox(message, "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
}

void speak(const wxString& message) {
#ifdef _WIN32
	speechSayA(message, 1);
#endif
}

std::string url_decode(const std::string& encoded) {
	try {
		std::string decoded;
		Poco::URI::decode(encoded, decoded);
		return decoded;
	} catch (const Poco::Exception&) {
		return encoded;
	}
}

Poco::Zip::ZipArchive::FileHeaders::const_iterator find_file_in_archive(const std::string& filename, const std::unique_ptr<Poco::Zip::ZipArchive>& archive) {
	auto header = archive->findHeader(filename);
	if (header != archive->headerEnd()) return header;
	std::string decoded = url_decode(filename);
	if (decoded != filename) {
		header = archive->findHeader(decoded);
		if (header != archive->headerEnd()) return header;
	}
	std::string encoded;
	try {
		Poco::URI::encode(filename, "", encoded);
		if (encoded != filename) {
			header = archive->findHeader(encoded);
			if (header != archive->headerEnd()) return header;
		}
	} catch (const Poco::Exception&) {
	}
	return archive->headerEnd();
}
