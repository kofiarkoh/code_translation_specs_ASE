#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
        // Extract the current bit from both integers
        int bit_i = (i >> bit) & 1; // Get the bit from i
        int bit_j = (j >> bit) & 1; // Get the bit from j

        // Determine the command based on the comparison of bits
        if (bit_i < bit_j) {
            commands[bit] = 'L'; // 'L' if bit_i is less than bit_j
        } else {
            commands[bit] = 'R'; // 'R' otherwise (including equal case)
        }
    }

    // Null-terminate the string
    commands[n] = '\0';
    return commands; // Return the generated command string
}

int main() {
    int n = 5; // Example number of bits
    long long i = 10; // Example integer i
    long long j = 12; // Example integer j

    char* result = generate_commands(n, i, j);
    if (result != NULL) {
        printf("Commands: %s\n", result);
        free(result); // Free allocated memory
    } else {
        printf("Invalid input or memory allocation failed.\n");
    }

    return 0;
}
