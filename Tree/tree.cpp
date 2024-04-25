#include "tree.h"

template <typename T>
inline void Tree<T>::insert(const T &node) {
    if (!root) {
        root = node;
    }
    else {
        Node *current = root.get();
        while (current) {
            if (node < current->data) {
                if (!current->leftChild) {
                    current->leftChild = std::make_unique<Node>(node);
                    return;
                }
                current = current->leftChild.get();
            }
            else {
                if (!current->rightChild) {
                    current->rightChild = std::make_unique<Node>(node);
                    return;
                }
                current = current->rightChild.get();
            }
        }
    }
    
}

template <typename T>
constexpr bool Tree<T>::empty() const noexcept {
    return root;
}

template <typename T>
T Tree<T>::leftChild(const Node &node) const noexcept {
    return node->leftChild->data;
}

template <typename T>
T Tree<T>::rightChild(const Node &node) const noexcept {
    return node->rightChild->data;
}
