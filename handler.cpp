
#include "handler.h"
#include "helper.h"

using namespace std;

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

void handler::handleStaticFiles(const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;

    f /= req.matches[1].str();
    serveFile(f, req.matches[2].str() == "css" ? "text/css" : "application/javascript", res);
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
        if (f.extension() == ".html" || f.extension() == ".xhtml") {
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
            dst /= to_wide_string(UrlDecode(req.get_param_value("dst")));
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
        CreateDesktopDirectory();
    } else if (action == "9") {
        std::filesystem::path f = to_wide_string(UrlDecode(req.get_param_value("path")));
        TidyDirectory(f.string());
    }else if (action == "10") {
        std::filesystem::path f = to_wide_string(UrlDecode(req.get_param_value("path")));
        MoveFile(f);
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

void handler::handlePage(const string &fileName, const httplib::Request &req, httplib::Response &res) {
    std::filesystem::path f = mDir;
    f /= fileName;
    serveFile(f, "text/html", res);
}
