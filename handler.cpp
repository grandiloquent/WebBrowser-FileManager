
#include <filesystem>
#include "handler.h"
#include "helper.h"

using namespace std;


std::string convertFile(const std::filesystem::path &filepath) {

    std::ifstream infile(filepath, std::ifstream::in);


    std::stringstream outfile;
//        if (!outfile.is_open()) {
//            throw ios_base::failure("Could not open .vtt file.");
//        }
//        outfile.imbue(locale(outfile.getloc(), new codecvt_utf8<wchar_t>));

    // Write mandatory starting for the WebVTT file
    outfile << "WEBVTT" << std::endl << std::endl;

    std::regex rgxDialogNumber("\\d+");
    std::regex rgxTimeFrame(R"((\d\d:\d\d:\d\d,\d{1,3}) --> (\d\d:\d\d:\d\d,\d{1,3}))");

    for (;;) {
        std::string sLine;

        if (!getline(infile, sLine)) break;

        //LOGE("%s", sLine.c_str());
        rtrim(sLine, '\r'); // Trim a possibly trailing CR character

        // Ignore dialog number lines
        if (regex_match(sLine, rgxDialogNumber))
            continue;

        std::smatch matchTimeFrame;
        regex_match(sLine, matchTimeFrame, rgxTimeFrame);

        if (!matchTimeFrame.empty()) {
            // Handle invalid SRT files where the time frame's milliseconds are less than 3 digits long
            bool msTooShort = matchTimeFrame[1].length() < 12 || matchTimeFrame[2].length() < 12;

            if (msTooShort) {
                // Extract the times in milliseconds from the time frame line
                int msStartTime = timeStringToMs(matchTimeFrame[1]);
                int msEndTime = timeStringToMs(matchTimeFrame[2]);

                // Modify the time with the offset, making sure the time
                // gets set to 0 if it is going to be negative
//                msStartTime += _timeOffsetMs;
//                msEndTime += _timeOffsetMs;
                if (msStartTime < 0) msStartTime = 0;
                if (msEndTime < 0) msEndTime = 0;

                // Construct the new time frame line
                sLine = msToVttTimeString(msStartTime) + " --> " + msToVttTimeString(msEndTime);
            } else {
                // Simply replace the commas in the time with a period
                sLine = str_replace(sLine, ",", ".");
            }
        }

        outfile << sLine << std::endl; // Output the line to the new file
    }
    return outfile.str();

}
//
//string GbkToUtf8(const char *src_str) {
//    int len = MultiByteToWideChar(CP_ACP, 0, src_str, -1, NULL, 0);
//    wchar_t *wstr = new wchar_t[len + 1];
//    memset(wstr, 0, len + 1);
//    MultiByteToWideChar(CP_ACP, 0, src_str, -1, wstr, len);
//    len = WideCharToMultiByte(CP_UTF8, 0, wstr, -1, NULL, 0, NULL, NULL);
//    char *str = new char[len + 1];
//    memset(str, 0, len + 1);
//    WideCharToMultiByte(CP_UTF8, 0, wstr, -1, str, len, NULL, NULL);
//    string strTemp = str;
//    if (wstr) delete[] wstr;
//    if (str) delete[] str;
//    return strTemp;
//}


// convert string to wstring
inline std::wstring to_wide_string(const std::string &input) {
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    return converter.from_bytes(input);
}

// convert wstring to string
inline std::string to_byte_string(const std::wstring &input) {
    //std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>> converter;
    std::wstring_convert<std::codecvt_utf8<wchar_t>> converter;
    return converter.to_bytes(input);
}

