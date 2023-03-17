
#include "helper.h"

void CreateDirectory() {
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

std::string &str_replace(std::string &subject, std::string search, std::string replace) {
    for (;;) {
        size_t index = subject.find_first_of(search);
        if (index == std::string::npos) break;
        subject.replace(index, search.length(), replace);
    }

    return subject;
}
