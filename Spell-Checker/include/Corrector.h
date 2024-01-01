/**
 * @file Corrector.h
 * @author Miguel Angel De la Vegar Rodríguez
 * @brief Class to check the spelling of a word and suggest corrections
 * for each possible language (Spanish and English).
 */

#ifndef SPELL_CHECKER_CORRECTOR_H
#define SPELL_CHECKER_CORRECTOR_H

#include "Dictionary.h"
#include <map>

class Corrector {
private:
    const Dictionary& dictionary;
    int CalculateLevenshteinDistance(const std::string& s1, const std::string& s2);
public:
    Corrector() = delete;
    ~Corrector() = default;
    explicit Corrector(const Dictionary& dictionary) : dictionary(dictionary) {};
    std::map<double, std::string> SuggestCorrections(const std::string& word);
    std::vector<std::string> GetTopSuggestions(const std::map<double, std::string>& corrections, int topN);
};


#endif //SPELL_CHECKER_CORRECTOR_H
