/**
 * @file Utils.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Utility functions for the project
 */

#ifndef SPELL_CHECKER_UTILS_H
#define SPELL_CHECKER_UTILS_H

#include <cmath>
#include <filesystem>
#include <map>
#include <string>
#include <utility>
#include <vector>

namespace Utils {
/**
 * @brief Parses the input file to extract words with their frequencies.
 * @param input The input file stream.
 * @param words A vectores to store the extracted words and their frequencies.
 * @return true if the input was successfully parsed, false otherwise
 */
bool ParseInput(const std::filesystem::path &filePath,
                std::vector<std::pair<std::string, unsigned int>> &words);

/**
 * @brief Normalizes a word by converting it to lowercase and removing non-alphabetic characters.
 * @param word The word to be normalized.
 * @return A normalized string.
 */
std::string NormalizeWord(std::string_view word) noexcept;

/**
 * @brief Gets the ANSI keyboard layout.
 * @return A map with character positions on the keyboard.
 */
std::map<char, std::pair<int, int>> GetANSILayout() noexcept;
} // namespace Utils

#endif // SPELL_CHECKER_UTILS_H
