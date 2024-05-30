cmake_minimum_required(VERSION 3.10)
project(Ridmaze)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(SFML 2.5 COMPONENTS graphics window system REQUIRED)

include_directories(${CMAKE_SOURCE_DIR}/include)

add_executable(Ridmaze src/main.cpp
        src/Dice.cpp
        include/Orientation.h
        include/GameCharacter.h
        src/LabyrinthCharacter.cpp
        include/LabyrinthCharacter.h
        src/Monster.cpp
        include/Monster.h
        src/Player.cpp
        include/Player.h
        src/Shield.cpp
        include/Shield.h
        src/Weapon.cpp
        include/Weapon.h
        src/CombatElement.cpp
        include/CombatElement.h
        src/Game.cpp
        include/Game.h
        src/Labyrinth.cpp
        include/Labyrinth.h
        src/GameState.cpp
        include/GameState.h
        src/GameController.cpp
        include/GameController.h
        src/GUI.cpp
        include/Level.h
        src/Level.cpp
        src/Menu.cpp
        include/Menu.h
        src/Settings.cpp
        include/Settings.h
)

target_link_libraries(Ridmaze sfml-graphics sfml-window sfml-system)
