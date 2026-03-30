#include <stdio.h>
#include <stdlib.h>

int compute_max_value(int n, int m, int k, int* values) {
    // Dynamic programming array to store maximum values for each iteration
    int* dp = (int*)malloc(n * sizeof(int));
    if (dp == NULL) {
        // Undefined behavior if memory allocation fails
        return 0; // Placeholder for undefined behavior
    }

    // Initialize the dp array with the input values for the first iteration
    for (int i = 0; i < n; i++) {
        dp[i] = values[i];
    }

    // Perform k iterations of dynamic programming
    for (int iter = 1; iter < k; iter++) {
        int* new_dp = (int*)malloc(n * sizeof(int));
        if (new_dp == NULL) {
            // Undefined behavior if memory allocation fails
            free(dp);
            return 0; // Placeholder for undefined behavior
        }

        // Initialize new_dp for the current iteration
        for (int i = 0; i < n; i++) {
            new_dp[i] = dp[i]; // Start with previous values
            // Check for valid previous indices based on the gap m
            for (int j = i - m; j >= 0; j--) {
                if (j < n) {
                    // Update new_dp[i] with the maximum value considering the gap
                    if (dp[j] + values[i] > new_dp[i]) {
                        new_dp[i] = dp[j] + values[i];
                    }
                }
            }
        }

        // Free the old dp array and point to the new one
        free(dp);
        dp = new_dp;
    }

    // Find the maximum value in the final dp array
    int max_value = dp[0];
    for (int i = 1; i < n; i++) {
        if (dp[i] > max_value) {
            max_value = dp[i];
        }
    }

    // Free the dp array before returning
    free(dp);
    return max_value;
}

int main() {
    // Example usage
    int n = 5; // Number of elements
    int m = 2; // Minimum index gap
    int k = 3; // Number of iterations
    int values[] = {1, 2, 3, 4, 5}; // Input values

    int result = compute_max_value(n, m, k, values);
    printf("Maximum computed value: %d\n", result);
    return 0;
}
