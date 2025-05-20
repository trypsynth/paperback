#include "text_parser.hpp"
#include <wx/filename.h>

std::unique_ptr<document> text_parser::load(const wxString& path) const {
	auto doc = std::make_unique<document>();
	doc->set_title(wxFileName(path).GetName());
	doc->set_author("Unknown");
	return doc;
}
