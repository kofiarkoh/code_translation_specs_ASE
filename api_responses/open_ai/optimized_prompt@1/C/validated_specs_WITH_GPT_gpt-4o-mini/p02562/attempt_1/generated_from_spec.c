#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_NODES 100 // Assuming a maximum number of nodes for simplicity

// Function prototypes
long compute_max_flow_cost(int N, int K, int cost_matrix[MAX_NODES][MAX_NODES], int flow_matrix[MAX_NODES][MAX_NODES]);

int main() {
    int N, K;
    int cost_matrix[MAX_NODES][MAX_NODES];
    int flow_matrix[MAX_NODES][MAX_NODES] = {0}; // Initialize flow matrix to zero

    // Input reading (assumed to be well-formed as per specification)
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%d", &cost_matrix[i][j]);
        }
    }

    // Compute the maximum flow cost
    long max_flow_cost = compute_max_flow_cost(N, K, cost_matrix, flow_matrix);

    // Output the maximum flow cost
    printf("%ld\n", max_flow_cost);

    // Output the flow configuration matrix
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            printf("%d ", flow_matrix[i][j]);
        }
        printf("\n");
    }

    return 0;
}

// Function to compute the maximum flow cost using a successive shortest path algorithm
long compute_max_flow_cost(int N, int K, int cost_matrix[MAX_NODES][MAX_NODES], int flow_matrix[MAX_NODES][MAX_NODES]) {
    long total_cost = 0;
    // Placeholder for the actual flow computation logic
    // This should implement the successive shortest path algorithm
    // For now, we will return a placeholder value indicating infeasibility
    // In a real implementation, this would involve complex logic to find the max flow

    // If flow cannot be computed, return an infinite cost
    return LONG_MAX; // Indicating infeasibility as per specification
}
