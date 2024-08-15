/**
 * @file Dictionary.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Class to store words from a dictionary in a given
 */

#ifndef SPELL_CHECKER_DICTIONARY_H
#define SPELL_CHECKER_DICTIONARY_H

#include "Trie.h"
#include <filesystem>

class Dictionary {
  private:
    std::string language;       // Language for the Dictionary
    std::unique_ptr<Trie> trie; // Structure that holds the dictionary words
  public:
    Dictionary() = default;
    ~Dictionary() = default;
    Dictionary(const std::filesystem::path &filename, const std::string &language);
    Dictionary(Dictionary &&other) noexcept;
    Dictionary(const Dictionary &other);
    Dictionary &operator=(const Dictionary &rhs) = delete;
    Dictionary &operator=(Dictionary &&rhs) noexcept;
    Dictionary operator+(std::string_view word) const;
    Dictionary operator-(std::string_view word) const;
    Dictionary operator+(const Dictionary &rhs) const;

    void SetLanguage(std::string_view newLang);

    /**
     * @brief Loads the words from a file into the dictionary. As well as the frequency of each
     * word.
     * @param filename
     * @return true if the file was loaded successfully, false otherwise.
     */
    bool LoadFromFile(const std::filesystem::path &filename);

    /**
     * @brief Adds a word to the dictionary.
     * @param word The word to be added.
     * @param frequency The frequency of the word.
     */
    void AddWord(std::string_view word, unsigned int frequency);

    /**
     * @brief Removes a word from the dictionary.
     * @param word The word to be removed.
     * @return true if the word was removed successfully, false otherwise.
     */
    bool RemoveWord(std::string_view word);
    bool CheckWord(std::string_view word) const;
    [[nodiscard]] int GetFrequency(const std::string &word) const;

    /**
     * @brief Gets all the words in a given range of length.
     * @param minLength of the words.
     * @param maxLength of the words.
     * @return A vector with all the words in the given range.
     */
    [[nodiscard]] std::vector<std::string> GetWordsOfLengthRange(int minLength,
                                                                 int maxLength) const noexcept;
};

#endif // SPELL_CHECKER_DICTIONARY_H
