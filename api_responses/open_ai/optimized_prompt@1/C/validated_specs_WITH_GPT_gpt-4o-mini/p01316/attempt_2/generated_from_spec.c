#include <stdio.h>
#include <limits.h>

int compute_min_transformation_cost(int n, int m, int adjustment[], int values[]) {
    // Initialize the minimum cost to a large value (infinity)
    int min_cost = INT_MAX;

    // Iterate over each value to be transformed
    for (int i = 0; i < n; i++) {
        // For each value, calculate the cost for each adjustment
        for (int j = 0; j < m; j++) {
            // Ensure adjusted values are within the range of 0 to 255
            if (adjustment[j] < 0 || adjustment[j] > 255) {
                continue; // Skip invalid adjustments
            }
            // Calculate the cost using the quadratic function
            int cost = (values[i] - adjustment[j]) * (values[i] - adjustment[j]);
            // Update the minimum cost if the current cost is lower
            if (cost < min_cost) {
                min_cost = cost;
            }
        }
    }

    // If no valid transformation was found, return an infinite cost
    return (min_cost == INT_MAX) ? -1 : min_cost; // -1 indicates infinite cost
}

int main() {
    int n, m;
    // Read n and m
    scanf("%d %d", &n, &m);
    
    // Ensure n and m are positive integers
    if (n <= 0 || m <= 0) {
        return -1; // Undefined behavior for non-positive integers
    }

    int adjustment[m];
    int values[n];

    // Read adjustment values
    for (int i = 0; i < m; i++) {
        scanf("%d", &adjustment[i]);
    }

    // Read values to be transformed
    for (int i = 0; i < n; i++) {
        scanf("%d", &values[i]);
    }

    // Compute and print the minimum transformation cost
    int result = compute_min_transformation_cost(n, m, adjustment, values);
    printf("%d\n", result);

    return 0;
}
