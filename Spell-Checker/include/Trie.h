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
    bool IsEndOfWord;
    std::unordered_map<char, TrieNode*> children;
    TrieNode() : IsEndOfWord(false) {};
    ~TrieNode() {
        for (auto& child : children) {
            delete child.second;
        }
    }
};

class Trie {
private:
    TrieNode* root;
    void clear(TrieNode* node);
    bool removeHelper(TrieNode* node, const std::string& word, int depth);

public:
    Trie() : root(new TrieNode()) {}
    ~Trie();
    void Insert(const std::string &word);
    bool Remove(const std::string &word);
    [[nodiscard]] bool Search(const std::string &word) const;
    [[nodiscard]] bool StartsWith(const std::string &prefix) const;
    [[nodiscard]] std::vector<std::string> AutoComplete (const std::string &prefix) const;
    void FindAllWords(const TrieNode* node, std::string& current, std::vector<std::string> words) const;
};


#endif //SPELL_CHECKER_TRIE_H
