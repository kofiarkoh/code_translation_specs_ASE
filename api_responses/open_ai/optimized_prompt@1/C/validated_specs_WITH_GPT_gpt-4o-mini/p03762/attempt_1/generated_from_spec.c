#include <stdio.h>

#define MODULO_CONSTANT 1000000007 // Specified constant for modulo operation

// Function to compute the product of two derived values based on two sequences
int compute_product(int n, int m, int *seq1, int *seq2) {
    // Derived value from the first sequence
    long long derived_value1 = 0; // Use long long to handle large sums
    for (int i = 0; i < n; i++) {
        derived_value1 += (i + 1) * seq1[i]; // Derived value formula
    }

    // Derived value from the second sequence
    long long derived_value2 = 0; // Use long long to handle large sums
    for (int j = 0; j < m; j++) {
        derived_value2 += (j + 1) * seq2[j]; // Derived value formula
    }

    // Compute the product of the two derived values
    long long product = derived_value1 * derived_value2;

    // Return the product modulo the specified constant
    return (int)(product % MODULO_CONSTANT); // Cast to int for final output
}

int main() {
    // Example usage
    int n = 3;
    int m = 2;
    int seq1[] = {1, 2, 3}; // Example sequence 1
    int seq2[] = {4, 5};    // Example sequence 2

    // Compute and print the result
    int result = compute_product(n, m, seq1, seq2);
    printf("%d\n", result); // Output the result

    return 0;
}
