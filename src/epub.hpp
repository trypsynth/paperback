#pragma once

#include <fstream>
#include <Poco/SAX/ContentHandler.h>
#include <stdexcept>
#include <string>
#include <vector>
#include <Poco/SAX/Locator.h>
#include <Poco/Zip/ZipArchive.h>

struct epub_section {
	std::vector<std::string> lines;
};

class epub_content_handler : public Poco::XML::ContentHandler {
public:
	epub_content_handler(epub_section& section);
	void set_line_length(int length);
	const epub_section& get_section() const {return section;}

protected:
	void setDocumentLocator(const Poco::XML::Locator* loc) override;
	void startDocument() override;
	void endDocument() override;
	void startElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname, const Poco::XML::Attributes& attributes) override;
	void endElement(const Poco::XML::XMLString& uri, const Poco::XML::XMLString& localName, const Poco::XML::XMLString& qname) override;
	void characters(const Poco::XML::XMLChar ch[], int start, int length) override;
	void ignorableWhitespace(const Poco::XML::XMLChar ch[], int start, int length) override;
	void processingInstruction(const Poco::XML::XMLString& target, const Poco::XML::XMLString& data) override;
	void startPrefixMapping(const Poco::XML::XMLString& prefix, const Poco::XML::XMLString& uri) override;
	void endPrefixMapping(const Poco::XML::XMLString& prefix) override;
	void skippedEntity(const Poco::XML::XMLString& name) override;

private:
	epub_section &section;
	const Poco::XML::Locator* locator;
	std::string line;
	bool in_paragraph;
	bool in_body;
	unsigned int max_line_length;
	void add_line(std::string line);
	bool ignore_whitespace;
	void ltrim(std::string& s);
};

class parse_error : public std::runtime_error {
public:
	using std::runtime_error::runtime_error;
};

class epub {
public:
	~epub() = default;
	bool load(const std::string& fname);
	int get_num_sections() const;
	std::string get_section_text(epub_section& section);
	epub_section parse_section(unsigned int n, std::vector<std::string>* lines, unsigned int line_length = 0);
	const std::string& title() const{return title_;}
	const std::string& author() const{return author_;}

private:
	void parse_opf(const std::string& filename);
	std::ifstream fp;
	std::unique_ptr<Poco::Zip::ZipArchive> archive;
	// Map of manifest ids to hrefs
	std::map<std::string, std::string> manifest_items;
	std::vector<std::string> spine_items;
	Poco::Path opf_path;
	std::string title_;
	std::string author_;
};
