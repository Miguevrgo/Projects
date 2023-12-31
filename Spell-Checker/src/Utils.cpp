/**
 * @file Utils.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Utils namespace
 */

#include "Utils.h"
#include <sstream>

bool Utils::ParseInput(std::ifstream &input, std::vector<std::string> &words) {
    std::string word;
    std::istringstream iss;

    while (std::getline(input, word)) {
        iss.str(word);
        while (iss >> word) {
            Utils::NormalizeWord(word);
            if (!word.empty()){
                words.emplace_back(word);
            }
        }
        iss.clear();
    }

    return true;
}

std::string Utils::NormalizeWord(const std::string &word) {
    std::string normalized;
    // Convert the word to lowercase and remove non-alphabetic characters
    std::transform(word.begin(), word.end(), std::back_inserter(normalized),
                   [](unsigned char c) -> unsigned char { return std::tolower(c); });
    return normalized;
}