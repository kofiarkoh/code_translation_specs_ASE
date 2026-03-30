#include <stdio.h>
#include <stdlib.h>

#define MAX_LIMIT 1000 // Define a limit for n + m based on the specification

// Function to compute the combinatorial result
int compute_combinatorial(int n, int m) {
    // Check for invalid inputs
    if (n < 0 || m < 0 || n + m > MAX_LIMIT) {
        return -1; // Return -1 for invalid inputs
    }

    // Dynamic programming arrays
    int *dp = (int *)malloc((n + 1) * sizeof(int));
    int *dq = (int *)malloc((m + 1) * sizeof(int));

    // Check for memory allocation failure
    if (dp == NULL || dq == NULL) {
        free(dp);
        free(dq);
        return -1; // Return -1 if memory allocation fails
    }

    // Initialize dp and dq arrays
    for (int i = 0; i <= n; i++) {
        dp[i] = 0; // Initialize dp array
    }
    for (int j = 0; j <= m; j++) {
        dq[j] = 0; // Initialize dq array
    }

    // Base case
    dp[0] = 1; // C(0, 0) = 1

    // Fill the dp array using dynamic programming
    for (int i = 1; i <= n; i++) {
        for (int j = 0; j <= m; j++) {
            // Update dp[i] based on previous states
            dp[i] = (dp[i] + (j > 0 ? dp[i - 1] : 0)) % 1000000007; // Modular arithmetic
        }
    }

    // The result is stored in dp[n]
    int result = dp[n];

    // Clean up memory
    free(dp);
    free(dq);

    return result; // Return the computed result
}

int main() {
    int n, m;
    // Read input values for n and m
    scanf("%d %d", &n, &m);
    
    // Compute the combinatorial result
    int result = compute_combinatorial(n, m);
    
    // Output the result
    printf("%d\n", result);
    
    return 0;
}
