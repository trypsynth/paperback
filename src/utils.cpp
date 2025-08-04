#include "utils.hpp"
#include <Poco/Exception.h>
#include <Poco/URI.h>
#include <cctype>
#include <sstream>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#endif
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
