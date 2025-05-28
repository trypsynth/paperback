#include "parser.hpp"
#include "epub_parser.hpp"
#include "text_parser.hpp"
#include <set>
#include <sstream>

const std::vector<parser*>& get_all_parsers() {
	static std::vector<parser*> parsers = {
		&epub_par,
		&text_par,
	};
	return parsers;
}

parser* find_parser_by_extension(const wxString& extension) {
	const wxString normalized = extension.Lower();
	for (parser* par : get_all_parsers())
		for (const auto& ext : par->extensions())
			if (ext.Lower() == normalized) return par;
	return nullptr;
}

wxString get_supported_wildcards() {
	std::set<wxString> all_exts;
	const auto& parsers = get_all_parsers();
	for (const parser* p : parsers)
		all_exts.insert(p->extensions().begin(), p->extensions().end());
	if (all_exts.empty()) return {};
	auto join_extensions = [](const auto& exts) {
		std::ostringstream oss;
		bool first = true;
		for (const auto& ext : exts) {
			if (!first) oss << ";";
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
		if (exts.empty()) continue;
		wxString ext_part = join_extensions(exts);
		result += p->name() + " (" + ext_part + ")|" + ext_part + "|";
	}
	result += "All Files (*.*)|*.*";
	return result;
}
