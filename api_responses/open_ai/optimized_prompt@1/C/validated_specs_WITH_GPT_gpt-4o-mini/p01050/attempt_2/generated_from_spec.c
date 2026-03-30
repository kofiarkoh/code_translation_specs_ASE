#include <stdio.h>
#include <string.h>

int count_unique_characters(const char *input) {
    // Array to track the frequency of each character
    int frequency[256] = {0}; // Assuming extended ASCII
    int unique_count = 0; // Count of unique characters
    int length = strlen(input);
    
    // Process the input string
    for (int i = 0; i < length; i++) {
        char current_char = input[i];
        
        // Count the frequency of the current character
        frequency[(unsigned char)current_char]++;
        
        // If this is the first occurrence of the character
        if (frequency[(unsigned char)current_char] == 1) {
            unique_count++;
        }
        
        // If the character appears consecutively more than 3 times
        if (i > 0 && input[i] == input[i - 1]) {
            // If frequency exceeds 3, decrement unique count
            if (frequency[(unsigned char)current_char] == 4) {
                unique_count--;
            }
        }
    }
    
    // Ensure the unique count is non-negative
    if (unique_count < 0) {
        unique_count = 0;
    }
    
    return unique_count;
}

int main() {
    char input[100]; // Buffer for input string (99 chars + null terminator)
    
    // Read input string
    printf("Enter a string (max 99 characters): ");
    fgets(input, sizeof(input), stdin);
    
    // Remove newline character if present
    input[strcspn(input, "\n")] = 0;
    
    // Get the adjusted count of unique characters
    int result = count_unique_characters(input);
    
    // Output the result
    printf("Adjusted count of unique characters: %d\n", result);
    
    return 0;
}