unsigned char ToHex(unsigned char x) {
    return x > 9 ? x + 55 : x + 48;
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

static void serveFile(const std::filesystem::path &f, const char *contentType, httplib::Response &res) {
    std::shared_ptr<std::ifstream> fs = std::make_shared<std::ifstream>();
    fs->exceptions(std::ifstream::failbit | std::ifstream::badbit);
    try {
        fs->open(f, std::ios_base::binary);
    } catch (std::system_error &e) {
        res.status = 404;
        return;
    }
    fs->seekg(0, std::ios_base::end);
    auto end = fs->tellg();

    if (end == 0)return;
    fs->seekg(0);
    std::map<std::string, std::string> file_extension_and_mimetype_map;
    res.set_content_provider(static_cast<size_t>(end),
                             contentType,
                             [fs](uint64_t offset,
                                  uint64_t length,
                                  httplib::DataSink &sink) {
                                 if (fs->fail()) {
                                     return false;
                                 }
                                 fs->seekg(offset, std::ios_base::beg);
                                 size_t bufSize = 81920;
                                 char buffer[bufSize];

                                 try {
                                     fs->read(buffer, bufSize);
                                 } catch (std::system_error &e) {
                                 }
                                 sink.write(buffer,
                                            static_cast<size_t>(fs->gcount()));
                                 return true;
                             });
}

void handler::handleIndex(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;
    f /= "index.html";
    serveFile(f, "text/html", res);
}

void handler::handleEditor(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;
    f /= "editor.html";
    serveFile(f, "text/html", res);
}

void handler::handleVideo(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;
    f /= "video.html";
    serveFile(f, "text/html", res);
}

void handler::handleMarkdown(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;
    f /= "markdown.html";
    serveFile(f, "text/html", res);
}

void handler::handleStaticFiles(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;

    f /= req.matches[1].str();
    serveFile(f, req.matches[2].str() == "css" ? "text/css" : "application/javascript", res);
}

std::string to_string(std::filesystem::file_time_type const &ftime) {
    std::time_t cftime = std::chrono::system_clock::to_time_t(
            std::chrono::file_clock::to_sys(ftime));
    std::string str = std::asctime(std::localtime(&cftime));
    str.pop_back();  // rm the trailing '\n' put by `asctime`
    return str;
}

void handler::handleFiles(const httplib::Request &req, httplib::Response &res) {
    auto path = req.get_param_value("path");

    std::filesystem::path p{to_wide_string(UrlDecode(path))};

    nlohmann::json doc = nlohmann::json::array();
    for (auto dir: std::filesystem::directory_iterator(p)) {

/*
  auto lstTime = dir.last_write_time();
        auto elapse = std::chrono::duration_cast<std::chrono::seconds>(
                std::filesystem::file_time_type::clock::now().time_since_epoch() -
                std::chrono::system_clock::now().time_since_epoch()).count();
        auto systemTime = std::chrono::duration_cast<std::chrono::seconds>(lstTime.time_since_epoch()).count() - elapse;
        tm *lsystemTime = localtime(&systemTime);
        std::cout << mktime(lsystemTime) << std::endl;
 */
        nlohmann::json j = {

                {"path",          dir.path().string()},
                {"filename",      dir.path().filename().string()},
                {"isDirectory",   dir.is_directory()},
                {"lastWriteTime", std::chrono::duration_cast<std::chrono::seconds>(
                        dir.last_write_time().time_since_epoch()).count()
                },
                {
                 "length",        dir.is_directory() ? 0 : dir.file_size()
                }
        };
        doc.push_back(j);
    }
    res.set_content(doc.dump(), "application/json");
}



void handler::handleFile(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = to_wide_string(UrlDecode(req.get_param_value("path")));
    auto action = req.get_param_value("action");
    if (action.empty()) {
        if (f.extension() == ".srt") {
            if (exists(f)) {
                res.set_content(convertFile(f), "text/vtt");
            }
            return;
        }
        if (f.extension() == ".html") {
            serveFile(f, "text/html", res);
            return;
        }
        res.set_header("Content-Disposition", "attachment; filename=\"" + f.filename().string() + "\"");
        serveFile(f, "application/octet-stream", res);
        return;
    }
    if (action == "1") {
        f /= to_wide_string(UrlDecode(req.get_param_value("dst")));
        if (!std::filesystem::exists(f)) {
            std::ofstream of(f);
            of.close();
        }
    } else if (action == "2") {
        f /= to_wide_string(UrlDecode(req.get_param_value("dst")));
        if (!std::filesystem::exists(f)) {
            std::filesystem::create_directory(f);
        }
    } else if (action == "3") {
        if (std::filesystem::exists(f)) {
            std::filesystem::remove_all(f);
        }
    } else if (action == "4") {
        if (std::filesystem::exists(f)) {
            std::filesystem::path dst = to_wide_string(UrlDecode(req.get_param_value("dst")));
            dst /= f.filename();
            std::filesystem::rename(f, dst);
        }
    } else if (action == "5") {
        if (std::filesystem::exists(f)) {
            std::filesystem::path dst = f.parent_path();
            dst /= req.get_param_value("dst");
            std::filesystem::rename(f, dst);
        }
    } else if (action == "6") {
        if (std::filesystem::is_directory(f)) {
            auto q = UrlDecode(req.get_param_value("q"));
            std::regex r(q);
            std::filesystem::path dst = to_wide_string(UrlDecode(req.get_param_value("dst")));
            if (dst.empty())return;
            for (const auto &entry: std::filesystem::directory_iterator(f)) {
                if (regex_search(entry.path().filename().string(), r)) {
                    std::cout << entry.path() << std::endl;
                    std::filesystem::rename(entry.path(), dst / entry.path().filename());
                }
            }
        }
    } else if (action == "7") {
        if (std::filesystem::is_directory(f)) {
            for (const auto &entry: std::filesystem::directory_iterator(f)) {
                if (entry.is_directory()) {
                    std::cout << entry.path() << std::endl;
                    std::filesystem::remove_all(entry.path());
                }
            }
        }
    } else if (action == "8") {
        CreateDirectory();
    }
}

void handler::handleZipFile(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = to_wide_string(UrlDecode(req.get_param_value("path")));
    auto action = req.get_param_value("action");
    if (action.empty()) {
        zipper::Unzipper unzipper(f.string());
        auto n = f.filename().string();
        auto d = f.parent_path() / n.substr(0, n.find_last_of("."));
        unzipper.extract(d.string());
        unzipper.close();
    }
}

handler::handler(const std::string &dir) {
    mDir = std::string{dir};
}

void handler::handlePostFile(const httplib::Request &req, httplib::Response &res,
                             const httplib::ContentReader &content_reader) {
    std::string body;
    content_reader([&](const char *data, size_t data_length) {
        body.append(data, data_length);
        return true;
    });
    std::filesystem::path f = to_wide_string(httplib::detail::decode_url(
            req.get_param_value("path"), true
    ));
    std::ofstream ofs;
    ofs.open(f, std::ofstream::out);
    ofs << body;
    ofs.close();
}
