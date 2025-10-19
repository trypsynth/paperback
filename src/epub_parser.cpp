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
#include "translation_manager.hpp"
#include "utils.hpp"
#include "xml_to_text.hpp"
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/Exception.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/NamespaceSupport.h>
#include <Poco/URI.h>
#include <cstddef>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/string.h>
#include <wx/wfstream.h>
#include <wx/zipstrm.h>

using namespace Poco;
using namespace Poco::XML;

std::unique_ptr<document> epub_parser::load(const wxString& path) const {
	try {
		auto fp = std::make_unique<wxFileInputStream>(path);
		if (!fp->IsOk()) {
			return nullptr;
		}
		wxZipInputStream zip_index(*fp);
		std::map<std::string, wxZipEntry*> entries;
		while (wxZipEntry* entry = zip_index.GetNextEntry()) {
			const std::string name = entry->GetName(wxPATH_UNIX).ToStdString();
			entries[name] = entry;
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
		std::istringstream container_stream(container_content);
		InputSource src(container_stream);
		DOMParser parser;
		auto* doc = parser.parse(&src);
		NamespaceSupport nsmap;
		nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
		auto* node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
		if (node == nullptr) {
			return nullptr;
		}
		auto opf_filename = dynamic_cast<Element*>(node)->getAttribute("full-path");
		ctx.opf_path = Path(opf_filename, Path::PATH_UNIX).makeParent();
		parse_opf(opf_filename, ctx);
		auto document_ptr = std::make_unique<document>();
		document_ptr->buffer.clear();
		for (size_t i = 0; i < ctx.spine_items.size(); ++i) {
			document_ptr->buffer.add_section_break(wxString::Format("Section %zu", i + 1));
			parse_section(i, ctx, document_ptr->buffer);
		}
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
	} catch (const Exception& e) {
		wxMessageBox(e.displayText(), "Error", wxICON_ERROR);
		return nullptr;
	}
}

void epub_parser::parse_opf(const std::string& filename, epub_context& ctx) {
	wxZipEntry* opf_entry = find_zip_entry(filename, ctx.zip_entries);
	if (opf_entry == nullptr) {
		throw parse_error("No OPF file found");
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*opf_entry)) {
		throw parse_error("Failed to open OPF file");
	}
	const std::string opf_content = read_zip_entry(zis);
	std::istringstream opf_stream(opf_content);
	InputSource src(opf_stream);
	DOMParser parser;
	auto* doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("opf", "http://www.idpf.org/2007/opf");
	nsmap.declarePrefix("dc", "http://purl.org/dc/elements/1.1/");
	auto* metadata = doc->getNodeByPathNS("opf:package/opf:metadata", nsmap);
	if (metadata != nullptr) {
		auto* children = metadata->childNodes();
		for (size_t i = 0; i < children->length(); ++i) {
			auto* node = children->item(i);
			if (node->nodeType() != Node::ELEMENT_NODE) {
				continue;
			}
			auto* element = dynamic_cast<Element*>(node);
			auto local_name = element->localName();
			if (local_name == "title" && ctx.title.empty()) {
				ctx.title = element->innerText();
			} else if (local_name == "creator" && ctx.author.empty()) {
				ctx.author = element->innerText();
			}
		}
	}
	const auto* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (manifest == nullptr) {
		throw parse_error("No manifest");
	}
	auto* children = manifest->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) {
			continue;
		}
		auto* element = dynamic_cast<Element*>(node);
		const auto href = element->getAttribute("href");
		const auto id = element->getAttribute("id");
		const auto media_type = element->getAttribute("media-type");
		const auto properties = element->getAttribute("properties");
		Path file_path(ctx.opf_path);
		file_path.append(href);
		manifest_item item;
		item.path = file_path.toString(Path::PATH_UNIX);
		item.media_type = media_type;
		ctx.manifest_items.emplace(id, std::move(item));
		if (media_type == "application/x-dtbncx+xml") {
			ctx.toc_ncx_id = id;
		} else if (properties.find("nav") != std::string::npos) {
			ctx.nav_doc_id = id;
		}
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (spine == nullptr) {
		throw parse_error("No spine");
	}
	if (ctx.toc_ncx_id.empty()) {
		auto toc_attr = dynamic_cast<Element*>(spine)->getAttribute("toc");
		if (!toc_attr.empty()) {
			ctx.toc_ncx_id = toc_attr;
		}
	}
	children = spine->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) {
			continue;
		}
		auto* element = dynamic_cast<Element*>(node);
		const auto idref = element->getAttribute("idref");
		ctx.spine_items.push_back(idref);
	}
}

