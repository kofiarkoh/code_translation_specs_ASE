#include <stdio.h>
#include <stdlib.h>

long long modified_absolute_value(long long initial_value, long long max_decrements, long long decrement_value) {
    // Preconditions check: decrement_value must be greater than zero
    if (decrement_value <= 0) {
        // Undefined behavior as per specification
        return -1; // Indicate error (not specified in the spec, but necessary for safety)
    }

    // Take the absolute value of the initial value
    long long abs_value = (initial_value < 0) ? -initial_value : initial_value;

    // Calculate the maximum possible decrement
    long long total_decrement = decrement_value * max_decrements;

    // Apply the decrements
    if (abs_value > total_decrement) {
        abs_value -= total_decrement;
    } else {
        abs_value = 0; // If the decrements exceed the absolute value, it becomes zero
    }

    // Calculate remaining decrements
    long long remaining_decrements = max_decrements - (abs_value / decrement_value);

    // If there are remaining decrements, adjust based on parity
    if (remaining_decrements > 0) {
        if (remaining_decrements % 2 != 0) {
            abs_value -= decrement_value; // Decrement by the decrement value if odd
            if (abs_value < 0) {
                abs_value = 0; // Ensure non-negative output
            }
        }
    }

    return abs_value; // Return the modified absolute value
}

int main() {
    // Example usage
    long long initial_value = -10;
    long long max_decrements = 3;
    long long decrement_value = 4;

    long long result = modified_absolute_value(initial_value, max_decrements, decrement_value);
    printf("Modified Absolute Value: %lld\n", result);

    return 0;
}
