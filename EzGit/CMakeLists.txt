cmake_minimum_required(VERSION 3.24)
project(EzGit)

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(dependencies/ftxui)

include_directories(
        ${CMAKE_SOURCE_DIR}/dependencies/ftxui/include
        ${CMAKE_SOURCE_DIR}/include
)


file(GLOB SOURCES "src/*.cpp")

add_executable(${PROJECT_NAME} ${SOURCES}
        src/GitManager.cpp
        include/GitManager.h)

# Linkear con las bibliotecas de ftxui
target_link_libraries(${PROJECT_NAME} ftxui::screen ftxui::dom ftxui::component)
