#include <stdio.h>

#define MODULO_CONSTANT 1000000007 // Specify the modulo constant as per requirement

int countInterleavings(int n, int m, int* s, int* t) {
    // Create a 2D array to store the number of interleavings
    long long dp[n + 1][m + 1];

    // Initialize the dp array
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= m; j++) {
            dp[i][j] = 0; // Set all values to 0 initially
        }
    }

    // Base case: There's one way to interleave two empty sequences
    dp[0][0] = 1;

    // Fill the dp array
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= m; j++) {
            // If we can take from s
            if (i > 0) {
                dp[i][j] += dp[i - 1][j];
                dp[i][j] %= MODULO_CONSTANT; // Apply modulo to prevent overflow
            }
            // If we can take from t
            if (j > 0) {
                dp[i][j] += dp[i][j - 1];
                dp[i][j] %= MODULO_CONSTANT; // Apply modulo to prevent overflow
            }
            // Adjust for overcounting if the last elements are equal
            if (i > 0 && j > 0 && s[i - 1] == t[j - 1]) {
                dp[i][j] -= dp[i - 1][j - 1];
                if (dp[i][j] < 0) {
                    dp[i][j] += MODULO_CONSTANT; // Ensure non-negative result
                }
            }
        }
    }

    // The result is the number of interleavings of the full sequences
    return (int)(dp[n][m] % MODULO_CONSTANT);
}

int main() {
    // Example usage
    int s[] = {1, 2, 2};
    int t[] = {2, 1};
    int n = sizeof(s) / sizeof(s[0]);
    int m = sizeof(t) / sizeof(t[0]);

    int result = countInterleavings(n, m, s, t);
    printf("Number of distinct interleavings: %d\n", result);
    return 0;
}
