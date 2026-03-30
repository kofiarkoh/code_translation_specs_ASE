#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MEW 1000 // Define a constant for maximum weight

int main() {
    int N, W;
    
    // Read the number of items and the maximum weight capacity
    if (scanf("%d %d", &N, &W) != 2 || N < 0 || W < 0) {
        // Error handling for invalid input
        printf("Error: Invalid input format or negative values.\n");
        return 1; // Return error code
    }

    // Initialize arrays for weights and values
    int *weights = (int *)malloc(N * sizeof(int));
    int *values = (int *)malloc(N * sizeof(int));
    
    // Read weights and values for each item
    for (int i = 0; i < N; i++) {
        if (scanf("%d %d", &weights[i], &values[i]) != 2) {
            // Error handling for invalid input
            printf("Error: Invalid input format for item %d.\n", i + 1);
            free(weights);
            free(values);
            return 1; // Return error code
        }
    }

    // Initialize the tracking array for minimum weights
    int *dp = (int *)malloc((W + 1) * sizeof(int));
    for (int j = 0; j <= W; j++) {
        dp[j] = INT_MAX; // Set to a value greater than maximum weight
    }
    dp[0] = 0; // Base case: 0 weight for 0 value

    // Process each item
    for (int i = 0; i < N; i++) {
        for (int j = W; j >= weights[i]; j--) {
            if (dp[j - weights[i]] != INT_MAX) {
                dp[j] = (dp[j] < dp[j - weights[i]] + values[i]) ? dp[j] : (dp[j - weights[i]] + values[i]);
            }
        }
    }

    // Find the maximum value that can be achieved without exceeding the weight limit
    int maxValue = 0;
    for (int j = 0; j <= W; j++) {
        if (dp[j] != INT_MAX) {
            maxValue = (maxValue < dp[j]) ? dp[j] : maxValue;
        }
    }

    // Check if a valid configuration exists
    if (maxValue == 0) {
        printf("Error: No valid configuration exists.\n");
    } else {
        printf("%d\n", maxValue); // Output the maximum value
    }

    // Free allocated memory
    free(weights);
    free(values);
    free(dp);
    
    return 0; // Successful execution
}
