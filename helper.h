#ifndef HELPER_H
#define HELPER_H

#include <filesystem>
#include <ctime>
#include <iostream>
#include <cassert>

#include <regex>
#include <fstream>
#include "httplib.h"

namespace fs = std::filesystem;

std::string &str_replace(std::string &subject, std::string search, std::string replace);

std::string convertFile(const std::filesystem::path &filepath);

void CreateDesktopDirectory();

unsigned char FromHex(unsigned char x);

std::string msToVttTimeString(int ms);

void rtrim(std::string &s, const char c);

void TidyDirectory(const std::string &dir);

int timeStringToMs(const std::string &time);

// convert wstring to string
std::string to_byte_string(const std::wstring &input);

std::string to_string(std::filesystem::file_time_type const &ftime);

// convert string to wstring
std::wstring to_wide_string(const std::string &input);

unsigned char ToHex(unsigned char x);

std::string UrlDecode(const std::string &str);

std::string UrlEncode(const std::string &str);

std::string GetTitle(const std::string &host, const std::string &path);

bool MoveFile(const fs::path &path);

int GetTimeStamp();
std::string ReadFile(fs::path path);
std::string slurp(std::ifstream& in);
#endif