#pragma once

#include "document.hpp"
#include <memory>
#include <span>
#include <string_view>
#include <vector>
#include <wx/string.h>

class parser {
public:
	virtual ~parser() = default;
	virtual wxString name() const = 0;
	virtual std::span<const wxString> extensions() const = 0;
	virtual std::unique_ptr<document> load(const wxString& path) const = 0;
};

class parser_registry {
public:
	static void register_parser(const parser& p) { get_parsers().push_back(&p); }

	static std::span<const parser* const> get_all() noexcept { return get_parsers(); }

private:
	static std::vector<const parser*>& get_parsers() {
		static std::vector<const parser*> parsers;
		return parsers;
	}
};

template <typename ParserType>
class parser_registrar {
public:
	parser_registrar() { parser_registry::register_parser(instance); }

private:
	static inline ParserType instance{};
};

#define REGISTER_PARSER(ParserType) static parser_registrar<ParserType> ParserType##_registrar;

const parser* find_parser_by_extension(const wxString& extension) noexcept;
wxString get_supported_wildcards();
