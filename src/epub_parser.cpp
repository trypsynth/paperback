#include "epub_parser.hpp"
#include "html_to_text.hpp"
#include <memory>
#include <Poco/AutoPtr.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/String.h>
#include <Poco/Zip/ZipStream.h>
#include <sstream>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>

using namespace Poco;
using namespace Poco::XML;
using namespace Poco::Zip;

std::unique_ptr<document> epub_parser::load(const wxString& path) const {
	std::ifstream fp;
	std::unique_ptr<ZipArchive> archive;
	std::map<std::string, std::string> manifest_items;
	std::vector<std::string> spine_items;
	Path opf_path;
	std::string title, author;
	std::string toc_ncx_id, nav_doc_id;
	try {
		fp.open(path.ToStdString(), std::ios::binary);
		if (fp.fail()) return nullptr;
		archive = std::make_unique<ZipArchive>(fp);
		auto header = archive->findHeader("META-INF/container.xml");
		if (header == archive->headerEnd()) return nullptr;
		ZipInputStream zis(fp, header->second, true);
		InputSource src(zis);
		fp.clear();
		DOMParser parser;
		auto doc = parser.parse(&src);
		NamespaceSupport nsmap;
		nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
		auto*node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
		if (!node) return nullptr;
		auto name = static_cast<Element*>(node)->getAttribute("full-path");
		opf_path = Path(name, Path::PATH_UNIX).makeParent();
		parse_opf(name, fp, archive, manifest_items, spine_items, opf_path, title, author, toc_ncx_id, nav_doc_id);
	} catch (Exception& e) {
		wxMessageBox(e.displayText(), "Error", wxICON_ERROR);
		return nullptr;
	}
	wxString content;
	auto document_ptr = std::make_unique<document>();
	document_ptr->section_offsets.clear();
	for (int i = 0; i < spine_items.size(); i++) {
		epub_section section = parse_section(i, fp, archive, manifest_items, spine_items);
		document_ptr->section_offsets.push_back(content.length());
		content += wxString::FromUTF8(get_section_text(section));
	}
	document_ptr->title = wxString::FromUTF8(title);
	document_ptr->author = wxString::FromUTF8(author);
	document_ptr->text_content = content;
	document_ptr->flags = document_flags::supports_sections | document_flags::supports_toc;
	try {
		if (!nav_doc_id.empty())
			parse_epub3_nav(nav_doc_id, fp, archive, manifest_items, opf_path, document_ptr->toc_items, manifest_items, spine_items, document_ptr->section_offsets);
		else if (!toc_ncx_id.empty())
			parse_epub2_ncx(toc_ncx_id, fp, archive, manifest_items, opf_path, document_ptr->toc_items, manifest_items, spine_items, document_ptr->section_offsets);
	} catch (Exception& e) {
		wxMessageBox("Warning: Could not parse table of contents: " + wxString(e.displayText()), "Warning", wxICON_WARNING);
	}
	return document_ptr;
}

void epub_parser::parse_opf(const std::string& filename, std::ifstream& fp, std::unique_ptr<ZipArchive>& archive, std::map<std::string, std::string>& manifest_items, std::vector<std::string>& spine_items, Path& opf_path, std::string& title, std::string& author, std::string& toc_ncx_id, std::string& nav_doc_id) const {
	auto header = archive->findHeader(filename);
	if (header == archive->headerEnd()) throw parse_error("No OPF file found");
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("opf", "http://www.idpf.org/2007/opf");
	nsmap.declarePrefix("dc", "http://purl.org/dc/elements/1.1/");
	auto* metadata = doc->getNodeByPathNS("opf:package/opf:metadata", nsmap);
	if (metadata) {
		auto children = metadata->childNodes();
		size_t len = children->length();
		for (size_t i = 0; i < len; i++) {
			auto* node = children->item(i);
			if (node->nodeType() != Node::ELEMENT_NODE) continue;
			auto* e = static_cast<Element*>(node);
			auto localName = e->localName();
			if (localName == "title" && title.empty())
				title = e->innerText();
			else if (localName == "creator" && author.empty())
				author = e->innerText();
		}
	}
	auto* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (!manifest) throw parse_error("No manifest");
	auto children = manifest->childNodes();
	size_t len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* e = static_cast<Element*>(node);
		const auto href = e->getAttribute("href");
		Path filePath(opf_path);
		filePath.append(href);
		const auto id = e->getAttribute("id");
		const auto media_type = e->getAttribute("media-type");
		const auto properties = e->getAttribute("properties");
		manifest_items.insert(std::make_pair(id, filePath.toString(Path::PATH_UNIX)));
		if (media_type == "application/x-dtbncx+xml") toc_ncx_id = id;
		else if (properties.find("nav") != std::string::npos) nav_doc_id = id;
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error("No spine");
	if (toc_ncx_id.empty()) {
		auto toc_attr = static_cast<Element*>(spine)->getAttribute("toc");
		if (!toc_attr.empty()) toc_ncx_id = toc_attr;
	}
	children = spine->childNodes();
	len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		const auto idref = element->getAttribute("idref");
		spine_items.push_back(idref);
	}
}

