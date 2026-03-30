#include <stdio.h>
#include <stdlib.h>

int compute_result(int N, int *sequence) {
    // Initialize the result to zero
    int result = 0;

    // If N is zero, we will return the initialized result (which is zero)
    if (N == 0) {
        return result;
    }

    // Process each element in the sequence
    for (int i = 0; i < N; i++) {
        // Example of a bitwise operation and modular arithmetic
        // Here we assume some operation based on the specification
        // This is a placeholder for the actual computation logic
        result += (sequence[i] & 0xFF) % 10; // Example operation
        result %= 100; // Keep result within bounds (0-99)
    }

    // Ensure the result is non-negative as per postconditions
    return result < 0 ? 0 : result;
}

int main() {
    int N;
    // Read the number of elements
    scanf("%d", &N);

    // Allocate memory for the sequence
    int *sequence = (int *)malloc(N * sizeof(int));
    if (sequence == NULL) {
        // Undefined behavior if memory allocation fails
        return 1; // Exit with error code
    }

    // Read the sequence of integers
    for (int i = 0; i < N; i++) {
        scanf("%d", &sequence[i]);
    }

    // Compute the result based on the sequence
    int result = compute_result(N, sequence);

    // Output the result
    printf("%d\n", result);

    // Free allocated memory
    free(sequence);
    return 0;
}
