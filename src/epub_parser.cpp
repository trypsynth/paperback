#include "epub_parser.hpp"
#include "html_to_text.hpp"
#include "utils.hpp"
#include <Poco/AutoPtr.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/String.h>
#include <Poco/Zip/ZipStream.h>
#include <memory>
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
	try {
		fp.open(path.ToStdString(), std::ios::binary);
		if (fp.fail()) return nullptr;
		archive = std::make_unique<ZipArchive>(fp);
		epub_context ctx(fp, archive);
		auto header = archive->findHeader("META-INF/container.xml");
		if (header == archive->headerEnd()) return nullptr;
		ZipInputStream zis(fp, header->second, true);
		InputSource src(zis);
		// If we don't call fp.clear() here, certain epubs (e.g. Bookshare) will fail to open, because the stream's
		// error bit will be set. This seems like a bug in Poco to me, but I dunno man, I just work here.
		fp.clear();
		DOMParser parser;
		auto doc = parser.parse(&src);
		NamespaceSupport nsmap;
		nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
		auto* node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
		if (!node) return nullptr;
		auto opf_filename = static_cast<Element*>(node)->getAttribute("full-path");
		ctx.opf_path = Path(opf_filename, Path::PATH_UNIX).makeParent();
		parse_opf(opf_filename, ctx);
		auto document_ptr = std::make_unique<document>();
		document_ptr->section_offsets.clear();
		wxString content;
		for (size_t i = 0; i < ctx.spine_items.size(); ++i) {
			document_ptr->section_offsets.push_back(content.length());
			auto section = parse_section(i, ctx);
			for (const auto& line : section.lines) {
				if (!content.empty()) content += "\n";
				content += wxString::FromUTF8(line);
			}
		}
		ctx.section_offsets = document_ptr->section_offsets;
		document_ptr->title = wxString::FromUTF8(ctx.title);
		document_ptr->author = wxString::FromUTF8(ctx.author);
		document_ptr->text_content = content;
		document_ptr->flags = document_flags::supports_sections | document_flags::supports_toc;
		parse_toc(ctx, document_ptr->toc_items);
		return document_ptr;
	} catch (const Exception& e) {
		wxMessageBox(e.displayText(), "Error", wxICON_ERROR);
		return nullptr;
	}
}

void epub_parser::parse_opf(const std::string& filename, epub_context& ctx) const {
	auto header = find_file_in_archive(filename, ctx.archive);
	if (header == ctx.archive->headerEnd()) throw parse_error("No OPF file found");
	ZipInputStream zis(ctx.file_stream, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("opf", "http://www.idpf.org/2007/opf");
	nsmap.declarePrefix("dc", "http://purl.org/dc/elements/1.1/");
	auto* metadata = doc->getNodeByPathNS("opf:package/opf:metadata", nsmap);
	if (metadata) {
		auto children = metadata->childNodes();
		for (size_t i = 0; i < children->length(); ++i) {
			auto* node = children->item(i);
			if (node->nodeType() != Node::ELEMENT_NODE) continue;
			auto* element = static_cast<Element*>(node);
			auto localName = element->localName();
			if (localName == "title" && ctx.title.empty())
				ctx.title = element->innerText();
			else if (localName == "creator" && ctx.author.empty())
				ctx.author = element->innerText();
		}
	}
	const auto* manifest = doc->getNodeByPathNS("opf:package/opf:manifest", nsmap);
	if (!manifest) throw parse_error("No manifest");
	auto children = manifest->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		const auto href = element->getAttribute("href");
		const auto id = element->getAttribute("id");
		const auto media_type = element->getAttribute("media-type");
		const auto properties = element->getAttribute("properties");
		Path filePath(ctx.opf_path);
		filePath.append(href);
		ctx.manifest_items.emplace(id, filePath.toString(Path::PATH_UNIX));
		if (media_type == "application/x-dtbncx+xml")
			ctx.toc_ncx_id = id;
		else if (properties.find("nav") != std::string::npos)
			ctx.nav_doc_id = id;
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error("No spine");
	if (ctx.toc_ncx_id.empty()) {
		auto toc_attr = static_cast<Element*>(spine)->getAttribute("toc");
		if (!toc_attr.empty()) ctx.toc_ncx_id = toc_attr;
	}
	children = spine->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		const auto idref = element->getAttribute("idref");
		ctx.spine_items.push_back(idref);
	}
}

