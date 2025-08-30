/* utils.cpp - various helper functions that didn't belong anywhere else.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "utils.hpp"
#include <Poco/Exception.h>
#include <Poco/RegularExpression.h>
#include <Poco/URI.h>
#include <Poco/Zip/ZipArchive.h>
#include <cctype>
#include <sstream>
#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#define UNIVERSAL_SPEECH_STATIC
#include <UniversalSpeech.h>
#endif
#include <wx/msgdlg.h>

long find_text(const wxString& haystack, const wxString& needle, long start, find_options options) {
	if (needle.empty()) return wxNOT_FOUND;
	const auto forward = has_option(options, find_options::forward);
	const auto match_case = has_option(options, find_options::match_case);
	const auto match_whole_word = has_option(options, find_options::match_whole_word);
	const auto use_regex = has_option(options, find_options::use_regex);
	if (use_regex) {
		try {
			auto pattern = needle.ToStdString();
			const auto text = haystack.ToStdString();
			if (match_whole_word) pattern = "\\b" + pattern + "\\b";
			int options = 0;
			if (!match_case) options |= Poco::RegularExpression::RE_CASELESS;
			Poco::RegularExpression regex(pattern, options);
			Poco::RegularExpression::Match match;
			if (forward)
				if (regex.match(text, start, match))
					return match.offset;
				else {
					const auto search_text = text.substr(0, start);
					int last_match = wxNOT_FOUND;
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
	const auto& search_haystack = match_case ? haystack : haystack.Lower();
	const auto& search_needle = match_case ? needle : needle.Lower();
	if (!match_whole_word)
		return forward ? search_haystack.find(search_needle, start) : search_haystack.Left(start).rfind(search_needle);
	size_t pos = start;
	while (true) {
		pos = forward ? search_haystack.find(search_needle, pos) : search_haystack.Left(pos).rfind(search_needle);
		if (pos == wxNOT_FOUND) break;
		const auto word_start = (pos == 0) || !wxIsalnum(haystack[pos - 1]);
		const auto word_end = (pos + needle.length() >= haystack.length()) || !wxIsalnum(haystack[pos + needle.length()]);
		if (word_start && word_end) return pos;
		pos = forward ? pos + 1 : pos - 1;
		if (forward && pos >= haystack.length()) break;
		if (!forward && pos < 0) break;
	}
	return wxNOT_FOUND;
}

std::string collapse_whitespace(std::string_view input) {
	auto result = std::ostringstream{};
	bool prev_was_space = false;
	for (const auto ch : input) {
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
	start = std::find_if(start, end, [](const unsigned char ch) noexcept {
		return !std::isspace(ch);
	});
	end = std::find_if(str.rbegin(), std::string::const_reverse_iterator(start), [](const unsigned char ch) noexcept {
		return !std::isspace(ch);
	}).base();
	return std::string(start, end);
}

bool should_open_as_txt(const wxString& path) {
	const auto message = wxString::Format("No suitable parser was found for %s. Would you like to treat it as plain text?", path);
	return wxMessageBox(message, "Warning", wxICON_WARNING | wxYES_NO) == wxYES;
}

void speak(const wxString& message) {
#ifdef _WIN32
	speechSayA(message, 1);
#endif
}

std::string url_decode(std::string_view encoded) {
	try {
		auto decoded = std::string{};
		Poco::URI::decode(std::string{encoded}, decoded);
		return decoded;
	} catch (const Poco::Exception&) {
		return std::string{encoded};
	}
}

Poco::Zip::ZipArchive::FileHeaders::const_iterator find_file_in_archive(std::string_view filename, const std::unique_ptr<Poco::Zip::ZipArchive>& archive) {
	const std::string filename_str{filename};
	auto header = archive->findHeader(filename_str);
	if (header != archive->headerEnd()) return header;
	const auto decoded = url_decode(filename);
	if (decoded != filename_str) {
		header = archive->findHeader(decoded);
		if (header != archive->headerEnd()) return header;
	}
	auto encoded = std::string{};
	try {
		Poco::URI::encode(filename_str, "", encoded);
		if (encoded != filename_str) {
			header = archive->findHeader(encoded);
			if (header != archive->headerEnd()) return header;
		}
	} catch (const Poco::Exception&) {
	}
	return archive->headerEnd();
}
