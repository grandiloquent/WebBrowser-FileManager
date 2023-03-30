//
// Created by psycho on 2023/1/24.
//

#ifndef MANAGER_HANDLER_H
#define MANAGER_HANDLER_H

#include <nlohmann/json.hpp>
#include "httplib.h"
#include <zipper/unzipper.h>
#include <zipper/zipper.h>
#include "SQLiteWrapper.h"

static const char db_name[] = "C:\\Users\\Administrator\\Desktop\\Resources\\Manager\\build\\notes.db";
using db = sqlite::Database<db_name>;
//static const char db1_name[] = "C:\\Users\\Administrator\\Downloads\\notes.db";
//using db1 = sqlite::Database<db1_name>;
class handler {
public:
    explicit handler(const std::string &dir);


    void handleStaticFiles(const httplib::Request &req, httplib::Response &res);

    void handleFiles(const httplib::Request &req, httplib::Response &res);

    void handleFile(const httplib::Request &req, httplib::Response &res);

    void
    handlePostFile(const httplib::Request &req, httplib::Response &res, const httplib::ContentReader &content_reader);

    void handleZipFile(const httplib::Request &req, httplib::Response &res);


    void handlePage(const std::string &fileName, const httplib::Request &req, httplib::Response &res);

    void listNotes(const httplib::Request &req, httplib::Response &res);

    void insertNote(const httplib::Request &req, httplib::Response &res, const httplib::ContentReader &content_reader);
    void getNote(const httplib::Request &req, httplib::Response &res);
    void searchNotes(const httplib::Request &req, httplib::Response &res);


private:
    std::string mDir;
};


#endif //MANAGER_HANDLER_H
