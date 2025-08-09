#include "document.hpp"
#include <wx/tokenzr.h>

int document::next_section_index(size_t position) const {
	for (size_t i = 0; i < section_offsets.size(); ++i)
		if (section_offsets[i] > position)
			return static_cast<int>(i);
	return -1;
}

int document::previous_section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (section_offsets[i] < position)
			return i;
	return -1;
}

int document::section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (position >= section_offsets[i])
			return i;
	return -1;
}

size_t document::offset_for_section(int section_index) const {
	if (section_index < 0 || section_index >= static_cast<int>(section_offsets.size())) return 0;
	return section_offsets[section_index];
}

void document::calculate_statistics() const {
	stats.char_count = text_content.Length();
	stats.char_count_no_whitespace = 0;
	for (size_t i = 0; i < text_content.Length(); ++i) {
		wxChar ch = text_content[i];
		if (ch != ' ' && ch != '\t' && ch != '\r' && ch != '\n') ++stats.char_count_no_whitespace;
	}
	if (text_content.IsEmpty())
		stats.line_count = 0;
	else {
		stats.line_count = 1;
		for (size_t i = 0; i < text_content.Length(); ++i) {
			if (text_content[i] == '\n') ++stats.line_count;
		}
	}
	if (text_content.IsEmpty())
		stats.word_count = 0;
	else {
		wxStringTokenizer tokenizer(text_content, " \t\r\n", wxTOKEN_STRTOK);
		stats.word_count = 0;
		while (tokenizer.HasMoreTokens()) {
			tokenizer.GetNextToken();
			++stats.word_count;
		}
	}
}
