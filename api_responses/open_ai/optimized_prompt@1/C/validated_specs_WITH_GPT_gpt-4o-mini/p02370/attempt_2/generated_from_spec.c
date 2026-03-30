#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *edges; // adjacency list
    int *in_degree; // count of incoming edges
    int num_vertices;
} Graph;

Graph* create_graph(int num_vertices, int num_edges) {
    Graph *graph = malloc(sizeof(Graph));
    if (!graph) return NULL; // Handle memory allocation failure

    graph->num_vertices = num_vertices;
    graph->edges = malloc(num_vertices * sizeof(int) * num_vertices); // max edges per vertex
    graph->in_degree = calloc(num_vertices, sizeof(int)); // initialize in-degrees to 0

    if (!graph->edges || !graph->in_degree) {
        free(graph->edges);
        free(graph->in_degree);
        free(graph);
        return NULL; // Handle memory allocation failure
    }

    // Initialize adjacency list
    for (int i = 0; i < num_vertices; i++) {
        for (int j = 0; j < num_vertices; j++) {
            graph->edges[i * num_vertices + j] = -1; // -1 indicates no edge
        }
    }

    return graph;
}

void add_edge(Graph *graph, int from, int to) {
    // Add edge from 'from' to 'to' in the adjacency list
    for (int i = 0; i < graph->num_vertices; i++) {
        if (graph->edges[from * graph->num_vertices + i] == -1) {
            graph->edges[from * graph->num_vertices + i] = to;
            graph->in_degree[to]++; // Increment in-degree of 'to'
            break;
        }
    }
}

void topological_sort(Graph *graph) {
    int *queue = malloc(graph->num_vertices * sizeof(int));
    int front = 0, rear = 0;

    // Enqueue all vertices with zero incoming edges
    for (int i = 0; i < graph->num_vertices; i++) {
        if (graph->in_degree[i] == 0) {
            queue[rear++] = i;
        }
    }

    // Process the queue
    while (front < rear) {
        int vertex = queue[front++]; // Dequeue vertex

        // Print the vertex (part of the topological order)
        printf("%d\n", vertex);

        // Decrease the in-degree of adjacent vertices
        for (int i = 0; i < graph->num_vertices; i++) {
            int adjacent = graph->edges[vertex * graph->num_vertices + i];
            if (adjacent != -1) {
                graph->in_degree[adjacent]--;
                if (graph->in_degree[adjacent] == 0) {
                    queue[rear++] = adjacent; // Enqueue if in-degree becomes zero
                }
            }
        }
    }

    free(queue);
}

void free_graph(Graph *graph) {
    free(graph->edges);
    free(graph->in_degree);
    free(graph);
}

int main() {
    int num_vertices, num_edges;
    scanf("%d %d", &num_vertices, &num_edges);

    Graph *graph = create_graph(num_vertices, num_edges);
    if (!graph) {
        return 1; // Handle memory allocation failure
    }

    for (int i = 0; i < num_edges; i++) {
        int from, to;
        scanf("%d %d", &from, &to);
        add_edge(graph, from, to);
    }

    topological_sort(graph);
    free_graph(graph);
    return 0;
}
