#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>

void compute_indices(int n, int a[], int b[], int c[]) {
    // Initialize cost matrix d and predecessor matrix fr
    long long d[1 << 20]; // Assuming n <= 20 for practical reasons
    int fr[1 << 20];
    
    for (int i = 0; i < (1 << n); i++) {
        d[i] = 1e18; // Initialize to a large value
        fr[i] = -1;  // Initialize predecessors to -1
    }
    
    d[0] = 0; // Base case: cost of empty set is 0

    // Iterate over all possible subsets represented by bitmask j
    for (int j = 1; j < (1 << n); j++) {
        for (int i = 0; i < n; i++) {
            if (j & (1 << i)) { // If the i-th index is included in the subset
                int k = __builtin_ctz(j & ~(1 << i)); // Find the first set bit in j excluding i
                long long cost = abs(b[k] - b[i]) + 20 * c[i]; // Compute cost based on the specification
                long long new_cost = d[j ^ (1 << i)] + cost; // Cost of the new subset
                
                if (new_cost < d[j]) { // If the new cost is lower, update
                    d[j] = new_cost;
                    fr[j] = i; // Update predecessor
                }
            }
        }
    }

    // Reconstruct the sequence of indices from the predecessor matrix
    int output[20]; // Assuming n <= 20 for output
    int idx = (1 << n) - 1; // Start from the full set
    int count = 0;

    while (idx > 0) {
        output[count++] = fr[idx]; // Store the index
        idx ^= (1 << fr[idx]); // Remove the index from the set
    }

    // Print the output sequence of indices
    for (int i = count - 1; i >= 0; i--) {
        printf("%d ", output[i]);
    }
    printf("\n");
}

int main() {
    // Example usage
    int n = 3; // Number of elements
    int a[] = {1, 2, 3}; // Example array a
    int b[] = {4, 5, 6}; // Example array b
    int c[] = {7, 8, 9}; // Example array c

    compute_indices(n, a, b, c); // Call the function
    return 0;
}
