#include "epub.hpp"
#include "epub_parser.hpp"
#include <wx/filename.h>
#include <wx/wfstream.h>

std::unique_ptr<document> epub_parser::load(const wxString& path) const {
	epub ep;
	try {
		bool result = ep.load(path.ToStdString());
	} catch (std::exception& e) {}
}
