#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

void dfs(int vertex, int n, int *visited, int *cumulative_cost, int **graph, int *costs) {
    visited[vertex] = 1; // Mark the vertex as visited
    for (int i = 0; i < n; i++) {
        if (graph[vertex][i] > 0 && !visited[i]) { // If there's an edge and it's not visited
            cumulative_cost[i] = cumulative_cost[vertex] + graph[vertex][i]; // Update cumulative cost
            dfs(i, n, visited, cumulative_cost, graph, costs); // Recursive DFS call
        }
    }
}

int max_cumulative_cost(int n, int *a, int *b, int *c) {
    // Create an adjacency matrix for the graph
    int **graph = (int **)malloc(n * sizeof(int *));
    for (int i = 0; i < n; i++) {
        graph[i] = (int *)calloc(n, sizeof(int)); // Initialize with 0
    }

    // Fill the graph with costs
    for (int i = 0; i < n - 1; i++) {
        graph[a[i]][b[i]] = c[i];
        graph[b[i]][a[i]] = c[i]; // Undirected graph
    }

    int *visited = (int *)calloc(n, sizeof(int)); // Track visited vertices
    int *cumulative_cost = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        cumulative_cost[i] = INT_MIN; // Initialize to a large negative value
    }
    cumulative_cost[0] = 0; // Starting vertex cost

    // Perform DFS from vertex 0
    dfs(0, n, visited, cumulative_cost, graph, c);

    // Find the maximum cumulative cost
    int max_cost = 0;
    for (int i = 0; i < n; i++) {
        if (cumulative_cost[i] > max_cost) {
            max_cost = cumulative_cost[i];
        }
    }

    // Free allocated memory
    for (int i = 0; i < n; i++) {
        free(graph[i]);
    }
    free(graph);
    free(visited);
    free(cumulative_cost);

    return max_cost > 0 ? max_cost : 0; // Return 0 if no path exists
}

int main() {
    // Example usage
    int n = 5; // Number of vertices
    int a[] = {0, 0, 1, 2}; // Edges
    int b[] = {1, 2, 3, 4}; // Edges
    int c[] = {10, 20, 30, 40}; // Costs

    int result = max_cumulative_cost(n, a, b, c);
    printf("Maximum cumulative cost: %d\n", result);
    return 0;
}
