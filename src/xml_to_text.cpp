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
	if (!text.empty()) current_line += preserve_whitespace ? text : collapse_whitespace(text);
}

void xml_to_text::add_line(std::string_view line) {
	std::string processed_line;
	if (preserve_whitespace) processed_line = std::string(line);
	else {
		processed_line = collapse_whitespace(line);
		processed_line = trim_string(processed_line);
	}
	if (!processed_line.empty()) lines.emplace_back(std::move(processed_line));
}

void xml_to_text::finalize_current_line() {
	add_line(current_line);
	current_line.clear();
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
		"th"};
	return std::find(block_elements.begin(), block_elements.end(), tag_name) != block_elements.end();
}
