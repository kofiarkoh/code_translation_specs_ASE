#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

// Function to update the BIT array
void update(int *BIT, int index, int value, int size) {
    while (index <= size) {
        BIT[index] = (BIT[index] + value) % MOD; // Ensure values are within modulus
        index += index & -index; // Move to the next index
    }
}

// Function to get the cumulative sum from the BIT array
int getSum(int *BIT, int index) {
    int sum = 0;
    while (index > 0) {
        sum = (sum + BIT[index]) % MOD; // Ensure values are within modulus
        index -= index & -index; // Move to the parent index
    }
    return sum;
}

// Main function to compute the specific value based on input n
int computeValue(int n) {
    if (n < 1) {
        // Undefined behavior for invalid input, but we assume n is always valid as per the specification
        return -1; // Placeholder for undefined behavior
    }

    int *BIT = (int *)calloc(n + 1, sizeof(int)); // BIT array initialized to 0
    int tmp = 0;

    // Initialize the first two values based on specific calculations
    if (n >= 1) {
        tmp = (tmp + 1) % MOD; // First value
        update(BIT, 1, tmp, n); // Update BIT with the first value
    }
    if (n >= 2) {
        tmp = (tmp + 2) % MOD; // Second value
        update(BIT, 2, tmp, n); // Update BIT with the second value
    }

    // Compute values for indices from 3 to n
    for (int i = 3; i <= n; i++) {
        int cumulativeSum = getSum(BIT, i - 3); // Get cumulative sum from BIT
        tmp = (cumulativeSum + (i - 3) * (n - 1) % MOD + (n - i + 2) * n % MOD) % MOD; // Calculate new tmp
        update(BIT, i, tmp, n); // Update BIT with the new tmp value
    }

    free(BIT); // Free allocated memory for BIT
    return tmp; // Return the final computed value
}

int main() {
    int n;
    scanf("%d", &n); // Read input
    int result = computeValue(n); // Compute the result
    printf("%d\n", result); // Output the result
    return 0;
}
