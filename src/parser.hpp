#pragma once

#include "document.hpp"
#include <memory>
#include <vector>
#include <wx/string.h>

enum class parser_flags {
	none = 0,
	supports_sections = 1 << 0,
	supports_toc = 1 << 1,
};

inline parser_flags operator|(parser_flags a, parser_flags b) {
	return static_cast<parser_flags>(static_cast<int>(a) | static_cast<int>(b));
}

inline parser_flags operator&(parser_flags a, parser_flags b) {
	return static_cast<parser_flags>(static_cast<int>(a) & static_cast<int>(b));
}

class section_navigable {
public:
	virtual ~section_navigable() = default;
	virtual int next_section_index(size_t position) const = 0;
	virtual int previous_section_index(size_t position) const = 0;
	virtual size_t offset_for_section(int section_index) const = 0;
	virtual int section_index(size_t position) const = 0;
	virtual size_t section_count() const = 0;

protected:
	mutable std::vector<size_t> section_offsets;
	mutable int cur_section = 0;
};

class tocable {
public:
	virtual ~tocable() = default;
	virtual int offset_for_toc_item(const toc_item& item) const = 0;
	virtual int toc_item_count() const = 0;
};

class parser {
public:
	virtual ~parser() = default;
	virtual wxString name() const = 0;
	virtual const std::vector<wxString>& extensions() const = 0;
	virtual parser_flags flags() const = 0;
	virtual std::unique_ptr<document> load(const wxString& path) const = 0;

	bool has_flag(parser_flags flag) const {
		return (flags() & flag) == flag;
	}

	section_navigable* as_section_navigable() {
		return dynamic_cast<section_navigable*>(this);
	}

	const section_navigable* as_section_navigable() const {
		return dynamic_cast<const section_navigable*>(this);
	}

	tocable* as_tocable() {
		return dynamic_cast<tocable*>(this);
	}

	const tocable* as_tocable() const {
		return dynamic_cast<const tocable*>(this);
	}
};

const std::vector<parser*>& get_all_parsers();
parser* find_parser_by_extension(const wxString& extension);
wxString get_supported_wildcards();
