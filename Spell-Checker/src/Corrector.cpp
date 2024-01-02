/**
 * @file Corrector.cpp
 * @author Miguel Ángel De la Vegar Rodríguez
 * @brief Implementation of the Corrector class.
 */

#include "Corrector.h"

std::vector<std::string> Corrector::GetTopSuggestions(const std::multimap<double, std::string> &corrections, int topN) {
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

std::multimap<double, std::string> Corrector::SuggestCorrections(const std::string &word) {
    std::multimap<double, std::string> corrections;
    std::string wordToCorrect = Utils::NormalizeWord(word);
    // Limit the search space to words of similar length, in this case, words that are 2 characters longer or shorter
    int minLength = std::max(0, static_cast<int>(word.length()) - 2);
    int maxLength = word.length() + 2;

    std::vector<std::string> wordsOfLengthRange = dictionary.GetWordsOfLengthRange(minLength, maxLength);

    for (const auto& dictWord : wordsOfLengthRange) {
        double distance = static_cast<double>(CalculateLevenshteinDistance(wordToCorrect, dictWord));

        if (distance < minLength){ // Performance optimization
            corrections.insert({distance, dictWord});
        }
    }

    return corrections;
}


int Corrector::CalculateLevenshteinDistance(const std::string &s1, const std::string &s2) {
    int len_s1 = s1.size();
    int len_s2 = s2.size();

    std::vector<std::vector<int>> dp(len_s1 + 1, std::vector<int>(len_s2 + 1));

    for (int i = 0; i <= len_s1; ++i) {
        dp[i][0] = i; // Deletion cost
    }
    for (int j = 0; j <= len_s2; ++j) {
        dp[0][j] = j; // Insertion cost
    }

    for (int i = 1; i <= len_s1; ++i) {
        for (int j = 1; j <= len_s2; ++j) {
            int cost = (s1[i - 1] == s2[j - 1]) ? 0 : 1;

            dp[i][j] = std::min({
                dp[i-1][j] + 1,    // Deletion
                dp[i][j-1] + 1,    // Insertion
                dp[i-1][j-1] + cost // Substitution
            });
        }
    }

    return dp[len_s1][len_s2];
}
