#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define INF INT_MAX // Define infinity as the maximum integer value

// Function to compute the minimum cost to reach the destination node
void compute_min_cost(int N, int M, int edges[M][3]) {
    // Array to store the minimum cost to reach each node
    int min_cost[N + 1]; // Using 1-based indexing
    for (int i = 1; i <= N; i++) {
        min_cost[i] = INF; // Initialize all costs to infinity
    }
    min_cost[1] = 0; // Cost to reach the source node is zero

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

    // Check for negative weight cycles affecting reachable nodes
    for (int j = 0; j < M; j++) {
        int u = edges[j][0];
        int w = edges[j][1];
        int c = edges[j][2];
        if (min_cost[u] != INF && min_cost[u] + (-c) < min_cost[w]) {
            // If we can still relax, it indicates a negative weight cycle
            printf("inf\n");
            return;
        }
    }

    // Output the minimum cost to reach the destination node (N)
    if (min_cost[N] == INF) {
        printf("inf\n"); // Destination is unreachable
    } else {
        printf("%d\n", -min_cost[N]); // Output the negated minimum cost
    }
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M); // Read number of nodes and edges

    // Allocate memory for edges
    int edges[M][3];
    for (int i = 0; i < M; i++) {
        scanf("%d %d %d", &edges[i][0], &edges[i][1], &edges[i][2]); // Read each edge
    }

    // Compute the minimum cost
    compute_min_cost(N, M, edges);

    return 0;
}
