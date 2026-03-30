#include <stdio.h>

int count_distinct_characters(const char *input) {
    // Array to track the presence of each lowercase alphabetic character
    int char_present[26] = {0};
    int distinct_count = 0;

    // Iterate through each character in the input string
    for (int i = 0; input[i] != '\0'; i++) {
        // Calculate the index for the character
        int index = input[i] - 'a';
        // Check if the character is already counted
        if (char_present[index] == 0) {
            char_present[index] = 1; // Mark character as present
            distinct_count++; // Increment distinct character count
        }
    }

    // Return half the number of distinct characters, rounded down
    return distinct_count / 2; // Integer division automatically rounds down
}

int main() {
    // Example usage
    const char *test_string = "abcdeabc"; // Example input
    int result = count_distinct_characters(test_string);
    printf("Distinct character count (halved): %d\n", result);
    return 0;
}
