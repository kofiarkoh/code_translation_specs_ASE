#include <stdio.h>
#include <stdlib.h>

int compute_maximum(int n, int m, int k, int* arr) {
    // Dynamic programming array to store maximum values
    int* dp = (int*)malloc((n + 1) * sizeof(int));
    if (dp == NULL) {
        // Undefined behavior if memory allocation fails
        return 0; // Placeholder for undefined behavior
    }

    // Initialize the dp array
    for (int i = 0; i <= n; i++) {
        dp[i] = 0; // Starting with zero, can be adjusted based on requirements
    }

    // Dynamic programming process
    for (int iter = 0; iter < k; iter++) {
        for (int i = 0; i < n; i++) {
            // Calculate the maximum value considering the range constraint
            for (int j = i; j < n && j < i + m; j++) {
                if (arr[j] > dp[i]) {
                    dp[i] = arr[j]; // Update dp[i] with the maximum found
                }
            }
        }
    }

    // Find the overall maximum from the dp array
    int max_value = dp[0];
    for (int i = 1; i < n; i++) {
        if (dp[i] > max_value) {
            max_value = dp[i];
        }
    }

    free(dp); // Free allocated memory
    return max_value; // Return the computed maximum value
}

int main() {
    // Example usage
    int n = 5; // Number of elements
    int m = 2; // Range constraint
    int k = 3; // Number of iterations
    int arr[] = {1, 3, 5, 2, 4}; // Input array

    int result = compute_maximum(n, m, k, arr);
    printf("Maximum value: %d\n", result); // Output the result

    return 0;
}
