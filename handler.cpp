
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
    std::cout << "Handling static files" << std::endl;
    std::filesystem::path f = mDir;
    std::cout << f << " " << req.path << std::endl;

    f /= req.path.substr(1);
    std::cout << f << " " << req.path << std::endl;
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
            serveFile(f, "text/html;charset=UTF-8", res);
            return;
        }
        res.set_header("Content-Disposition", "attachment; filename=\"" + f.filename().string() + "\"");
        serveFile(f, "application/octet-stream", res);
        return;
    }
    if (action == "1") {
        f /= to_wide_string(UrlDecode(req.get_param_value("dst")));
        if (!std::filesystem::exists(f)) {
            if (!std::filesystem::exists(f.parent_path())) {
                std::filesystem::create_directory(f.parent_path());
            }
            std::ofstream of(f);
            of.close();
        }
    } else if (action == "2") {
        f /= to_wide_string(UrlDecode(req.get_param_value("dst")));
        if (!std::filesystem::exists(f)) {
            std::filesystem::create_directories(f);
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
    } else if (action == "10") {
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
    static const char table[] = R"(CREATE TABLE IF NOT EXISTS notes(_id INTEGER PRIMARY KEY AUTOINCREMENT,title TEXT,content TEXT,create_at INTEGER NOT NULL,update_at  INTEGER NOT NULL))";
    db::QueryResult fetch_row = db::query<table>();
    /*
    static const char query[]
            = R"(select _id,title,content,create_at,update_at from notes ORDER by update_at DESC)";
    db1::QueryResult fetch = db1::query<query>();
    std::string id, title, content, create_at, update_at;
    static const char in[]
            = R"(INSERT INTO notes (title,content,create_at,update_at) VALUES (?1,?2,?3,?4))";

    while (fetch(id, title, content, create_at, update_at)) {
        db::QueryResult insert_row = db::query<in>(  title, content,create_at.substr(0, 10), update_at.substr(0, 10));
        std::cout << insert_row.resultCode() << std::endl;
    }

    std::filesystem::path doc(L"C:\\Users\\Administrator\\Desktop\\文档");
    if (std::filesystem::exists(doc)) {
        static const char query[]
                = R"(INSERT INTO notes (title,content,create_at,update_at) VALUES(?1,?2,?3,?4))";

        for (const auto &entry: std::filesystem::recursive_directory_iterator(doc)) {
            if (!entry.is_regular_file() || entry.path().extension() != ".md")continue;
            auto title = entry.path().filename().stem().string();
            auto content = ReadFile(entry.path());
            db::QueryResult
                    fetch_row = db::query<query>(title,
                                                 title + "\n" + content,
                                                 GetTimeStamp(),
                                                 GetTimeStamp()
            );
        }
    }*/
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

void handler::listNotes(const httplib::Request &req, httplib::Response &res) {
    std::cout << "Listing notes..." << std::endl;
    auto limit = req.get_param_value("limit");
    if (limit.empty()) {
        limit = "50";
    }
    static const char query[]
            = R"(select _id,title,update_at from notes ORDER by update_at DESC LIMIT ?1)";
    db::QueryResult fetch_row = db::query<query>(limit);
    std::string_view id, title, update_at;

    nlohmann::json doc = nlohmann::json::array();
    while (fetch_row(id, title, update_at)) {
        nlohmann::json j = {

                {"id",        id},
                {"title",     title},
                {"update_at", update_at},

        };
        doc.push_back(j);
    }
    res.set_content(doc.dump(), "application/json");
}

void
handler::insertNote(const httplib::Request &req, httplib::Response &res, const httplib::ContentReader &content_reader) {
    std::string body;
    content_reader([&](const char *data, size_t data_length) {
        body.append(data, data_length);
        return true;
    });
    nlohmann::json doc = nlohmann::json::parse(body);
    std::string title = doc["title"];
    std::string content = doc["content"];
    if (doc.contains("id")) {
        int id = doc["id"];
        static const char query[]
                = R"(UPDATE notes SET title=?1,content=?2,update_at=?3 where _id =?4)";
        db::QueryResult fetch_row = db::query<query>(title,
                                                     content,
                                                     GetTimeStamp(),
                                                     id
        );
        res.set_content(to_string(fetch_row.resultCode()),
                        "text/plain; charset=UTF-8");
    } else {
        static const char query[]
                = R"(INSERT INTO notes (title,content,create_at,update_at) VALUES(?1,?2,?3,?4))";
        db::QueryResult fetch_row = db::query<query>(title,
                                                     content,
                                                     GetTimeStamp(),
                                                     GetTimeStamp()
        );
        res.set_content(to_string(fetch_row.resultCode()),
                        "text/plain; charset=UTF-8");
    }
}

void handler::getNote(const httplib::Request &req, httplib::Response &res) {
    auto id = req.get_param_value("id");
    static const char query[]
            = R"(select title,content,update_at from notes where _id=?1)";
    db::QueryResult fetch_row = db::query<query>(id);
    std::string_view title, content, update_at;

    if (fetch_row(title, content, update_at)) {
        nlohmann::json j = {
                {"title",     title},
                {"content",   content},
                {"update_at", update_at},

        };
        res.set_content(j.dump(), "application/json");
    }

}

void handler::searchNotes(const httplib::Request &req, httplib::Response &res) {
    auto q = UrlDecode(req.get_param_value("q"));
    bool found = false;
    if (q.starts_with("*")) {
        q = q.substr(1);
        found = true;
    }
    static const char query[]
            = R"(select _id,title,content,update_at from notes ORDER by update_at DESC)";
    db::QueryResult fetch_row = db::query<query>();
    std::string_view id, title, content, update_at;

    nlohmann::json doc = nlohmann::json::array();
    std::regex qr(q);
    while (fetch_row(id, title, content, update_at)) {
        if (std::regex_search((std::string) title, qr) || (
                found && std::regex_search((std::string) content, qr)
        )) {
            nlohmann::json j = {

                    {"id",        id},
                    {"title",     title},
                    {"update_at", update_at},

            };
            doc.push_back(j);
        }

    }
    res.set_content(doc.dump(), "application/json");
}

void handler::handleMoveFiles(const httplib::Request &req, httplib::Response &res,
                              const httplib::ContentReader &content_reader) {
    std::string body;
    content_reader([&](const char *data, size_t data_length) {
        body.append(data, data_length);
        return true;
    });
    std::filesystem::path dir = to_wide_string(UrlDecode(req.get_param_value("dst")));
    nlohmann::json j = nlohmann::json::parse(body);
    for (auto &element: j) {
        std::filesystem::path f(element);
        auto t = dir / f.filename();
        if (!std::filesystem::exists(t)) {
            std::filesystem::rename(f, t);
        }
    }
    nlohmann::json d = {"Result", "OK"};
    res.set_content(d.dump(), "application/json");
}
