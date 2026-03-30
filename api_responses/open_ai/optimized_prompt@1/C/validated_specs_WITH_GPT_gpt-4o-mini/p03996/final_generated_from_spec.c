#include <stdio.h>
#include <stdlib.h>

#define MAX_IDENTIFIERS 1000 // Assuming a reasonable upper limit for m

int main() {
    int n, m, q;
    
    // Read inputs
    if (scanf("%d %d %d", &n, &m, &q) != 3 || n < 0 || m < 0 || q < 0 || m >= MAX_IDENTIFIERS) {
        return 1; // Undefined behavior for invalid input
    }

    int *counts = (int *)calloc(m + 1, sizeof(int)); // Array to hold counts of identifiers
    if (counts == NULL) {
        return 1; // Memory allocation failure
    }

    int *identifiers = (int *)malloc(q * sizeof(int)); // Array to hold the identifiers
    if (identifiers == NULL) {
        free(counts);
        return 1; // Memory allocation failure
    }

    // Read the identifiers
    for (int i = 0; i < q; i++) {
        if (scanf("%d", &identifiers[i]) != 1 || identifiers[i] < 0 || identifiers[i] > m) {
            free(counts);
            free(identifiers);
            return 1; // Undefined behavior for invalid identifier
        }
        counts[identifiers[i]]++; // Increment the count for the identifier
    }

    // Check if any identifier exceeds the threshold n
    for (int i = 0; i <= m; i++) {
        if (counts[i] > n) {
            printf("No\n");
            free(counts);
            free(identifiers);
            return 0; // Output "No" if any identifier exceeds n
        }
    }

    // Output "Yes" since all identifiers are within the threshold
    printf("Yes\n");

    // Clean up
    free(counts);
    free(identifiers);
    return 0;
}
