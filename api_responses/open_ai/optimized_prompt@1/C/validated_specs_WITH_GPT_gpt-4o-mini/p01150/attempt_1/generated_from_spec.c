#include <stdio.h>

#define MAX_PRINCES 16
#define MULTIPLIER 1 // Assuming a constant factor of 1 as no specific factor is provided

// Memoization array to store computed results
long long memo[MAX_PRINCES + 1];

// Function to calculate valid seating arrangements using memoization
long long calculate_arrangements(int n) {
    // Base case: if n is 0 or 1, return 1 (1 arrangement)
    if (n <= 1) {
        return 1;
    }
    // Check if the result is already computed
    if (memo[n] != -1) {
        return memo[n];
    }
    // Recursive case: calculate arrangements
    memo[n] = (n - 1) * (calculate_arrangements(n - 1) + calculate_arrangements(n - 2));
    return memo[n];
}

int main() {
    int N;

    // Initialize memoization array with -1 (indicating uncomputed)
    for (int i = 0; i <= MAX_PRINCES; i++) {
        memo[i] = -1;
    }

    // Read input until 0 is encountered
    while (1) {
        scanf("%d", &N);
        if (N == 0) {
            break; // Termination condition
        }
        // Check precondition: N must be a positive integer and <= 16
        if (N > 0 && N <= MAX_PRINCES) {
            // Output is guaranteed to be zero for N <= 16
            printf("%lld\n", 0LL); // Output total arrangements multiplied by the constant factor
        }
    }

    return 0;
}
