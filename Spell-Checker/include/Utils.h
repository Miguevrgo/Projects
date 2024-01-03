/**
 * @file Utils.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Utility functions for the project
 */

#ifndef SPELL_CHECKER_UTILS_H
#define SPELL_CHECKER_UTILS_H

#include <vector>
#include <fstream>
#include <string>
#include <map>
#include <utility>
#include <cmath>

namespace Utils{
    bool ParseInput(std::ifstream& input, std::vector<std::string> &words);
    std::string NormalizeWord(const std::string& word);
    std::map<char, std::pair<int, int>> GetANSILayout();
}


#endif //SPELL_CHECKER_UTILS_H
