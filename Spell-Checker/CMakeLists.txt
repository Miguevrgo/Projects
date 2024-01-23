cmake_minimum_required(VERSION 3.27)
project(Spell-Checker)

set(CMAKE_CXX_STANDARD 20)
set(BASE_FOLDER ${CMAKE_SOURCE_DIR})

# wxwidgets
find_package(wxWidgets REQUIRED core base)
include(${wxWidgets_USE_FILE})

include_directories(${BASE_FOLDER}/include)

add_executable(Spell-Checker src/main.cpp src/Corrector.cpp
        src/Dictionary.cpp
        src/Utils.cpp
        src/Trie.cpp
)

# Linking
target_link_libraries(Spell-Checker ${wxWidgets_LIBRARIES})
