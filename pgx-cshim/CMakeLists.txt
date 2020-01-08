cmake_minimum_required(VERSION 3.5)
project(pgx-cshim)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        pgx-cshim.c)

include_directories(src/c /usr/local/src/pg11/src/include /usr/local/src/pg11/src/backend)

add_library(pgx-shim ${SOURCE_FILES})


