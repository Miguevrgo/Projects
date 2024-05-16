cmake_minimum_required(VERSION 3.17)
project(Ridmaze)

set(CMAKE_CXX_STANDARD 20)

include_directories(include)

file(GLOB_RECURSE SOURCES "src/*.cpp")

add_executable(MazeGame ${SOURCES})

find_package(SFML 2.5 COMPONENTS system window graphics REQUIRED)
target_link_libraries(MazeGame sfml-system sfml-window sfml-graphics)