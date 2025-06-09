#include "epub_parser.hpp"
#include "html_to_text.hpp"
#include <memory>
#include <sstream>
#include <Poco/AutoPtr.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/String.h>
#include <Poco/Zip/ZipStream.h>
#include <wx/filename.h>
#include <wx/msgdlg.h>
#include <wx/wfstream.h>

using namespace Poco;
using namespace Poco::XML;
using namespace Poco::Zip;

std::unique_ptr<document> epub_parser::load(const wxString& path) {
	reset();
	try {
		fp.open(path.ToStdString(), std::ios::binary);
		if (fp.fail()) return nullptr;
		archive = std::make_unique<ZipArchive>(fp);
		auto header = archive->findHeader("META-INF/container.xml");
		if (header == archive->headerEnd()) return nullptr;
		ZipInputStream zis(fp, header->second, true);
		InputSource src(zis);
		DOMParser parser;
		auto doc = parser.parse(&src);
		NamespaceSupport nsmap;
		nsmap.declarePrefix("container", "urn:oasis:names:tc:opendocument:xmlns:container");
		auto*node = doc->getNodeByPathNS("container:container/container:rootfiles/container:rootfile", nsmap);
		if (node == nullptr) return nullptr;
		auto name = static_cast<Element*>(node)->getAttribute("full-path");
		opf_path = Path(name, Path::PATH_UNIX).makeParent();
		parse_opf(name);
	} catch (std::exception& e) {
		wxMessageBox(e.what(), "Error parsing epub file", wxICON_ERROR);
		return nullptr;
	}
	wxString content;
	section_offsets.clear();
	for (int i = 0; i < spine_items.size(); i++) {
		epub_section section = parse_section(i);
		section_offsets.push_back(content.length());
		content += wxString::FromUTF8(get_section_text(section));
	}
	auto doc = std::make_unique<document>();
	doc->title = title;
	doc->author = author;
	doc->text_content = content;
	return doc;
}

int epub_parser::next_section_index(size_t position) const {
	for (size_t i = 0; i < section_offsets.size(); ++i)
		if (section_offsets[i] > position)
			return static_cast<int>(i);
	return -1;
}

int epub_parser::previous_section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (section_offsets[i] < position)
			return i;
	return -1;
}

int epub_parser::section_index(size_t position) const {
	for (int i = static_cast<int>(section_offsets.size()) - 1; i >= 0; --i)
		if (position >= section_offsets[i])
			return i;
	return -1;
}

size_t epub_parser::offset_for_section(int section_index) const {
	if (section_index < 0 || section_index >= static_cast<int>(section_offsets.size()))
		return 0;
	return section_offsets[section_index];
}

size_t epub_parser::section_count() const {
	return section_offsets.size();
}

void epub_parser::parse_opf(const std::string& filename) {
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
		manifest_items.insert(std::make_pair(id, filePath.toString(Path::PATH_UNIX)));
	}
	auto* spine = doc->getNodeByPathNS("opf:package/opf:spine", nsmap);
	if (!spine) throw parse_error("No spine");
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

epub_section epub_parser::parse_section(size_t n) {
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
	} else {
		section.lines.clear();
	}
	return section;
}

std::string epub_parser::get_section_text(epub_section& section) {
	std::string data;
	for (auto& line : section.lines) {
		line = trimInPlace(line);
		if (line.empty()) continue;
		data += line + "\n";
	}
	return data;
}

void epub_parser::reset() {
	if (fp.is_open()) fp.close();
	archive.reset();
	manifest_items.clear();
	spine_items.clear();
	opf_path.clear();
	title.clear();
	author.clear();
	section_offsets.clear();
}
