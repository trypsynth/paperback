#pragma once
#include <memory>
#include <vector>
#include <wx/string.h>

enum class document_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
	supports_pages = 1 << 2,
};

inline constexpr document_flags operator|(document_flags a, document_flags b) noexcept {
	return static_cast<document_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline constexpr document_flags operator&(document_flags a, document_flags b) noexcept {
	return static_cast<document_flags>(static_cast<int>(a) & static_cast<int>(b));
}

inline constexpr document_flags& operator|=(document_flags& a, document_flags b) noexcept {
	return a = a | b;
}

struct toc_item {
	wxString name;
	wxString ref;
	std::vector<std::unique_ptr<toc_item>> children;
	int offset;
};

struct document_stats {
	int word_count{0};
	int line_count{0};
	int char_count{0};
	int char_count_no_whitespace{0};
};

struct document {
	wxString title;
	wxString author;
	wxString text_content;
	document_flags flags{document_flags::none};
	std::vector<size_t> section_offsets;
	std::vector<size_t> page_offsets;
	std::vector<std::unique_ptr<toc_item>> toc_items;
	mutable document_stats stats;

	inline bool has_flag(document_flags flag) const noexcept {
		return (flags & flag) == flag;
	}

	inline int get_word_count() const noexcept {
		return stats.word_count;
	}

	inline int get_line_count() const noexcept {
		return stats.line_count;
	}

	inline int get_char_count() const noexcept {
		return stats.char_count;
	}

	inline int get_char_count_no_whitespace() const noexcept {
		return stats.char_count_no_whitespace;
	}

	int next_section_index(size_t position) const noexcept;
	int previous_section_index(size_t position) const noexcept;
	int section_index(size_t position) const noexcept;
	size_t offset_for_section(int section_index) const noexcept;
	int next_page_index(size_t position) const noexcept;
	int previous_page_index(size_t position) const noexcept;
	int page_index(size_t position) const noexcept;
	size_t offset_for_page(int page_index) const noexcept;
	void calculate_statistics() const;
};
