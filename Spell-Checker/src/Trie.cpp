/**
 * @file Trie.cpp
 * @author Miguel Angel De la Vega Rodriguez
 * @brief Implementation of Trie class
 */
#include "Trie.h"

TrieNode::TrieNode(const TrieNode &other) : frequency(other.frequency) {
    for (const auto &pair : other.children) {
        children[pair.first] = std::make_unique<TrieNode>(*pair.second);
    }
}

TrieNode &TrieNode::operator=(const TrieNode &rhs) {
    if (this != &rhs) {
        frequency = rhs.frequency;
        children.clear();
        for (const auto &pair : rhs.children) {
            children[pair.first] = std::make_unique<TrieNode>(*pair.second);
        }
    }
    return *this;
}

Trie::Trie(const Trie &other) {
    root = std::make_unique<TrieNode>(*other.root);
}

Trie &Trie::operator=(const Trie &rhs) {
    if (this != &rhs) {
        root = std::make_unique<TrieNode>(*rhs.root);
    }
    return *this;
}

void Trie::clear(TrieNode *node) noexcept {
    if (!node)
        return;
    for (auto &[_, child] : node->children) {
        clear(child.get());
    }
}

void Trie::insert(std::string_view word, unsigned int frequency) noexcept {
    TrieNode *node = root.get();
    for (char c : word) {
        if (!node->children.count(c)) {
            node->children[c] = std::make_unique<TrieNode>();
        }
        node = node->children[c].get();
    }
    node->frequency = frequency;
}

bool Trie::remove(std::string_view word) noexcept { return removeHelper(root.get(), word, 0); }

bool Trie::removeHelper(TrieNode *node, std::string_view word, size_t depth) noexcept {
    if (!node)
        return false;

    if (depth == word.size()) {
        if (node->frequency) {
            node->frequency = 0;
            return node->children.empty();
        }
        return false;
    }

    char c = word[depth];
    auto child = node->children.find(c);
    if (child == node->children.end())
        return false;

    bool shouldDeleteChild = removeHelper(child->second.get(), word, depth + 1);

    if (shouldDeleteChild) {
        node->children.erase(c);
        return node->children.empty();
    }

    return false;
}

bool Trie::search(std::string_view word) const noexcept {
    const TrieNode *node = root.get();
    for (char c : word) {
        auto child = node->children.find(c);
        if (child == node->children.end())
            return false;
        node = child->second.get();
    }
    return node && node->frequency > 0;
}

bool Trie::startsWith(std::string_view prefix) const noexcept {
    const TrieNode *node = root.get();
    for (char c : prefix) {
        auto child = node->children.find(c);
        if (child == node->children.end())
            return false;
        node = child->second.get();
    }
    return true;
}

std::vector<std::pair<std::string, unsigned int>>
Trie::autoComplete(std::string_view prefix) const noexcept {
    std::vector<std::pair<std::string, unsigned int>> completions;
    if (!startsWith(prefix))
        return completions;

    const TrieNode *node = root.get();
    for (char c : prefix) {
        auto child = node->children.find(c);
        if (child == node->children.end())
            return completions;
        node = child->second.get();
    }

    std::string current = std::string(prefix);
    findAllWords(node, current, completions);
    return completions;
}

void Trie::findAllWords(const TrieNode *node, std::string &current,
                        std::vector<std::pair<std::string, unsigned int>> &words) const noexcept {
    if (node->frequency > 0) {
        words.emplace_back(current, node->frequency);
    }

    for (const auto &[key, child] : node->children) {
        current.push_back(key);
        findAllWords(child.get(), current, words);
        current.pop_back();
    }
}

int Trie::getFrequency(std::string_view word) const noexcept {
    const TrieNode *node = root.get();
    for (char c : word) {
        auto child = node->children.find(c);
        if (child == node->children.end())
            return 0;
        node = child->second.get();
    }
    return node->frequency;
}

const TrieNode *Trie::getRoot() const noexcept { return root.get(); }
