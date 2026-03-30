#include <stdio.h>
#include <limits.h>

#define MAX_COST 1000000000

int compute_min_cost(int n, int m, int k[], int t[]) {
    // Initialize the cost matrix with a high constant value
    int cost_matrix[m + 1][n + 1];

    // Set the first column to zero (base case)
    for (int i = 0; i <= m; i++) {
        cost_matrix[i][0] = 0;
    }

    // Initialize the rest of the matrix to MAX_COST
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            cost_matrix[i][j] = MAX_COST;
        }
    }

    // Fill the cost matrix based on the relationship between tasks and weights
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            // Calculate the cost based on the weights and costs
            int current_cost = cost_matrix[i - 1][j] + t[i - 1] * k[j - 1];
            if (current_cost < cost_matrix[i][j]) {
                cost_matrix[i][j] = current_cost;
            }
        }
    }

    // Find the minimum cost in the last row of the matrix
    int min_cost = INT_MAX;
    for (int j = 1; j <= n; j++) {
        if (cost_matrix[m][j] < min_cost) {
            min_cost = cost_matrix[m][j];
        }
    }

    return min_cost;
}

int main() {
    // Example usage
    int n = 3; // Number of tasks
    int m = 2; // Number of weights
    int k[] = {0, 10, 20, 30}; // Weights (1-based indexing)
    int t[] = {0, 5, 15}; // Costs (1-based indexing)

    int min_cost = compute_min_cost(n, m, k, t);
    printf("Minimum Cost: %d\n", min_cost);

    return 0;
}
