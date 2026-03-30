#include <stdio.h>

#define MAX_SIZE 101 // Maximum size for the sequence (0 to 100)

void transform_sequence(int *sequence, int n, int *transform_count) {
    int stable = 0; // Flag to check if the sequence is stable
    int temp[MAX_SIZE]; // Temporary array for transformations

    // Continue transforming until the sequence is stable
    while (!stable) {
        stable = 1; // Assume stable until a change is detected

        // Perform transformation: each element becomes the average of its neighbors
        for (int i = 1; i < n - 1; i++) {
            temp[i] = (sequence[i - 1] + sequence[i + 1]) / 2; // Average of neighbors
        }
        temp[0] = sequence[0]; // First element remains the same
        temp[n - 1] = sequence[n - 1]; // Last element remains the same

        // Check if the new sequence is different from the old one
        for (int i = 0; i < n; i++) {
            if (temp[i] != sequence[i]) {
                stable = 0; // Not stable if any element changed
                break;
            }
        }

        // Update the sequence and increment transformation count if changed
        if (!stable) {
            for (int i = 0; i < n; i++) {
                sequence[i] = temp[i];
            }
            (*transform_count)++; // Increment transformation count
        }
    }
}

int main() {
    int n; // Number of elements in the sequence
    int sequence[MAX_SIZE]; // Array to hold the sequence

    // Read sequences until n is zero
    while (1) {
        scanf("%d", &n); // Read the size of the sequence
        if (n == 0) break; // Termination condition

        // Read the sequence elements
        for (int i = 0; i < n; i++) {
            scanf("%d", &sequence[i]);
        }

        int transform_count = 0; // Initialize transformation count
        transform_sequence(sequence, n, &transform_count); // Transform the sequence

        // Output the number of transformations and the final sequence
        printf("%d\n", transform_count);
        for (int i = 0; i < n; i++) {
            printf("%d ", sequence[i]);
        }
        printf("\n");
    }

    return 0;
}
