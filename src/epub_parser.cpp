#include "epub.hpp"
#include "epub_parser.hpp"
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>

std::unique_ptr<document> epub_parser::load(const wxString& path) const {
	epub ep;
	try {
		bool result = ep.load(path.ToStdString());
		if (!result) return nullptr;
	} catch (std::exception& e) {
		wxMessageBox(e.what(), "Error parsing epub file", wxICON_ERROR);
		return nullptr;
	}
	wxString content;
	for (int i = 0; i < ep.get_num_sections(); i++) {
		std::vector<std::string> lines;
		epub_section* section = ep.parse_section(i, &lines);
		content += ep.get_section_text(*section);
	}
	Beep(500, 500);
	auto doc = std::make_unique<document>();
	doc->set_text_content(content);
	return doc;
}
