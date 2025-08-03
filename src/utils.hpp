#pragma once
#include <string>
#include <string_view>
#include <wx/string.h>
#include <Poco/Zip/ZipArchive.h>

long find_text(const wxString& haystack, const wxString& needle, long start, bool forward = true, bool match_case = false);
std::string collapse_whitespace(std::string_view input);
bool should_open_as_txt(const wxString& path);
void speak(const wxString& message);
std::string url_decode(const std::string& encoded);
Poco::Zip::ZipArchive::FileHeaders::const_iterator find_file_in_archive(const std::string& filename, const std::unique_ptr<Poco::Zip::ZipArchive>& archive);
