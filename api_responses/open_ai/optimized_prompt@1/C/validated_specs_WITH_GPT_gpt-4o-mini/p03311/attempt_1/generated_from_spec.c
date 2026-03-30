#include <stdio.h>
#include <stdlib.h>

int main() {
    int n;
    // Read the number of elements
    scanf("%d", &n);
    
    // The behavior is undefined if n is non-positive, as per the specification
    int *sequence = malloc(n * sizeof(int));
    
    // Read the sequence of integers
    for (int i = 0; i < n; i++) {
        scanf("%d", &sequence[i]);
    }

    // Modify the sequence: each element is decreased by its index plus one
    for (int i = 0; i < n; i++) {
        sequence[i] -= (i + 1);
    }

    // Sort the sequence in non-decreasing order
    // Simple bubble sort for clarity, though not optimal
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (sequence[j] > sequence[j + 1]) {
                // Swap sequence[j] and sequence[j + 1]
                int temp = sequence[j];
                sequence[j] = sequence[j + 1];
                sequence[j + 1] = temp;
            }
        }
    }

    // The minimum value after processing is the first element of the sorted array
    printf("%d\n", sequence[0]);

    // Free allocated memory
    free(sequence);
    return 0;
}
