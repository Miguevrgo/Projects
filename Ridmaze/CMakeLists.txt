cmake_minimum_required(VERSION 3.27)
project(Ridmaze)

set(CMAKE_CXX_STANDARD 20)

add_executable(Ridmaze
        Weapon.cpp
        Weapon.h
        Dice.cpp
        Dice.h
        Player.cpp
        Player.h
        Chest.cpp
        Chest.h)
