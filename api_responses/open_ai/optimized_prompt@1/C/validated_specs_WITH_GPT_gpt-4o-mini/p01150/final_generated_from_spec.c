#include <stdio.h>

#define MAX_N 16
#define MULTIPLIER 1 // Assuming a constant factor for output as per specification

// Memoization arrays
long long memo_even[MAX_N + 1] = {0};
long long memo_odd[MAX_N + 1] = {0};

// Function to calculate arrangements for even N
long long calc_even(int n) {
    if (n == 0) return 1; // Base case
    if (memo_even[n] != 0) return memo_even[n]; // Return cached result

    long long arrangements = 0;
    for (int i = 0; i < n; i++) {
        arrangements += calc_even(n - 2); // Recursive logic for even N
    }
    memo_even[n] = arrangements; // Cache the result
    return arrangements;
}

// Function to calculate arrangements for odd N
long long calc_odd(int n) {
    return 0; // As per specification, odd N results in zero arrangements
}

int main() {
    int N;
    while (1) {
        scanf("%d", &N); // Read input
        if (N == 0) break; // Termination condition

        long long result;
        if (N <= MAX_N) {
            if (N % 2 == 1) {
                result = calc_odd(N); // Call for odd N
            } else {
                result = calc_even(N) * MULTIPLIER; // Call for even N
            }
        } else {
            // For N > 16, we assume the same logic applies as per the specification
            if (N % 2 == 1) {
                result = calc_odd(N);
            } else {
                result = calc_even(N) * MULTIPLIER;
            }
        }
        printf("%lld\n", result); // Output the result
    }
    return 0;
}
