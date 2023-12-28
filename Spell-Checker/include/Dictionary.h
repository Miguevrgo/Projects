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
public:
    Dictionary();
    Dictionary(const std::string &filename);
    ~Dictionary();

    bool LoadFromFile(const std::string &filename);
    bool AddWord(const std::string &word);
    bool RemoveWord(const std::string &word);
    bool CheckWord(const std::string &word);
    std::vector<std::string> SuggestCorrections(const std::string &word, int maxDistance) const;
    bool SaveToFile(const std::string &filename) const;
};


#endif //SPELL_CHECKER_DICTIONARY_H
