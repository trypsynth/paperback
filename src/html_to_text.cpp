#include "html_to_text.hpp"
#include <sstream>
#include "utils.hpp"

html_to_text::html_to_text() :doc{lxb_html_document_create()} {
	if (!doc) throw std::runtime_error("Failed to create Lexbor HTML document");
}

html_to_text::~html_to_text() {
	if (doc) lxb_html_document_destroy(doc);
}

bool html_to_text::convert(const std::string& html_content) {
	lines.clear();
	current_line.clear();
	in_body = false;
	preserve_whitespace = false;
	if (lxb_status_t status = lxb_html_document_parse(doc, reinterpret_cast<const lxb_char_t*>(html_content.c_str()), html_content.length()); status != LXB_STATUS_OK) return false;
	lxb_dom_node_t* node = lxb_dom_interface_node(doc);
	process_node(node);
	if (!current_line.empty()) {
		add_line(current_line);
		current_line.clear();
	}
	return true;
}

std::string html_to_text::get_text() const {
	if (lines.empty()) return {};
	std::ostringstream oss;
	for (const auto& line : lines)
		oss << line << '\n';
	std::string result = oss.str();
	result.pop_back();
	return result;
}

void html_to_text::process_node(lxb_dom_node_t* node) {
	if (!node) return;
	std::string_view tag_name;
	if (node->type == LXB_DOM_NODE_TYPE_ELEMENT) {
		auto* element = lxb_dom_interface_element(node);
		tag_name = get_tag_name(element);
	}
	switch (node->type) {
		case LXB_DOM_NODE_TYPE_ELEMENT:
			if (tag_name == "body") in_body = true;
			if (tag_name == "pre") preserve_whitespace = true;
			if (tag_name == "br") {
				add_line(current_line);
				current_line.clear();
			}
			break;
		case LXB_DOM_NODE_TYPE_TEXT:
			process_text_node(lxb_dom_interface_text(node));
			break;
		default:
			break;
	}
	for (auto* child = node->first_child; child; child = child->next)
		process_node(child);
	if (node->type == LXB_DOM_NODE_TYPE_ELEMENT) {
		if (is_block_element(tag_name)) {
			add_line(current_line);
			current_line.clear();
		}
		if (tag_name == "pre") preserve_whitespace = false;
	}
}

void html_to_text::process_text_node(lxb_dom_text_t* text_node) {
	if (!in_body) return;
	size_t length;
	const auto* text_data = lxb_dom_node_text_content(lxb_dom_interface_node(text_node), &length);
	if (text_data && length > 0) {
		std::string text(reinterpret_cast<const char*>(text_data), length);
		if (!text.empty()) current_line += preserve_whitespace ? text : collapse_whitespace(text);
	}
}

void html_to_text::add_line(std::string_view line) {
	if (line.empty()) return;
	lines.emplace_back(line);
}

bool html_to_text::is_block_element(std::string_view tag_name) const noexcept {
	switch (tag_name[0]) {
		case 'd': return tag_name == "div";
		case 'h': return tag_name == "h1" || tag_name == "h2" || tag_name == "h3" || tag_name == "h4" || tag_name == "h5" || tag_name == "h6";
		case 'p': return tag_name == "p" || tag_name == "pre";
		default: return false;
	}
}

std::string_view html_to_text::get_tag_name(lxb_dom_element_t* element) const noexcept {
	const lxb_char_t* name = lxb_dom_element_qualified_name(element, nullptr);
	if (!name) return {};
	return {reinterpret_cast<const char*>(name)};
}
