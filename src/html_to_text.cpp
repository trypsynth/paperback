#include "html_to_text.hpp"
#include <cctype>
#include <sstream>
#include <unordered_map>
#include <unordered_set>

html_to_text::html_to_text() :in_body{false}, in_paragraph{false}, doc{nullptr} {
	doc = lxb_html_document_create();
	if (!doc) throw std::runtime_error("Failed to create Lexbor HTML document");
}

html_to_text::~html_to_text() {
	if (doc) lxb_html_document_destroy(doc);
}

bool html_to_text::convert(const std::string& html_content) {
	lines.clear();
	current_line.clear();
	in_body = false;
	in_paragraph = false;
	lxb_status_t status = lxb_html_document_parse(doc, reinterpret_cast<const lxb_char_t*>(html_content.c_str()), html_content.length());
	if (status != LXB_STATUS_OK) return false;
	lxb_dom_node_t* node = lxb_dom_interface_node(doc);
	process_node(node);
	if (!current_line.empty()) {
		add_line(current_line);
		current_line.clear();
	}
	return true;
}

std::string html_to_text::get_text() const {
	std::ostringstream oss;
	for (size_t i = 0; i < lines.size(); ++i) {
		if (i > 0) oss << "\n";
		oss << lines[i];
	}
	return oss.str();
}

void html_to_text::process_node(lxb_dom_node_t* node) {
	if (!node) return;
	switch (node->type) {
		case LXB_DOM_NODE_TYPE_ELEMENT:
			process_element_node(lxb_dom_interface_element(node));
			break;
		case LXB_DOM_NODE_TYPE_TEXT:
			process_text_node(lxb_dom_interface_text(node));
			break;
		default:
			break;
	}
	lxb_dom_node_t* child = node->first_child;
	while (child != nullptr) {
		process_node(child);
		child = child->next;
	}
	if (node->type == LXB_DOM_NODE_TYPE_ELEMENT) {
		lxb_dom_element_t* element = lxb_dom_interface_element(node);
		std::string tag_name = get_tag_name(element);
		if (tag_name == "p" || tag_name == "div" || tag_name == "h1" || tag_name == "h2" || tag_name == "h3" || tag_name == "h4" || tag_name == "h5" || tag_name == "h6" || tag_name == "br") {
			add_line(current_line);
			current_line.clear();
		}
		if (tag_name == "p" || tag_name == "div") in_paragraph = false;
	}
}

void html_to_text::process_element_node(lxb_dom_element_t* element) {
	std::string tag_name = get_tag_name(element);
	if (tag_name == "body") in_body = true;
	else if (tag_name == "p" || tag_name == "div") in_paragraph = true;
	if (tag_name == "br") {
		add_line(current_line);
		current_line.clear();
	}
}

void html_to_text::process_text_node(lxb_dom_text_t* text_node) {
	if (!in_body) return;
	size_t length;
	const lxb_char_t* text_data = lxb_dom_node_text_content(lxb_dom_interface_node(text_node), &length);
	if (text_data && length > 0) {
		std::string text(reinterpret_cast<const char*>(text_data), length);
		if (!text.empty()) current_line += text;
	}
}

std::string html_to_text::get_tag_name(lxb_dom_element_t* element) {
	const lxb_char_t* name = lxb_dom_element_qualified_name(element, nullptr);
	if (!name) return "";
	return std::string(reinterpret_cast<const char*>(name));
}

void html_to_text::add_line(const std::string& line) {
	if (line.empty()) {
		lines.push_back("");
		return;
	}
	std::string processed_line = in_paragraph ? collapse_whitespace(line) : line;
	lines.push_back(processed_line);
}

std::string html_to_text::collapse_whitespace(const std::string& input) {
	std::ostringstream oss;
	bool in_space = false;
	for (char ch : input) {
		if (std::isspace(static_cast<unsigned char>(ch))) {
			if (!in_space) {
				oss << ' ';
				in_space = true;
			}
		} else {
			oss << ch;
			in_space = false;
		}
	}
	return oss.str();
}
