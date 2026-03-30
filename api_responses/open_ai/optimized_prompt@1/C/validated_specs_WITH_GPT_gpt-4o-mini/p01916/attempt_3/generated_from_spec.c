#include <stdio.h>

int count_odd_characters(const char *str) {
    // Array to track the parity of occurrences for each lowercase alphabetic character
    int char_count[26] = {0}; // Initialize all counts to 0
    int odd_count = 0; // To count distinct characters with odd occurrences

    // Iterate through the input string
    for (int i = 0; str[i] != '\0'; i++) {
        // Calculate the index for the character (0 for 'a', 1 for 'b', ..., 25 for 'z')
        int index = str[i] - 'a';
        
        // Toggle the count for the character
        char_count[index]++;

        // Check if the count is odd or even after incrementing
        if (char_count[index] % 2 == 1) {
            // If it's odd, increment the odd_count
            odd_count++;
        } else {
            // If it's even, decrement the odd_count
            odd_count--;
        }
    }

    // Return the count of distinct characters that appear an odd number of times
    return odd_count;
}

int main() {
    // Example usage
    const char *input = "abacabad";
    int result = count_odd_characters(input);
    printf("Count of distinct characters with odd occurrences: %d\n", result);
    return 0;
}
