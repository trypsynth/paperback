#pragma once
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/Text.h>
#include <memory>
#include <string>
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
	[[nodiscard]] const std::vector<std::string>& get_lines() const noexcept { return lines; }
	[[nodiscard]] std::string get_text() const;
	void clear() noexcept;

private:
	std::vector<std::string> lines;
	std::string current_line;
	bool in_body = false;
	bool preserve_whitespace = false;

	void process_node(Poco::XML::Node* node);
	void process_text_node(Poco::XML::Text* text_node);
	void add_line(std::string_view line);
	void finalize_current_line();
	void finalize_text();
	[[nodiscard]] static constexpr bool is_block_element(std::string_view tag_name) noexcept;
};
