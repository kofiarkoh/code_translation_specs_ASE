#include <stdio.h>
#include <stdlib.h>

int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b; // Comparison function for qsort
}

int main() {
    int n;
    scanf("%d", &n); // Read the number of elements

    // Preconditions: n must be positive
    if (n <= 0) {
        return 1; // Undefined behavior for non-positive n
    }

    int *sequence = malloc(n * sizeof(int)); // Allocate memory for the sequence
    for (int i = 0; i < n; i++) {
        scanf("%d", &sequence[i]); // Read each integer into the sequence
    }

    // Modify each element by subtracting its index + 1
    for (int i = 0; i < n; i++) {
        sequence[i] -= (i + 1);
    }

    // Sort the modified sequence
    qsort(sequence, n, sizeof(int), compare);

    // Initialize variables for minimum value calculation
    int sadness = 0;
    for (int i = 1; i < n; i++) {
        sadness += (sequence[i] - sequence[0]); // Total difference from the first element
    }

    int level = sequence[0]; // Start with the first element as the initial level
    int min = sadness; // Initialize min with the initial sadness

    for (int i = 1; i < n; i++) {
        if (sequence[i] == level) {
            continue; // Skip if the current level is equal to the next element
        }

        // Calculate the height difference
        int height_diff = sequence[i] - level;
        int remaining_elements = n - i; // Remaining elements in the sorted sequence

        // Check if the height difference can be accommodated by sadness
        if (height_diff * remaining_elements <= sadness) {
            level = sequence[i]; // Update level
            sadness -= height_diff * remaining_elements; // Adjust sadness
        } else {
            // Calculate new level based on available sadness
            level += sadness / remaining_elements; // New level based on remaining sadness
            sadness = 0; // All sadness is used up
            if (level < min) {
                min = level; // Update min if new level is lower
            }
            break; // Exit the loop as we cannot proceed further
        }
    }

    // Print the final computed minimum value
    printf("%d\n", min);

    free(sequence); // Free allocated memory
    return 0;
}
