cmake_minimum_required(VERSION 3.27)
project(Graph_Algo_Visual)

set(CMAKE_CXX_STANDARD 20)
set(BASE_FOLDER ${CMAKE_SOURCE_DIR})

include_directories(${BASE_FOLDER}/include)

add_executable(Graph_Algo_Visual src/main.cpp
        src/Graph.cpp
        include/Graph.h
        src/Algorithms.cpp
        include/Algorithms.h
        src/Algorithms/DFS.cpp
        include/Algorithms/DFS.h)
