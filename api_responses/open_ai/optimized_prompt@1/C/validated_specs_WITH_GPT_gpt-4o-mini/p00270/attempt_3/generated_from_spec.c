#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct Edge {
    int destination;
    int weight;
    struct Edge* next;
} Edge;

typedef struct Graph {
    int numNodes;
    Edge** adjacencyList;
} Graph;

Graph* createGraph(int numNodes) {
    Graph* graph = malloc(sizeof(Graph));
    graph->numNodes = numNodes;
    graph->adjacencyList = malloc(numNodes * sizeof(Edge*));
    for (int i = 0; i < numNodes; i++) {
        graph->adjacencyList[i] = NULL; // Initialize adjacency list
    }
    return graph;
}

void addEdge(Graph* graph, int src, int dest, int weight) {
    Edge* newEdge = malloc(sizeof(Edge));
    newEdge->destination = dest;
    newEdge->weight = weight;
    newEdge->next = graph->adjacencyList[src];
    graph->adjacencyList[src] = newEdge; // Add edge to the adjacency list
}

void dijkstra(Graph* graph, int startNode, int* distances) {
    int* visited = malloc(graph->numNodes * sizeof(int));
    for (int i = 0; i < graph->numNodes; i++) {
        distances[i] = INT_MAX; // Initialize distances to infinity
        visited[i] = 0; // Mark all nodes as unvisited
    }
    distances[startNode] = 0; // Distance to the start node is 0

    for (int i = 0; i < graph->numNodes - 1; i++) {
        int minDistance = INT_MAX;
        int minIndex = -1;

        // Find the unvisited node with the smallest distance
        for (int j = 0; j < graph->numNodes; j++) {
            if (!visited[j] && distances[j] < minDistance) {
                minDistance = distances[j];
                minIndex = j;
            }
        }

        if (minIndex == -1) break; // All reachable nodes are visited

        visited[minIndex] = 1; // Mark the node as visited

        // Update distances for adjacent nodes
        for (Edge* edge = graph->adjacencyList[minIndex]; edge != NULL; edge = edge->next) {
            if (!visited[edge->destination] && 
                distances[minIndex] != INT_MAX && 
                distances[minIndex] + edge->weight < distances[edge->destination]) {
                distances[edge->destination] = distances[minIndex] + edge->weight;
            }
        }
    }
    free(visited); // Free the visited array
}

int main() {
    int numNodes, numEdges;
    scanf("%d %d", &numNodes, &numEdges);

    Graph* graph = createGraph(numNodes);

    for (int i = 0; i < numEdges; i++) {
        int src, dest, weight;
        scanf("%d %d %d", &src, &dest, &weight);
        addEdge(graph, src, dest, weight);
        addEdge(graph, dest, src, weight); // Since the graph is undirected
    }

    int startNode, goalNode;
    scanf("%d %d", &startNode, &goalNode);

    int* distances = malloc(numNodes * sizeof(int));
    dijkstra(graph, startNode, distances);

    int queryCount;
    scanf("%d", &queryCount);
    for (int i = 0; i < queryCount; i++) {
        int queryNode;
        scanf("%d", &queryNode);
        if (distances[queryNode] < INT_MAX) {
            printf("Yes\n"); // Node is reachable
        } else {
            printf("No\n"); // Node is not reachable
        }
    }

    free(distances); // Free the distances array
    for (int i = 0; i < numNodes; i++) {
        Edge* edge = graph->adjacencyList[i];
        while (edge) {
            Edge* temp = edge;
            edge = edge->next;
            free(temp); // Free each edge
        }
    }
    free(graph->adjacencyList);
    free(graph); // Free the graph structure

    return 0;
}
