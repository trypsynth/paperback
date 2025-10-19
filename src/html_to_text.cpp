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
#include <lexbor/dom/interfaces/element.h>
#include <lexbor/html/html.h>
#include <lexbor/html/parser.h>
#include <lexbor/html/serialize.h>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <wx/string.h>

html_to_text::html_to_text() : doc(lxb_html_document_create()) {
	if (!doc) {
		throw std::runtime_error("Failed to create Lexbor HTML document");
	}
}

bool html_to_text::convert(const std::string& html_content, html_source_mode mode) {
	clear();
	source_mode = mode;
	const auto status = lxb_html_document_parse(doc.get(), reinterpret_cast<const lxb_char_t*>(html_content.data()), html_content.length());
	if (status != LXB_STATUS_OK) {
		return false;
	}
	if (auto* node = lxb_dom_interface_node(doc.get())) {
		process_node(node);
	}
	finalize_current_line();
	finalize_text();
	return true;
}

std::string html_to_text::get_text() const {
	if (lines.empty()) {
		return {};
	}
	std::ostringstream oss;
	for (const auto& line : lines) {
		oss << line << '\n';
	}
	auto result = oss.str();
	if (!result.empty()) {
		result.pop_back(); // Remove trailing newline
	}
	return result;
}

void html_to_text::clear() noexcept {
	lines.clear();
	preserve_line_whitespace.clear();
	current_line.clear();
	id_positions.clear();
	headings.clear();
	links.clear();
	title.clear();
	in_body = false;
	preserve_whitespace = false;
	in_code = false;
	in_link = false;
	current_link_href.clear();
	current_link_text.clear();
	link_start_pos = 0;
	cached_char_length = 0;
}

void html_to_text::process_node(lxb_dom_node_t* node) {
	if (!node) {
		return;
	}
	std::string_view tag_name;
	size_t link_start_pos = 0;
	const bool is_element = (node->type == LXB_DOM_NODE_TYPE_ELEMENT);
	if (is_element) {
		auto* element = lxb_dom_interface_element(node);
		tag_name = get_tag_name(element);
	}
	switch (node->type) {
		case LXB_DOM_NODE_TYPE_ELEMENT: {
			auto* element = lxb_dom_interface_element(node);
			if (tag_name == "a") {
				if (!in_link) {
					in_link = true;
					size_t href_len;
					const lxb_char_t* href_attr = lxb_dom_element_get_attribute(element, (const lxb_char_t*)"href", 4, &href_len);
					if (href_attr && href_len > 0) {
						current_link_href = std::string(reinterpret_cast<const char*>(href_attr), href_len);
					}
					link_start_pos = get_current_text_position();
				}
			}
			if (tag_name == "title" && title.empty()) {
				title = get_element_text(element);
				title = trim_string(collapse_whitespace(title));
			} else if (tag_name == "body") {
				in_body = true;
			} else if (tag_name == "pre") {
				finalize_current_line();
				preserve_whitespace = true;
			} else if (tag_name == "code") {
				in_code = true;
			} else if (tag_name == "br" || tag_name == "li") {
				finalize_current_line();
			}
			if (in_body && element) {
				size_t id_len;
				const lxb_char_t* id_attr = lxb_dom_element_get_attribute(element, (const lxb_char_t*)"id", 2, &id_len);
				if (id_attr && id_len > 0) {
					std::string id{reinterpret_cast<const char*>(id_attr), id_len};
					id_positions[id] = cached_char_length;
				}
				if (tag_name.length() == 2 && tag_name[0] == 'h' && tag_name[1] >= '1' && tag_name[1] <= '6') {
					int level = tag_name[1] - '0';
					finalize_current_line();
					size_t heading_offset = get_current_text_position();
					std::string heading_text = get_element_text(element);
					if (!heading_text.empty()) {
						headings.push_back({heading_offset, level, heading_text});
					}
				}
			}
			break;
		}
		case LXB_DOM_NODE_TYPE_TEXT:
			process_text_node(lxb_dom_interface_text(node));
			break;
		case LXB_DOM_NODE_TYPE_COMMENT:
			return;
		default:
			break;
	}
	if (is_element && (tag_name == "script" || tag_name == "style")) {
		return;
	}
	if (source_mode == html_source_mode::markdown && in_code && preserve_whitespace && is_element && tag_name == "code") {
		for (auto* child = node->first_child; child; child = child->next) {
			if (child->type == LXB_DOM_NODE_TYPE_ELEMENT) {
				lexbor_str_t str = {0};
				lxb_html_serialize_tree_str(child, &str);
				if (str.data && str.length > 0) {
					current_line += std::string(reinterpret_cast<const char*>(str.data), str.length);
					lexbor_str_destroy(&str, doc.get()->dom_document.text, false);
				}
			} else {
				process_node(child);
			}
		}
	} else {
		for (auto* child = node->first_child; child; child = child->next) {
			process_node(child);
		}
	}
	if (is_element) {
		if (tag_name == "a") {
			if (in_link) {
				in_link = false;
				if (!current_link_text.empty()) {
					links.push_back({link_start_pos, trim_string(collapse_whitespace(current_link_text)), current_link_href});
					current_line += current_link_text;
				}
				current_link_href.clear();
				current_link_text.clear();
			}
		}
		if (tag_name == "pre") {
			preserve_whitespace = false;
		}
		if (tag_name == "code") {
			in_code = false;
		}
		if (is_block_element(tag_name)) {
			finalize_current_line();
		}
	}
}

