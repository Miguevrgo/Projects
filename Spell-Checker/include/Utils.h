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

namespace Utils{
    bool ParseInput(std::ifstream& input, std::vector<std::string> &words);
    std::string NormalizeWord(const std::string& word);
}


#endif //SPELL_CHECKER_UTILS_H
