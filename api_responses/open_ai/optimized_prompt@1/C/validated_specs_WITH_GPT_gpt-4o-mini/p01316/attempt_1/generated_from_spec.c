#include <stdio.h>
#include <limits.h>

int compute_min_cost(int n, int m, int adjustments[], int values[]) {
    // Initialize the minimum cost to a large value (infinity)
    int min_cost = INT_MAX;

    // Iterate over each value to be transformed
    for (int i = 0; i < n; i++) {
        // Iterate over each adjustment value
        for (int j = 0; j < m; j++) {
            // Calculate the transformed value
            int transformed_value = values[i] + adjustments[j];
            
            // Check if the transformed value is within the valid range
            // Assuming valid integers means we don't have a specific range check
            // Here we just calculate the cost as the square of the transformed value
            // This is a placeholder for the actual quadratic function
            int cost = transformed_value * transformed_value; // Quadratic cost function
            
            // Update the minimum cost if the current cost is lower
            if (cost < min_cost) {
                min_cost = cost;
            }
        }
    }

    // If no valid transformation was found, return an infinite cost
    // Here we assume that if min_cost is still INT_MAX, it indicates no valid transformation
    return (min_cost == INT_MAX) ? INT_MAX : min_cost;
}

int main() {
    // Example usage
    int n = 3; // Number of values
    int m = 2; // Number of adjustments
    int adjustments[] = {1, -1}; // Adjustment values
    int values[] = {5, 10, 15}; // Values to transform

    int result = compute_min_cost(n, m, adjustments, values);
    if (result == INT_MAX) {
        printf("Infinite cost\n");
    } else {
        printf("Minimum transformation cost: %d\n", result);
    }

    return 0;
}
