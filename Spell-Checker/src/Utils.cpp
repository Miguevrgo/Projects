/**
 * @file Utils.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Utils namespace
 */

#include "Utils.h"
#include <algorithm>
#include <fstream>
#include <sstream>

bool Utils::ParseInput(const std::filesystem::path &filePath,
                       std::vector<std::pair<std::string, unsigned int>> &words) {
    std::ifstream file(filePath);

    if (!file.is_open()) {
        return false;
    }

    std::string line;
    unsigned int freq;
    std::istringstream iss;

    while (std::getline(file, line)) {
        iss.clear();
        iss.str(line);
        std::string word;

        while (iss >> word >> freq) {
            std::string normalized = NormalizeWord(word);
            if (!normalized.empty()) {
                words.emplace_back(std::move(normalized), freq);
            }
        }
        iss.clear();
    }

    return true;
}

std::string Utils::NormalizeWord(std::string_view word) noexcept {
    std::string normalized;
    normalized.reserve(word.size());
    // Convert the word to lowercase and remove non-alphabetic characters
    std::transform(word.begin(), word.end(), std::back_inserter(normalized),
                   [](unsigned char c) noexcept -> unsigned char { return std::tolower(c); });
    return normalized;
}

std::map<char, std::pair<int, int>> Utils::GetANSILayout() noexcept {

    return {// First row
            {{'q', {0, 0}},
             {'w', {0, 1}},
             {'e', {0, 2}},
             {'r', {0, 3}},
             {'t', {0, 4}},
             {'y', {0, 5}},
             {'u', {0, 6}},
             {'i', {0, 7}},
             {'o', {0, 8}},
             {'p', {0, 9}},
             // Second row
             {'a', {1, 0}},
             {'s', {1, 1}},
             {'d', {1, 2}},
             {'f', {1, 3}},
             {'g', {1, 4}},
             {'h', {1, 5}},
             {'j', {1, 6}},
             {'k', {1, 7}},
             {'l', {1, 8}},
             // Third row
             {'z', {2, 0}},
             {'x', {2, 1}},
             {'c', {2, 2}},
             {'v', {2, 3}},
             {'b', {2, 4}},
             {'n', {2, 5}},
             {'m', {2, 6}}}};
}
