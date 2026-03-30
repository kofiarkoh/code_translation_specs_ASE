#include <stdio.h>
#include <stdlib.h>

#define MODULO 1000000007 // Define a constant for modulo operation

int countSelections(int n, int s, int* arr) {
    // Create a dynamic programming table
    int** dp = (int**)malloc((n + 1) * sizeof(int*));
    for (int i = 0; i <= n; i++) {
        dp[i] = (int*)malloc((s + 1) * sizeof(int));
    }

    // Initialize the dp table
    for (int i = 0; i <= n; i++) {
        dp[i][0] = 1; // There's one way to make sum 0: select nothing
    }
    for (int j = 1; j <= s; j++) {
        dp[0][j] = 0; // No way to make positive sum with 0 elements
    }

    // Fill the dp table
    for (int i = 1; i <= n; i++) {
        for (int j = 0; j <= s; j++) {
            dp[i][j] = dp[i - 1][j]; // Exclude the current element
            if (j >= arr[i - 1]) {
                // Include the current element
                dp[i][j] = (dp[i][j] + dp[i - 1][j - arr[i - 1]]) % MODULO;
            }
        }
    }

    int result = dp[n][s]; // The result is in the bottom-right cell

    // Free the allocated memory
    for (int i = 0; i <= n; i++) {
        free(dp[i]);
    }
    free(dp);

    return result; // Return the number of ways to select elements
}

int main() {
    // Example usage
    int n = 3; // Number of elements
    int s = 4; // Target sum
    int arr[] = {1, 2, 3}; // The array of elements

    int result = countSelections(n, s, arr);
    printf("%d\n", result); // Output the result

    return 0;
}
