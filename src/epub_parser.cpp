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
	section_offsets.clear();
	for (int i = 0; i < ep.get_num_sections(); i++) {
		epub_section section = ep.parse_section(i);
		section_offsets.push_back(content.length());
		content += wxString::FromUTF8(ep.get_section_text(section));
	}
	auto doc = std::make_unique<document>();\
	doc->title = ep.title();
	doc->author = ep.author();
	doc->text_content = content;
	return doc;
}

int epub_parser::next_section_index(size_t position) const {
	for (size_t i = 0; i < section_offsets.size(); ++i)
		if (section_offsets[i] > position)
			return static_cast<int>(i);
	return -1;
}

int epub_parser::previous_section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (section_offsets[i] < position)
			return i;
	return -1;
}

int epub_parser::section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (position >= section_offsets[i])
			return i;
	return -1;
}

size_t epub_parser::offset_for_section(int section_index) const {
	if (section_index < 0 || section_index >= static_cast<int>(section_offsets.size()))
		return 0;
	return section_offsets[section_index];
}

size_t epub_parser::section_count() const {
	return section_offsets.size();
}
