#include <stdio.h>

#define MAX_N 16
#define MULTIPLIER 1 // Assuming a constant factor for output as per specification

// Memoization array to store computed results
long long memo[MAX_N + 1];

// Function to calculate valid seating arrangements
long long calculate_arrangements(int n) {
    // Base cases
    if (n <= 0) return 0; // No arrangements for non-positive integers
    if (n == 1) return 0; // Odd case, output is zero
    if (n == 2) return 1; // One valid arrangement for 2 princes
    if (n == 3) return 0; // Odd case, output is zero
    if (n == 4) return 2; // Two valid arrangements for 4 princes

    // Check if already computed
    if (memo[n] != -1) return memo[n];

    // Recursive case for even n
    long long arrangements = 0;
    for (int i = 0; i < n; i += 2) {
        arrangements += calculate_arrangements(n - 2);
    }

    // Store the computed result in memoization array
    memo[n] = arrangements;
    return arrangements;
}

int main() {
    int N;

    // Initialize memoization array with -1 (indicating uncomputed)
    for (int i = 0; i <= MAX_N; i++) {
        memo[i] = -1;
    }

    // Read input until zero is encountered
    while (1) {
        scanf("%d", &N);
        if (N == 0) break; // Termination condition

        // Calculate arrangements based on the parity of N
        long long result = (N <= MAX_N && N % 2 != 0) ? 0 : calculate_arrangements(N);
        printf("%lld\n", result * MULTIPLIER); // Output the result multiplied by the constant factor
    }

    return 0;
}
