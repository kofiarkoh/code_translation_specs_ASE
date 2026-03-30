#include <stdio.h>

int count_odd_characters(const char *str) {
    // Array to count occurrences of each lowercase alphabetic character
    int count[26] = {0};
    int odd_count = 0;

    // Iterate through the string and count occurrences of each character
    for (int i = 0; str[i] != '\0'; i++) {
        // Increment the count for the character
        count[str[i] - 'a']++;
    }

    // Check which characters have odd occurrences
    for (int i = 0; i < 26; i++) {
        if (count[i] % 2 == 1) {
            odd_count++; // Increment odd_count for each character with odd occurrences
        }
    }

    return odd_count; // Return the count of distinct characters with odd occurrences
}

int main() {
    char input[1010]; // Buffer for input string (1009 + 1 for null terminator)
    
    // Read input string
    fgets(input, sizeof(input), stdin);
    
    // Call the function and print the result
    int result = count_odd_characters(input);
    printf("%d\n", result);
    
    return 0;
}