void epub_parser::parse_epub2_ncx(const std::string& ncx_id, std::ifstream& fp, std::unique_ptr<ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const Path& opf_path, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::map<std::string, std::string>& all_manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	auto it = manifest_items.find(ncx_id);
	if (it == manifest_items.end()) return;
	const auto& ncx_file = it->second;
	auto header = archive->findHeader(ncx_file);
	if (header == archive->headerEnd()) return;
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("ncx", "http://www.daisy.org/z3986/2005/ncx/");
	auto* nav_map = doc->getNodeByPathNS("ncx:ncx/ncx:navMap", nsmap);
	if (!nav_map) return;
	auto children = nav_map->childNodes();
	size_t len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto toc_entry = parse_ncx_nav_point(element, nsmap, opf_path, all_manifest_items, spine_items, section_offsets);
			if (toc_entry) toc_items.push_back(std::move(toc_entry));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_ncx_nav_point(Element* nav_point, const NamespaceSupport& nsmap, const Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	auto item = std::make_unique<toc_item>();
	auto* nav_label = nav_point->getNodeByPathNS("ncx:navLabel/ncx:text", nsmap);
	if (nav_label) item->name = wxString::FromUTF8(nav_label->innerText());
	auto* content = nav_point->getNodeByPathNS("ncx:content", nsmap);
	if (content) {
		auto src = static_cast<Element*>(content)->getAttribute("src");
		item->ref = wxString::FromUTF8(src);
		item->offset = calculate_offset_from_href(src, opf_path, manifest_items, spine_items, section_offsets);
	}
	auto children = nav_point->childNodes();
	size_t len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto child_item = parse_ncx_nav_point(element, nsmap, opf_path, manifest_items, spine_items, section_offsets);
			if (child_item) item->children.push_back(std::move(child_item));
		}
	}
	return item;
}

void epub_parser::parse_epub3_nav(const std::string& nav_id, std::ifstream& fp, std::unique_ptr<ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const Path& opf_path, std::vector<std::unique_ptr<toc_item>>& toc_items, const std::map<std::string, std::string>& all_manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	auto it = manifest_items.find(nav_id);
	if (it == manifest_items.end()) return;
	const auto& nav_file = it->second;
	auto header = archive->findHeader(nav_file);
	if (header == archive->headerEnd()) return;
	ZipInputStream zis(fp, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("html", "http://www.w3.org/1999/xhtml");
	nsmap.declarePrefix("epub", "http://www.idpf.org/2007/ops");
	auto nav_nodes = doc->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "nav");
	Element* toc_nav = nullptr;
	size_t len = nav_nodes->length();
	for (size_t i = 0; i < len; i++) {
		auto* nav = static_cast<Element*>(nav_nodes->item(i));
		auto epub_type = nav->getAttributeNS("http://www.idpf.org/2007/ops", "type");
		if (epub_type.empty()) epub_type = nav->getAttribute("epub:type");
		if (epub_type == "toc") {
			toc_nav = nav;
			break;
		}
	}
	if (!toc_nav)
		if (nav_nodes->length() > 0)
			toc_nav = static_cast<Element*>(nav_nodes->item(0));
	if (toc_nav) {
		auto ol_nodes = toc_nav->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
		if (ol_nodes->length() > 0) {
			auto* ol = static_cast<Element*>(ol_nodes->item(0));
			parse_epub3_nav_list(ol, toc_items, opf_path, all_manifest_items, spine_items, section_offsets);
		}
	}
}

