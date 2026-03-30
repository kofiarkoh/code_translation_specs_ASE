#include <stdio.h>

double calculate_expected_value(int position, double probability_rate);

int main() {
    int position;
    double probability_rate;

    // Read input values
    if (scanf("%d %lf", &position, &probability_rate) != 2) {
        // Undefined behavior if input cannot be read correctly
        return 1; // Exit with error code
    }

    // Check that probability_rate is in the range [0, 100]
    if (probability_rate < 0 || probability_rate > 100) {
        // Undefined behavior if probability_rate is out of bounds
        return 1; // Exit with error code
    }

    // Convert probability_rate from percentage to decimal
    probability_rate /= 100.0;

    // Calculate expected value
    double expected_value = calculate_expected_value(position, probability_rate);

    // Output the result formatted to seven decimal places
    printf("%.7f\n", expected_value);

    return 0;
}

double calculate_expected_value(int position, double probability_rate) {
    // Base case for recursion
    if (position == 0) {
        return 0.0; // Expected value for position 0
    }

    if (position % 2 == 0) {
        // Even position: expected value is position divided by probability rate
        return position / probability_rate;
    } else {
        // Odd position: expected value based on adjacent positions
        double expected_value_prev = calculate_expected_value(position - 1, probability_rate);
        double expected_value_next = calculate_expected_value(position + 1, probability_rate);
        return (expected_value_prev * (1 - probability_rate)) + (expected_value_next * probability_rate);
    }
}
