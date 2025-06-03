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
		std::vector<std::string> lines;
		epub_section section = ep.parse_section(i, &lines);
		section_offsets.push_back(content.length());
		content += wxString::FromUTF8(ep.get_section_text(section));
	}
	auto doc = std::make_unique<document>();\
	doc->set_title(ep.title());
	doc->set_author(ep.author());
	doc->set_text_content(content);
	return doc;
}

inline int epub_parser::next_section_index() const {
	if (cur_section + 1 < static_cast<int>(section_offsets.size()))
		return ++cur_section;
	return -1;
}

inline int epub_parser::previous_section_index() const {
	if (cur_section > 0)
		return --cur_section;
	return -1;
}

inline size_t epub_parser::current_offset() const {
	if (cur_section < static_cast<int>(section_offsets.size()))
		return section_offsets[cur_section];
	return 0;
}

inline size_t epub_parser::offset_for_section(int section_index) const {
	if (section_index >= 0 && section_index < static_cast<int>(section_offsets.size()))
		return section_offsets[section_index];
	return 0;
}

inline int epub_parser::current_section_index() const {
	return cur_section;
}

size_t epub_parser::section_count() const {
	return section_offsets.size();
}
