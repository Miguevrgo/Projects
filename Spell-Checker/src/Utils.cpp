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

std::string Utils::NormalizeWord(const std::string &word) {
    std::string normalized;
    // Convert the word to lowercase and remove non-alphabetic characters
    std::transform(word.begin(), word.end(), std::back_inserter(normalized),
                   [](unsigned char c) -> unsigned char { return std::tolower(c); });
    return normalized;
}