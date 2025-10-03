/* chm_parser.cpp - parser for Compiled HTML Help files.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "chm_parser.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/Element.h>
#include <Poco/DOM/Node.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/String.h>
#include <algorithm>
#include <iostream>
#include <memory>
#include <set>
#include <sstream>
#include <wx/filename.h>
#include <wx/log.h>
#include <wx/msgdlg.h>

std::unique_ptr<document> chm_parser::load(const wxString& path) const {
	chmFile* file = nullptr;
	try {
		file = chm_open(path.ToStdString().c_str());
		if (!file) return nullptr;
		chm_context ctx(file);
		enumerate_files(ctx);
		parse_system_file(ctx);
		auto document_ptr = std::make_unique<document>();
		parse_hhc_file(ctx, document_ptr->toc_items);
		document_ptr->buffer.clear();
		parse_html_files(ctx, document_ptr->buffer, document_ptr->toc_items);
		document_ptr->flags = document_flags::supports_toc;
		if (!document_ptr->toc_items.empty()) calculate_toc_offsets(document_ptr->toc_items, ctx);
		if (!ctx.title.empty()) document_ptr->title = wxString::FromUTF8(ctx.title);
		chm_close(file);
		return document_ptr;
	} catch (const std::exception& e) {
		if (file) chm_close(file);
		wxMessageBox(wxString::FromUTF8(e.what()), "Error", wxICON_ERROR);
		return nullptr;
	} catch (...) {
		if (file) chm_close(file);
		wxMessageBox("Unknown error while parsing CHM file", "Error", wxICON_ERROR);
		return nullptr;
	}
}

void chm_parser::enumerate_files(chm_context& ctx) const {
	chm_enumerate(ctx.file, CHM_ENUMERATE_ALL, file_enumerator, &ctx);
	std::sort(ctx.html_files.begin(), ctx.html_files.end());
}

void chm_parser::parse_html_files(chm_context& ctx, document_buffer& buffer, const std::vector<std::unique_ptr<toc_item>>& toc_items) const {
	std::vector<std::string> ordered_files;
	std::map<std::string, std::string> toc_to_actual;
	for (const auto& file : ctx.html_files) {
		std::string normalized = normalize_path(file);
		toc_to_actual[normalized] = file;
	}
	if (!toc_items.empty()) {
		std::vector<std::string> toc_files;
		collect_html_files_from_toc(toc_items, toc_files);
		std::set<std::string> processed_files;
		for (const auto& toc_file : toc_files) {
			auto it = toc_to_actual.find(toc_file);
			if (it != toc_to_actual.end()) {
				ordered_files.push_back(it->second);
				processed_files.insert(it->first);
			}
		}
		for (const auto& [normalized, actual] : toc_to_actual)
			if (processed_files.find(normalized) == processed_files.end()) ordered_files.push_back(actual);
	} else
		ordered_files = ctx.html_files;
	for (size_t i = 0; i < ordered_files.size(); ++i) {
		const auto& file_path = ordered_files[i];
		size_t section_start = buffer.str().length();
		std::string content = read_file_content(ctx.file, file_path);
		if (content.empty()) continue;
		content = convert_to_utf8(content);
		html_to_text converter;
		if (!converter.convert(content)) continue;
		const auto& text = converter.get_text();
		const auto& headings = converter.get_headings();
		const auto& id_positions = converter.get_id_positions();
		std::string normalized_path = normalize_path(file_path);
		ctx.id_positions[normalized_path][""] = section_start;
		for (const auto& [id, relative_pos] : id_positions) ctx.id_positions[normalized_path][id] = section_start + relative_pos;
		wxString wx_text = wxString::FromUTF8(text);
		buffer.append(wx_text);
		for (const auto& heading : headings) {
			marker_type type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
			buffer.add_marker(section_start + heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
		}
		if (buffer.str().length() > 0 && !buffer.str().EndsWith("\n")) buffer.append("\n");
	}
}

std::string chm_parser::read_file_content(chmFile* file, const std::string& path) const {
	chmUnitInfo ui;
	if (chm_resolve_object(file, path.c_str(), &ui) != CHM_RESOLVE_SUCCESS) return "";
	if (ui.length == 0) return "";
	std::vector<unsigned char> buffer(static_cast<size_t>(ui.length));
	LONGINT64 bytes_read = chm_retrieve_object(file, &ui, buffer.data(), 0, ui.length);
	if (bytes_read != static_cast<LONGINT64>(ui.length)) return "";
	return std::string(buffer.begin(), buffer.end());
}

std::string chm_parser::normalize_path(const std::string& path) const {
	std::string result = path;
	std::replace(result.begin(), result.end(), '\\', '/');
	std::transform(result.begin(), result.end(), result.begin(), [](unsigned char c) { return std::tolower(c); });
	if (!result.empty() && result[0] != '/') result = "/" + result;
	return result;
}

void chm_parser::parse_system_file(chm_context& ctx) const {
	std::string system_content = read_file_content(ctx.file, "/#SYSTEM");
	if (system_content.length() < 4) return;
	size_t index = 4;
	while (index + 4 <= system_content.length()) {
		uint16_t code = static_cast<unsigned char>(system_content[index]) | (static_cast<unsigned char>(system_content[index + 1]) << 8);
		uint16_t length = static_cast<unsigned char>(system_content[index + 2]) | (static_cast<unsigned char>(system_content[index + 3]) << 8);
		if (index + 4 + length > system_content.length()) break;
		std::string data = system_content.substr(index + 4, length);
		if (code == 3 && !data.empty()) {
			if (data.back() == '\0') data.pop_back();
			ctx.title = data;
		}
		index += 4 + length;
	}
}

int chm_parser::file_enumerator(chmFile* h, chmUnitInfo* ui, void* context) {
	auto* ctx = static_cast<chm_context*>(context);
	std::string path(ui->path);
	std::string lower_path = path;
	std::transform(lower_path.begin(), lower_path.end(), lower_path.begin(), [](unsigned char c) { return std::tolower(c); });
	if (lower_path.find(".hhc") != std::string::npos) {
		if (ctx->hhc_file.empty() || lower_path.find("index.hhc") != std::string::npos) ctx->hhc_file = path;
	}
	if (lower_path.find(".htm") != std::string::npos || lower_path.find(".html") != std::string::npos) {
		if (path.find("/#") == std::string::npos && path.find("/$") == std::string::npos) ctx->html_files.push_back(path);
	}
	return CHM_ENUMERATOR_CONTINUE;
}

void chm_parser::parse_hhc_file(chm_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const {
	if (ctx.hhc_file.empty()) return;
	std::string hhc_content = read_file_content(ctx.file, ctx.hhc_file);
	if (hhc_content.empty()) return;
	hhc_content = convert_to_utf8(hhc_content);
	lxb_html_document_t* document = lxb_html_document_create();
	if (!document) return;
	lxb_status_t status = lxb_html_document_parse(document, reinterpret_cast<const lxb_char_t*>(hhc_content.data()), hhc_content.length());
	if (status != LXB_STATUS_OK) {
		lxb_html_document_destroy(document);
		return;
	}
	std::function<void(lxb_dom_node_t*, std::vector<std::unique_ptr<toc_item>>&)> parse_node;
	parse_node = [&](lxb_dom_node_t* node, std::vector<std::unique_ptr<toc_item>>& items) {
		toc_item* last_item = nullptr;
		for (lxb_dom_node_t* child = node->first_child; child != nullptr; child = child->next) {
			if (child->type != LXB_DOM_NODE_TYPE_ELEMENT) continue;
			lxb_dom_element_t* element = lxb_dom_interface_element(child);
			size_t tag_name_len;
			const lxb_char_t* tag_name = lxb_dom_element_qualified_name(element, &tag_name_len);
			if (!tag_name) continue;
			std::string tag_str(reinterpret_cast<const char*>(tag_name), tag_name_len);
			std::transform(tag_str.begin(), tag_str.end(), tag_str.begin(), [](unsigned char c) { return std::tolower(c); });
			if (tag_str == "li") {
				auto item = std::make_unique<toc_item>();
				item->offset = -1;
				std::string name_str, local_str;
				for (lxb_dom_node_t* li_child = child->first_child; li_child != nullptr; li_child = li_child->next) {
					if (li_child->type != LXB_DOM_NODE_TYPE_ELEMENT) continue;
					lxb_dom_element_t* li_elem = lxb_dom_interface_element(li_child);
					const lxb_char_t* li_tag_name = lxb_dom_element_qualified_name(li_elem, &tag_name_len);
					if (!li_tag_name) continue;
					std::string li_tag(reinterpret_cast<const char*>(li_tag_name), tag_name_len);
					std::transform(li_tag.begin(), li_tag.end(), li_tag.begin(), [](unsigned char c) { return std::tolower(c); });
					if (li_tag == "object") {
						for (lxb_dom_node_t* param_node = li_child->first_child; param_node != nullptr; param_node = param_node->next) {
							if (param_node->type != LXB_DOM_NODE_TYPE_ELEMENT) continue;
							lxb_dom_element_t* param_elem = lxb_dom_interface_element(param_node);
							const lxb_char_t* param_tag = lxb_dom_element_qualified_name(param_elem, &tag_name_len);
							if (!param_tag) continue;
							std::string param_tag_str(reinterpret_cast<const char*>(param_tag), tag_name_len);
							std::transform(param_tag_str.begin(), param_tag_str.end(), param_tag_str.begin(), [](unsigned char c) { return std::tolower(c); });
							if (param_tag_str == "param") {
								size_t attr_len;
								const lxb_char_t* name_attr = lxb_dom_element_get_attribute(param_elem, reinterpret_cast<const lxb_char_t*>("name"), 4, &attr_len);
								const lxb_char_t* value_attr = lxb_dom_element_get_attribute(param_elem, reinterpret_cast<const lxb_char_t*>("value"), 5, &attr_len);
								if (name_attr && value_attr) {
									std::string attr_name(reinterpret_cast<const char*>(name_attr));
									std::string attr_value(reinterpret_cast<const char*>(value_attr), attr_len);
									std::transform(attr_name.begin(), attr_name.end(), attr_name.begin(), [](unsigned char c) { return std::tolower(c); });
									if (attr_name == "name")
										name_str = attr_value;
									else if (attr_name == "local")
										local_str = attr_value;
								}
							}
						}
					} else if (li_tag == "ul")
						parse_node(li_child, item->children);
				}
				if (!name_str.empty()) {
					item->name = wxString::FromUTF8(name_str);
					item->ref = wxString::FromUTF8(local_str);
					items.push_back(std::move(item));
					last_item = items.back().get();
				}
			} else if (tag_str == "ul") {
				if (last_item)
					parse_node(child, last_item->children);
				else
					parse_node(child, items);
			}
		}
	};
	lxb_dom_node_t* body = lxb_dom_interface_node(lxb_html_document_body_element(document));
	if (body) parse_node(body, toc_items);
	lxb_html_document_destroy(document);
}

void chm_parser::collect_html_files_from_toc(const std::vector<std::unique_ptr<toc_item>>& items, std::vector<std::string>& files) const {
	for (const auto& item : items) {
		if (!item->ref.IsEmpty()) {
			std::string file_path = item->ref.ToStdString();
			size_t fragment_pos = file_path.find('#');
			if (fragment_pos != std::string::npos) file_path = file_path.substr(0, fragment_pos);
			file_path = normalize_path(file_path);
			if (std::find(files.begin(), files.end(), file_path) == files.end()) files.push_back(file_path);
		}
		collect_html_files_from_toc(item->children, files);
	}
}

int chm_parser::calculate_offset_from_path(const std::string& path, const chm_context& ctx) const {
	std::string file_path = path;
	std::string fragment_id;
	size_t fragment_pos = path.find('#');
	if (fragment_pos != std::string::npos) {
		file_path = path.substr(0, fragment_pos);
		fragment_id = path.substr(fragment_pos + 1);
	}
	file_path = normalize_path(file_path);
	auto it = ctx.id_positions.find(file_path);
	if (it == ctx.id_positions.end()) return -1;
	if (!fragment_id.empty()) {
		auto id_it = it->second.find(fragment_id);
		if (id_it != it->second.end()) return static_cast<int>(id_it->second);
		return -1;
	}
	auto start_it = it->second.find("");
	if (start_it != it->second.end()) return static_cast<int>(start_it->second);
	return -1;
}

void chm_parser::calculate_toc_offsets(std::vector<std::unique_ptr<toc_item>>& items, const chm_context& ctx) const {
	for (auto& item : items) {
		if (!item->ref.IsEmpty()) item->offset = calculate_offset_from_path(item->ref.ToStdString(), ctx);
		calculate_toc_offsets(item->children, ctx);
	}
}
