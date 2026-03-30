#include <stdio.h>
#include <string.h>

int count_unique_characters(const char *input) {
    // Array to track the frequency of each character (assuming ASCII)
    int frequency[256] = {0};
    int unique_count = 0; // Count of unique characters
    int length = strlen(input);
    
    // Process the input string
    for (int i = 0; i < length; i++) {
        char current_char = input[i];
        frequency[(unsigned char)current_char]++; // Increment frequency of current character
        
        // If this is the first occurrence of the character
        if (frequency[(unsigned char)current_char] == 1) {
            unique_count++;
        }
        
        // Check for consecutive occurrences
        if (i > 0 && input[i] == input[i - 1]) {
            // If the character has appeared more than 3 times consecutively
            if (frequency[(unsigned char)current_char] > 3) {
                unique_count--; // Decrement unique count for excess occurrences
            }
        }
    }
    
    // Ensure the unique count is non-negative
    if (unique_count < 0) {
        unique_count = 0;
    }
    
    return unique_count; // Return the adjusted count of unique characters
}

int main() {
    char input[100]; // Buffer for input string (99 characters + null terminator)
    printf("Enter a string (max 99 characters): ");
    fgets(input, sizeof(input), stdin);
    
    // Remove newline character if present
    input[strcspn(input, "\n")] = 0;
    
    int result = count_unique_characters(input);
    printf("Adjusted count of unique characters: %d\n", result);
    
    return 0;
}
