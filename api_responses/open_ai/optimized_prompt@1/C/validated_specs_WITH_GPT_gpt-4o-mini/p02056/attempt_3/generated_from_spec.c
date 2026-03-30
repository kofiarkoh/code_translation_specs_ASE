#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int vertex;
    int weight;
} Edge;

typedef struct {
    int n; // number of vertices
    int m; // number of edges
    Edge **adjList; // adjacency list for the graph
} Graph;

typedef struct {
    int *dist; // distances from source
    int *weights; // vertex weights
} ShortestPathResult;

Graph* createGraph(int n, int m) {
    Graph *graph = malloc(sizeof(Graph));
    graph->n = n;
    graph->m = m;
    graph->adjList = malloc(n * sizeof(Edge*));
    for (int i = 0; i < n; i++) {
        graph->adjList[i] = malloc(m * sizeof(Edge)); // allocate space for edges
    }
    return graph;
}

void addEdge(Graph *graph, int src, int dest, int weight, int index) {
    graph->adjList[src][index].vertex = dest;
    graph->adjList[src][index].weight = weight;
}

ShortestPathResult dijkstra(Graph *graph, int source) {
    int *dist = malloc(graph->n * sizeof(int));
    int *visited = calloc(graph->n, sizeof(int));
    for (int i = 0; i < graph->n; i++) {
        dist[i] = INT_MAX; // initialize distances
    }
    dist[source] = 0;

    for (int count = 0; count < graph->n - 1; count++) {
        int minIndex = -1;
        for (int v = 0; v < graph->n; v++) {
            if (!visited[v] && (minIndex == -1 || dist[v] < dist[minIndex])) {
                minIndex = v; // find the vertex with the minimum distance
            }
        }
        visited[minIndex] = 1;

        for (int i = 0; i < graph->m; i++) {
            Edge edge = graph->adjList[minIndex][i];
            if (edge.vertex != 0 && dist[minIndex] != INT_MAX && dist[minIndex] + edge.weight < dist[edge.vertex]) {
                dist[edge.vertex] = dist[minIndex] + edge.weight; // update distance
            }
        }
    }

    ShortestPathResult result = {dist, NULL}; // weights will be set later
    return result;
}

void computeMaxValues(ShortestPathResult spResult, int *weights, int k, int *output) {
    for (int i = 0; i < k; i++) {
        int maxWeight = INT_MIN;
        for (int j = 0; j < spResult.n; j++) {
            if (spResult.dist[j] != INT_MAX) {
                int value = weights[j] - spResult.dist[j]; // compute value
                if (value > maxWeight) {
                    maxWeight = value; // find maximum value
                }
            }
        }
        output[i] = maxWeight; // store maximum value
    }
}

int main() {
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);
    
    int *weights = malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &weights[i]); // read vertex weights
    }

    Graph *graph = createGraph(n, m);
    for (int i = 0; i < m; i++) {
        int src, dest, weight;
        scanf("%d %d %d", &src, &dest, &weight);
        addEdge(graph, src, dest, weight, i); // add edges
    }

    ShortestPathResult spResult = dijkstra(graph, 0); // compute shortest paths

    int *output = malloc(k * sizeof(int));
    computeMaxValues(spResult, weights, k, output); // compute maximum values

    for (int i = 0; i < k; i++) {
        printf("%d\n", output[i]); // output results
    }

    // Free allocated memory
    free(spResult.dist);
    free(weights);
    for (int i = 0; i < n; i++) {
        free(graph->adjList[i]);
    }
    free(graph->adjList);
    free(graph);
    free(output);

    return 0;
}
