#include "parser.hpp"
#include "epub_parser.hpp"
#include "text_parser.hpp"

const std::vector<parser*>& get_all_parsers() {
	static std::vector<parser*> parsers = {
		&epub_par,
		&text_par,
	};
	return parsers;
}

parser* find_parser_by_extension(const wxString& extension) {
	wxString normalized = extension.Lower();
	for (parser* par : get_all_parsers())
		for (const wxString ext : par->extensions())
			if (ext.Lower() == normalized) return par;
	return nullptr;
}

wxString get_supported_wildcards() {
	wxString result;
	const auto& parsers = get_all_parsers();
	for (const parser* p : parsers) {
		const wxString& name = p->name();
		const auto& exts = p->extensions();
		if (exts.empty()) continue;
		wxString ext_part;
		bool first = true;
		for (const wxString& ext : exts) {
			if (!first) ext_part += ";";
			ext_part += "*." + ext;
			first = false;
		}
		result += name + " (" + ext_part + ")|" + ext_part + "|";
	}
	result += "All Files (*.*)|*.*";
	return result;
}
