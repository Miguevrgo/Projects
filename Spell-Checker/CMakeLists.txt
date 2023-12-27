cmake_minimum_required(VERSION 3.27)
project(Spell-Checker)

set(CMAKE_CXX_STANDARD 20)
set(BASE_FOLDER ${CMAKE_SOURCE_DIR})

include_directories(${BASE_FOLDER}/include)

add_executable(Spell-Checker src/main.cpp src/Corrector.cpp
        src/Dictionary.cpp
        src/Learner.cpp
        src/Utils.cpp)

