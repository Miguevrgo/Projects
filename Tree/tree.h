/**
 * @file tree.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Tree class implementation for C++, namespace std is not use
 * to avoid conflicts in a remote future where some tree class is added to 
 * the standard, however, the purpose of this header is to create the equivalent
 * of some of the STL Data Structures in Tree, which I found really useful in 
 * certain situations, where I lack of a implementation.
 */

#ifndef TREE_TREE_H
#define TREE_TREE_H
#if __cplusplus > 201703L

#include <memory>

template<typename T>
class Tree {
private:
    struct Node {
        T data;
        std::unique_ptr<Node> leftChild;
        std::unique_ptr<Node> rightChild;

        inline explicit Node (const T& value) noexcept : data(value), 
            leftChild(nullptr), rightChild(nullptr) {}
    };

    std::unique_ptr<Node> root;

public:
    inline Tree() noexcept : root(nullptr) {}

    inline explicit Tree(const T& value) noexcept : 
        root(std::make_unique<Node>(value)) {}
    
    inline void insert(const T& node);

    constexpr bool empty() const noexcept;

    T leftChild(const Node& node) const noexcept;
    T rightChild(const Node& node) const noexcept;

    inline void remove(const T& node);
    bool find(const T& value) const;

    



};



#endif //__cplusplus > 201703L
#endif //TREE_TREE_H
