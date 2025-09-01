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
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/Exception.h>
#include <Poco/SAX/InputSource.h>
#include <algorithm>
#include <array>
#include <sstream>
#include <string_view>

using namespace Poco::XML;

bool xml_to_text::convert(const std::string& xml_content) {
	clear();
	try {
		std::istringstream iss(xml_content);
		InputSource src(iss);
		DOMParser parser;
		parser.setFeature(XMLReader::FEATURE_NAMESPACES, true);
		parser.setFeature(XMLReader::FEATURE_NAMESPACE_PREFIXES, false);
		auto doc = parser.parse(&src);
		if (doc) {
			process_node(doc);
			finalize_current_line();
			return true;
		}
	} catch (const Poco::Exception&) {
		clear();
		return false;
	}
	return false;
}

std::string xml_to_text::get_text() const {
	if (lines.empty()) return {};
	std::ostringstream oss;
	for (const auto& line : lines)
		oss << line << '\n';
	auto result = oss.str();
	if (!result.empty()) result.pop_back();
	return result;
}

void xml_to_text::clear() noexcept {
	lines.clear();
	current_line.clear();
	id_positions.clear();
	headings.clear();
	in_body = false;
	preserve_whitespace = false;
}

void xml_to_text::process_node(Node* node) {
	if (!node) return;
	const auto node_type = node->nodeType();
	std::string tag_name;
	if (node_type == Node::ELEMENT_NODE) {
		auto* element = static_cast<Element*>(node);
		tag_name = element->localName();
		std::transform(tag_name.begin(), tag_name.end(), tag_name.begin(), ::tolower);
		if (tag_name == "body")
			in_body = true;
		else if (tag_name == "pre")
			preserve_whitespace = true;
		else if (tag_name == "br" || tag_name == "li")
			finalize_current_line();
		if (in_body && element->hasAttributeNS("", "id")) {
			std::string id = element->getAttributeNS("", "id");
			if (!id.empty()) {
				size_t total_length = 0;
				for (const auto& line : lines) total_length += line.length() + 1;
				id_positions[id] = total_length;
			}
		}
		if (in_body && tag_name.length() == 2 && tag_name[0] == 'h' && tag_name[1] >= '1' && tag_name[1] <= '6') {
			int level = tag_name[1] - '0';
			finalize_current_line();
			size_t heading_offset = get_current_text_position();
			std::string heading_text = get_element_text(element);
			if (!heading_text.empty()) headings.push_back({heading_offset, level, heading_text});
		}
	} else if (node_type == Node::TEXT_NODE)
		process_text_node(static_cast<Text*>(node));
	auto* child = node->firstChild();
	while (child) {
		process_node(child);
		child = child->nextSibling();
	}
	if (node_type == Node::ELEMENT_NODE) {
		if (is_block_element(tag_name)) finalize_current_line();
		if (tag_name == "pre") preserve_whitespace = false;
	}
}

void xml_to_text::process_text_node(Text* text_node) {
	if (!in_body || !text_node) return;
	const auto text = text_node->data();
	if (!text.empty()) {
		std::string processed_text = remove_soft_hyphens(text);
		current_line += preserve_whitespace ? processed_text : collapse_whitespace(processed_text);
	}
}

void xml_to_text::add_line(std::string_view line) {
	std::string processed_line;
	processed_line = preserve_whitespace ? std::string(line) : collapse_whitespace(line);
	processed_line = trim_string(processed_line);
	if (!processed_line.empty()) lines.emplace_back(std::move(processed_line));
}

void xml_to_text::finalize_current_line() {
	add_line(current_line);
	current_line.clear();
}

size_t xml_to_text::get_current_text_position() const {
	size_t total_length = 0;
	for (const auto& line : lines) total_length += line.length() + 1;
	total_length += current_line.length();
	return total_length;
}

constexpr bool xml_to_text::is_block_element(std::string_view tag_name) noexcept {
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
		"th"
	};
	return std::find(block_elements.begin(), block_elements.end(), tag_name) != block_elements.end();
}

std::string xml_to_text::get_element_text(Element* element) noexcept {
	if (!element) return {};
	std::string text;
	auto* child = element->firstChild();
	while (child) {
		if (child->nodeType() == Node::TEXT_NODE) {
			auto* text_node = static_cast<Text*>(child);
			text += text_node->data();
		} else if (child->nodeType() == Node::ELEMENT_NODE)
			text += get_element_text(static_cast<Element*>(child));
		child = child->nextSibling();
	}
	return collapse_whitespace(trim_string(text));
}
