#include <stdio.h>
#include <stdbool.h>

// Function to check if a number is a palindrome
bool is_palindrome(int num) {
    int original = num, reversed = 0;
    
    // Reverse the number
    while (num > 0) {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }
    
    // Check if the original number is equal to its reverse
    return original == reversed;
}

// Function to find the nearest palindrome
int nearest_palindrome(int input) {
    // Start searching from the input
    int lower = input, upper = input;

    // Check if the input itself is a palindrome
    if (is_palindrome(input)) {
        return input; // Return the input if it's already a palindrome
    }

    // Search for the nearest palindrome
    while (true) {
        // Check lower number
        if (lower >= 0 && is_palindrome(lower)) {
            return lower; // Return the lower palindrome if found
        }
        // Check upper number
        if (is_palindrome(upper)) {
            return upper; // Return the upper palindrome if found
        }
        // Increment/decrement to check the next numbers
        lower--;
        upper++;
    }
}

int main() {
    int input;

    // Read input from the user
    printf("Enter a non-negative integer: ");
    scanf("%d", &input);

    // Find and print the nearest palindrome
    int result = nearest_palindrome(input);
    printf("Nearest palindrome: %d\n", result);

    return 0;
}
