#include <stdio.h>

int calculate_value(int A, int B, int X) {
    // Preconditions: A, B, and X are non-negative integers
    // At least one of A or B must be greater than zero

    // Check if A is at least twice B
    if (A >= 2 * B) {
        // Calculate output as ((X - 1) / 500 + 1) * B
        return ((X - 1) / 500 + 1) * B;
    } else {
        // Calculate base output as (X / 1000) * A
        int base_output = (X / 1000) * A;
        int c = X % 1000; // Remainder of X when divided by 1000

        // If there is a remainder
        if (c > 0) {
            // If c is less than or equal to 500 and B is less than A
            if (c <= 500 && B < A) {
                return base_output + B; // Add B to base output
            } else {
                return base_output + A; // Add A to base output
            }
        }
        // If no remainder, return base output
        return base_output;
    }
}

int main() {
    int A, B, X;
    // Example input; in a real program, you would get these from user input or another source
    A = 1000; // Example value
    B = 400;  // Example value
    X = 2500; // Example value

    int result = calculate_value(A, B, X);
    printf("Calculated value: %d\n", result);
    return 0;
}
