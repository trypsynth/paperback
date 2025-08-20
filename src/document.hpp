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

	toc_item() = default;
	~toc_item() = default;
	toc_item(const toc_item&) = delete;
	toc_item& operator=(const toc_item&) = delete;
	toc_item(toc_item&&) = default;
	toc_item& operator=(toc_item&&) = default;
};

struct document_stats {
	int word_count{0};
	int line_count{0};
	int char_count{0};
	int char_count_no_whitespace{0};

	document_stats() = default;
	~document_stats() = default;
	document_stats(const document_stats&) = default;
	document_stats& operator=(const document_stats&) = default;
	document_stats(document_stats&&) = default;
	document_stats& operator=(document_stats&&) = default;
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

	document() = default;
	~document() = default;
	document(const document&) = delete;
	document& operator=(const document&) = delete;
	document(document&&) = default;
	document& operator=(document&&) = default;

	[[nodiscard]] inline bool has_flag(document_flags flag) const noexcept {
		return (flags & flag) == flag;
	}

	[[nodiscard]] inline int get_word_count() const noexcept {
		return stats.word_count;
	}

	[[nodiscard]] inline int get_line_count() const noexcept {
		return stats.line_count;
	}

	[[nodiscard]] inline int get_char_count() const noexcept {
		return stats.char_count;
	}

	[[nodiscard]] inline int get_char_count_no_whitespace() const noexcept {
		return stats.char_count_no_whitespace;
	}

	[[nodiscard]] int next_section_index(size_t position) const noexcept;
	[[nodiscard]] int previous_section_index(size_t position) const noexcept;
	[[nodiscard]] int section_index(size_t position) const noexcept;
	[[nodiscard]] size_t offset_for_section(int section_index) const noexcept;
	[[nodiscard]] int next_page_index(size_t position) const noexcept;
	[[nodiscard]] int previous_page_index(size_t position) const noexcept;
	[[nodiscard]] int page_index(size_t position) const noexcept;
	[[nodiscard]] size_t offset_for_page(int page_index) const noexcept;
	void calculate_statistics() const;
};
