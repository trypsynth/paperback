/* xml_to_text.hpp - XML to plain text conversion header file.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#pragma once
#include "document.hpp"
#include "html_to_text.hpp" // For link_info struct
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/Text.h>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

class xml_to_text {
public:
	xml_to_text() = default;
	~xml_to_text() = default;
	xml_to_text(const xml_to_text&) = delete;
	xml_to_text& operator=(const xml_to_text&) = delete;
	xml_to_text(xml_to_text&&) = default;
	xml_to_text& operator=(xml_to_text&&) = default;
	[[nodiscard]] bool convert(const std::string& xml_content);

	[[nodiscard]] const std::vector<std::string>& get_lines() const noexcept {
		return lines;
	}

	[[nodiscard]] std::string get_text() const;

	[[nodiscard]] const std::unordered_map<std::string, size_t>& get_id_positions() const noexcept {
		return id_positions;
	}

	[[nodiscard]] const std::vector<heading_info>& get_headings() const noexcept {
		return headings;
	}

	[[nodiscard]] const std::vector<link_info>& get_links() const noexcept {
		return links;
	}

	[[nodiscard]] const std::vector<size_t>& get_section_offsets() const noexcept {
		return section_offsets;
	}

	void clear() noexcept;

private:
	std::vector<std::string> lines{};
	std::string current_line{};
	std::unordered_map<std::string, size_t> id_positions{};
	std::vector<heading_info> headings{};
	std::vector<link_info> links{};
	std::vector<size_t> section_offsets{};
	bool in_body{false};
	bool preserve_whitespace{false};
	size_t cached_char_length{0};

	void process_node(Poco::XML::Node* node);
	void process_text_node(Poco::XML::Text* text_node);
	void add_line(std::string_view line);
	void finalize_current_line();
	size_t get_current_text_position() const;
	[[nodiscard]] static constexpr bool is_block_element(std::string_view tag_name) noexcept;
	[[nodiscard]] static std::string get_element_text(Poco::XML::Element* element);
};
