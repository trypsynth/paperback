#pragma once
#include "parser.hpp"

class text_parser : public parser {
public:
	text_parser() = default;
	~text_parser() = default;
	text_parser(const text_parser&) = delete;
	text_parser& operator=(const text_parser&) = delete;
	text_parser(text_parser&&) = delete;
	text_parser& operator=(text_parser&&) = delete;
	[[nodiscard]] wxString name() const override { return "Text Files"; }
	[[nodiscard]] std::span<const wxString> extensions() const override {
		static const wxString exts[] = {"txt", "log"};
		return exts;
	}
	[[nodiscard]] std::unique_ptr<document> load(const wxString& path) const override;
};

REGISTER_PARSER(text_parser)
