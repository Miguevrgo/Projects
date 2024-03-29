/**
 * @file Trie.h
 * @author Miguel Angel De la Vega Rodriguez
 * @brief Class of TDA Trie used by dictionary to store words in an efficient way
 */

#ifndef SPELL_CHECKER_TRIE_H
#define SPELL_CHECKER_TRIE_H

#include <unordered_map>
#include <string>
#include <vector>

class TrieNode {
public:
    unsigned int frequency; // Frequency of the word | 0 for non-existent words
    std::unordered_map<char, TrieNode*> children;
    TrieNode() : frequency(0) {};
};

class Trie {
private:
    TrieNode* root;
    void clear(TrieNode* node);
    bool removeHelper(TrieNode* node, const std::string& word, int depth);

public:
    Trie() : root(new TrieNode()) {}
    ~Trie() { clear(root);}
    void Insert(const std::string &word, unsigned int frequency);
    bool Remove(const std::string &word);
    int GetFrequency(const std::string &word) const;
    TrieNode* GetRoot() const;
    [[nodiscard]] bool Search(const std::string &word) const;
    [[nodiscard]] bool StartsWith(const std::string &prefix) const;
    [[nodiscard]] std::vector<std::pair<std::string,unsigned int>> AutoComplete (const std::string &prefix) const;
    void FindAllWords(const TrieNode* node, std::string& current, std::vector<std::pair<std::string, unsigned int>> words) const;
};


#endif //SPELL_CHECKER_TRIE_H
