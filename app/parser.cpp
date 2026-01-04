/* parser.cpp - parser logic not specific to any given parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "parser.hpp"
#include "document_data.hpp"
#include "libpaperback/src/bridge.rs.h"
#include "utils.hpp"
#include <algorithm>
#include <memory>
#include <optional>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>
#include <vector>
#include <wx/msgdlg.h>
#include <wx/string.h>
#include <wx/translation.h>

namespace {
using parser_list = std::vector<parser_info>;

constexpr std::string_view PASSWORD_REQUIRED_PREFIX = "[password_required]";

parser_list& get_parser_infos() {
	static parser_list parsers;
	return parsers;
}

parser_exception make_parser_exception(const std::exception& e, const wxString& path) {
	const std::string message = e.what();
	if (message.rfind(PASSWORD_REQUIRED_PREFIX, 0) == 0) {
		std::string_view trimmed{message};
		trimmed.remove_prefix(PASSWORD_REQUIRED_PREFIX.size());
		const wxString localized = trimmed.empty() ? _("Password required or incorrect.") : wxString::FromUTF8(trimmed.data(), trimmed.size());
		return parser_exception(localized, path, error_severity::error, parser_error_code::password_required);
	}
	return parser_exception(wxString::FromUTF8(message.c_str()), path);
}
} // namespace

bool initialize_parser_registry() {
	try {
		const auto parser_infos = get_available_parsers();
		auto& parsers = get_parser_infos();
		parsers.clear();
		parsers.reserve(parser_infos.size());
		for (const auto& info : parser_infos) {
			parser_info record;
			record.name = to_wxstring(info.name);
			record.flags = static_cast<parser_flags>(info.flags);
			record.extensions.reserve(info.extensions.size());
			for (const auto& ext : info.extensions) {
				record.extensions.push_back(to_wxstring(ext));
			}
			parsers.push_back(std::move(record));
		}
		return true;
	} catch (const std::exception& e) {
		wxMessageBox(e.what(), _("Error"), wxICON_ERROR);
		return false;
	}
}

const parser_info* find_parser_by_extension(const wxString& extension) {
	if (extension.IsEmpty()) {
		return nullptr;
	}
	const wxString normalized = extension.Lower();
	const auto& parsers = get_parser_infos();
	const auto parser_it = std::find_if(parsers.begin(), parsers.end(), [&](const parser_info& parser) {
		return std::any_of(parser.extensions.begin(), parser.extensions.end(), [&](const wxString& ext) {
			return ext.Lower() == normalized;
		});
	});
	return parser_it != parsers.end() ? &(*parser_it) : nullptr;
}

wxString get_supported_wildcards() {
	const auto& parsers = get_parser_infos();
	if (parsers.empty()) {
		return {};
	}
	std::set<wxString> all_exts;
	for (const auto& parser : parsers) {
		all_exts.insert(parser.extensions.begin(), parser.extensions.end());
	}
	auto join_extensions = [](const auto& exts) {
		std::ostringstream oss;
		bool first = true;
		for (const auto& ext : exts) {
			if (!first) {
				oss << ";";
			}
			oss << "*." << std::string(ext.mb_str());
			first = false;
		}
		return wxString::FromUTF8(oss.str());
	};
	wxString result;
	const wxString all_ext_part = join_extensions(all_exts);
	result += "All Supported Files (" + all_ext_part + ")|" + all_ext_part + "|";
	for (const auto& parser : parsers) {
		if (parser.extensions.empty()) {
			continue;
		}
		const wxString ext_part = join_extensions(parser.extensions);
		result += parser.name + " (" + ext_part + ")|" + ext_part + "|";
	}
	result += "All Files (*.*)|*.*";
	return result;
}

// Legacy function - no longer used, documents now created via session_new()
// Kept commented for reference during migration
/*
std::unique_ptr<document> load_document_from_rust(const wxString& path, const std::optional<std::string>& password, const wxString& forced_extension) {
	try {
		const std::string file_path = path.ToUTF8().data();
		const std::string password_value = password.value_or(std::string());
		const std::string extension = forced_extension.ToUTF8().data();
		auto handle = parse_document_handle(rust::Str(file_path), rust::Str(password_value), rust::Str(extension));
		auto doc = std::make_unique<document>();
		doc->handle = std::move(handle);
		const auto& handle_ref = **doc->handle;
		doc->title = to_wxstring(document_title(handle_ref));
		doc->author = to_wxstring(document_author(handle_ref));
		doc->content = to_wxstring(document_content(handle_ref));
		doc->stats = document_stats(handle_ref);
		populate_id_positions(*doc, document_id_positions(handle_ref));
		populate_spine_items(*doc, document_spine_items(handle_ref));
		populate_manifest_items(*doc, document_manifest_items(handle_ref));
		return doc;
	} catch (const std::exception& e) {
		throw make_parser_exception(e, path);
	}
}
*/
