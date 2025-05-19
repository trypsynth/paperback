#include "parser_registry.hpp"
#include "text_parser.hpp"

static std::vector<parser*>& parsers() {
	static std::vector<parser*> list;
	return list;
}

void parser_registry::register_parser(parser* p) {
	if (p) parsers().push_back(p);
}

parser* parser_registry::find_by_extension(const wxString& extension) {
	wxString normalized = extension.Lower();
	for (parser* par : parsers())
		for (const wxString ext : par->extensions())
			if (ext.Lower() == normalized) return par;
	return nullptr;
}

const std::vector<parser*>& parser_registry::all() {
	return parsers();
}

void register_parsers() {
	parser_registry::register_parser(&text_par);
}

wxString get_supported_wildcards() {
	wxString result;
	const auto& parsers = parser_registry::all();
	for (const parser* p : parsers) {
		const wxString& name = p->name();
		const std::vector<wxString>& exts = p->extensions();
		if (exts.empty()) continue;
		wxString ext_part;
		for (size_t i = 0; i < exts.size(); ++i) {
			ext_part += "*." + exts[i];
			if (i + 1 < exts.size()) ext_part += ";";
		}
		result += name + " (" + ext_part + ")|" + ext_part + "|";
	}
	result += "All Files (*.*)|*.*";
	return result;
}
