/**
 * @file Dictionary.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation of Dictionary class
 */

#include "Dictionary.h"
#include "Utils.h"
#include <functional>

Dictionary::Dictionary(const std::filesystem::path &filename, const std::string &language)
    : language(language), trie(std::make_unique<Trie>()) {
    LoadFromFile(filename);
}

Dictionary::Dictionary(Dictionary &&other) noexcept
    : language(std::move(other.language)), trie(std::move(other.trie)) {}

Dictionary::Dictionary(const Dictionary &other) { trie = std::make_unique<Trie>(*other.trie); }

Dictionary &Dictionary::operator=(Dictionary &&rhs) noexcept {
    if (this != &rhs) {
        language = std::move(rhs.language);
        trie = std::move(rhs.trie);
    }
    return *this;
}

bool Dictionary::LoadFromFile(const std::filesystem::path &filename) {
    std::vector<std::pair<std::string, unsigned int>> words;

    if (!Utils::ParseInput(filename, words)) {
        return false;
    }

    for (const auto &word : words) {
        trie->insert(word.first, word.second);
    }

    return true;
}

void Dictionary::SetLanguage(std::string_view newLang) { language = newLang; }

void Dictionary::AddWord(std::string_view word, unsigned int frequency) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    trie->insert(normalizedWord, frequency);
}

bool Dictionary::CheckWord(std::string_view word) const {
    std::string normalizedWord = Utils::NormalizeWord(word);
    return trie->search(normalizedWord);
}

bool Dictionary::RemoveWord(std::string_view word) {
    std::string normalizedWord = Utils::NormalizeWord(word);
    return trie->remove(normalizedWord);
}

Dictionary Dictionary::operator+(std::string_view word) const {
    Dictionary newDictionary(*this);
    newDictionary.AddWord(word, 1);
    return newDictionary;
}

Dictionary Dictionary::operator-(std::string_view word) const {
    Dictionary newDictionary(*this);
    newDictionary.RemoveWord(word);
    return newDictionary;
}

Dictionary Dictionary::operator+(const Dictionary &other) const {
    Dictionary newDictionary(*this);
    for (const auto &word : other.trie->autoComplete("")) {
        newDictionary.AddWord(word.first, word.second);
    }
    return newDictionary;
}

[[nodiscard]] std::vector<std::string>
Dictionary::GetWordsOfLengthRange(int minLength, int maxLength) const noexcept {
    std::vector<std::string> wordsInRange;
    std::string currentWord;
    const TrieNode *rootNode = trie->getRoot();

    std::function<void(const TrieNode *, std::string)> traverse = [&](const TrieNode *node,
                                                                      std::string word) {
        if (!node) {
            return;
        }

        if (node->frequency && word.length() >= static_cast<size_t>(minLength) &&
            word.length() <= static_cast<size_t>(maxLength)) {
            wordsInRange.push_back(word);
        }

        for (const auto &child : node->children) {
            traverse(child.second.get(), word + child.first);
        }
    };

    traverse(rootNode, currentWord);

    return wordsInRange;
}

int Dictionary::GetFrequency(const std::string &word) const { return trie->getFrequency(word); }
