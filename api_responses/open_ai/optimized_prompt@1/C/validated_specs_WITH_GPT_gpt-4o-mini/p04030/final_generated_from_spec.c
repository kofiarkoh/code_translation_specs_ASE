#include <stdio.h>

void process_string(const char *input, char *output) {
    int output_index = 0; // Index for the output string

    // Process each character in the input string
    for (int i = 0; input[i] != '\0'; i++) {
        if (input[i] == 'B') {
            // If 'B' is encountered, remove the last character from output if possible
            if (output_index > 0) {
                output_index--; // Decrement the output index to "remove" the last character
            }
        } else {
            // Add the current character to the output string
            if (output_index < 10) { // Ensure we do not exceed the output length
                output[output_index] = input[i];
                output_index++;
            }
        }
    }
    
    // Null-terminate the output string
    output[output_index] = '\0';
}

int main() {
    char input[11]; // Input string with max length 10 + null terminator
    char output[11]; // Output string with max length 10 + null terminator

    // Example input
    snprintf(input, sizeof(input), "abcBdeBf"); // Example input string

    process_string(input, output);
    
    // Print the output string
    printf("Output: %s\n", output);
    return 0;
}
