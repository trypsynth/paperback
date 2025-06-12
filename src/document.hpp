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

struct document {
	wxString title;
	wxString author;
	wxString text_content;
	document_flags flags;
	std::vector<size_t> section_offsets;
	std::vector<std::unique_ptr<toc_item>> toc_items;

	bool has_flag(document_flags flag) const {
		return (flags & flag) == flag;
	}

	int next_section_index(size_t position) const {
		for (size_t i = 0; i < section_offsets.size(); ++i)
			if (section_offsets[i] > position)
				return static_cast<int>(i);
		return -1;
	}

	int previous_section_index(size_t position) const {
		for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
			if (section_offsets[i] < position)
				return i;
		return -1;
	}

	int section_index(size_t position) const {
		for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
			if (position >= section_offsets[i])
				return i;
		return -1;
	}

	size_t offset_for_section(int section_index) const {
		if (section_index < 0 || section_index >= static_cast<int>(section_offsets.size()))
			return 0;
		return section_offsets[section_index];
	}

	size_t section_count() const {
		return section_offsets.size();
	}

	size_t toc_item_count() const {
		return toc_items.size();
	}
};
