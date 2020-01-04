cmake_minimum_required(VERSION 3.5)
project(bridge-c-shim)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES
        bridge-c-shim.c)

include_directories(src/c /usr/local/src/pg11/src/include /usr/local/src/pg11/src/backend)

add_library(pg-rs-bridge-shim ${SOURCE_FILES})


