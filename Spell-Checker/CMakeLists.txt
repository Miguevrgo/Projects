cmake_minimum_required(VERSION 3.14)

project(Spell-Checker VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS "-Wall -Wextra -Wpedantic -O2")

find_package(Qt5 COMPONENTS Widgets REQUIRED)
include_directories(${PROJECT_SOURCE_DIR}/include)
add_executable(Spell-Checker src/main.cpp src/Corrector.cpp
        src/Dictionary.cpp
        src/Utils.cpp
        src/Trie.cpp
)

target_link_libraries(Spell-Checker Qt5::Widgets)

