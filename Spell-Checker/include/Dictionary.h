/**
 * @file Dictionary.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Class to store words from a dictionary in a given
 */

#ifndef SPELL_CHECKER_DICTIONARY_H
#define SPELL_CHECKER_DICTIONARY_H

#include "Trie.h"

class Dictionary {
private:
    Trie trie; // Structure that holds the dictionary words
    std::string language; // Language for the Dictionary
public:
    Dictionary() = default;
    ~Dictionary() = default;
    Dictionary(const std::string &filename, const std::string &language);
    void SetLanguage(const std::string &newLang);

    /**
     * @brief Loads the words from a file into the dictionary. As well as the frequency of each word.
     * @param filename
     * @return true if the file was loaded successfully, false otherwise.
     */
    bool LoadFromFile(const std::string &filename);

    /**
     * @brief Adds a word to the dictionary.
     * @param word The word to be added.
     * @param frequency The frequency of the word.
     */
    void AddWord(const std::string &word, unsigned int frequency);

    /**
     * @brief Removes a word from the dictionary.
     * @param word The word to be removed.
     * @return true if the word was removed successfully, false otherwise.
     */
    bool RemoveWord(const std::string &word);
    bool CheckWord(const std::string &word);
    int GetFrequency(const std::string &word) const;

    /**
     * @brief Gets all the words in a given range of length.
     * @param minLength of the words.
     * @param maxLength of the words.
     * @return A vector with all the words in the given range.
     */
    [[nodiscard]] std::vector<std::string> GetWordsOfLengthRange(int minLength, int maxLength) const;
    Dictionary &operator=(const Dictionary &rhs);
    Dictionary operator+(const std::string &word);
    Dictionary operator-(const std::string &word);
    Dictionary operator+(const Dictionary &rhs);

    /**
     * @brief Overloads the operator >> to read a dictionary from a stream.
     * @param is The stream to read from.
     * @param dictionary The dictionary to be read.
     * @return The stream and the dictionary.
     */
    friend std::istream &operator>>(std::ifstream &is, Dictionary &dictionary);
};


#endif //SPELL_CHECKER_DICTIONARY_H
