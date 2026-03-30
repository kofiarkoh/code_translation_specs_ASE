#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct Edge {
    int dest;
    int weight;
    struct Edge* next;
} Edge;

typedef struct Vertex {
    int weight;
    Edge* edges;
} Vertex;

typedef struct Graph {
    int numVertices;
    Vertex* vertices;
} Graph;

typedef struct MinHeapNode {
    int vertex;
    int distance;
} MinHeapNode;

typedef struct MinHeap {
    int size;
    int capacity;
    MinHeapNode* array;
} MinHeap;

// Function prototypes
Graph* createGraph(int numVertices);
void addEdge(Graph* graph, int src, int dest, int weight);
MinHeap* createMinHeap(int capacity);
void insertMinHeap(MinHeap* minHeap, int vertex, int distance);
MinHeapNode extractMin(MinHeap* minHeap);
void dijkstra(Graph* graph, int src, int* distances);
void computeMaxValues(int* distances, int numVertices, int k, int* output);

int main() {
    int numVertices, numEdges, k;
    scanf("%d %d %d", &numVertices, &numEdges, &k);

    // Preconditions: non-negative integers
    if (numVertices < 0 || numEdges < 0 || k < 0) {
        return 1; // Error handling for invalid input
    }

    Graph* graph = createGraph(numVertices);
    if (!graph) return 1; // Error handling for memory allocation failure

    int* vertexWeights = malloc(numVertices * sizeof(int));
    for (int i = 0; i < numVertices; i++) {
        scanf("%d", &vertexWeights[i]);
        graph->vertices[i].weight = vertexWeights[i];
        graph->vertices[i].edges = NULL; // Initialize edges
    }

    for (int i = 0; i < numEdges; i++) {
        int src, dest, weight;
        scanf("%d %d %d", &src, &dest, &weight);
        addEdge(graph, src, dest, weight);
    }

    int* distances = malloc(numVertices * sizeof(int));
    dijkstra(graph, 0, distances); // Assuming source vertex is 0

    int* output = malloc(k * sizeof(int));
    computeMaxValues(distances, numVertices, k, output);

    for (int i = 0; i < k; i++) {
        printf("%d\n", output[i]);
    }

    // Free allocated memory
    free(vertexWeights);
    free(distances);
    free(output);
    // Free graph and its edges (not shown for brevity)

    return 0;
}

Graph* createGraph(int numVertices) {
    Graph* graph = malloc(sizeof(Graph));
    if (!graph) return NULL; // Error handling for memory allocation failure
    graph->numVertices = numVertices;
    graph->vertices = malloc(numVertices * sizeof(Vertex));
    return graph;
}

void addEdge(Graph* graph, int src, int dest, int weight) {
    Edge* newEdge = malloc(sizeof(Edge));
    newEdge->dest = dest;
    newEdge->weight = weight;
    newEdge->next = graph->vertices[src].edges;
    graph->vertices[src].edges = newEdge;
}

MinHeap* createMinHeap(int capacity) {
    MinHeap* minHeap = malloc(sizeof(MinHeap));
    minHeap->size = 0;
    minHeap->capacity = capacity;
    minHeap->array = malloc(capacity * sizeof(MinHeapNode));
    return minHeap;
}

void insertMinHeap(MinHeap* minHeap, int vertex, int distance) {
    MinHeapNode newNode = {vertex, distance};
    minHeap->array[minHeap->size++] = newNode;
    // Min-heap property should be maintained (not implemented for brevity)
}

MinHeapNode extractMin(MinHeap* minHeap) {
    // Extract the minimum node (not implemented for brevity)
    MinHeapNode minNode = minHeap->array[0];
    // Min-heap property should be maintained (not implemented for brevity)
    return minNode;
}

void dijkstra(Graph* graph, int src, int* distances) {
    for (int i = 0; i < graph->numVertices; i++) {
        distances[i] = INT_MAX; // Initialize distances
    }
    distances[src] = 0;

    MinHeap* minHeap = createMinHeap(graph->numVertices);
    insertMinHeap(minHeap, src, 0);

    while (minHeap->size > 0) {
        MinHeapNode minNode = extractMin(minHeap);
        int u = minNode.vertex;

        Edge* edge = graph->vertices[u].edges;
        while (edge) {
            int v = edge->dest;
            int weight = edge->weight;
            if (distances[u] != INT_MAX && distances[u] + weight < distances[v]) {
                distances[v] = distances[u] + weight;
                insertMinHeap(minHeap, v, distances[v]);
            }
            edge = edge->next;
        }
    }
}

void computeMaxValues(int* distances, int numVertices, int k, int* output) {
    // Simple selection of k maximum values (not implemented for brevity)
}
