#include <stdio.h>

int count_divisible(int lower_bound, int upper_bound, int divisor) {
    // Preconditions: lower_bound <= upper_bound and divisor > 0
    int count = 0;

    // Iterate through the range from lower_bound to upper_bound (inclusive)
    for (int i = lower_bound; i <= upper_bound; i++) {
        // Check if the current number is divisible by the divisor
        if (i % divisor == 0) {
            count++; // Increment the count if divisible
        }
    }

    // Postcondition: count is non-negative and represents the total count
    return count; // Return the final count of divisible integers
}

int main() {
    // Example usage of the count_divisible function
    int lower = 1; // Example lower bound
    int upper = 10; // Example upper bound
    int divisor = 2; // Example divisor

    int result = count_divisible(lower, upper, divisor);
    printf("Count of integers divisible by %d in range [%d, %d]: %d\n", divisor, lower, upper, result);

    return 0;
}
