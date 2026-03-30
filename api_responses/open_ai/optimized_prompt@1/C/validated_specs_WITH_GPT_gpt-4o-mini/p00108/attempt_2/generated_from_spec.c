#include <stdio.h>

#define MAX_SIZE 101 // Maximum size for the integer values (0 to 100)

// Function to transform the sequence based on the count of occurrences
void transform(int *sequence, int n) {
    int count[MAX_SIZE] = {0}; // Array to count occurrences of each integer

    // Count occurrences of each integer in the sequence
    for (int i = 0; i < n; i++) {
        count[sequence[i]]++;
    }

    // Update the sequence with the counts
    for (int i = 0; i < n; i++) {
        sequence[i] = count[sequence[i]];
    }
}

// Function to check if the sequence is stable (no changes after transformation)
int isStable(int *sequence, int *previous, int n) {
    for (int i = 0; i < n; i++) {
        if (sequence[i] != previous[i]) {
            return 0; // Not stable
        }
    }
    return 1; // Stable
}

int main() {
    int n; // Number of elements in the sequence

    // Read sequences until n is zero
    while (1) {
        scanf("%d", &n);
        if (n == 0) {
            break; // Termination condition
        }

        int sequence[n]; // Array to hold the sequence
        for (int i = 0; i < n; i++) {
            scanf("%d", &sequence[i]);
        }

        int transformations = 0; // Count of transformations
        int previous[n]; // Array to hold the previous state for stability check

        // Repeat transformations until stable
        do {
            // Copy current sequence to previous for stability check
            for (int i = 0; i < n; i++) {
                previous[i] = sequence[i];
            }

            transform(sequence, n); // Transform the sequence
            transformations++; // Increment transformation count

        } while (!isStable(sequence, previous, n)); // Check for stability

        // Output the number of transformations and the final sequence
        printf("%d\n", transformations);
        for (int i = 0; i < n; i++) {
            printf("%d ", sequence[i]);
        }
        printf("\n");
    }

    return 0;
}
