cmake_minimum_required(VERSION 3.23.2)
#set(CMAKE_FIND_DEBUG_MODE 1)
set( CMAKE_VERBOSE_MAKEFILE on )
project(Manager)


set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -fexec-charset=GBK -std=c++2a")

set(_HTTPLIB_OPENSSL_MIN_VER "1.1.1")
find_package(OpenSSL ${_HTTPLIB_OPENSSL_MIN_VER} COMPONENTS Crypto SSL REQUIRED)

add_library(httplib SHARED httplib.cc)
target_compile_definitions(
        httplib
        PUBLIC
        CPPHTTPLIB_OPENSSL_SUPPORT
)
target_link_libraries(httplib PUBLIC ws2_32
        OpenSSL::Crypto OpenSSL::SSL -lcrypt32)
add_subdirectory(zipper)
add_subdirectory(nlohmann_json)
#link_libraries(httplib
#        nlohmann_json::nlohmann_json)

add_library(sqlite3 SHARED sqlite3.c)

add_executable(Manager main.cpp handler.cpp helper.cpp)

target_link_libraries(
        Manager
        PUBLIC
        staticZipper
        httplib
        nlohmann_json::nlohmann_json
        sqlite3
)
