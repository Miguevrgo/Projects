cmake_minimum_required(VERSION 3.26)
project(ImagenRGB)

set(CMAKE_CXX_STANDARD 17)
set(BASE_FOLDER ${CMAKE_CURRENT_SOURCE_DIR})

include_directories(${BASE_FOLDER}/include)

add_executable(ImagenRGB src/main.cpp
        src/image.cpp
        include/image.h)
