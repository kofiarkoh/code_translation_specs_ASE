#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct Edge {
    int to;
    int weight;
    struct Edge* next;
} Edge;

typedef struct Graph {
    int numNodes;
    Edge** adjList;
} Graph;

Graph* createGraph(int numNodes) {
    Graph* graph = malloc(sizeof(Graph));
    if (!graph) exit(EXIT_FAILURE); // Handle memory allocation failure
    graph->numNodes = numNodes;
    graph->adjList = malloc(numNodes * sizeof(Edge*));
    if (!graph->adjList) exit(EXIT_FAILURE); // Handle memory allocation failure
    for (int i = 0; i < numNodes; i++) {
        graph->adjList[i] = NULL;
    }
    return graph;
}

void addEdge(Graph* graph, int from, int to, int weight) {
    Edge* edge = malloc(sizeof(Edge));
    if (!edge) exit(EXIT_FAILURE); // Handle memory allocation failure
    edge->to = to;
    edge->weight = weight;
    edge->next = graph->adjList[from];
    graph->adjList[from] = edge;
}

void dijkstra(Graph* graph, int start, int* distances, int* reachable) {
    int* visited = calloc(graph->numNodes, sizeof(int));
    for (int i = 0; i < graph->numNodes; i++) {
        distances[i] = INT_MAX; // Initialize distances to infinity
        reachable[i] = 0; // Initialize reachability
    }
    distances[start] = 0; // Distance to start node is 0

    for (int count = 0; count < graph->numNodes - 1; count++) {
        int minDist = INT_MAX;
        int minIndex = -1;

        for (int v = 0; v < graph->numNodes; v++) {
            if (!visited[v] && distances[v] <= minDist) {
                minDist = distances[v];
                minIndex = v;
            }
        }

        if (minIndex == -1) break; // All reachable nodes processed

        visited[minIndex] = 1; // Mark the node as visited
        reachable[minIndex] = 1; // Mark the node as reachable

        for (Edge* edge = graph->adjList[minIndex]; edge != NULL; edge = edge->next) {
            if (!visited[edge->to] && distances[minIndex] != INT_MAX &&
                distances[minIndex] + edge->weight < distances[edge->to]) {
                distances[edge->to] = distances[minIndex] + edge->weight;
            }
        }
    }

    free(visited);
}

int main() {
    int numNodes, numEdges;
    scanf("%d %d", &numNodes, &numEdges);

    Graph* graph = createGraph(numNodes);

    for (int i = 0; i < numEdges; i++) {
        int from, to, weight;
        scanf("%d %d %d", &from, &to, &weight);
        addEdge(graph, from, to, weight);
        addEdge(graph, to, from, weight); // Undirected graph
    }

    int start, goal;
    scanf("%d %d", &start, &goal);

    int* distances = malloc(numNodes * sizeof(int));
    int* reachable = malloc(numNodes * sizeof(int));
    dijkstra(graph, start, distances, reachable);

    // Output reachability for the goal node
    if (reachable[goal]) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    // Process queries
    int queryCount;
    scanf("%d", &queryCount);
    for (int i = 0; i < queryCount; i++) {
        int queryNode;
        scanf("%d", &queryNode);
        if (reachable[queryNode]) {
            printf("Yes\n");
        } else {
            printf("No\n");
        }
    }

    // Free allocated memory
    for (int i = 0; i < numNodes; i++) {
        Edge* edge = graph->adjList[i];
        while (edge) {
            Edge* temp = edge;
            edge = edge->next;
            free(temp);
        }
    }
    free(graph->adjList);
    free(graph);
    free(distances);
    free(reachable);

    return 0;
}
