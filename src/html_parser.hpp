#pragma once
#include "parser.hpp"

class html_parser : public parser {
public:
	html_parser() = default;
	~html_parser() = default;
	html_parser(const html_parser&) = delete;
	html_parser& operator=(const html_parser&) = delete;
	html_parser(html_parser&&) = delete;
	html_parser& operator=(html_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "HTML Documents"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"htm", "html", "xhtml"};
		return exts;
	}
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(html_parser)