template <typename conv>
void epub_parser::process_section_content(conv& converter, const std::string& content, const std::string& href, epub_context& ctx, document_buffer& buffer) const {
	if (converter.convert(content)) {
		const auto& text = converter.get_text();
		const auto& headings = converter.get_headings();
		const auto& links = converter.get_links();
		const auto& id_positions = converter.get_id_positions();
		const size_t section_start = buffer.str().length();
		Path section_base_path(href, Path::PATH_UNIX);
		section_base_path.makeParent();
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
			} else {
				Path resolved_path(section_base_path);
				resolved_path.append(link.ref);
				resolved_href = resolved_path.toString(Path::PATH_UNIX);
			}
			buffer.add_link(section_start + link.offset, wxString::FromUTF8(link.text), resolved_href);
		}
		if (!buffer.str().empty() && !buffer.str().EndsWith("\n")) {
			buffer.append("\n");
		}
	}
}

void epub_parser::parse_section(size_t index, epub_context& ctx, document_buffer& buffer) const {
	if (index >= ctx.spine_items.size()) {
		throw parse_error("Section index out of range");
	}
	const auto& id = ctx.spine_items[index];
	auto it = ctx.manifest_items.find(id);
	if (it == ctx.manifest_items.end()) {
		throw parse_error("Unknown spine item id: " + id);
	}
	const auto& manifest_item = it->second;
	const auto& href = manifest_item.path;
	const auto& media_type = manifest_item.media_type;
	wxZipEntry* section_entry = find_zip_entry(href, ctx.zip_entries);
	if (section_entry == nullptr) {
		throw parse_error("File not found: " + href);
	}
	ctx.file_stream.SeekI(0);
	wxZipInputStream zis(ctx.file_stream);
	if (!zis.OpenEntry(*section_entry)) {
		throw parse_error("Failed to open section file: " + href);
	}
	const std::string content = read_zip_entry(zis);
	if (is_html_content(media_type)) {
		html_to_text converter;
		process_section_content(converter, content, href, ctx, buffer);
	} else {
		xml_to_text converter;
		if (!converter.convert(content)) {
			// The file may contain HTML syntax despite claiming to be XML; try parsing as HTML instead.
			static bool warning_shown = false;
			if (!warning_shown) {
				wxMessageBox(_("This book contains malformed content. Some files claim to be XML but contain HTML syntax."), _("Warning"), wxICON_WARNING);
				warning_shown = true;
			}
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
	} catch (const Exception& e) {
		wxMessageBox("Warning: Could not parse table of contents: " + wxString(e.displayText()), "Warning", wxICON_WARNING);
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
	std::istringstream ncx_stream(ncx_content);
	InputSource src(ncx_stream);
	DOMParser parser;
	auto* doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("ncx", "http://www.daisy.org/z3986/2005/ncx/");
	auto* nav_map = doc->getNodeByPathNS("ncx:ncx/ncx:navMap", nsmap);
	if (nav_map == nullptr) {
		return;
	}
	auto* children = nav_map->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) {
			continue;
		}
		auto* element = dynamic_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto toc_entry = parse_ncx_nav_point(element, nsmap, ctx, buffer);
			if (toc_entry) {
				toc_items.push_back(std::move(toc_entry));
			}
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_ncx_nav_point(Element* nav_point, const NamespaceSupport& nsmap, const epub_context& ctx, const document_buffer& buffer) const {
	auto item = std::make_unique<toc_item>();
	auto* nav_label = nav_point->getNodeByPathNS("ncx:navLabel/ncx:text", nsmap);
	if (nav_label != nullptr) {
		item->name = wxString::FromUTF8(nav_label->innerText());
	}
	auto* content = nav_point->getNodeByPathNS("ncx:content", nsmap);
	if (content != nullptr) {
		auto src = dynamic_cast<Element*>(content)->getAttribute("src");
		item->ref = wxString::FromUTF8(src);
		item->offset = calculate_offset_from_href(src, ctx, buffer);
	}
	auto* children = nav_point->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) {
			continue;
		}
		auto* element = dynamic_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto child_item = parse_ncx_nav_point(element, nsmap, ctx, buffer);
			if (child_item) {
				item->children.push_back(std::move(child_item));
			}
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
	Path nav_base_path(nav_file, Path::PATH_UNIX);
	nav_base_path.makeParent();
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
	std::istringstream nav_stream(nav_content);
	InputSource src(nav_stream);
	DOMParser parser;
	auto* doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("html", "http://www.w3.org/1999/xhtml");
	nsmap.declarePrefix("epub", "http://www.idpf.org/2007/ops");
	auto* nav_nodes = doc->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "nav");
	const Element* toc_nav = nullptr;
	for (size_t i = 0; i < nav_nodes->length(); ++i) {
		auto* nav = dynamic_cast<Element*>(nav_nodes->item(i));
		auto epub_type = nav->getAttributeNS("http://www.idpf.org/2007/ops", "type");
		if (epub_type.empty()) {
			epub_type = nav->getAttribute("epub:type");
		}
		if (epub_type == "toc") {
			toc_nav = nav;
			break;
		}
	}
	if (toc_nav == nullptr && nav_nodes->length() > 0) {
		toc_nav = dynamic_cast<Element*>(nav_nodes->item(0));
	}
	if (toc_nav != nullptr) {
		auto* ol_nodes = toc_nav->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
		if (ol_nodes->length() > 0) {
			auto* ol = dynamic_cast<Element*>(ol_nodes->item(0));
			parse_epub3_nav_list(ol, toc_items, ctx, buffer, nav_base_path);
		}
	}
}

