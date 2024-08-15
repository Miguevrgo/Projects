/**
 * @file Corrector.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Class to check the spelling of a word and suggest corrections
 * for each possible language (Spanish and English).
 */

#ifndef SPELL_CHECKER_CORRECTOR_H
#define SPELL_CHECKER_CORRECTOR_H

#include "Dictionary.h"
#include <map>

class Corrector {
  private:
    const Dictionary &dictionary;

    /**
     * @brief Calculates the Levenshtein distance between two strings.
     * @param s1 First string.
     * @param s2 Second string.
     * @return The Levenshtein distance between s1 and s2.
     */
    int CalculateLevenshteinDistance(std::string_view s1, std::string_view s2);

    /**
     * @brief Calculates the distance between two strings using the keyboard layout.
     * @param s1 First string.
     * @param s2 Second string.
     * @return The distance between s1 and s2 in an QWERTY keyboard.
     */
    double GetDistanceFromKeyboard(std::string_view s1, std::string_view s2);

  public:
    Corrector() = delete;
    ~Corrector() = default;
    explicit Corrector(const Dictionary &dictionary) : dictionary(dictionary){};

    /**
     * @brief Suggests all the possible corrections of a word where the Levenshtein distance is less
     * than 2.
     * @param word the word to be corrected.
     * @return A multimap with the possible corrections and their distance to the original word.
     */
    std::multimap<double, std::string> SuggestCorrections(std::string_view word);

    /**
     * @brief Gets the top N suggestions from a multimap of corrections, it orders the corrections
     * by distance. if two corrections have the same distance, it orders them by frequency.
     * @param corrections The multimap of corrections.
     * @param topN The number of suggestions to be returned.
     * @return A vector with the top N suggestions.
     */
    std::vector<std::string>
    GetTopSuggestions(const std::multimap<double, std::string> &corrections, int topN);
};

#endif // SPELL_CHECKER_CORRECTOR_H