void html_to_text::process_text_node(lxb_dom_text_t* text_node) {
	if (!in_body) {
		return;
	}
	size_t length;
	const auto* text_data = lxb_dom_node_text_content(lxb_dom_interface_node(text_node), &length);
	if (!text_data || length == 0) {
		return;
	}
	const std::string_view text{reinterpret_cast<const char*>(text_data), length};
	if (!text.empty()) {
		std::string processed_text = remove_soft_hyphens(text);
		if (preserve_whitespace) {
			size_t pos = 0;
			size_t found;
			while ((found = processed_text.find('\n', pos)) != std::string::npos) {
				current_line += processed_text.substr(pos, found - pos);
				finalize_current_line();
				pos = found + 1;
			}
			current_line += processed_text.substr(pos);
		} else if (in_link) {
			current_link_text += collapse_whitespace(processed_text);
		} else {
			current_line += collapse_whitespace(processed_text);
		}
	}
}

void html_to_text::add_line(std::string_view line) {
	std::string processed_line;
	if (preserve_whitespace) {
		processed_line = std::string(line);
		cached_char_length += wxString::FromUTF8(processed_line).length() + 1; // +1 for newline
		lines.emplace_back(std::move(processed_line));
		preserve_line_whitespace.push_back(true);
	} else {
		processed_line = collapse_whitespace(line);
		processed_line = trim_string(processed_line);
		if (!processed_line.empty()) {
			cached_char_length += wxString::FromUTF8(processed_line).length() + 1; // +1 for newline
			lines.emplace_back(std::move(processed_line));
			preserve_line_whitespace.push_back(false);
		}
	}
}

void html_to_text::finalize_current_line() {
	add_line(current_line);
	current_line.clear();
}

void html_to_text::finalize_text() {
	std::vector<std::string> cleaned_lines;
	std::vector<bool> cleaned_preserve;
	cached_char_length = 0;
	for (size_t i = 0; i < lines.size(); ++i) {
		auto& line = lines[i];
		bool preserve_ws = i < preserve_line_whitespace.size() ? preserve_line_whitespace[i] : false;
		if (preserve_ws) {
			cached_char_length += wxString::FromUTF8(line).length() + 1; // +1 for newline
			cleaned_lines.emplace_back(std::move(line));
			cleaned_preserve.push_back(true);
		} else {
			line = collapse_whitespace(line);
			line = trim_string(line);
			if (!line.empty()) {
				cached_char_length += wxString::FromUTF8(line).length() + 1; // +1 for newline
				cleaned_lines.emplace_back(std::move(line));
				cleaned_preserve.push_back(false);
			}
		}
	}
	lines = std::move(cleaned_lines);
	preserve_line_whitespace = std::move(cleaned_preserve);
}

size_t html_to_text::get_current_text_position() const {
	return cached_char_length + wxString::FromUTF8(current_line).length();
}

constexpr bool html_to_text::is_block_element(std::string_view tag_name) noexcept {
	if (tag_name.empty()) {
		return false;
	}
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
		"dl",
		"dt",
		"dd",
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
		"th",
	};
	return std::find(block_elements.begin(), block_elements.end(), tag_name) != block_elements.end();
}

std::string_view html_to_text::get_tag_name(lxb_dom_element_t* element) noexcept {
	if (!element) {
		return {};
	}
	size_t len;
	const auto* name = lxb_dom_element_qualified_name(element, &len);
	return name ? std::string_view{reinterpret_cast<const char*>(name), len} : std::string_view{};
}

std::string html_to_text::get_element_text(lxb_dom_element_t* element) noexcept {
	if (!element) {
		return {};
	}
	size_t text_length;
	const auto* text = lxb_dom_node_text_content(lxb_dom_interface_node(element), &text_length);
	if (!text || text_length == 0) {
		return {};
	}
	return std::string{reinterpret_cast<const char*>(text), text_length};
}
