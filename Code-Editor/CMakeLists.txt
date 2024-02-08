cmake_minimum_required(VERSION 3.27)
project(Code_Editor)

set(CMAKE_CXX_STANDARD 20)
set(BASE_FOLDER ${CMAKE_SOURCE_DIR})

include_directories(${BASE_FOLDER}/include)

add_executable(Code_Editor main.cpp
        src/Buffer.cpp
        include/Buffer.h
        src/Editor.cpp
        include/Editor.h
        Cursor.cpp
        Cursor.h)
