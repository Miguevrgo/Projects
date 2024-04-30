#include "tree.h"

template <typename T>
void Tree<T>::operator=(const Tree &tree) noexcept {
    root = tree.root;
}

template <typename T>
void Tree<T>::clear() noexcept
{
    root.reset();
}

template <typename T>
inline void Tree<T>::insert(const T &node)
{
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

template <typename T>
inline void Tree<T>::remove(const T &node){
    if (!root) {
        return;
    }
    else{
        Node *current = root.get();
        Node *parent = nullptr;
        while (current) {
            if (node < current->data) {
                parent = current;
                current = current->leftChild.get();
            }
            else if (node > current->data) {
                parent = current;
                current = current->rightChild.get();
            }
            else {
                if (!current->leftChild && !current->rightChild) {
                    if (current == root.get()) {
                        root.reset();
                    }
                    else if (parent->leftChild.get() == current) {
                        parent->leftChild.reset();
                    }
                    else {
                        parent->rightChild.reset();
                    }
                }
                else if (current->leftChild && current->rightChild) {
                    Node *successor = current->rightChild.get();
                    while (successor->leftChild) {
                        successor = successor->leftChild.get();
                    }
                    T temp = successor->data;
                    remove(successor->data);
                    current->data = temp;
                }
                else {
                    Node *child = current->leftChild ? current->leftChild.get() : current->rightChild.get();
                    if (current == root.get()) {
                        root.reset(child);
                    }
                    else if (parent->leftChild.get() == current) {
                        parent->leftChild.reset(child);
                    }
                    else {
                        parent->rightChild.reset(child);
                    }
                }
                return;
            }
        }
    }
}

template <typename T>
bool Tree<T>::find(const T &value) const {
    if (!root) {
        return false;
    }
    else {
        Node *current = root.get();
        while (current) {
            if (value < current->data) {
                current = current->leftChild.get();
            }
            else if (value > current->data) {
                current = current->rightChild.get();
            }
            else {
                return true;
            }
        }
        return false;
    }
}
