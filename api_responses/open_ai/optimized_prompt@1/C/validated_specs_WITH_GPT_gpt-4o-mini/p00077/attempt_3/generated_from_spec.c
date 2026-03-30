#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define MAX_LENGTH 1024 // Define a maximum length for processed strings

void process_string(const char *input) {
    char output[MAX_LENGTH];
    int output_index = 0;
    int length = strlen(input);

    for (int i = 0; i < length; i++) {
        if (input[i] == '@') {
            // Check if the next character is a digit
            if (i + 1 < length && isdigit(input[i + 1])) {
                // Include the digit and any subsequent characters as-is
                output[output_index++] = input[i + 1]; // Add the digit
                i += 1; // Move past the digit
                // Add all subsequent characters until the end of the string
                while (i < length) {
                    output[output_index++] = input[i++];
                }
                break; // Exit the loop as we've processed the rest of the string
            } else if (i + 1 < length) {
                // If '@' is not followed by a digit, replicate the next character
                output[output_index++] = input[i + 1]; // Add the next character
                i += 1; // Move past the character
            }
        } else {
            // Regular character, just add to output
            output[output_index++] = input[i];
        }
    }

    // Null-terminate the output string
    output[output_index] = '\0';
    // Print the processed output string
    printf("%s\n", output);
}

int main() {
    char input[MAX_LENGTH];

    // Read input strings until EOF
    while (fgets(input, sizeof(input), stdin)) {
        // Remove newline character if present
        input[strcspn(input, "\n")] = 0;
        process_string(input);
    }

    return 0;
}
