#include "html_to_text.hpp"
#include <sstream>
#include <string_view>
#include <stdexcept>
#include <array>
#include <algorithm>
#include "utils.hpp"

html_to_text::html_to_text() : doc_(lxb_html_document_create()) {
	if (!doc_) throw std::runtime_error("Failed to create Lexbor HTML document");
}

bool html_to_text::convert(const std::string& html_content) {
	clear();
	const auto status = lxb_html_document_parse(doc_.get(), reinterpret_cast<const lxb_char_t*>(html_content.data()), html_content.length());
	if (status != LXB_STATUS_OK) return false;
	if (auto* node = lxb_dom_interface_node(doc_.get())) process_node(node);
	finalize_current_line();
	return true;
}

std::string html_to_text::get_text() const {
	if (lines_.empty()) return {};
	std::ostringstream oss;
	for (const auto& line : lines_) oss << line << '\n';
	auto result = oss.str();
	if (!result.empty()) result.pop_back(); // Remove trailing newline
	return result;
}

void html_to_text::clear() noexcept {
	lines_.clear();
	current_line_.clear();
	in_body_ = false;
	preserve_whitespace_ = false;
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
		case LXB_DOM_NODE_TYPE_ELEMENT:
			if (tag_name == "body") in_body_ = true;
			else if (tag_name == "pre")preserve_whitespace_ = true;
			else if (tag_name == "br")  finalize_current_line();
			break;
		case LXB_DOM_NODE_TYPE_TEXT:
			process_text_node(lxb_dom_interface_text(node));
			break;
		default:
			break;
	}
	for (auto* child = node->first_child; child; child = child->next)
		process_node(child);
	if (is_element) {
		if (is_block_element(tag_name)) finalize_current_line();
		if (tag_name == "pre") preserve_whitespace_ = false;
	}
}

void html_to_text::process_text_node(lxb_dom_text_t* text_node) {
	if (!in_body_) return;
	size_t length;
	const auto* text_data = lxb_dom_node_text_content(lxb_dom_interface_node(text_node), &length);
	if (!text_data || length == 0) return;
	const std::string_view text{reinterpret_cast<const char*>(text_data), length};
	if (!text.empty())
		if (preserve_whitespace_)
			current_line_ += text;
		else
			current_line_ += collapse_whitespace(std::string{text});
}

void html_to_text::add_line(std::string_view line) {
	if (!line.empty()) lines_.emplace_back(line);
}

void html_to_text::finalize_current_line() {
	add_line(current_line_);
	current_line_.clear();
}

constexpr bool html_to_text::is_block_element(std::string_view tag_name) noexcept {
	if (tag_name.empty()) return false;
	constexpr std::array block_elements = {
		"div", "p", "pre", "h1", "h2", "h3", "h4", "h5", "h6",
		"blockquote", "ul", "ol", "li", "section", "article", "header", "footer"
	};
	return std::find(block_elements.begin(), block_elements.end(), tag_name) != block_elements.end();
}

std::string_view html_to_text::get_tag_name(lxb_dom_element_t* element) noexcept {
	if (!element) return {};
	const auto* name = lxb_dom_element_qualified_name(element, nullptr);
	return name ? std::string_view{reinterpret_cast<const char*>(name)} : std::string_view{};
}
