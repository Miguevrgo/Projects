/**
 * @file Trie.cpp
 * @author Miguel Angel De la Vega Rodriguez
 * @brief Implementation of Trie class
 */

#include "Trie.h"

void Trie::clear(TrieNode *node) {
    if(!node){
        return;
    }
    for (auto& child : node->children){
        clear(child.second);
    }
    delete node;
}

Trie::~Trie() {
    clear(root);
}

void Trie::Insert(const std::string &word) {
    TrieNode* node = root;
    for (char c : word) {
        if (!node->children.count(c)) {
            node->children[c] = new TrieNode();
        }
        node = node->children[c];
    }
    node->IsEndOfWord = true;
}

bool Trie::Remove(const std::string &word) {
    return removeHelper(root, word, 0);
}

bool Trie::removeHelper(TrieNode *node, const std::string &word, int depth) {
    if (!node){
        return false;
    }
    if (depth == word.size()) {
        if (node->IsEndOfWord) {
            node->IsEndOfWord = false;
            return node->children.empty();
        }

        return false;
    }

    char c = word[depth];
    TrieNode* child = node->children[c];
    bool ShouldDeleteChild = removeHelper(child, word, depth + 1);

    if (ShouldDeleteChild) {
        node->children.erase(c);
        return node->children.empty();
    }

    return false;
}

bool Trie::Search(const std::string &word) const {
    const TrieNode* node = root;
    for (auto& c : word){
        if (!node->children.count(c)){
            return false;
        }
        node = node->children.at(c);
    }
    return (node && node->IsEndOfWord);
}

bool Trie::StartsWith(const std::string &prefix) const {
    const TrieNode* node = root;
    for (auto& c : prefix){
        if (!node->children.contains(c)) {
            return false;
        }
        node = node->children.at(c);
    }
    return true;
}
