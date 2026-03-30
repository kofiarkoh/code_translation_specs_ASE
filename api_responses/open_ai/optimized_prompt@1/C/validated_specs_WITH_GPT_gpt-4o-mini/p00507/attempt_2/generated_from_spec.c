#include <stdio.h>
#include <limits.h>
#include <string.h>

int smallest_unique_concatenation(int N, int *list) {
    // Check precondition: N must be at least 2
    if (N < 2) {
        // Undefined behavior as per specification; no handling implemented
        return INT_MAX; // Placeholder for undefined behavior
    }

    long long min_concat = LLONG_MAX; // Use long long to handle large concatenated values
    int found = 0; // Flag to check if a valid concatenation is found

    // Iterate through all pairs of distinct integers
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (i != j) { // Ensure distinct integers
                // Create concatenated string
                char concat1[32], concat2[32];
                snprintf(concat1, sizeof(concat1), "%d%d", list[i], list[j]);
                snprintf(concat2, sizeof(concat2), "%d%d", list[j], list[i]);

                // Convert concatenated strings to long long
                long long value1 = atoll(concat1);
                long long value2 = atoll(concat2);

                // Update min_concat if a smaller value is found
                if (value1 < min_concat) {
                    min_concat = value1;
                    found = 1; // Mark that we found a valid concatenation
                }
                if (value2 < min_concat) {
                    min_concat = value2;
                    found = 1; // Mark that we found a valid concatenation
                }
            }
        }
    }

    // If no valid concatenation was found, return maximum integer
    if (!found) {
        return INT_MAX; // As per specification
    }

    return (int)min_concat; // Return the smallest concatenated value
}
