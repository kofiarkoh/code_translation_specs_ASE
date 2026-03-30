#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define INF INT_MAX // Define infinity as the maximum integer value

// Function to compute the minimum cost to reach a destination node in a directed graph
void compute_min_cost(int N, int M, int edges[][3], int source, int destination) {
    // Array to store the minimum cost to reach each node
    int *min_cost = (int *)malloc((N + 1) * sizeof(int));
    
    // Initialize min_cost array
    for (int i = 1; i <= N; i++) {
        min_cost[i] = INF; // Set all costs to infinity
    }
    min_cost[source] = 0; // Cost to reach the source is zero

    // Relax edges up to N-1 times
    for (int i = 1; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int u = edges[j][0];
            int w = edges[j][1];
            int c = edges[j][2];
            if (min_cost[u] != INF && min_cost[u] + (-c) < min_cost[w]) {
                min_cost[w] = min_cost[u] + (-c); // Update cost with negated edge cost
            }
        }
    }

    // Check for negative weight cycles affecting the destination
    for (int j = 0; j < M; j++) {
        int u = edges[j][0];
        int w = edges[j][1];
        int c = edges[j][2];
        if (min_cost[u] != INF && min_cost[u] + (-c) < min_cost[w]) {
            // If we can still relax, it indicates a negative weight cycle
            min_cost[w] = -INF; // Mark as affected by negative cycle
        }
    }

    // Output the result
    if (min_cost[destination] == INF) {
        printf("inf\n"); // Destination is unreachable
    } else if (min_cost[destination] == -INF) {
        printf("inf\n"); // Negative cycle affects the destination
    } else {
        printf("%d\n", -min_cost[destination]); // Output the negated minimum cost
    }

    free(min_cost); // Free allocated memory
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M); // Read number of nodes and edges

    // Allocate memory for edges
    int (*edges)[3] = malloc(M * sizeof(*edges));

    // Read edges
    for (int i = 0; i < M; i++) {
        scanf("%d %d %d", &edges[i][0], &edges[i][1], &edges[i][2]);
    }

    int source, destination;
    scanf("%d %d", &source, &destination); // Read source and destination

    // Call the function to compute minimum cost
    compute_min_cost(N, M, edges, source, destination);

    free(edges); // Free allocated memory for edges
    return 0;
}
