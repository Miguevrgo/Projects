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
