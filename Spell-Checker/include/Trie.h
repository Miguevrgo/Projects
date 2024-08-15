/**
 * @file Trie.h
 * @author Miguel Angel De la Vega Rodriguez
 * @brief Class of TDA Trie used by dictionary to store words in an efficient way
 */

#ifndef SPELL_CHECKER_TRIE_H
#define SPELL_CHECKER_TRIE_H

#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

class TrieNode {
  public:
    unsigned int frequency; // Frequency of the word | 0 for non-existent words
    std::unordered_map<char, std::unique_ptr<TrieNode>> children;
    TrieNode() : frequency(0){};
    TrieNode(const TrieNode &other);
    TrieNode &operator=(const TrieNode &rhs);
};

class Trie {
  private:
    std::unique_ptr<TrieNode> root;

    void clear(TrieNode *node) noexcept;
    bool removeHelper(TrieNode *node, std::string_view word, size_t depth) noexcept;
    void findAllWords(const TrieNode *node, std::string &current,
                      std::vector<std::pair<std::string, unsigned int>> &words) const noexcept;

  public:
    Trie() noexcept : root(std::make_unique<TrieNode>()) {}
    Trie(const Trie &other);
    Trie &operator=(const Trie &other);
    Trie(Trie &&) noexcept = default;            // Allow move
    Trie &operator=(Trie &&) noexcept = default; // Allow move
    ~Trie() noexcept { clear(root.get()); }

    void insert(std::string_view word, unsigned int frequency) noexcept;
    bool remove(std::string_view word) noexcept;
    [[nodiscard]] bool search(std::string_view word) const noexcept;
    [[nodiscard]] bool startsWith(std::string_view prefix) const noexcept;
    [[nodiscard]] std::vector<std::pair<std::string, unsigned int>>
    autoComplete(std::string_view prefix) const noexcept;
    [[nodiscard]] int getFrequency(std::string_view word) const noexcept;
    [[nodiscard]] const TrieNode *getRoot() const noexcept;
};

#endif // SPELL_CHECKER_TRIE_H
