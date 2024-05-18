#include <algorithm>

/**
 * @brief Quick Sort
 * 
 * @tparam Iterator 
 * @param begin Iterator pointing to first element of the container 
 * @param end Iterator pointing to last element of the container
 * @note Efficiency O(nlog(n))
 * @pre Container elements must have both * and < operators
 */
template<typename Iterator>
void quick_sort (Iterator& begin, Iterator& end) {
    if (begin == end) return;

    auto pivot = *std::next(begin, std::distance(begin, end) / 2);
    auto first_mid = std::partition(begin, end, [pivot](const auto& elem) {return elem < pivot;});
    auto second_mid = std::partition(first_mid, end, [pivot](const auto& elem) {return !(pivot < elem);});
    quick_sort(begin, first_mid);
    quick_sort(second_mid, end);
}   

/**
 * @brief Bubble Sort
 * 
 * @tparam Iterator 
 * @param begin Iterator pointing to first element of the container 
 * @param end Iterator pointing to last element of the container
 * @note Efficiency O(n^2)
 * @pre Container elements must have both * and < operators
 */
template <typename Iterator>
void bubble_sort(Iterator& begin, Iterator& end) {
    for (auto i = begin, i != end, ++i) {
        for (auto j = i, j != end, ++j) {
            if (*(j+1) < *j) {
                std::swap(*(j+1), j);
            }
        }
    }
}

/**
 * @brief Insertion Sort
 * 
 * @tparam Iterator 
 * @param begin Iterator pointing to first element of the container 
 * @param end Iterator pointing to last element of the container
 * @note Efficiency O(n^2)
 * @pre Container elements must have both * and < operators
 */
template <typename Iterator>
void insertion_sort(Iterator& begin, Iterator& end) {
    for (auto i = begin, i != end, ++i) {
        std::rotate(std::upper_bound(begin, i, *i), i, i+1);
    }
}

/**
 * @brief Merge Sort
 * 
 * @tparam Iterator 
 * @param begin Iterator pointing to first element of the container 
 * @param end Iterator pointing to last element of the container
 * @note Efficiency O(nlog(n))
 * @pre Container elements must have both * and < operators
 */
template <typename Iterator>
void merge_sort(Iterator& begin, Iterator& end) {
    if (end - begin > 1) {
        auto middle = begin + (end - begin) / 2;
        merge_sort(begin, middle);
        merge_sort(middle, end);
        std::inplace_merge(begin, middle, end);
    }
}