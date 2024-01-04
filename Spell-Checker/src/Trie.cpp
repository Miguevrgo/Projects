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

void Trie::Insert(const std::string &word, unsigned int frequency) {
    TrieNode* node = root;
    for (char c : word) {
        if (!node->children.count(c)) {
            node->children[c] = new TrieNode();
        }
        node = node->children[c];
    }
    node->frequency = frequency;
}

bool Trie::Remove(const std::string &word) {
    return removeHelper(root, word, 0);
}

bool Trie::removeHelper(TrieNode *node, const std::string &word, int depth) {
    if (!node){
        return false;
    }
    if (depth == word.size()) {
        if (node->frequency) {
            node->frequency = 0;
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
    return (node && node->frequency);
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

std::vector<std::pair<std::string,unsigned int>> Trie::AutoComplete(const std::string &prefix) const {
    std::vector<std::pair<std::string, unsigned int>> completions;
    if (!StartsWith(prefix)){
        return completions;
    }

    const TrieNode* node = root;
    for (auto& c : prefix){
        node = node->children.at(c);
    }
    std::string current = prefix;
    FindAllWords(node, current, completions);
    return completions;
}

void Trie::FindAllWords(const TrieNode *node, std::string &current, std::vector<std::pair<std::string, unsigned int>> words) const {
    if (node->frequency){
        words.emplace_back(current, node->frequency);
    }
    for (const auto& child : node->children){
        current.push_back(child.first);
        FindAllWords(child.second,current,words);
        current.pop_back();
    }
}

TrieNode *Trie::GetRoot() const {
    return root;
}

int Trie::GetFrequency(const std::string &word) const {
    const TrieNode* node = root;
    for (auto& c : word){
        if (!node->children.count(c)){
            return 0;
        }
        node = node->children.at(c);
    }
    return node->frequency;
}
