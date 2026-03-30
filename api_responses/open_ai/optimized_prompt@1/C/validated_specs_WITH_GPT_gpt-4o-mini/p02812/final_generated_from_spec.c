#include <stdio.h>

int count_ABC_occurrences(int n, const char *str) {
    // Preconditions check
    if (n < 3) {
        // Undefined behavior as per specification
        return -1; // Indicating an error condition
    }
    
    // Initialize count of occurrences
    int count = 0;

    // Iterate through the string while ensuring we do not exceed bounds
    for (int i = 0; i <= n - 3; i++) {
        // Check for the substring "ABC"
        if (str[i] == 'A' && str[i + 1] == 'B' && str[i + 2] == 'C') {
            count++; // Increment count for each occurrence
        }
    }

    return count; // Return the total count of occurrences
}

int main() {
    // Example usage
    const char *test_str = "ABCABCABC";
    int length = 9; // Length of the test string

    int result = count_ABC_occurrences(length, test_str);
    printf("Count of 'ABC': %d\n", result); // Expected output: 3

    return 0;
}
