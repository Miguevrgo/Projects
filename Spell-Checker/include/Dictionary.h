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
    bool LoadFromFile(const std::string &filename);
    void AddWord(const std::string &word);
    bool RemoveWord(const std::string &word);
    bool CheckWord(const std::string &word);
    std::vector<std::string> GetWordsOfLengthRange(int minLength, int maxLength) const;
    Dictionary &operator=(const Dictionary &rhs);
    Dictionary operator+(const std::string &word);
    Dictionary operator-(const std::string &word);
    Dictionary operator+(const Dictionary &rhs);
    friend std::istream &operator>>(std::ifstream &is, Dictionary &dictionary);
};


#endif //SPELL_CHECKER_DICTIONARY_H
