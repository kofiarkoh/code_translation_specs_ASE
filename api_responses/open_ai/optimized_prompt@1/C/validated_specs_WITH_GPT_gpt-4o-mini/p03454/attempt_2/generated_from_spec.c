#include <stdio.h>

#define N 10 // Define the maximum length of integers
#define MOD 1000000007 // Predefined constant for modulo operation

// Function to calculate the number of valid combinations
int count_combinations(int s) {
    // The output must be non-negative and less than MOD
    int count = 0;

    // Iterate through possible lengths of integers from 1 to N
    for (int l1 = 1; l1 <= N; l1++) {
        for (int l2 = 1; l2 <= N; l2++) {
            // Check if the sum of lengths is less than or equal to s
            if (l1 + l2 <= s) {
                // Update count based on the lengths
                count = (count + 1) % MOD; // Increment count and apply modulo
            }
        }
    }

    return count; // Return the count of valid combinations
}

int main() {
    int s;
    // Read input value for s
    scanf("%d", &s);
    
    // Call the function to calculate combinations
    int result = count_combinations(s);
    
    // Output the result
    printf("%d\n", result);
    
    return 0;
}
