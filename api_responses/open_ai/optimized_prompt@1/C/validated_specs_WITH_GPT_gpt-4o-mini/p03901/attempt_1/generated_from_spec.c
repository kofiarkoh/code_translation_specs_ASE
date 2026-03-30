#include <stdio.h>

double calculate_expected_value(int position, double probability_rate) {
    // Base case for recursion: if position is 0, return 0.0
    if (position == 0) {
        return 0.0;
    }
    // Recursive case: expected value is position multiplied by probability rate
    return position * (probability_rate / 100.0) + calculate_expected_value(position - 1, probability_rate);
}

int main() {
    int position;
    double probability_rate;

    // Read input values: an integer followed by a double
    if (scanf("%d %lf", &position, &probability_rate) != 2) {
        // Undefined behavior if input cannot be read or parsed correctly
        return 1; // Exit with error code
    }

    // Check if the probability rate is within the valid range [0, 100]
    if (probability_rate < 0.0 || probability_rate > 100.0) {
        // Undefined behavior if probability rate is out of bounds
        return 1; // Exit with error code
    }

    // Calculate the expected value
    double expected_value = calculate_expected_value(position, probability_rate);

    // Output the result formatted to seven decimal places
    printf("%.7f\n", expected_value);

    return 0; // Successful execution
}