epub_section epub_parser::parse_section(size_t index, const epub_context& ctx) const {
	if (index >= ctx.spine_items.size()) throw parse_error("Section index out of range");
	const auto& id = ctx.spine_items[index];
	auto it = ctx.manifest_items.find(id);
	if (it == ctx.manifest_items.end()) throw parse_error("Unknown spine item id: " + id);
	const auto& href = it->second;
	auto header = find_file_in_archive(href, ctx.archive);
	if (header == ctx.archive->headerEnd()) throw parse_error("File not found: " + href);
	ZipInputStream zis(ctx.file_stream, header->second, true);
	std::ostringstream html_buffer;
	html_buffer << zis.rdbuf();
	epub_section section;
	html_to_text converter;
	if (converter.convert(html_buffer.str())) {
		const auto& lines = converter.get_lines();
		section.lines.assign(lines.begin(), lines.end());
	}
	return section;
}

void epub_parser::parse_toc(epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const {
	try {
		if (!ctx.nav_doc_id.empty())
			parse_epub3_nav(ctx.nav_doc_id, ctx, toc_items);
		else if (!ctx.toc_ncx_id.empty())
			parse_epub2_ncx(ctx.toc_ncx_id, ctx, toc_items);
	} catch (const Exception& e) {
		wxMessageBox("Warning: Could not parse table of contents: " + wxString(e.displayText()), "Warning", wxICON_WARNING);
	}
}

void epub_parser::parse_epub2_ncx(const std::string& ncx_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const {
	auto it = ctx.manifest_items.find(ncx_id);
	if (it == ctx.manifest_items.end()) return;
	const auto& ncx_file = it->second;
	auto header = find_file_in_archive(ncx_file, ctx.archive);
	if (header == ctx.archive->headerEnd()) return;
	ZipInputStream zis(ctx.file_stream, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("ncx", "http://www.daisy.org/z3986/2005/ncx/");
	auto* nav_map = doc->getNodeByPathNS("ncx:ncx/ncx:navMap", nsmap);
	if (!nav_map) return;
	auto children = nav_map->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto toc_entry = parse_ncx_nav_point(element, nsmap, ctx);
			if (toc_entry) toc_items.push_back(std::move(toc_entry));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_ncx_nav_point(Element* nav_point, const NamespaceSupport& nsmap, const epub_context& ctx) const {
	auto item = std::make_unique<toc_item>();
	auto* nav_label = nav_point->getNodeByPathNS("ncx:navLabel/ncx:text", nsmap);
	if (nav_label) item->name = wxString::FromUTF8(nav_label->innerText());
	auto* content = nav_point->getNodeByPathNS("ncx:content", nsmap);
	if (content) {
		auto src = static_cast<Element*>(content)->getAttribute("src");
		item->ref = wxString::FromUTF8(src);
		item->offset = calculate_offset_from_href(src, ctx);
	}
	auto children = nav_point->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "navPoint") {
			auto child_item = parse_ncx_nav_point(element, nsmap, ctx);
			if (child_item) item->children.push_back(std::move(child_item));
		}
	}
	return item;
}

void epub_parser::parse_epub3_nav(const std::string& nav_id, const epub_context& ctx, std::vector<std::unique_ptr<toc_item>>& toc_items) const {
	auto it = ctx.manifest_items.find(nav_id);
	if (it == ctx.manifest_items.end()) return;
	const auto& nav_file = it->second;
	auto header = find_file_in_archive(nav_file, ctx.archive);
	if (header == ctx.archive->headerEnd()) return;
	ZipInputStream zis(ctx.file_stream, header->second, true);
	InputSource src(zis);
	DOMParser parser;
	auto doc = parser.parse(&src);
	NamespaceSupport nsmap;
	nsmap.declarePrefix("html", "http://www.w3.org/1999/xhtml");
	nsmap.declarePrefix("epub", "http://www.idpf.org/2007/ops");
	auto nav_nodes = doc->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "nav");
	Element* toc_nav = nullptr;
	for (size_t i = 0; i < nav_nodes->length(); ++i) {
		auto* nav = static_cast<Element*>(nav_nodes->item(i));
		auto epub_type = nav->getAttributeNS("http://www.idpf.org/2007/ops", "type");
		if (epub_type.empty()) epub_type = nav->getAttribute("epub:type");
		if (epub_type == "toc") {
			toc_nav = nav;
			break;
		}
	}
	if (!toc_nav && nav_nodes->length() > 0) toc_nav = static_cast<Element*>(nav_nodes->item(0));
	if (toc_nav) {
		auto ol_nodes = toc_nav->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
		if (ol_nodes->length() > 0) {
			auto* ol = static_cast<Element*>(ol_nodes->item(0));
			parse_epub3_nav_list(ol, toc_items, ctx);
		}
	}
}

void epub_parser::parse_epub3_nav_list(Element* ol_element, std::vector<std::unique_ptr<toc_item>>& toc_items, const epub_context& ctx) const {
	auto children = ol_element->childNodes();
	for (size_t i = 0; i < children->length(); ++i) {
		auto* node = children->item(i);
		if (node->nodeType() != Node::ELEMENT_NODE) continue;
		auto* element = static_cast<Element*>(node);
		if (element->localName() == "li") {
			auto item = parse_epub3_nav_item(element, ctx);
			if (item) toc_items.push_back(std::move(item));
		}
	}
}

std::unique_ptr<toc_item> epub_parser::parse_epub3_nav_item(Element* li_element, const epub_context& ctx) const {
	auto item = std::make_unique<toc_item>();
	auto a_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "a");
	if (a_nodes->length() > 0) {
		auto* a = static_cast<Element*>(a_nodes->item(0));
		item->name = wxString::FromUTF8(a->innerText());
		auto href = a->getAttribute("href");
		item->ref = wxString::FromUTF8(href);
		item->offset = calculate_offset_from_href(href, ctx);
	} else {
		auto span_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "span");
		if (span_nodes->length() > 0) {
			auto* span = static_cast<Element*>(span_nodes->item(0));
			item->name = wxString::FromUTF8(span->innerText());
		} else
			item->name = wxString::FromUTF8(li_element->innerText()).BeforeFirst('\n').Trim();
		item->offset = -1;
	}
	auto ol_nodes = li_element->getElementsByTagNameNS("http://www.w3.org/1999/xhtml", "ol");
	if (ol_nodes->length() > 0) {
		auto* ol = static_cast<Element*>(ol_nodes->item(0));
		parse_epub3_nav_list(ol, item->children, ctx);
	}
	return item;
}

int epub_parser::calculate_offset_from_href(const std::string& href, const epub_context& ctx) const {
	std::string file_path = href;
	std::string fragment;
	size_t hash_pos = href.find('#');
	if (hash_pos != std::string::npos) {
		file_path = href.substr(0, hash_pos);
		fragment = href.substr(hash_pos + 1);
	}
	Path full_path(ctx.opf_path);
	if (!file_path.empty()) full_path.append(file_path);
	std::string resolved_path = full_path.toString(Path::PATH_UNIX);
	std::string manifest_id;
	for (const auto& [id, path] : ctx.manifest_items) {
		if (path == resolved_path) {
			manifest_id = id;
			break;
		}
	}
	if (manifest_id.empty()) return -1;
	auto it = std::find(ctx.spine_items.begin(), ctx.spine_items.end(), manifest_id);
	if (it == ctx.spine_items.end()) return -1;
	size_t spine_index = std::distance(ctx.spine_items.begin(), it);
	if (spine_index >= ctx.section_offsets.size()) return -1;
	return static_cast<int>(ctx.section_offsets[spine_index]);
}
