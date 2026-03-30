#include <stdio.h>
#include <stdlib.h>

char* generate_commands(int n, long long i, long long j) {
    // Check for precondition: n must be greater than 0
    if (n <= 0) {
        return NULL; // Graceful termination for non-positive n
    }

    // Allocate memory for the output string (+1 for null terminator)
    char* commands = (char*)malloc(n + 1);
    if (commands == NULL) {
        return NULL; // Handle memory allocation failure
    }

    // Process each bit from 0 to n-1
    for (int bit = 0; bit < n; bit++) {
        // Extract the current bit from i and j
        long long bit_i = (i >> bit) & 1; // Get the bit from i
        long long bit_j = (j >> bit) & 1; // Get the bit from j

        // If the bit in i is 0, negate the bit in j
        if (bit_i == 0) {
            bit_j = !bit_j; // Negate the bit
        }

        // Determine the command based on the comparison
        if (bit_i < bit_j) {
            commands[bit] = 'L'; // i is less than j
        } else {
            commands[bit] = 'R'; // i is greater than or equal to j
        }
    }

    // Null-terminate the string
    commands[n] = '\0';
    return commands; // Return the generated command string
}

int main() {
    int n = 5; // Example input for number of bits
    long long i = 3; // Example input for i
    long long j = 5; // Example input for j

    char* result = generate_commands(n, i, j);
    if (result != NULL) {
        printf("Commands: %s\n", result);
        free(result); // Free allocated memory
    } else {
        printf("Invalid input or memory allocation failed.\n");
    }

    return 0;
}
