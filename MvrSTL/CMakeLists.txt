cmake_minimum_required(VERSION 3.10)

project(MvrSTL VERSION 1.0
        DESCRIPTION "Custom STL implementation by Miguevr"
        LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS "-Wall -Wextra -Wpedantic -O2")

include_directories(${PROJECT_SOURCE_DIR}/include)

set(SOURCES
    src/unique_ptr.cpp)

add_library(MvrSTL STATIC ${SOURCES})
