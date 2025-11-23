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

wxString to_wxstring(const rust::String& rust_str) {
	const std::string utf8 = std::string(rust_str);
	return wxString::FromUTF8(utf8.c_str());
}

void populate_markers(std::vector<marker>& markers, const rust::Vec<FfiMarker>& ffi_markers) {
	markers.clear();
	markers.reserve(ffi_markers.size());
	for (const auto& rust_marker : ffi_markers) {
		marker m{
			rust_marker.position,
			static_cast<marker_type>(rust_marker.marker_type),
			to_wxstring(rust_marker.text),
			to_wxstring(rust_marker.reference),
			rust_marker.level};
		markers.push_back(std::move(m));
	}
	std::sort(markers.begin(), markers.end(), [](const marker& a, const marker& b) {
		return a.pos < b.pos;
	});
}

void populate_toc_items(std::vector<std::unique_ptr<toc_item>>& toc_items, const rust::Vec<FfiTocItem>& ffi_toc_items) {
	if (ffi_toc_items.empty()) {
		return;
	}
	constexpr int MAX_DEPTH = 32;
	std::vector<std::vector<std::unique_ptr<toc_item>>*> depth_stacks(MAX_DEPTH + 1, nullptr);
	depth_stacks[0] = &toc_items;
	for (const auto& rust_toc : ffi_toc_items) {
		auto item = std::make_unique<toc_item>();
		item->name = to_wxstring(rust_toc.name);
		item->ref = to_wxstring(rust_toc.reference);
		item->offset = rust_toc.offset;
		const int depth = rust_toc.depth;
		if (depth < 0 || depth > MAX_DEPTH) {
			continue;
		}
		std::vector<std::unique_ptr<toc_item>>* parent_list = nullptr;
		for (int i = depth; i >= 0; --i) {
			if (depth_stacks[i] != nullptr) {
				parent_list = depth_stacks[i];
				break;
			}
		}
		if (parent_list == nullptr) {
			parent_list = &toc_items;
		}
		parent_list->push_back(std::move(item));
		depth_stacks[depth + 1] = &parent_list->back()->children;
		for (int i = depth + 2; i <= MAX_DEPTH; ++i) {
			depth_stacks[i] = nullptr;
		}
	}
}


void populate_id_positions(document& doc, const rust::Vec<FfiIdPosition>& ffi_positions) {
	doc.id_positions.clear();
	for (const auto& entry : ffi_positions) {
		doc.id_positions[std::string(entry.id)] = entry.offset;
	}
}

void populate_spine_items(document& doc, const rust::Vec<rust::String>& ffi_spine_items) {
	doc.spine_items.clear();
	for (const auto& item : ffi_spine_items) {
		doc.spine_items.emplace_back(std::string(item));
	}
}

void populate_manifest_items(document& doc, const rust::Vec<FfiManifestItem>& ffi_manifest) {
	doc.manifest_items.clear();
	for (const auto& entry : ffi_manifest) {
		doc.manifest_items[std::string(entry.id)] = std::string(entry.path);
	}
}

parser_exception make_parser_exception(const std::exception& e, const wxString& path) {
	const std::string message = e.what();
	if (message.rfind(PASSWORD_REQUIRED_PREFIX, 0) == 0) {
		const std::string_view trimmed(message.c_str() + PASSWORD_REQUIRED_PREFIX.size(), message.size() - PASSWORD_REQUIRED_PREFIX.size());
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
	for (const auto& parser : get_parser_infos()) {
		for (const auto& ext : parser.extensions) {
			if (ext.Lower() == normalized) {
				return &parser;
			}
		}
	}
	return nullptr;
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

std::unique_ptr<document> load_document_from_rust(const wxString& path, const std::optional<std::string>& password) {
	try {
		const std::string file_path = path.ToUTF8().data();
		const std::string password_value = password.value_or(std::string());
		auto handle = parse_document_handle(rust::Str(file_path), rust::Str(password_value));
		auto doc = std::make_unique<document>();
		doc->handle = std::move(handle);
		const auto& handle_ref = **doc->handle;
		doc->title = to_wxstring(document_title(handle_ref));
		doc->author = to_wxstring(document_author(handle_ref));
		doc->content = to_wxstring(document_content(handle_ref));
		populate_markers(doc->markers, document_markers(handle_ref));
		populate_toc_items(doc->toc_items, document_toc_items(handle_ref));
		doc->stats = document_stats(handle_ref);
		populate_id_positions(*doc, document_id_positions(handle_ref));
		populate_spine_items(*doc, document_spine_items(handle_ref));
		populate_manifest_items(*doc, document_manifest_items(handle_ref));
		return doc;
	} catch (const std::exception& e) {
		throw make_parser_exception(e, path);
	}
}
