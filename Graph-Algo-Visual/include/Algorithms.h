/**
 * @file Algorithms.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Class to hold the definition of different Algorithms
 */

#ifndef GRAPH_ALGO_VISUAL_ALGORITHMS_H
#define GRAPH_ALGO_VISUAL_ALGORITHMS_H

#include <vector>
#include "Graph.h"

class Algorithms {
public:
    // Depth-First Search
    static std::vector<int> DFS( int startVertex);

    // Breadth-First Search
    static std::vector<int> BFS(int startVertex);

    // Dijkstra's Algorithm for Shortest Paths
    static std::vector<int> Dijkstra(int startVertex);

    // Kruskal's Algorithm for Minimum Spanning Tree
    static std::vector<Edge> Kruskal();

    // Prim's Algorithm
    static std::vector<Edge> Prim();

private:

};



#endif //GRAPH_ALGO_VISUAL_ALGORITHMS_H
