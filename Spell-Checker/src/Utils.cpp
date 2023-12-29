/**
 * @file Utils.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Utils namespace
 */

#include "Utils.h"

bool Utils::ParseInput(const std::string &filename, std::vector<std::string> words) {
    std::ifstream input(filename);
    if (!input.is_open()){
        return false;
    }

    std::string word;

    //TODO: Parse input words,


    return true;

}