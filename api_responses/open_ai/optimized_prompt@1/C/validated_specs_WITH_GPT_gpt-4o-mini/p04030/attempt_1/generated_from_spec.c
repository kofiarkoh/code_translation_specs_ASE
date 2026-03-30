#include <stdio.h>

void process_string(const char *input, char *output) {
    int b_count = 0; // Counter for 'B' characters
    int output_index = 0; // Current index for the output string

    // Process each character in the input string
    for (int i = 0; input[i] != '\0'; i++) {
        if (input[i] == 'B') {
            b_count++; // Increment the counter for each 'B'
        } else {
            // If there are 'B's counted, we remove the last added character
            if (b_count > 0) {
                b_count--; // Decrement the counter, effectively removing a character
                if (output_index > 0) {
                    output_index--; // Move back in the output string
                }
            } else {
                // Add the current character to the output string
                output[output_index++] = input[i];
            }
        }
    }
    
    // Null-terminate the output string
    output[output_index] = '\0';
}

int main() {
    char input[11]; // Input string with max length of 10 + null terminator
    char output[11]; // Output string with max length of 10 + null terminator

    // Example input
    snprintf(input, sizeof(input), "ABCDAB"); // Example input string

    process_string(input, output);
    
    // Print the output string
    printf("Output: %s\n", output);
    
    return 0;
}