void epub_parser::parse_epub3_nav_list(Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const epub_context& ctx, const document_buffer& buffer, const Path& nav_base_path) const {
	auto* children = ol_element->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) {
			continue;
		}
		auto* element = dynamic_cast<Element*>(node);
		if (element->localName() == "li") {
			auto item = parse_epub3_nav_item(element, ctx, buffer, nav_base_path);
			if (item) {
				toc_items.push_back(std::move(item));
			}
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_epub3_nav_item(Element* li_element, const epub_context& ctx, const document_buffer& buffer, const Path& nav_base_path) const {
	auto item = std::make_unique<toc_item>();
	auto* a_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "a");
	if (a_nodes->length() > 0) {
		auto* a = dynamic_cast<Element*>(a_nodes->item(0));
		item->name = wxString::FromUTF8(a->innerText());
		auto href = a->getAttribute("href");
		item->ref = wxString::FromUTF8(href);
		Path resolved_path(nav_base_path);
		resolved_path.append(href);
		const std::string abs_str = resolved_path.toString(Path::PATH_UNIX);
		const std::string opf_str = ctx.opf_path.toString(Path::PATH_UNIX);
		std::string href_relative_to_opf;
		if (abs_str.starts_with(opf_str)) {
			href_relative_to_opf = abs_str.substr(opf_str.length());
		} else {
			href_relative_to_opf = href;
		}
		item->offset = calculate_offset_from_href(href_relative_to_opf, ctx, buffer);
	} else {
		auto* span_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "span");
		if (span_nodes->length() > 0) {
			auto* span = dynamic_cast<Element*>(span_nodes->item(0));
			item->name = wxString::FromUTF8(span->innerText());
		} else {
			item->name = wxString::FromUTF8(li_element->innerText()).BeforeFirst('\n').Trim();
		}
		item->offset = -1;
	}
	auto* ol_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
	if (ol_nodes->length() > 0) {
		auto* ol = dynamic_cast<Element*>(ol_nodes->item(0));
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
	Path full_path(ctx.opf_path);
	if (!file_path.empty()) {
		full_path.append(file_path);
	}
	const std::string resolved_path = full_path.toString(Path::PATH_UNIX);
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
