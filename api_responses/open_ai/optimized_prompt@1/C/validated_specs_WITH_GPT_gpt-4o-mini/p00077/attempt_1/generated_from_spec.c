#include <stdio.h>
#include <stdlib.h>

#define MAX_LENGTH 1024 // Define a maximum length for processed strings

void process_string(const char *input) {
    char output[MAX_LENGTH];
    int i = 0, j = 0;

    while (input[i] != '\0') {
        if (input[i] == '@') {
            // Move to the next character to get the digit
            i++;
            if (input[i] >= '0' && input[i] <= '9') {
                int count = input[i] - '0'; // Convert char digit to integer
                i++; // Move to the character to replicate
                if (input[i] != '\0') {
                    // Replicate the character 'count' times
                    for (int k = 0; k < count; k++) {
                        if (j < MAX_LENGTH - 1) { // Ensure we don't exceed output buffer
                            output[j++] = input[i];
                        }
                    }
                }
            }
        } else {
            // Copy regular characters to output
            if (j < MAX_LENGTH - 1) { // Ensure we don't exceed output buffer
                output[j++] = input[i];
            }
        }
        i++;
    }
    output[j] = '\0'; // Null-terminate the output string

    // Print the transformed string
    printf("%s\n", output);
}

int main() {
    char input[MAX_LENGTH];

    // Read input strings until EOF
    while (fgets(input, sizeof(input), stdin) != NULL) {
        process_string(input);
    }

    return 0; // Terminate the program
}
