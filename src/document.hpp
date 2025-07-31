#pragma once
#include <memory>
#include <vector>
#include <wx/string.h>

enum class document_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
};

inline document_flags operator|(document_flags a, document_flags b) {
	return static_cast<document_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline document_flags operator&(document_flags a, document_flags b) {
	return static_cast<document_flags>(static_cast<int>(a) & static_cast<int>(b));
}

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	int offset;
};

struct document_stats {
	int word_count = 0;
	int line_count = 0;
	int char_count = 0;
	int char_count_no_whitespace = 0;
};

struct document {
	wxString title;
	wxString author;
	wxString text_content;
	document_flags flags;
	std::vector<size_t> section_offsets;
	std::vector<std::unique_ptr<toc_item>> toc_items;
	mutable document_stats stats;

	bool has_flag(document_flags flag) const;
	int next_section_index(size_t position) const;
	int previous_section_index(size_t position) const;
	int section_index(size_t position) const;
	size_t offset_for_section(int section_index) const;
	void calculate_statistics() const;
	int get_word_count() const;
	int get_line_count() const;
	int get_char_count() const;
	int get_char_count_no_whitespace() const;
};
