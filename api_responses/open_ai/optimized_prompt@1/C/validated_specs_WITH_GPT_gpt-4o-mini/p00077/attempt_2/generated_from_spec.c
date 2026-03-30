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
            // Check if '@' is at the beginning or followed directly by a digit
            if (i == 0 || (i + 1 < length && isdigit(input[i + 1]))) {
                // Skip this '@' as it does not conform to the expected pattern
                continue;
            }
            // Check for valid character after '@'
            if (i + 1 < length && isdigit(input[i + 1]) && (i + 2 < length)) {
                int count = input[i + 1] - '0'; // Convert character to integer
                char char_to_repeat = input[i + 2]; // Character to replicate
                for (int j = 0; j < count; j++) {
                    if (output_index < MAX_LENGTH - 1) { // Ensure we don't overflow
                        output[output_index++] = char_to_repeat;
                    }
                }
                i += 2; // Move past the digit and the character
            }
        } else {
            // Regular character, add to output
            if (output_index < MAX_LENGTH - 1) { // Ensure we don't overflow
                output[output_index++] = input[i];
            }
        }
    }
    output[output_index] = '\0'; // Null-terminate the output string

    // Print the output if it's not empty
    if (output_index > 0) {
        printf("%s\n", output);
    }
}

int main() {
    char input[MAX_LENGTH];

    // Read input strings until EOF
    while (fgets(input, sizeof(input), stdin)) {
        // Remove newline character if present
        input[strcspn(input, "\n")] = 0;
        process_string(input);
    }

    return 0; // Terminate the program
}
