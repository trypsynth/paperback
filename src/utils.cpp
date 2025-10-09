/* utils.cpp - various helper functions that didn't belong anywhere else.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "utils.hpp"
#include "config_manager.hpp"
#include "dialogs.hpp"
#include "parser.hpp"
#include <Poco/Exception.h>
#include <Poco/RegularExpression.h>
#include <Poco/URI.h>
#include <Poco/UTF8String.h>
#include <cctype>
#include <optional>
#include <sstream>
#include <wx/strconv.h>
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
	for (size_t i = 0; i < input.size(); ++i) {
		const auto ch = static_cast<unsigned char>(input[i]);
		// Check for non-breaking space (UTF-8: 0xC2A0)
		const bool is_nbsp = (i + 1 < input.size() && ch == 0xC2 && static_cast<unsigned char>(input[i + 1]) == 0xA0);
		if (std::isspace(ch) || is_nbsp) {
			if (!prev_was_space) {
				result << ' ';
				prev_was_space = true;
			}
			if (is_nbsp) ++i; // Skip the second byte of the UTF-8 sequence.
		} else {
			result << input[i];
			prev_was_space = false;
		}
	}
	return result.str();
}

std::string trim_string(const std::string& str) {
	auto start = str.begin();
	auto end = str.end();
	auto is_nbsp = [&](std::string::const_iterator it) -> bool {
		return it != str.end() && std::next(it) != str.end() && static_cast<unsigned char>(*it) == 0xC2 && static_cast<unsigned char>(*std::next(it)) == 0xA0;
	};
	while (start != end && (std::isspace(static_cast<unsigned char>(*start)) || is_nbsp(start))) {
		if (is_nbsp(start))
			start += 2;
		else
			++start;
	}
	while (start != end) {
		auto prev = std::prev(end);
		if (std::isspace(static_cast<unsigned char>(*prev)))
			end = prev;
		else if (prev != start && std::prev(prev) != start && is_nbsp(std::prev(prev)))
			end = std::prev(prev);
		else
			break;
	}
	return std::string(start, end);
}

std::string remove_soft_hyphens(std::string_view input) {
	try {
		std::string result(input);
		Poco::RegularExpression regex("\xC2\xAD", Poco::RegularExpression::RE_UTF8);
		regex.subst(result, "", Poco::RegularExpression::RE_GLOBAL);
		return result;
	} catch (const Poco::Exception&) {
		return std::string(input);
	}
}

const parser* get_parser_for_unknown_file(const wxString& path, config_manager& config) {
	wxString saved_format = config.get_document_format(path);
	if (!saved_format.IsEmpty()) {
		auto* par = find_parser_by_extension(saved_format);
		if (par) return par;
	}
	open_as_dialog dlg(nullptr, path);
	if (dlg.ShowModal() != wxID_OK) return nullptr;
	wxString format = dlg.get_selected_format();
	config.set_document_format(path, format);
	return find_parser_by_extension(format);
}

void speak(const wxString& message) {
#ifdef _WIN32
	speechSay(message, 1);
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

std::string convert_to_utf8(const std::string& input) {
	if (input.empty()) return input;
	const auto* data = reinterpret_cast<const unsigned char*>(input.data());
	const size_t len = input.length();
	auto try_convert = [&](size_t bom_size, wxMBConv& conv) -> std::optional<std::string> {
		wxString content(input.data() + bom_size, conv, len - bom_size);
		if (!content.empty()) return std::string(content.ToUTF8());
		return std::nullopt;
	};
	if (len >= 4 && data[0] == 0xFF && data[1] == 0xFE && data[2] == 0x00 && data[3] == 0x00) {
		wxMBConvUTF32LE conv;
		if (auto result = try_convert(4, conv)) return *result;
	}
	if (len >= 4 && data[0] == 0x00 && data[1] == 0x00 && data[2] == 0xFE && data[3] == 0xFF) {
		wxMBConvUTF32BE conv;
		if (auto result = try_convert(4, conv)) return *result;
	}
	if (len >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) return input.substr(3);
	if (len >= 2 && data[0] == 0xFF && data[1] == 0xFE) {
		wxMBConvUTF16LE conv;
		if (auto result = try_convert(2, conv)) return *result;
	}
	if (len >= 2 && data[0] == 0xFE && data[1] == 0xFF) {
		wxMBConvUTF16BE conv;
		if (auto result = try_convert(2, conv)) return *result;
	}
	const std::pair<const char*, wxMBConv*> fallback_encodings[] = {
		{nullptr, nullptr}, // UTF-8 without BOM
		{"local", &wxConvLocal},
		{"windows-1252", nullptr},
		{"iso-8859-1", &wxConvISO8859_1}};
	for (const auto& [name, conv] : fallback_encodings) {
		wxString content;
		if (!name)
			content = wxString::FromUTF8(input.data(), len);
		else if (conv)
			content = wxString(input.data(), *conv, len);
		else {
			wxCSConv csconv(name);
			content = wxString(input.data(), csconv, len);
		}
		if (!content.empty()) return std::string(content.ToUTF8());
	}
	return input;
}

void cleanup_toc(std::vector<std::unique_ptr<toc_item>>& items) {
	for (auto& item : items) {
		if (!item->children.empty()) {
			auto& first_child = item->children[0];
			if (item->name.CmpNoCase(first_child->name) == 0 && (item->ref == first_child->ref || item->ref.IsEmpty())) {
				if (item->ref.IsEmpty() && !first_child->ref.IsEmpty()) {
					item->ref = first_child->ref;
					item->offset = first_child->offset;
				}
				auto grandchildren = std::move(first_child->children);
				item->children.erase(item->children.begin());
				item->children.insert(item->children.begin(), std::make_move_iterator(grandchildren.begin()), std::make_move_iterator(grandchildren.end()));
			}
		}
		cleanup_toc(item->children);
	}
}

std::vector<std::unique_ptr<toc_item>> build_toc_from_headings(const document_buffer& buffer) {
	std::vector<std::unique_ptr<toc_item>> result;
	const auto heading_markers = buffer.get_heading_markers();
	if (heading_markers.empty()) return result;
	std::vector<std::vector<std::unique_ptr<toc_item>>*> level_stacks(7, nullptr);
	level_stacks[0] = &result;
	for (const auto* marker : heading_markers) {
		auto item = std::make_unique<toc_item>();
		item->name = marker->text;
		item->offset = static_cast<int>(marker->pos);
		const int level = marker->level;
		if (level < 1 || level > 6) continue;
		std::vector<std::unique_ptr<toc_item>>* parent_list = nullptr;
		for (int i = level - 1; i >= 0; --i) {
			if (level_stacks[i]) {
				parent_list = level_stacks[i];
				break;
			}
		}
		if (!parent_list) parent_list = &result;
		parent_list->push_back(std::move(item));
		level_stacks[level] = &parent_list->back()->children;
		for (int i = level + 1; i < 7; ++i) level_stacks[i] = nullptr;
	}
	return result;
}

std::string read_zip_entry(wxZipInputStream& zip) {
	std::ostringstream buffer;
	char buf[4096];
	while (zip.Read(buf, sizeof(buf)).LastRead() > 0)
		buffer.write(buf, zip.LastRead());
	return buffer.str();
}

wxZipEntry* find_zip_entry(const std::string& filename, const std::map<std::string, wxZipEntry*>& entries) {
	auto it = entries.find(filename);
	if (it != entries.end()) return it->second;
	auto decoded = url_decode(filename);
	if (decoded != filename) {
		it = entries.find(decoded);
		if (it != entries.end()) return it->second;
	}
	std::string encoded;
	try {
		Poco::URI::encode(filename, "", encoded);
		if (encoded != filename) {
			it = entries.find(encoded);
			if (it != entries.end()) return it->second;
		}
	} catch (const Exception&) {}
	return nullptr;
}
