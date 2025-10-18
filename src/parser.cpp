/* parser.cpp - parser logic not specific to any given parser.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "parser.hpp"
#include <set>
#include <sstream>

const parser* find_parser_by_extension(const wxString& extension) noexcept {
	const wxString normalized = extension.Lower();
	for (auto* par : parser_registry::get_all()) {
		for (const auto& ext : par->extensions()) {
			if (ext.Lower() == normalized) {
				return par;
			}
		}
	}
	return nullptr;
}

wxString get_supported_wildcards() {
	std::set<wxString> all_exts;
	const auto& parsers = parser_registry::get_all();
	for (const parser* p : parsers) {
		all_exts.insert(p->extensions().begin(), p->extensions().end());
	}
	if (all_exts.empty()) {
		return {};
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
	wxString all_ext_part = join_extensions(all_exts);
	result += "All Supported Files (" + all_ext_part + ")|" + all_ext_part + "|";
	for (const parser* p : parsers) {
		const auto& exts = p->extensions();
		if (exts.empty()) {
			continue;
		}
		wxString ext_part = join_extensions(exts);
		result += p->name() + " (" + ext_part + ")|" + ext_part + "|";
	}
	result += "All Files (*.*)|*.*";
	return result;
}
