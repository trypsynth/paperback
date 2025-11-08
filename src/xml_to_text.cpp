/* xml_to_text.cpp - handles the conversion of XML content into plaintext.
 * This file has the same purpose as html_to_text.cpp, but it uses Poco's XML parser instead of lexbor so we can properly handle things like XHTML inside epub 2 books.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "xml_to_text.hpp"
#include "utils.hpp"
#include <algorithm>
#include <array>
#include <cctype>
#include <cstddef>
#include <pugixml.hpp>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>
#include <wx/string.h>

bool xml_to_text::convert(const std::string& xml_content) {
	clear();
	pugi::xml_document doc;
	const auto result = doc.load_buffer(xml_content.data(), xml_content.size(), pugi::parse_default | pugi::parse_ws_pcdata);
	if (!result) {
		clear();
		return false;
	}
	process_node(doc);
	finalize_current_line();
	return true;
}

std::string xml_to_text::get_text() const {
	if (lines.empty()) {
		return {};
	}
	std::ostringstream oss;
	for (const auto& line : lines) {
		oss << line << '\n';
	}
	auto result = oss.str();
	if (!result.empty()) {
		result.pop_back();
	}
	return result;
}

void xml_to_text::clear() noexcept {
	lines.clear();
	current_line.clear();
	id_positions.clear();
	headings.clear();
	links.clear();
	lists.clear();
	list_items.clear();
	section_offsets.clear();
	in_body = false;
	preserve_whitespace = false;
	cached_char_length = 0;
	list_level = 0;
	while (!list_style_stack.empty()) {
		list_style_stack.pop();
	}
}

std::string xml_to_text::get_bullet_for_level(int level) noexcept {
	constexpr std::array<const char*, 3> bullets = {"•", "◦", "-"};
	if (level > 0 && level <= bullets.size()) {
		return bullets[level - 1];
	}
	return "•";
}

static std::string get_local_name(const char* qname) {
	if (qname == nullptr) {
		return {};
	}
	std::string s(qname);
	auto pos = s.find(':');
	return pos == std::string::npos ? s : s.substr(pos + 1);
}

void xml_to_text::process_node(pugi::xml_node node) {
	if (node == nullptr) {
		return;
	}
	const auto node_type = node.type();
	std::string tag_name;
	bool skip_children{false};
	if (node_type == pugi::node_element) {
		auto element = node;
		tag_name = get_local_name(element.name());
		std::ranges::transform(tag_name, tag_name.begin(), ::tolower);
		if (tag_name == "section") {
			section_offsets.push_back(get_current_text_position());
		}
		if (tag_name == "a") {
			const std::string href = element.attribute("href").as_string();
			const std::string link_text = get_element_text(element);
			if (!link_text.empty()) {
				const std::string processed_link_text = collapse_whitespace(link_text);
				const size_t link_offset = get_current_text_position();
				current_line += processed_link_text;
				links.push_back({.offset = link_offset, .text = processed_link_text, .ref = href});
				skip_children = true;
			}
		} else if (tag_name == "body") {
			in_body = true;
		} else if (tag_name == "pre") {
			finalize_current_line();
			preserve_whitespace = true;
		} else if (tag_name == "br") {
			finalize_current_line();
		} else if (tag_name == "li") {
			finalize_current_line();
			const std::string li_text = get_element_text(element);
			list_items.push_back({.offset = get_current_text_position(), .level = list_level, .text = li_text});
			current_line += std::string(list_level * 2, ' ');
			if (!list_style_stack.empty()) {
				auto& style = list_style_stack.top();
				if (style.ordered) {
					current_line += std::to_string(style.item_number++) + ". ";
				} else {
					current_line += get_bullet_for_level(list_level) + " ";
				}
			} else {
				current_line += get_bullet_for_level(list_level) + " ";
			}
		} else if (tag_name == "ul" || tag_name == "ol") {
			list_level++;
			list_style_info style;
			if (tag_name == "ol") {
				style.ordered = true;
			}
			list_style_stack.push(style);
			int item_count = 0;
			for (auto child : node.children()) {
				if (child.type() == pugi::node_element && get_local_name(child.name()) == "li") {
					item_count++;
				}
			}
			if (item_count > 0) {
				finalize_current_line();
				lists.push_back({.offset = get_current_text_position(), .item_count = item_count});
			}
		}
		if (in_body) {
			const std::string id = element.attribute("id").as_string();
			if (!id.empty()) {
				id_positions[id] = get_current_text_position();
			}
		}
		if (in_body && tag_name.length() == 2 && tag_name[0] == 'h' && tag_name[1] >= '1' && tag_name[1] <= '6') {
			const int level = tag_name[1] - '0';
			finalize_current_line();
			const size_t heading_offset = get_current_text_position();
			const std::string heading_text = get_element_text(element);
			if (!heading_text.empty()) {
				headings.push_back({.offset = heading_offset, .level = level, .text = trim_string(collapse_whitespace(heading_text))});
			}
		}
	} else if (node_type == pugi::node_pcdata || node_type == pugi::node_cdata) {
		process_text_node(node);
	}
	if (!skip_children) {
		for (auto child : node.children()) {
			process_node(child);
		}
	}
	if (node_type == pugi::node_element) {
		if (is_block_element(tag_name)) {
			finalize_current_line();
		}
		if (tag_name == "pre") {
			preserve_whitespace = false;
		}
		if (tag_name == "ul" || tag_name == "ol") {
			list_level--;
			if (!list_style_stack.empty()) {
				list_style_stack.pop();
			}
		}
	}
}

void xml_to_text::process_text_node(pugi::xml_node text_node) {
	if (!in_body || text_node == nullptr) {
		return;
	}
	const auto* text = text_node.value();
	if (text != nullptr && *text != '\0') {
		const std::string processed_text = remove_soft_hyphens(text);
		current_line += preserve_whitespace ? processed_text : collapse_whitespace(processed_text);
	}
}

void xml_to_text::add_line(std::string_view line) {
	std::string processed_line{};
	if (preserve_whitespace) {
		processed_line = std::string(line);
		while (!processed_line.empty() && (processed_line.back() == '\n' || processed_line.back() == '\r')) {
			processed_line.pop_back();
		}
		cached_char_length += wxString::FromUTF8(processed_line).length() + 1; // +1 for newline
		lines.emplace_back(std::move(processed_line));
	} else {
		processed_line = collapse_whitespace(line);
		processed_line = trim_string(processed_line);
		if (!processed_line.empty()) {
			cached_char_length += wxString::FromUTF8(processed_line).length() + 1; // +1 for newline
			lines.emplace_back(std::move(processed_line));
		}
	}
}

void xml_to_text::finalize_current_line() {
	add_line(current_line);
	current_line.clear();
}

size_t xml_to_text::get_current_text_position() const {
	std::string trimmed_line = current_line;
	while (!trimmed_line.empty() && trimmed_line.back() == ' ') {
		trimmed_line.pop_back();
	}
	return cached_char_length + wxString::FromUTF8(trimmed_line).length();
}

constexpr bool xml_to_text::is_block_element(std::string_view tag_name) noexcept {
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
	return std::ranges::find(block_elements, tag_name) != block_elements.end();
}

std::string xml_to_text::get_element_text(pugi::xml_node element) {
	if (element == nullptr) {
		return {};
	}
	std::string text;
	for (auto child : element.children()) {
		if (child.type() == pugi::node_pcdata || child.type() == pugi::node_cdata) {
			text += child.value();
		} else if (child.type() == pugi::node_element) {
			text += get_element_text(child);
		}
	}
	return text;
}
