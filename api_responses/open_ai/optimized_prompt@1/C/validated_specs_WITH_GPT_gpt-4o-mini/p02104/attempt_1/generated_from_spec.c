#include <stdio.h>
#include <stdlib.h>

int main() {
    int n;
    scanf("%d", &n); // Read the number of chair identifiers

    // Allocate an array to store the last occurrence of each identifier
    int *last_occurrence = (int *)malloc((n + 1) * sizeof(int));
    // Initialize the last occurrence array to zero
    for (int i = 0; i <= n; i++) {
        last_occurrence[i] = 0; // Default to zero for identifiers not seen
    }

    // Read the chair identifiers and record their last occurrences
    for (int i = 0; i < n; i++) {
        int identifier;
        scanf("%d", &identifier); // Read each chair identifier
        if (identifier >= 0 && identifier <= n) {
            last_occurrence[identifier] = i + 1; // Store the 1-based index
        }
    }

    // Output the last occurrence for each identifier in the input sequence
    for (int i = 0; i < n; i++) {
        int identifier;
        scanf("%d", &identifier); // Read again to output the last occurrence
        if (identifier >= 0 && identifier <= n) {
            printf("%d ", last_occurrence[identifier]); // Output the last occurrence
        } else {
            printf("0 "); // Output zero for invalid identifiers
        }
    }

    // Free allocated memory
    free(last_occurrence);
    return 0;
}
