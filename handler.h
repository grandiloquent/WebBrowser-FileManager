//
// Created by psycho on 2023/1/24.
//

#ifndef MANAGER_HANDLER_H
#define MANAGER_HANDLER_H

#include <nlohmann/json.hpp>
#include "httplib.h"
#include <zipper/unzipper.h>
#include <zipper/zipper.h>

class handler {
public:
    explicit handler(const std::string &dir);

    void handleIndex( const httplib::Request &req, httplib::Response &res);
    void handleEditor( const httplib::Request &req, httplib::Response &res);
    void handleVideo( const httplib::Request &req, httplib::Response &res);
    void handleMarkdown( const httplib::Request &req, httplib::Response &res);
    void handleStaticFiles( const httplib::Request &req, httplib::Response &res);
    void handleFiles( const httplib::Request &req, httplib::Response &res);
    void handleFile( const httplib::Request &req, httplib::Response &res);
    void handlePostFile( const httplib::Request &req, httplib::Response &res,const httplib::ContentReader &content_reader);
    void handleZipFile( const httplib::Request &req, httplib::Response &res);

private:
    std::string mDir;
};


#endif //MANAGER_HANDLER_H
