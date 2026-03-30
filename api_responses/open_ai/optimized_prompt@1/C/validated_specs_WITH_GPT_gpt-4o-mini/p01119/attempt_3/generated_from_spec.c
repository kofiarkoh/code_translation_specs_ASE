#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int main() {
    int n, m;
    
    // Read the number of integers
    if (scanf("%d", &n) != 1 || n < 0) {
        return -1; // Invalid input for number of integers
    }
    
    int *integers = (int *)malloc(n * sizeof(int));
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
    
    int *weights = (int *)malloc(m * sizeof(int));
    for (int i = 0; i < m; i++) {
        if (scanf("%d", &weights[i]) != 1 || weights[i] < 0) {
            free(integers);
            free(weights);
            return -1; // Invalid input for weights
        }
    }
    
    int min_difference = INT_MAX; // Initialize to maximum possible value
    int found_valid_difference = 0; // Flag to check if any valid difference is found

    // Generate computed values from weights
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            // Compute the value by adding and subtracting weights
            int computed_value_add = integers[i] + weights[j];
            int computed_value_sub = integers[i] - weights[j];

            // Check the computed values for non-negativity
            if (computed_value_add >= 0) {
                int difference = abs(integers[i] - computed_value_add);
                if (difference < min_difference) {
                    min_difference = difference;
                    found_valid_difference = 1; // A valid difference is found
                }
            }
            if (computed_value_sub >= 0) {
                int difference = abs(integers[i] - computed_value_sub);
                if (difference < min_difference) {
                    min_difference = difference;
                    found_valid_difference = 1; // A valid difference is found
                }
            }
        }
    }

    // Output the result
    if (found_valid_difference) {
        printf("%d\n", min_difference);
    } else {
        printf("-1\n"); // No valid difference found
    }

    // Free allocated memory
    free(integers);
    free(weights);
    
    return 0;
}
