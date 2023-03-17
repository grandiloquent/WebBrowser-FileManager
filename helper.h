#ifndef HELPER_H
#define HELPER_H

#include <filesystem>
#include <ctime>
#include <iostream>

void CreateDirectory();

void rtrim(std::string &s, const char c);
int timeStringToMs(const std::string &time);
std::string msToVttTimeString(int ms);
std::string &str_replace(std::string &subject, std::string search, std::string replace);
#endif