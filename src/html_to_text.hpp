/* html_to_text.hpp - HTML to text header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include <lexbor/html/html.h>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

// This is needed because we don't want to render HTML code inside of markdown code blocks, but don't want HTML code blocks to have tons of tags in them either.
enum class html_source_mode {
	native_html,
	markdown
};

struct link_info {
	size_t offset;
	std::string text;
	std::string ref;
};

class html_to_text {
public:
	html_to_text();
	~html_to_text() = default;
	html_to_text(const html_to_text&) = delete;
	html_to_text& operator=(const html_to_text&) = delete;
	html_to_text(html_to_text&&) = default;
	html_to_text& operator=(html_to_text&&) = default;
	[[nodiscard]] bool convert(const std::string& html_content, html_source_mode mode = html_source_mode::native_html);
	[[nodiscard]] const std::vector<std::string>& get_lines() const noexcept { return lines; }
	[[nodiscard]] std::string get_text() const;
	[[nodiscard]] const std::unordered_map<std::string, size_t>& get_id_positions() const noexcept { return id_positions; }
	[[nodiscard]] const std::vector<heading_info>& get_headings() const noexcept { return headings; }
	[[nodiscard]] const std::vector<link_info>& get_links() const noexcept { return links; }
	[[nodiscard]] const std::string& get_title() const noexcept { return title; }
	void clear() noexcept;

private:
	struct DocumentDeleter {
		void operator()(lxb_html_document_t* doc) const noexcept {
			if (doc) {
				lxb_html_document_destroy(doc);
			}
		}
	};
	using DocumentPtr = std::unique_ptr<lxb_html_document_t, DocumentDeleter>;

	std::vector<std::string> lines;
	std::string current_line;
	std::unordered_map<std::string, size_t> id_positions;
	std::vector<heading_info> headings;
	std::vector<link_info> links;
	std::vector<bool> preserve_line_whitespace;
	std::string title;
	bool in_body = false;
	bool preserve_whitespace = false;
	bool in_code = false;
	bool in_link = false;
	std::string current_link_href;
	std::string current_link_text;
	size_t link_start_pos = 0;
	html_source_mode source_mode = html_source_mode::native_html;
	size_t cached_char_length = 0;
	DocumentPtr doc;

	void process_node(lxb_dom_node_t* node);
	void process_text_node(lxb_dom_text_t* text_node);
	void add_line(std::string_view line);
	void finalize_current_line();
	void finalize_text(); // New method for final cleanup
	size_t get_current_text_position() const;
	[[nodiscard]] static constexpr bool is_block_element(std::string_view tag_name) noexcept;
	[[nodiscard]] static std::string_view get_tag_name(lxb_dom_element_t* element) noexcept;
	[[nodiscard]] static std::string get_element_text(lxb_dom_element_t* element) noexcept;
};
