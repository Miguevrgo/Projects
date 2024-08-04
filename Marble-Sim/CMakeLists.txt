cmake_minimum_required(VERSION 3.10)
project(Marble-Sim)

set(CMAKE_CXX_STANDARD 20)

find_package(OpenGL REQUIRED)
find_package(glfw3 REQUIRED)
find_package(GLEW REQUIRED)

include_directories(
    ${OPENGL_INCLUDE_DIR}
    ${GLFW_INCLUDE_DIRS}
    ${GLEW_INCLUDE_DIRS}
)

file(GLOB SOURCES "src/*.cpp")

add_executable(Marble-Sim ${SOURCES})

target_link_libraries(Marble-Sim
    ${OPENGL_LIBRARIES}
    glfw
    GLEW::GLEW
)
