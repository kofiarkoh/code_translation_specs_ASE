#include <stdio.h>
#include <stdlib.h>

#define MD 1000000007 // Example modulus, can be adjusted as needed

// Function to compute factorials and inverse factorials
void precompute_factorials(int n, long long *fact, long long *inv_fact) {
    fact[0] = 1;
    for (int i = 1; i <= n; i++) {
        fact[i] = (fact[i - 1] * i) % MD;
    }
    // Compute inverse factorial using Fermat's Little Theorem
    inv_fact[n] = 1; // inv_fact[n] = fact[n]^(MD-2) % MD
    for (int i = n - 1; i >= 0; i--) {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1)) % MD;
    }
}

// Function to compute nCk % MD
long long comb(int n, int k, long long *fact, long long *inv_fact) {
    if (k > n) return 0; // Return 0 if k > n
    return (fact[n] * inv_fact[k] % MD) * inv_fact[n - k] % MD;
}

int main() {
    int n, m;
    scanf("%d %d", &n, &m); // Read n and m

    // Preconditions check
    if (n < 0 || n > 16 || m < 0) {
        return 1; // Exit on invalid input
    }

    int size = 1 << n; // 2^n
    int *input = (int *)malloc(m * sizeof(int));
    for (int i = 0; i < m; i++) {
        scanf("%d", &input[i]); // Read m integers
        // Check each integer against the precondition
        if (input[i] < 0 || input[i] > size) {
            free(input);
            return 1; // Exit on invalid input
        }
    }

    long long *fact = (long long *)malloc((n + 1) * sizeof(long long));
    long long *inv_fact = (long long *)malloc((n + 1) * sizeof(long long));
    precompute_factorials(n, fact, inv_fact); // Precompute factorials

    long long result = 0; // Initialize result

    // Dynamic programming array to maintain states
    long long *dp = (long long *)calloc(size, sizeof(long long));
    dp[0] = 1; // Base case

    // Update dp array based on input values
    for (int i = 0; i < m; i++) {
        for (int j = size - 1; j >= input[i]; j--) {
            dp[j] = (dp[j] + dp[j - input[i]]) % MD; // Update dp
        }
    }

    // Compute final result using combinatorial logic
    for (int k = 0; k <= n; k++) {
        result = (result + comb(n, k, fact, inv_fact) * dp[k]) % MD; // Combine results
    }

    // Ensure result is non-negative
    if (result < 0) {
        result += MD;
    }

    printf("%lld\n", result); // Output the result

    // Free allocated memory
    free(input);
    free(fact);
    free(inv_fact);
    free(dp);

    return 0;
}
