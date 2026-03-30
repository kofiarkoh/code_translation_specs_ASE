#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int node;
    int weight;
} Edge;

typedef struct {
    Edge* edges;
    int edgeCount;
} Graph;

void addEdge(Graph* graph, int from, int to, int weight) {
    graph->edges[graph->edgeCount].node = to;
    graph->edges[graph->edgeCount].weight = weight;
    graph->edgeCount++;
}

void dijkstra(Graph* graph, int nodeCount, int startNode, int* distances) {
    int* visited = (int*)calloc(nodeCount, sizeof(int));
    for (int i = 0; i < nodeCount; i++) {
        distances[i] = INT_MAX; // Initialize distances to infinity
    }
    distances[startNode] = 0;

    for (int i = 0; i < nodeCount; i++) {
        int minDistance = INT_MAX;
        int currentNode = -1;

        // Find the unvisited node with the smallest distance
        for (int j = 0; j < nodeCount; j++) {
            if (!visited[j] && distances[j] < minDistance) {
                minDistance = distances[j];
                currentNode = j;
            }
        }

        if (currentNode == -1) break; // All reachable nodes are visited

        visited[currentNode] = 1; // Mark the node as visited

        // Update distances for neighbors
        for (int j = 0; j < graph->edgeCount; j++) {
            if (graph->edges[j].node == currentNode) {
                int neighbor = graph->edges[j].node;
                int weight = graph->edges[j].weight;
                if (distances[currentNode] + weight < distances[neighbor]) {
                    distances[neighbor] = distances[currentNode] + weight;
                }
            }
        }
    }

    free(visited);
}

int main() {
    int nodeCount, edgeCount;
    scanf("%d %d", &nodeCount, &edgeCount);

    Graph graph;
    graph.edges = (Edge*)malloc(edgeCount * sizeof(Edge));
    graph.edgeCount = 0;

    for (int i = 0; i < edgeCount; i++) {
        int from, to, weight;
        scanf("%d %d %d", &from, &to, &weight);
        addEdge(&graph, from, to, weight);
        addEdge(&graph, to, from, weight); // Undirected graph
    }

    int startNode, goalNode;
    scanf("%d %d", &startNode, &goalNode);

    int* distances = (int*)malloc(nodeCount * sizeof(int));
    dijkstra(&graph, nodeCount, startNode, distances);

    int queryCount;
    scanf("%d", &queryCount);
    for (int i = 0; i < queryCount; i++) {
        int queryNode;
        scanf("%d", &queryNode);
        if (distances[queryNode] < INT_MAX) {
            printf("Yes\n");
        } else {
            printf("No\n");
        }
    }

    free(graph.edges);
    free(distances);
    return 0;
}
