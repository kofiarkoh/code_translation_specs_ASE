#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>

void compute_indices(int n, int a[], int b[], int c[]) {
    // Initialize cost matrix d and predecessor matrix fr
    long long d[1 << 20][20]; // Assuming n <= 20 for practical reasons
    int fr[1 << 20][20];
    
    // Set initial values for d and fr
    for (int j = 0; j < (1 << n); j++) {
        for (int k = 0; k < n; k++) {
            d[j][k] = 1e18; // Initialize to a large value
            fr[j][k] = -1;  // Initialize predecessors to -1
        }
    }

    // Iterate over all possible subsets represented by bitmask j
    for (int j = 0; j < (1 << n); j++) {
        for (int k = 0; k < n; k++) {
            // Only consider valid subsets
            if (j & (1 << k)) {
                // Calculate cost for the current subset and index k
                long long cost = 0;
                for (int i = 0; i < n; i++) {
                    if (j & (1 << i)) {
                        cost += abs(b[k] - b[i]) * (c[i] * 20);
                    }
                }
                
                // Update the cost matrix if a lower cost is found
                if (d[j][k] > cost) {
                    d[j][k] = cost;
                    fr[j][k] = -1; // Reset predecessor for this state
                }
            }
        }
    }

    // Find the minimum cost and corresponding indices
    long long min_cost = 1e18;
    int best_mask = 0;
    for (int j = 0; j < (1 << n); j++) {
        for (int k = 0; k < n; k++) {
            if (d[j][k] < min_cost) {
                min_cost = d[j][k];
                best_mask = j;
            }
        }
    }

    // Output the indices corresponding to the best mask
    for (int i = 0; i < n; i++) {
        if (best_mask & (1 << i)) {
            printf("%d ", a[i]); // Print the index from array a
        }
    }
    printf("\n");
}

int main() {
    // Example usage
    int n = 3;
    int a[] = {0, 1, 2};
    int b[] = {10, 20, 30};
    int c[] = {1, 2, 3};

    compute_indices(n, a, b, c);
    return 0;
}
