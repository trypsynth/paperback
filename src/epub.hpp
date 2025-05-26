#pragma once

#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
#include <Poco/AutoPtr.h>
#include <Poco/DOM/Document.h>
#include <Poco/DOM/DOMParser.h>
#include <Poco/DOM/NamedNodeMap.h>
#include <Poco/DOM/NodeList.h>
#include <Poco/SAX/ContentHandler.h>
#include <Poco/SAX/InputSource.h>
#include <Poco/SAX/Locator.h>
#include <Poco/SAX/SAXParser.h>
#include <Poco/Zip/ZipArchive.h>
#include <Poco/Zip/ZipStream.h>

class epub_section {
public:
	epub_section(std::vector<std::string>* v);
	std::vector<std::string>* lines;
};

class epub_content_handler : public Poco::XML::ContentHandler {
public:
	epub_content_handler(epub_section& section);
	void set_line_length(int length);
	epub_section& get_section() {return section;}

protected:
	void setDocumentLocator(const Locator* loc);
	void startDocument();
	void endDocument();
	void startElement(const XMLString& uri, const XMLString& localName, const XMLString& qname, const Attributes& attributes);
	void endElement(const XMLString& uri, const XMLString& localName, const XMLString& qname);
	void characters(const XMLChar ch[], int start, int length);
	void ignorableWhitespace(const XMLChar ch[], int start, int length);
	void processingInstruction(const XMLString& target, const XMLString& data);
	void startPrefixMapping(const XMLString& prefix, const XMLString& uri);
	void endPrefixMapping(const XMLString& prefix);
	void skippedEntity(const XMLString& name);

private:
	epub_section &section;
	const Locator* locator;
	std::string line;
	bool in_paragraph;
	bool in_body;
	unsigned int max_line_length;
	void add_line(std::string line);
	bool ignore_whitespace;
	void ltrim(std::string &s);
};

class parse_error : public std::exception {
public:
	parse_error(const char* msg) {
		message = std::string(msg);
	}

	const char* what() const noexcept {return message.c_str();}

private:
	std::string message;
};

class epub {
public:
	epub();
	~epub();
	bool load(const char* fname);
	bool load();
	int get_num_sections();
	std::string get_section_text(epub_section& section);
	epub_section* parse_section(unsigned int n, std::vector<std::string>* lines, unsigned int line_length = 0);

private:
	void parse_opf(std::string filename);
	std::ifstream fp;
	ZipArchive* archive;
	// Map of manifest ids to hrefs
	std::map<std::string, std::string> manifest_items;
	std::vector<std::string> spine_items;
	Poco::Path opf_path;
};
