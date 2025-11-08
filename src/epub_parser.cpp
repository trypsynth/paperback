/* epub_parser.cpp - parser for Epub 2/3 ebooks.
 *
 * Paperback.
 * Copyright (c) 2025 Quin Gillespie.
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#include "epub_parser.hpp"
#include "document.hpp"
#include "document_buffer.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include "xml_to_text.hpp"
#include <algorithm>
#include <cstddef>
#include <filesystem>
#include <iterator>
#include <limits>
#include <map>
#include <memory>
#include <pugixml.hpp>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <wx/filename.h>
#include <wx/string.h>
#include <wx/translation.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

namespace fs = std::filesystem;

std::unique_ptr<document> epub_parser::load(const wxString& path) const {
	auto fp = std::make_unique<wxFileInputStream>(path);
	if (!fp->IsOk()) {
		return nullptr;
	}
	wxZipInputStream zip_index(*fp);
	std::map<std::string, std::unique_ptr<wxZipEntry>> entries;
	while (wxZipEntry* entry = zip_index.GetNextEntry()) {
		const std::string name = entry->GetName(wxPATH_UNIX).ToStdString();
		entries[name] = std::unique_ptr<wxZipEntry>(entry);
	}
	fp->SeekI(0);
	epub_context ctx(*fp);
	ctx.zip_entries = std::move(entries);
	wxZipEntry* container_entry = find_zip_entry("META-INF/container.xml", ctx.zip_entries);
	if (container_entry == nullptr) {
		return nullptr;
	}
	wxZipInputStream container_zip(*fp);
	if (!container_zip.OpenEntry(*container_entry)) {
		return nullptr;
	}
	const std::string container_content = read_zip_entry(container_zip);
	pugi::xml_document doc;
	if (!doc.load_buffer(container_content.data(), container_content.size())) {
		return nullptr;
	}
	std::string opf_filename;
	for (auto rootfile : doc.select_nodes("/container:container/container:rootfiles/container:rootfile")) {
		opf_filename = rootfile.node().attribute("full-path").as_string();
		break;
	}
	if (opf_filename.empty()) {
		auto c = doc.child("container").child("rootfiles").child("rootfile");
		opf_filename = c.attribute("full-path").as_string();
		if (opf_filename.empty()) {
			return nullptr;
		}
	}
	auto slashpos = opf_filename.find_last_of('/');
	ctx.opf_dir = (slashpos == std::string::npos) ? std::string() : opf_filename.substr(0, slashpos);
	parse_opf(opf_filename, ctx);
	auto document_ptr = std::make_unique<document>();
	document_ptr->buffer.clear();
	for (size_t i = 0; i < ctx.spine_items.size(); ++i) {
		document_ptr->buffer.add_section_break(wxString::Format("Section %zu", i + 1));
		parse_section(i, ctx, document_ptr->buffer);
	}
	document_ptr->buffer.finalize_markers();
	document_ptr->title = wxString::FromUTF8(ctx.title);
	if (!ctx.author.empty()) {
		document_ptr->author = wxString::FromUTF8(ctx.author);
	}
	for (const auto& [section_href, id_map] : ctx.id_positions) {
		for (const auto& [id, pos] : id_map) {
			document_ptr->id_positions[id] = pos;
		}
	}
	document_ptr->spine_items = ctx.spine_items;
	for (const auto& [id, item] : ctx.manifest_items) {
		document_ptr->manifest_items[id] = item.path;
	}
	parse_toc(ctx, document_ptr->toc_items, document_ptr->buffer);
	return document_ptr;
}

void epub_parser::parse_opf(const std::string& filename, epub_context& ctx) {
	wxZipEntry* opf_entry = find_zip_entry(filename, ctx.zip_entries);
	if (opf_entry == nullptr) {
		throw parser_exception("No OPF file found");
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*opf_entry)) {
		throw parser_exception("Failed to open OPF file");
	}
	const std::string opf_content = read_zip_entry(zis);
	pugi::xml_document doc;
	if (!doc.load_buffer(opf_content.data(), opf_content.size())) {
		throw parser_exception("Invalid OPF");
	}
	auto package = doc.child("package");
	if (package == nullptr) {
		package = doc.first_child();
	}
	if (auto metadata = package.child("metadata")) {
		for (auto child : metadata.children()) {
			std::string name = child.name();
			auto pos = name.find(':');
			if (pos != std::string::npos) {
				name = name.substr(pos + 1);
			}
			if (name == "title" && ctx.title.empty()) {
				ctx.title = child.text().as_string();
			} else if (name == "creator" && ctx.author.empty()) {
				ctx.author = child.text().as_string();
			}
		}
	}
	auto manifest = package.child("manifest");
	if (manifest == nullptr) {
		throw parser_exception("No manifest");
	}
	for (auto item_node : manifest.children("item")) {
		const std::string href = item_node.attribute("href").as_string();
		const std::string id = item_node.attribute("id").as_string();
		const std::string media_type = item_node.attribute("media-type").as_string();
		const std::string properties = item_node.attribute("properties").as_string();
		std::string full = ctx.opf_dir.empty() ? href : (ctx.opf_dir + "/" + href);
		manifest_item item;
		item.path = full;
		item.media_type = media_type;
		ctx.manifest_items.emplace(id, std::move(item));
		if (media_type == "application/x-dtbncx+xml") {
			ctx.toc_ncx_id = id;
		} else if (properties.find("nav") != std::string::npos) {
			ctx.nav_doc_id = id;
		}
	}
	auto spine = package.child("spine");
	if (spine == nullptr) {
		throw parser_exception("No spine");
	}
	if (ctx.toc_ncx_id.empty()) {
		const auto toc_attr = spine.attribute("toc").as_string();
		if (*toc_attr) {
			ctx.toc_ncx_id = toc_attr;
		}
	}
	for (auto itemref : spine.children("itemref")) {
		ctx.spine_items.push_back(itemref.attribute("idref").as_string());
	}
}

template <typename conv>
void epub_parser::process_section_content(conv& converter, const std::string& content, const std::string& href, epub_context& ctx, document_buffer& buffer) const {
	if (converter.convert(content)) {
		const auto& text = converter.get_text();
		const auto& headings = converter.get_headings();
		const auto& links = converter.get_links();
		const auto& lists = converter.get_lists();
		const auto& list_items = converter.get_list_items();
		const auto& id_positions = converter.get_id_positions();
		const size_t section_start = buffer.str().length();
		std::string section_base_dir = href;
		auto pos = section_base_dir.find_last_of('/');
		section_base_dir = (pos == std::string::npos) ? std::string() : section_base_dir.substr(0, pos);
		for (const auto& [id, relative_pos] : id_positions) {
			ctx.id_positions[href][id] = section_start + relative_pos;
		}
		buffer.append(wxString::FromUTF8(text));
		for (const auto& heading : headings) {
			const auto type = static_cast<marker_type>(static_cast<int>(marker_type::heading_1) + heading.level - 1);
			buffer.add_marker(section_start + heading.offset, type, wxString::FromUTF8(heading.text), wxString(), heading.level);
		}
		for (const auto& link : links) {
			wxString resolved_href;
			const wxString href_lower = wxString(link.ref).Lower();
			if (href_lower.StartsWith("http:") || href_lower.StartsWith("https:") || href_lower.StartsWith("mailto:")) {
				resolved_href = link.ref;
			} else if (!link.ref.empty() && link.ref[0] == '#') {
				resolved_href = link.ref;
			} else {
				resolved_href = section_base_dir.empty() ? link.ref : (section_base_dir + "/" + link.ref);
			}
			buffer.add_link(section_start + link.offset, wxString::FromUTF8(link.text), resolved_href);
		}
		for (const auto& list : lists) {
			buffer.add_marker(section_start + list.offset, marker_type::list, wxString(), wxString(), list.item_count);
		}
		for (const auto& list_item : list_items) {
			buffer.add_marker(section_start + list_item.offset, marker_type::list_item, wxString::FromUTF8(list_item.text), wxString(), list_item.level);
		}
		if (!buffer.str().empty() && !buffer.str().EndsWith("\n")) {
			buffer.append("\n");
		}
	}
}

void epub_parser::parse_section(size_t index, epub_context& ctx, document_buffer& buffer) const {
	if (index >= ctx.spine_items.size()) {
		throw parser_exception("Section index out of range");
	}
	const auto& id = ctx.spine_items[index];
	auto it = ctx.manifest_items.find(id);
	if (it == ctx.manifest_items.end()) {
		return;
	}
	const auto& manifest_item = it->second;
	const auto& href = manifest_item.path;
	const auto& media_type = manifest_item.media_type;
	wxZipEntry* section_entry = find_zip_entry(href, ctx.zip_entries);
	if (section_entry == nullptr) {
		return;
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*section_entry)) {
		throw parser_exception(wxString::FromUTF8("Failed to open section file: " + href));
	}
	const std::string content = read_zip_entry(zis);
	if (is_html_content(media_type)) {
		html_to_text converter;
		process_section_content(converter, content, href, ctx, buffer);
	} else {
		xml_to_text converter;
		if (!converter.convert(content)) {
			// The file may contain HTML syntax despite claiming to be XML; try parsing as HTML instead.
			html_to_text html_converter;
			process_section_content(html_converter, content, href, ctx, buffer);
		} else {
			process_section_content(converter, content, href, ctx, buffer);
		}
	}
}

bool epub_parser::is_html_content(const std::string& media_type) {
	return media_type == "text/html";
}

void epub_parser::parse_toc(epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const {
	try {
		if (!ctx.nav_doc_id.empty()) {
			parse_epub3_nav(ctx.nav_doc_id, ctx, toc_items, buffer);
		} else if (!ctx.toc_ncx_id.empty()) {
			parse_epub2_ncx(ctx.toc_ncx_id, ctx, toc_items, buffer);
		}
	} catch (...) {
		throw parser_exception(_("Couldn't parse table of contents"), error_severity::warning);
	}
}

void epub_parser::parse_epub2_ncx(const std::string& ncx_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const {
	auto it = ctx.manifest_items.find(ncx_id);
	if (it == ctx.manifest_items.end()) {
		return;
	}
	const auto& ncx_file = it->second.path;
	wxZipEntry* ncx_entry = find_zip_entry(ncx_file, ctx.zip_entries);
	if (ncx_entry == nullptr) {
		return;
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*ncx_entry)) {
		return;
	}
	const std::string ncx_content = read_zip_entry(zis);
	pugi::xml_document doc;
	if (!doc.load_buffer(ncx_content.data(), ncx_content.size())) {
		return;
	}
	auto nav_map = doc.child("ncx").child("navMap");
	if (nav_map == nullptr) {
		return;
	}
	for (auto nav_point : nav_map.children("navPoint")) {
		auto toc_entry = parse_ncx_nav_point(nav_point, ctx, buffer);
		if (toc_entry) {
			toc_items.push_back(std::move(toc_entry));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_ncx_nav_point(pugi::xml_node nav_point, const epub_context& ctx, const document_buffer& buffer) const {
	auto item = std::make_unique<toc_item>();
	auto nav_label = nav_point.child("navLabel").child("text");
	if (nav_label != nullptr) {
		item->name = wxString::FromUTF8(nav_label.text().as_string());
	}
	auto content = nav_point.child("content");
	if (content != nullptr) {
		std::string src = content.attribute("src").as_string();
		item->ref = wxString::FromUTF8(src);
		item->offset = calculate_offset_from_href(src, ctx, buffer);
	}
	for (auto child_np : nav_point.children("navPoint")) {
		auto child_item = parse_ncx_nav_point(child_np, ctx, buffer);
		if (child_item) {
			item->children.push_back(std::move(child_item));
		}
	}
	return item;
}

void epub_parser::parse_epub3_nav(const std::string& nav_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items, const document_buffer& buffer) const {
	auto it = ctx.manifest_items.find(nav_id);
	if (it == ctx.manifest_items.end()) {
		return;
	}
	const auto& nav_file = it->second.path;
	std::string nav_base_dir = nav_file.substr(0, nav_file.find_last_of('/'));
	wxZipEntry* nav_entry = find_zip_entry(nav_file, ctx.zip_entries);
	if (nav_entry == nullptr) {
		return;
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*nav_entry)) {
		return;
	}
	const std::string nav_content = read_zip_entry(zis);
	pugi::xml_document doc;
	if (!doc.load_buffer(nav_content.data(), nav_content.size())) {
		return;
	}
	pugi::xml_node toc_nav;
	for (auto nav : doc.select_nodes("//nav")) {
		auto n = nav.node();
		auto t = n.attribute("epub:type").as_string();
		if (std::string(t) == "toc") {
			toc_nav = n;
			break;
		}
	}
	if (toc_nav == nullptr) {
		toc_nav = doc.find_node([](pugi::xml_node n) { return std::string(n.name()) == "nav"; });
	}
	if (toc_nav) {
		auto ol = toc_nav.child("ol");
		if (ol) {
			parse_epub3_nav_list(ol, toc_items, ctx, buffer, nav_base_dir);
		}
	}
}

void epub_parser::parse_epub3_nav_list(pugi::xml_node ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const epub_context& ctx, const document_buffer& buffer, const std::string& nav_base_path) const {
	for (auto li : ol_element.children("li")) {
		auto item = parse_epub3_nav_item(li, ctx, buffer, nav_base_path);
		if (item) {
			toc_items.push_back(std::move(item));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_epub3_nav_item(pugi::xml_node li_element, const epub_context& ctx, const document_buffer& buffer, const std::string& nav_base_path) const {
	auto item = std::make_unique<toc_item>();
	if (auto a = li_element.child("a")) {
		item->name = wxString::FromUTF8(a.text().as_string());
		std::string href = a.attribute("href").as_string();
		item->ref = wxString::FromUTF8(href);
		std::string abs_str = nav_base_path.empty() ? href : (nav_base_path + "/" + href);
		const std::string& opf_str = ctx.opf_dir;
		std::string href_relative_to_opf;
		if (!opf_str.empty() && abs_str.rfind(opf_str, 0) == 0) {
			href_relative_to_opf = abs_str.substr(opf_str.size() + (opf_str.back() == '/' ? 0 : 1));
		} else {
			href_relative_to_opf = href;
		}
		item->offset = calculate_offset_from_href(href_relative_to_opf, ctx, buffer);
	} else {
		if (auto span = li_element.child("span")) {
			item->name = wxString::FromUTF8(span.text().as_string());
		} else {
			item->name = wxString::FromUTF8(li_element.text().as_string()).BeforeFirst('\n').Trim();
		}
		item->offset = std::numeric_limits<size_t>::max();
	}
	if (auto ol = li_element.child("ol")) {
		parse_epub3_nav_list(ol, item->children, ctx, buffer, nav_base_path);
	}
	return item;
}

int epub_parser::calculate_offset_from_href(const std::string& href, const epub_context& ctx, const document_buffer& buffer) {
	std::string file_path = href;
	std::string fragment;
	const size_t hash_pos = href.find('#');
	if (hash_pos != std::string::npos) {
		file_path = href.substr(0, hash_pos);
		fragment = href.substr(hash_pos + 1);
	}
	file_path = url_decode(file_path);
	fragment = url_decode(fragment);
	std::string resolved_path = ctx.opf_dir;
	if (!file_path.empty()) {
		resolved_path = (resolved_path.empty() ? file_path : (resolved_path + "/" + file_path));
	}
	if (!fragment.empty()) {
		for (const auto& [stored_path, id_map] : ctx.id_positions) {
			if (url_decode(stored_path) == resolved_path) {
				auto id_pos_it = id_map.find(fragment);
				if (id_pos_it != id_map.end()) {
					return static_cast<int>(id_pos_it->second);
				}
				break;
			}
		}
	}
	std::string manifest_id;
	for (const auto& [id, item] : ctx.manifest_items) {
		if (url_decode(item.path) == resolved_path) {
			manifest_id = id;
			break;
		}
	}
	if (manifest_id.empty()) {
		return -1;
	}
	auto it = std::ranges::find(ctx.spine_items, manifest_id);
	if (it == ctx.spine_items.end()) {
		return -1;
	}
	const size_t spine_index = std::distance(ctx.spine_items.begin(), it);
	return static_cast<int>(buffer.get_marker_position_by_index(marker_type::section_break, spine_index));
}
