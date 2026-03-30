#include <stdio.h>
#include <stdlib.h>

#define MODULO_CONSTANT 1000000007 // Specify the constant for modulo operation

int countInterleavings(int n, int m, int* s, int* t) {
    // Create a 2D array for dynamic programming
    int** dp = (int**)malloc((n + 1) * sizeof(int*));
    for (int i = 0; i <= n; i++) {
        dp[i] = (int*)malloc((m + 1) * sizeof(int));
    }

    // Initialize the dp array
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= m; j++) {
            if (i == 0 && j == 0) {
                dp[i][j] = 1; // Base case: one way to interleave two empty sequences
            } else {
                dp[i][j] = 0; // Initialize to 0
                if (i > 0) {
                    dp[i][j] += dp[i - 1][j]; // Include s[i-1]
                    dp[i][j] %= MODULO_CONSTANT; // Apply modulo
                }
                if (j > 0) {
                    dp[i][j] += dp[i][j - 1]; // Include t[j-1]
                    dp[i][j] %= MODULO_CONSTANT; // Apply modulo
                }
                // Adjust for overcounting if elements are equal
                if (i > 0 && j > 0 && s[i - 1] == t[j - 1]) {
                    dp[i][j] -= dp[i - 1][j - 1]; // Remove overcount
                    if (dp[i][j] < 0) {
                        dp[i][j] += MODULO_CONSTANT; // Ensure non-negative
                    }
                }
            }
        }
    }

    int result = dp[n][m]; // The result is in dp[n][m]

    // Free allocated memory
    for (int i = 0; i <= n; i++) {
        free(dp[i]);
    }
    free(dp);

    return result; // Return the count of distinct interleavings
}

int main() {
    // Example usage
    int n = 3, m = 3;
    int s[] = {1, 2, 1};
    int t[] = {2, 1, 1};

    int result = countInterleavings(n, m, s, t);
    printf("%d\n", result); // Output the result

    return 0;
}
