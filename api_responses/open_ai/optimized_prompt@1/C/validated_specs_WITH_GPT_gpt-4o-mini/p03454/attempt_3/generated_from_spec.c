#include <stdio.h>

#define N 10 // Define the maximum length of integers
#define MOD 1000000007 // Predefined constant for modulo operation

// Function to calculate the number of valid combinations that sum to s
int count_combinations(int s) {
    // Check precondition: s must be non-negative
    if (s < 0) {
        // Undefined behavior as per specification; no handling for invalid input
        return -1; // Placeholder for undefined behavior
    }

    // Array to store the count of combinations for each sum
    int dp[s + 1];
    for (int i = 0; i <= s; i++) {
        dp[i] = 0; // Initialize all counts to zero
    }
    
    dp[0] = 1; // Base case: one way to make sum 0 (using no integers)

    // Iterate over lengths from 1 to N
    for (int length = 1; length <= N; length++) {
        // Update the dp array for current length
        for (int j = length; j <= s; j++) {
            dp[j] = (dp[j] + dp[j - length]) % MOD; // Update count with combinations
        }
    }

    // The result is the count of combinations that sum to s
    return dp[s]; // Return the result modulo MOD
}

int main() {
    int s;
    printf("Enter a non-negative integer for the target sum: ");
    scanf("%d", &s); // Read input

    int result = count_combinations(s); // Calculate combinations
    if (result >= 0) {
        printf("Number of valid combinations: %d\n", result); // Output result
    } else {
        printf("Invalid input.\n"); // Placeholder for undefined behavior
    }

    return 0;
}
