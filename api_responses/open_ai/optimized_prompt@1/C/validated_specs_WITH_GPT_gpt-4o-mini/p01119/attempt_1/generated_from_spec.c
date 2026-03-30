#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int main() {
    int n, m;
    
    // Read the number of integers
    if (scanf("%d", &n) != 1 || n < 0) {
        return -1; // Invalid input for number of integers
    }
    
    int *integers = malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &integers[i]) != 1 || integers[i] < 0) {
            free(integers);
            return -1; // Invalid input for integers
        }
    }
    
    // Read the number of weights
    if (scanf("%d", &m) != 1 || m < 0) {
        free(integers);
        return -1; // Invalid input for number of weights
    }
    
    int *weights = malloc(m * sizeof(int));
    for (int i = 0; i < m; i++) {
        if (scanf("%d", &weights[i]) != 1 || weights[i] < 0) {
            free(integers);
            free(weights);
            return -1; // Invalid input for weights
        }
    }
    
    // To store the minimum absolute difference
    int min_diff = INT_MAX;
    int found_valid_difference = 0;

    // Generate all combinations of computed values
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < (1 << m); j++) { // 2^m combinations
            int computed_value = integers[i];
            for (int k = 0; k < m; k++) {
                if (j & (1 << k)) {
                    computed_value += weights[k]; // Add weight
                } else {
                    computed_value -= weights[k]; // Subtract weight
                }
            }
            // Check if computed_value is non-negative
            if (computed_value >= 0) {
                found_valid_difference = 1;
                int diff = abs(integers[i] - computed_value);
                if (diff < min_diff) {
                    min_diff = diff; // Update minimum difference
                }
            }
        }
    }

    // If no valid difference was found, return -1
    if (!found_valid_difference) {
        printf("-1\n");
    } else {
        printf("%d\n", min_diff);
    }

    // Free allocated memory
    free(integers);
    free(weights);
    
    return 0;
}
