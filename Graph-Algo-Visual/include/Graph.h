/**
 * @file Graph.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Class to design the structure of a Graph
 */

#ifndef GRAPH_ALGO_VISUAL_GRAPH_H
#define GRAPH_ALGO_VISUAL_GRAPH_H

#include <list>
#include <vector>

struct Edge {
    int dest;
    int weight;
};

struct AdjList {
    std::list<Edge> edges;
};

class Graph {
public:
    Graph(int V);
    Graph(const Graph &rhs);
    ~Graph();
    void AddGraph(const AdjList& newGraph); // TODO: Revise
private:
    int nVerts;
    std::vector<AdjList> adjList;
};


#endif //GRAPH_ALGO_VISUAL_GRAPH_H
