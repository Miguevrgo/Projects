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

std::map<char, std::pair<int, int>> Utils::GetANSILayout() {

    std::map<char, std::pair<int, int>> layout =
            // First row
            {{'q',{0,0}},
             {'w',{0,1}},
             {'e',{0,2}},
             {'r',{0,3}},
             {'t',{0,4}},
             {'y',{0,5}},
             {'u',{0,6}},
             {'i',{0,7}},
             {'o',{0,8}},
             {'p',{0,9}},
                    // Second row
             {'a',{1,0}},
             {'s',{1,1}},
             {'d',{1,2}},
             {'f',{1,3}},
             {'g',{1,4}},
             {'h',{1,5}},
             {'j',{1,6}},
             {'k',{1,7}},
             {'l',{1,8}},
                    // Third row
             {'z',{2,0}},
             {'x',{2,1}},
             {'c',{2,2}},
             {'v',{2,3}},
             {'b',{2,4}},
             {'n',{2,5}},
             {'m',{2,6}}};
    return layout;
}