/**
 * @file Dictionary.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Dictionary class
 */

#include "Dictionary.h"
#include "Utils.h"


Dictionary::Dictionary(const std::string &filename, const std::string &language) {
    this->language = language;
    LoadFromFile(filename);
}

bool Dictionary::LoadFromFile(const std::string &filename) {
    std::vector<std::string> words;
    if (!Utils::ParseInput(filename, words)){
        return false;
    }

    for (const auto& word : words){
        trie.Insert(word);
    }

    return true;
}

void Dictionary::SetLanguage(const std::string &newLang) {
    this->language = newLang;
}

void Dictionary::AddWord(const std::string &word) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    this->trie.Insert(word);
}

bool Dictionary::CheckWord(const std::string &word) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    return (this->trie.Search(word));
}

bool Dictionary::RemoveWord(const std::string &word) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    return (this->trie.Remove(word));
}

Dictionary Dictionary::operator+(const std::string &word) {
    Dictionary newDictionary = *this;
    newDictionary.AddWord(word);
    return newDictionary;
}

Dictionary Dictionary::operator-(const std::string &word) {
    Dictionary newDictionary = *this;
    newDictionary.RemoveWord(word);
    return newDictionary;
}

Dictionary Dictionary::operator+(const Dictionary &other) {
    Dictionary newDictionary = *this;
    for (const auto& word : other.trie.AutoComplete("")){
        newDictionary.AddWord(word);
    }
    return newDictionary;
}
