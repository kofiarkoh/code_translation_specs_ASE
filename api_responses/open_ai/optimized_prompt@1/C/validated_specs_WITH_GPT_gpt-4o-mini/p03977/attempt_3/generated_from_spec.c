#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_INPUT_SIZE 10000
#define MAX_OUTPUT_SIZE 20000

int main() {
    char input[MAX_INPUT_SIZE];
    char output[MAX_OUTPUT_SIZE];
    char *ptr = output; // Pointer to track the output position
    int N, D;
    int count = 0; // To count the number of valid integers found

    // Read input from standard input
    if (fgets(input, sizeof(input), stdin) == NULL) {
        return 1; // Handle empty input
    }

    // Process the input to extract non-negative integers
    char *current = input;
    while (*current) {
        // Skip non-digit characters
        while (*current && !isdigit(*current)) {
            current++;
        }

        // If we find a digit, parse the integer
        if (isdigit(*current)) {
            N = strtol(current, &current, 10); // Read the integer N
            count++; // Increment valid integer count

            // Check for the next integer
            while (*current && !isdigit(*current)) {
                current++;
            }

            // If we found another digit, parse it as D
            if (isdigit(*current)) {
                D = strtol(current, &current, 10); // Read the integer D

                // Compute the result based on the parity of N
                int result;
                if (N % 2 == 1) { // N is odd
                    result = 127 * (N - 1) + D;
                } else { // N is even
                    result = 127 * N - D;
                }

                // Format the result as a string and append to output
                ptr += sprintf(ptr, "%d\n", result);
            }
        }
    }

    // Ensure output does not exceed the maximum size
    if (ptr - output < MAX_OUTPUT_SIZE) {
        // Print the output
        printf("%s", output);
    }

    return 0;
}
