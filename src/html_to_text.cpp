/* html_to_text.cpp - handles the conversion of HTML content into plaintext.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "html_to_text.hpp"
#include "utils.hpp"
#include <algorithm>
#include <array>
#include <cctype>
#include <sstream>
#include <stdexcept>
#include <string_view>

html_to_text::html_to_text() : doc(lxb_html_document_create()) {
	if (!doc) throw std::runtime_error("Failed to create Lexbor HTML document");
}

bool html_to_text::convert(const std::string& html_content) {
	clear();
	const auto status = lxb_html_document_parse(doc.get(), reinterpret_cast<const lxb_char_t*>(html_content.data()), html_content.length());
	if (status != LXB_STATUS_OK) return false;
	if (auto* node = lxb_dom_interface_node(doc.get())) process_node(node);
	finalize_current_line();
	finalize_text();
	return true;
}

std::string html_to_text::get_text() const {
	if (lines.empty()) return {};
	std::ostringstream oss;
	for (const auto& line : lines) oss << line << '\n';
	auto result = oss.str();
	if (!result.empty()) result.pop_back(); // Remove trailing newline
	return result;
}

void html_to_text::clear() noexcept {
	lines.clear();
	current_line.clear();
	id_positions.clear();
	headings.clear();
	in_body = false;
	preserve_whitespace = false;
}

void html_to_text::process_node(lxb_dom_node_t* node) {
	if (!node) return;
	std::string_view tag_name;
	const bool is_element = (node->type == LXB_DOM_NODE_TYPE_ELEMENT);
	if (is_element) {
		auto* element = lxb_dom_interface_element(node);
		tag_name = get_tag_name(element);
	}
	switch (node->type) {
	case LXB_DOM_NODE_TYPE_ELEMENT: {
		auto* element = lxb_dom_interface_element(node);
		if (tag_name == "body")
			in_body = true;
		else if (tag_name == "pre")
			preserve_whitespace = true;
		else if (tag_name == "br" || tag_name == "li")
			finalize_current_line();
		if (in_body && element) {
			size_t id_len;
			const lxb_char_t* id_attr = lxb_dom_element_get_attribute(element, (const lxb_char_t*)"id", 2, &id_len);
			if (id_attr && id_len > 0) {
				std::string id{reinterpret_cast<const char*>(id_attr), id_len};
				size_t total_length = 0;
				for (const auto& line : lines) total_length += line.length() + 1;
				id_positions[id] = total_length;
			}
			if (tag_name.length() == 2 && tag_name[0] == 'h' && tag_name[1] >= '1' && tag_name[1] <= '6') {
				int level = tag_name[1] - '0';
				finalize_current_line();
				size_t heading_offset = get_current_text_position();
				std::string heading_text = get_element_text(element);
				if (!heading_text.empty()) headings.push_back({heading_offset, level, heading_text});
			}
		}
		break;
	}
	case LXB_DOM_NODE_TYPE_TEXT:
		process_text_node(lxb_dom_interface_text(node));
		break;
	default:
		break;
	}
	for (auto* child = node->first_child; child; child = child->next) process_node(child);
	if (is_element) {
		if (is_block_element(tag_name)) finalize_current_line();
		if (tag_name == "pre") preserve_whitespace = false;
	}
}

void html_to_text::process_text_node(lxb_dom_text_t* text_node) {
	if (!in_body) return;
	size_t length;
	const auto* text_data = lxb_dom_node_text_content(lxb_dom_interface_node(text_node), &length);
	if (!text_data || length == 0) return;
	const std::string_view text{reinterpret_cast<const char*>(text_data), length};
	if (!text.empty()) {
		std::string processed_text = remove_soft_hyphens(text);
		current_line += preserve_whitespace ? processed_text : collapse_whitespace(processed_text);
	}
}

void html_to_text::add_line(std::string_view line) {
	std::string processed_line = collapse_whitespace(line);
	processed_line = trim_string(processed_line);
	if (!processed_line.empty()) lines.emplace_back(std::move(processed_line));
}

void html_to_text::finalize_current_line() {
	add_line(current_line);
	current_line.clear();
}

void html_to_text::finalize_text() {
	std::vector<std::string> cleaned_lines;
	for (auto& line : lines) {
		line = collapse_whitespace(line);
		line = trim_string(line);
		if (!line.empty()) cleaned_lines.emplace_back(std::move(line));
	}
	lines = std::move(cleaned_lines);
}

size_t html_to_text::get_current_text_position() const {
	size_t total_length = 0;
	for (const auto& line : lines) total_length += line.length() + 1;
	total_length += current_line.length();
	return total_length;
}

constexpr bool html_to_text::is_block_element(std::string_view tag_name) noexcept {
	if (tag_name.empty()) return false;
	constexpr std::array block_elements = {
		"div",
		"p",
		"pre",
		"h1",
		"h2",
		"h3",
		"h4",
		"h5",
		"h6",
		"blockquote",
		"ul",
		"ol",
		"li",
		"section",
		"article",
		"header",
		"footer",
		"nav",
		"aside",
		"main",
		"figure",
		"figcaption",
		"address",
		"hr",
		"table",
		"thead",
		"tbody",
		"tfoot",
		"tr",
		"td",
		"th"};
	return std::find(block_elements.begin(), block_elements.end(), tag_name) != block_elements.end();
}

std::string_view html_to_text::get_tag_name(lxb_dom_element_t* element) noexcept {
	if (!element) return {};
	const auto* name = lxb_dom_element_qualified_name(element, nullptr);
	return name ? std::string_view{reinterpret_cast<const char*>(name)} : std::string_view{};
}

std::string html_to_text::get_element_text(lxb_dom_element_t* element) noexcept {
	if (!element) return {};
	size_t text_length;
	const auto* text = lxb_dom_node_text_content(lxb_dom_interface_node(element), &text_length);
	if (!text || text_length == 0) return {};
	return std::string{reinterpret_cast<const char*>(text), text_length};
}
