/**
 * @file Corrector.cpp
 * @author Miguel Ángel De la Vegar Rodríguez
 * @brief Implementation of the Corrector class.
 */

#include "Corrector.h"
#include "Utils.h"
#include <algorithm>

std::vector<std::string>
Corrector::GetTopSuggestions(const std::multimap<double, std::string> &corrections, int topN) {
    std::vector<std::pair<std::pair<double, unsigned int>, std::string>> Suggestions;
    std::vector<std::string> topSuggestions;

    for (const auto &correction : corrections) {
        Suggestions.emplace_back(
            std::make_pair(correction.first, dictionary.GetFrequency(correction.second)),
            correction.second);
    }

    // Order corrections by distance and if they have the same distance, order
    // them by frequency
    std::sort(Suggestions.begin(), Suggestions.end(), [](const auto &lhs, const auto &rhs) {
        if (lhs.first.first == rhs.first.first) {
            return lhs.first.second > rhs.first.second;
        }
        return lhs.first.first < rhs.first.first;
    });

    topSuggestions.reserve(Suggestions.size());

    int i = 0;
    for (const auto &suggestion : Suggestions) {
        if (i >= topN) {
            break;
        }
        topSuggestions.emplace_back(suggestion.second);
        i++;
    }

    return topSuggestions;
}

std::multimap<double, std::string> Corrector::SuggestCorrections(std::string_view word) {
    std::multimap<double, std::string> corrections;
    std::string wordToCorrect = Utils::NormalizeWord(word);

    // Limit the search space to words of similar length, in this case, words that
    // are 2 characters longer or shorter
    int minLength = std::max(1, static_cast<int>(word.length()) - 2);
    int maxLength = word.length() + 2;
    bool oneinserted = false;

    std::vector<std::string> wordsOfLengthRange =
        dictionary.GetWordsOfLengthRange(minLength, maxLength);

    for (const auto &dictWord : wordsOfLengthRange) {
        double distance =
            static_cast<double>(CalculateLevenshteinDistance(wordToCorrect, dictWord));

        distance += GetDistanceFromKeyboard(wordToCorrect, dictWord);

        if (distance <= minLength + 1) { // Performance optimization
            corrections.insert({distance, dictWord});
        } else if (!oneinserted) {
            oneinserted = true;
            corrections.insert({distance, dictWord});
        }
    }

    return corrections;
}

int Corrector::CalculateLevenshteinDistance(std::string_view s1, std::string_view s2) {
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
                dp[i - 1][j] + 1,       // Deletion
                dp[i][j - 1] + 1,       // Insertion
                dp[i - 1][j - 1] + cost // Substitution
            });
        }
    }

    return dp[len_s1][len_s2];
}

double Corrector::GetDistanceFromKeyboard(std::string_view s1, std::string_view s2) {
    std::map<char, std::pair<int, int>> layout = Utils::GetANSILayout();

    // Compare chars from both strings and calculate the distance between them
    const auto length = std::min(s1.length(), s2.length());
    double distance = 0;

    double maxDistance = 9.22; // Maximum distance between two characters in the keyboard (p and z)

    for (size_t i = 0; i < length; ++i) {
        std::pair<int, int> pos1 = layout[s1[i]];
        std::pair<int, int> pos2 = layout[s2[i]];

        if (pos1.first == pos2.first && pos1.second == pos2.second) {
            continue;
        }
        // Calculate the distance between the two characters
        distance += std::sqrt(std::pow(pos1.first - pos2.first, 2) +
                              std::pow(pos1.second - pos2.second, 2));
    }

    return distance / maxDistance;
}
