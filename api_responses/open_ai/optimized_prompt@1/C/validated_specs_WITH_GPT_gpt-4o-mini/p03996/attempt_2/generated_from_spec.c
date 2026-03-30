#include <stdio.h>
#include <stdlib.h>

int main() {
    int n, m, q;
    
    // Read inputs for threshold count, number of distinct identifiers, and number of input integers
    if (scanf("%d %d %d", &n, &m, &q) != 3 || n < 0 || m < 0 || q < 0) {
        return 1; // Undefined behavior for invalid input
    }

    // Array to count occurrences of each identifier
    int *counts = (int *)calloc(m + 1, sizeof(int)); // Allocate memory for counts
    if (counts == NULL) {
        return 1; // Terminate if memory allocation fails
    }

    // Array to store the sequence of identifiers
    int *identifiers = (int *)malloc(q * sizeof(int)); // Allocate memory for identifiers
    if (identifiers == NULL) {
        free(counts); // Free previously allocated memory
        return 1; // Terminate if memory allocation fails
    }

    // Read the identifiers and count their occurrences
    for (int i = 0; i < q; i++) {
        if (scanf("%d", &identifiers[i]) != 1 || identifiers[i] < 0 || identifiers[i] > m) {
            free(counts);
            free(identifiers);
            return 1; // Undefined behavior for invalid identifier
        }
        counts[identifiers[i]]++; // Increment the count for the identifier
    }

    // Check if any identifier exceeds the threshold count
    for (int i = 0; i <= m; i++) {
        if (counts[i] > n) {
            printf("No\n");
            free(counts);
            free(identifiers);
            return 0; // Output "No" if any identifier exceeds the threshold
        }
    }

    // If all counts are within the threshold, output "Yes"
    printf("Yes\n");

    // Free allocated memory
    free(counts);
    free(identifiers);
    return 0;
}
