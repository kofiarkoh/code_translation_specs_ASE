#include <stdio.h>

// Function to compute the digit sum of a positive integer
long digit_sum(long value) {
    long sum = 0;
    while (value > 0) {
        sum += value % 10; // Add the last digit to the sum
        value /= 10;       // Remove the last digit
    }
    return sum;
}

int main() {
    long n; // Number of values to compute
    // Read input
    if (scanf("%ld", &n) != 1 || n < 0) {
        // If input is not a valid integer or is negative, behavior is undefined
        return 1; // Exit with an error code
    }

    long current_value = 1; // Starting value for the sequence
    for (long i = 0; i < n; i++) {
        printf("%ld\n", current_value); // Output the current value
        // Update the current value based on the digit sum
        current_value += digit_sum(current_value); // Ensure the next value is >= current_value
    }

    return 0; // Successful execution
}
