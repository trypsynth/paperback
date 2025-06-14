#pragma once

#include <lexbor/html/html.h>
#include <string>
#include <vector>

class html_to_text {
public:
	html_to_text();
	~html_to_text();
	[[nodiscard]] bool convert(const std::string& html_content);
	[[nodiscard]] const std::vector<std::string>& get_lines() const {return lines;}
	[[nodiscard]] std::string get_text() const;

private:
	std::vector<std::string> lines;
	std::string current_line;
	bool in_body = false;
	bool preserve_whitespace = false;
	lxb_html_document_t* doc = nullptr;

	void process_node(lxb_dom_node_t* node);
	void process_text_node(lxb_dom_text_t* text_node);
	void add_line(std::string_view line);
	[[nodiscard]] inline bool is_block_element(std::string_view tag_name) const noexcept;
	[[nodiscard]] inline std::string_view get_tag_name(lxb_dom_element_t* element) const noexcept;
};
