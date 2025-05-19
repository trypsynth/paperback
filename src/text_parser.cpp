#include "text_parser.hpp"

std::unique_ptr<document> text_parser::load(const wxString& path) const {
	auto doc = std::make_unique<document>();
	doc->set_title(path);
	return doc;
}