void epub_parser::parse_epub3_nav_list(Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	auto children = ol_element->childNodes();
	size_t len = children->length();
	for (size_t i = 0; i < len; i++) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "li") {
			auto item = parse_epub3_nav_item(element, opf_path, manifest_items, spine_items, section_offsets);
			if (item) toc_items.push_back(std::move(item));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_epub3_nav_item(Element* li_element, const Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	auto item = std::make_unique<toc_item>();
	auto a_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "a");
	if (a_nodes->length() > 0) {
		auto* a = static_cast<Element*>(a_nodes->item(0));
		item->name = wxString::FromUTF8(a->innerText());
		auto href = a->getAttribute("href");
		item->ref = wxString::FromUTF8(href);
		item->offset = calculate_offset_from_href(href, opf_path, manifest_items, spine_items, section_offsets);
	} else {
		auto span_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "span");
		if (span_nodes->length() > 0) {
			auto* span = static_cast<Element*>(span_nodes->item(0));
			item->name = wxString::FromUTF8(span->innerText());
		} else item->name = wxString::FromUTF8(li_element->innerText()).BeforeFirst('\n').Trim();
		item->offset = -1;
	}
	auto ol_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
	if (ol_nodes->length() > 0) {
		auto* ol = static_cast<Element*>(ol_nodes->item(0));
		parse_epub3_nav_list(ol, item->children, opf_path, manifest_items, spine_items, section_offsets);
	}
	return item;
}

int epub_parser::calculate_offset_from_href(const std::string& href, const Path& opf_path, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items, const std::vector<size_t>& section_offsets) const {
	std::string file_path = href;
	std::string fragment;
	size_t hash_pos = href.find('#');
	if (hash_pos != std::string::npos) {
		file_path = href.substr(0, hash_pos);
		fragment = href.substr(hash_pos + 1);
	}
	Path full_path(opf_path);
	if (!file_path.empty()) full_path.append(file_path);
	std::string resolved_path = full_path.toString(Path::PATH_UNIX);
	std::string manifest_id;
	for (const auto& pair : manifest_items) {
		if (pair.second == resolved_path) {
			manifest_id = pair.first;
			break;
		}
	}
	if (manifest_id.empty()) return -1;
	int spine_index = -1;
	for (size_t i = 0; i < spine_items.size(); i++) {
		if (spine_items[i] == manifest_id) {
			spine_index = static_cast<int>(i);
			break;
		}
	}
	if (spine_index == -1) return -1;
	if (spine_index >= static_cast<int>(section_offsets.size())) return -1;
	int base_offset = static_cast<int>(section_offsets[spine_index]);
	return base_offset;
}

epub_section epub_parser::parse_section(size_t n, std::ifstream& fp, std::unique_ptr<ZipArchive>& archive, const std::map<std::string, std::string>& manifest_items, const std::vector<std::string>& spine_items) const {
	const auto id = spine_items[n];
	auto it = manifest_items.find(id);
	if (it == manifest_items.end()) throw parse_error("Unknown id: " + id);
	const auto href = it->second;
	auto header = archive->findHeader(href);
	if (header == archive->headerEnd()) throw parse_error("File not found: " + href);
	ZipInputStream zis(fp, header->second, true);
	std::ostringstream html_buffer;
	html_buffer << zis.rdbuf();
	std::string html_content = html_buffer.str();
	epub_section section;
	html_to_text converter;
	if (converter.convert(html_content)) {
		const auto& lines = converter.get_lines();
		section.lines.assign(lines.begin(), lines.end());
	} else section.lines.clear();
	return section;
}

std::string epub_parser::get_section_text(epub_section& section) const {
	std::string data;
	for (auto& line : section.lines) {
		line = trimInPlace(line);
		if (line.empty()) continue;
		data += line + "\n";
	}
	return data;
}
