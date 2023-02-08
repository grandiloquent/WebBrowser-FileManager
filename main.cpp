#include <iostream>
#include <filesystem>
#include "httplib.h"
#include "handler.h"
#include <Windns.h>

static inline void WritePrefix(std::ostream *os, const char *prefix, bool odd) {
    if (prefix != nullptr) {
        *os << prefix;
    }
    *os << "  ";
    if (!odd) {
        *os << " ";
    }
}

static bool RunCommand(const std::string &cmd, std::ostream *os, const char *prefix) {
    FILE *stream = popen(cmd.c_str(), "r");
    if (stream) {
        if (os != nullptr) {
            bool odd_line = true;               // We indent them differently.
            bool wrote_prefix = false;          // Have we already written a prefix?
            constexpr size_t kMaxBuffer = 128;  // Relatively small buffer. Should be OK as we're on an
            // alt stack, but just to be sure...
            char buffer[kMaxBuffer];
            while (!feof(stream)) {
                if (fgets(buffer, kMaxBuffer, stream) != nullptr) {
                    // Split on newlines.
                    char *tmp = buffer;
                    for (;;) {
                        char *new_line = strchr(tmp, '\n');
                        if (new_line == nullptr) {
                            // Print the rest.
                            if (*tmp != 0) {
                                if (!wrote_prefix) {
                                    WritePrefix(os, prefix, odd_line);
                                }
                                wrote_prefix = true;
                                *os << tmp;
                            }
                            break;
                        }
                        if (!wrote_prefix) {
                            WritePrefix(os, prefix, odd_line);
                        }
                        char saved = *(new_line + 1);
                        *(new_line + 1) = 0;
                        *os << tmp;
                        *(new_line + 1) = saved;
                        tmp = new_line + 1;
                        odd_line = !odd_line;
                        wrote_prefix = false;
                    }
                }
            }
        }
        pclose(stream);
        return true;
    } else {
        return false;
    }
}

int main() {

    WSADATA wsa_Data;
    int wsa_ReturnCode = WSAStartup(0x101, &wsa_Data);

// Get the local hostname
    char szHostName[255];
    gethostname(szHostName, 255);
    struct hostent *host_entry;
    host_entry = gethostbyname(szHostName);
    char *szLocalIP;
    szLocalIP = inet_ntoa(*(struct in_addr *) *host_entry->h_addr_list);
    std::cout << "http://" << szLocalIP << ":8080" << std::endl;
    WSACleanup();
    auto dir = R"(C:\Users\Administrator\Desktop\Resources\Manager)";
    handler h{dir};
    httplib::Server server;
    server.Get("/", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleIndex(req, res);
    });
    server.Get("/editor", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleEditor(req, res);
    });
    server.Get("/video", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleVideo(req, res);
    });
    server.Get(R"(/([a-z-]+\.(js|css)))", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleStaticFiles(req, res);
    });
    server.Get("/api/files", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleFiles(req, res);
    });
    server.Get("/api/file", [&h](const httplib::Request &req, httplib::Response &res) {
        h.handleFile(req, res);

    });
    server.Post("/api/file", [&h](const httplib::Request &req, httplib::Response &res,
                                  const httplib::ContentReader &content_reader) {
        h.handlePostFile(req, res, content_reader);
    });
    server.Get("/api/cmd", [](const httplib::Request &request,
                              httplib::Response &response) {
        response.set_header("Access-Control-Allow-Origin", "*");
        auto cmd = httplib::detail::decode_url(request.get_param_value("q"), true);
        std::stringbuf buff;
        std::ostream out{&buff};

        RunCommand(cmd, &out, nullptr);
        std::ostringstream ss;
        ss << out.rdbuf();
        response.set_content(ss.str(), "text/plain");
    });
    server.listen(szLocalIP, 8080);
    return 0;
}
