cmake_minimum_required(VERSION 3.23.2)
project(Manager)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -fexec-charset=GBK")

add_library(httplib SHARED httplib.cc)

target_link_libraries(httplib PUBLIC ws2_32)
add_subdirectory(nlohmann_json)
link_libraries(httplib
        nlohmann_json::nlohmann_json)

add_executable(Manager main.cpp handler.cpp handler.h)
