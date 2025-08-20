#pragma once
#include "parser.hpp"

class markdown_parser : public parser {
public:
	markdown_parser() = default;
	~markdown_parser() = default;
	markdown_parser(const markdown_parser&) = delete;
	markdown_parser& operator=(const markdown_parser&) = delete;
	markdown_parser(markdown_parser&&) = delete;
	markdown_parser& operator=(markdown_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "Markdown Documents"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"};
		return exts;
	}
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(markdown_parser)
