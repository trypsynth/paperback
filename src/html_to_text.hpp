#pragma once
#include <lexbor/html/html.h>
#include <memory>
#include <string>
#include <vector>

class html_to_text {
public:
	html_to_text();
	~html_to_text() = default;
	html_to_text(const html_to_text&) = delete;
	html_to_text& operator=(const html_to_text&) = delete;
	html_to_text(html_to_text&&) = default;
	html_to_text& operator=(html_to_text&&) = default;
	[[nodiscard]] bool convert(const std::string& html_content);
	[[nodiscard]] const std::vector<std::string>& get_lines() const noexcept { return lines; }
	[[nodiscard]] std::string get_text() const;
	void clear() noexcept;

private:
	struct DocumentDeleter {
		void operator()(lxb_html_document_t* doc) const noexcept {
			if (doc) lxb_html_document_destroy(doc);
		}
	};
	using DocumentPtr = std::unique_ptr<lxb_html_document_t, DocumentDeleter>;

	std::vector<std::string> lines;
	std::string current_line;
	bool in_body = false;
	bool preserve_whitespace = false;
	DocumentPtr doc;

	void process_node(lxb_dom_node_t* node);
	void process_text_node(lxb_dom_text_t* text_node);
	void add_line(std::string_view line);
	void finalize_current_line();
	void finalize_text(); // New method for final cleanup
	[[nodiscard]] static constexpr bool is_block_element(std::string_view tag_name) noexcept;
	[[nodiscard]] static std::string_view get_tag_name(lxb_dom_element_t* element) noexcept;
};
