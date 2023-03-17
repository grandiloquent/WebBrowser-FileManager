#ifndef HELPER_H
#define HELPER_H

#include <filesystem>
#include <ctime>
#include <iostream>
#include <cassert>

std::string &str_replace(std::string &subject, std::string search, std::string replace);

void CreateDesktopDirectory();

unsigned char FromHex(unsigned char x);

std::string msToVttTimeString(int ms);

void rtrim(std::string &s, const char c);

int timeStringToMs(const std::string &time);

// convert wstring to string
std::string to_byte_string(const std::wstring &input);

// convert string to wstring
std::wstring to_wide_string(const std::string &input);

unsigned char ToHex(unsigned char x);

std::string UrlDecode(const std::string &str);

std::string UrlEncode(const std::string &str);

#endif