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
    // If the input is already a palindrome, return it
    if (is_palindrome(input)) {
        return input;
    }

    int lower = input - 1;
    int upper = input + 1;

    // Search for the nearest palindrome
    while (true) {
        // Check lower palindrome
        if (lower >= 0 && is_palindrome(lower)) {
            return lower; // Return lower palindrome if found
        }
        
        // Check upper palindrome
        if (is_palindrome(upper)) {
            return upper; // Return upper palindrome if found
        }
        
        // Move to the next integers
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
