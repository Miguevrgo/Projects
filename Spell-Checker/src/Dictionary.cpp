/**
 * @file Dictionary.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Dictionary class
 */

#include "Dictionary.h"
#include "Utils.h"
#include <functional>



Dictionary::Dictionary(const std::string &filename, const std::string &language) {
    this->language = language;
    LoadFromFile(filename);
}

bool Dictionary::LoadFromFile(const std::string &filename) {
    std::ifstream file(filename);
    if (!file.is_open()){
        return false;
    }

    std::vector<std::pair<std::string,int>> words;
    if (!Utils::ParseInput(file, words)){
        return false;
    }

    for (const auto& word : words){
        trie.Insert(word.first, word.second);
    }

    return true;
}

void Dictionary::SetLanguage(const std::string &newLang) {
    this->language = newLang;
}

void Dictionary::AddWord(const std::string &word, unsigned int frequency) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    this->trie.Insert(word, frequency);
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

std::istream &operator>>(std::ifstream &is, Dictionary &dictionary) {
    std::vector<std::pair<std::string,unsigned int>> words;
    Utils::ParseInput(is, words);

    for (const auto& word : words){
        dictionary.trie.Insert(word);
    }
    return is;
}

Dictionary &Dictionary::operator=(const Dictionary &rhs) {
    this->language = rhs.language;
    this->trie = rhs.trie;
    return *this;
}

std::vector<std::string> Dictionary::GetWordsOfLengthRange(int minLength, int maxLength) const {
    std::vector<std::string> wordsInRange;
    std::string currentWord;
    TrieNode* rootNode = trie.GetRoot();

    std::function<void(TrieNode*, const std::string&)> traverse = [&](TrieNode* node, const std::string& word){
        if (!node) {
            return;
        }

        if (node->frequency && word.length() >= minLength && word.length() <= maxLength) {
            wordsInRange.push_back(word);
        }

        for (const auto& child : node->children) {
            traverse(child.second, word + child.first);
        }
    };

    traverse(rootNode, currentWord);

    return wordsInRange;
}

int Dictionary::GetFrequency(const std::string &word) const {
    return trie.GetFrequency(word);
}
