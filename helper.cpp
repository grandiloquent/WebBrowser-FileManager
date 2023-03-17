
#include "helper.h"

std::string &str_replace(std::string &subject, std::string search, std::string replace) {
    for (;;) {
        size_t index = subject.find_first_of(search);
        if (index == std::string::npos) break;
        subject.replace(index, search.length(), replace);
    }
    return subject;
}
void CreateDesktopDirectory() {
    std::filesystem::path desktop(R"(C:\Users\Administrator\Desktop)");
//    std::time_t tt = std::time(nullptr);
//    std::tm *tm = std::localtime(&tt);
//
//    desktop /= fmt::format("%Y%m%d", *tm);
    std::time_t now = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());
    std::string s(9, '\0');
    std::strftime(&s[0], s.size(), "%Y%m%d", std::localtime(&now));
    desktop /= s;
    if (!std::filesystem::is_directory(desktop)) {
        std::filesystem::create_directory(desktop);
    }
    for (auto i = 0; i < 3; i++) {
        auto n = std::to_string(i + 1);
        n.insert(0, 2 - n.length(), '0');
        std::cout << desktop.string() + n << std::endl;
        if (!std::filesystem::is_directory(desktop / n)) {
            std::filesystem::create_directory(desktop / n);
        }
    }
}
unsigned char FromHex(unsigned char x) {
    unsigned char y;
    if (x >= 'A' && x <= 'Z') y = x - 'A' + 10;
    else if (x >= 'a' && x <= 'z') y = x - 'a' + 10;
    else if (x >= '0' && x <= '9') y = x - '0';
    else
        assert(0);
    return y;
}
std::string msToVttTimeString(int ms) {
    int hours = ms / 3600000;
    ms -= hours * 3600000;
    int minutes = ms / 60000;
    ms -= minutes * 60000;
    int seconds = ms / 1000;
    ms -= seconds * 1000;
    return (hours < 10 ? "0" : "") + std::to_string(hours)
           + ":" + (minutes < 10 ? "0" : "") + std::to_string(minutes)
           + ":" + (seconds < 10 ? "0" : "") + std::to_string(seconds)
           + "." + (ms < 100 ? "0" : "") + (ms < 10 ? "0" : "") + std::to_string(ms);
}
void rtrim(std::string &s, const char c) {
    while (!s.empty() && s.back() == c) {
        s.pop_back();
    }
}
int timeStringToMs(const std::string &time) {
    // Time format: hh:mm:ss,### (where # = ms)
    int hours = stoi(time.substr(0, 2));
    int minutes = stoi(time.substr(3, 2));
    int seconds = stoi(time.substr(6, 2));
    int milliseconds = stoi(time.substr(9));
    return hours * 3600000 + minutes * 60000 + seconds * 1000 + milliseconds;
}
// convert wstring to string
std::string to_byte_string(const std::wstring &input) {
    //std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>> converter;
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    return converter.to_bytes(input);
}
// convert string to wstring
std::wstring to_wide_string(const std::string &input) {
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    return converter.from_bytes(input);
}
unsigned char ToHex(unsigned char x) {
    return x > 9 ? x + 55 : x + 48;
}
std::string UrlDecode(const std::string &str) {
    std::string strTemp = "";
    size_t length = str.length();
    for (size_t i = 0; i < length; i++) {
        if (str[i] == '+') strTemp += '+';
        else if (str[i] == '%') {
            assert(i + 2 < length);
            unsigned char high = FromHex((unsigned char) str[++i]);
            unsigned char low = FromHex((unsigned char) str[++i]);
            strTemp += high * 16 + low;
        } else strTemp += str[i];
    }
    return strTemp;
}
std::string UrlEncode(const std::string &str) {
    std::string strTemp = "";
    size_t length = str.length();
    for (size_t i = 0; i < length; i++) {
        if (isalnum((unsigned char) str[i]) ||
            (str[i] == '-') ||
            (str[i] == '_') ||
            (str[i] == '.') ||
            (str[i] == '~'))
            strTemp += str[i];
        else if (str[i] == ' ')
            strTemp += "+";
        else {
            strTemp += '%';
            strTemp += ToHex((unsigned char) str[i] >> 4);
            strTemp += ToHex((unsigned char) str[i] % 16);
        }
    }
    return strTemp;
}