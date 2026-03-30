#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_LENGTH 79
#define MAX_UNIQUE_VARIATIONS 1000 // Arbitrary limit for unique variations

// Function to generate a unique variation of a string by reversing it
void reverse_string(const char *input, char *output) {
    size_t len = strlen(input);
    for (size_t i = 0; i < len; i++) {
        output[i] = input[len - 1 - i];
    }
    output[len] = '\0'; // Null-terminate the output string
}

// Function to check if a string is unique in the array of unique variations
int is_unique(const char *variation, char unique_variations[][MAX_STRING_LENGTH], int count) {
    for (int i = 0; i < count; i++) {
        if (strcmp(variation, unique_variations[i]) == 0) {
            return 0; // Not unique
        }
    }
    return 1; // Unique
}

// Function to process the strings and generate unique variations
int process_strings(int num_strings, char strings[][MAX_STRING_LENGTH]) {
    char unique_variations[MAX_UNIQUE_VARIATIONS][MAX_STRING_LENGTH];
    int unique_count = 0;

    for (int i = 0; i < num_strings; i++) {
        char variation[MAX_STRING_LENGTH];

        // Generate a reversed variation
        reverse_string(strings[i], variation);

        // Check if the variation is unique and store it
        if (is_unique(variation, unique_variations, unique_count)) {
            if (unique_count < MAX_UNIQUE_VARIATIONS) {
                strcpy(unique_variations[unique_count], variation);
                unique_count++;
            }
            // If we exceed the limit, behavior is undefined as per specification
        }
    }

    return unique_count; // Return the count of unique variations
}

int main() {
    int num_strings;
    printf("Enter the number of strings: ");
    scanf("%d", &num_strings);

    // Check for non-negative input
    if (num_strings < 0) {
        fprintf(stderr, "Error: Number of strings must be non-negative.\n");
        return 1;
    }

    char strings[num_strings][MAX_STRING_LENGTH];

    // Read the strings
    for (int i = 0; i < num_strings; i++) {
        printf("Enter string %d: ", i + 1);
        scanf("%s", strings[i]); // Assumes input is valid and null-terminated

        // Check for maximum length
        if (strlen(strings[i]) > MAX_STRING_LENGTH - 1) {
            fprintf(stderr, "Error: String exceeds maximum length.\n");
            return 1; // Undefined behavior as per specification
        }
    }

    // Process the strings and get the count of unique variations
    int unique_variation_count = process_strings(num_strings, strings);
    printf("Total unique variations generated: %d\n", unique_variation_count);

    return 0;
}
