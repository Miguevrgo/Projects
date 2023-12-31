/**
 * @file Corrector.cpp
 * @author Miguel Ángel De la Vegar Rodríguez
 * @brief Implementation of the Corrector class.
 */

#include "Corrector.h"

std::vector<std::string> Corrector::GetTopSuggestions(const std::map<double, std::string> &corrections, int topN) {
    std::vector<std::string> topSuggestions;
    int i = 0;
    for (const auto& correction : corrections){
        if (i >= topN){
            break;
        }
        topSuggestions.emplace_back(correction.second);
        i++;
    }
    return topSuggestions;
}
