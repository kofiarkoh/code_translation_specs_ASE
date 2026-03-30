#include <stdio.h>

double expected_value(int position, double probability_rate);

int main() {
    int position;
    double probability_rate;

    // Read input values
    if (scanf("%d %lf", &position, &probability_rate) != 2) {
        // Undefined behavior if input cannot be read or parsed correctly
        return 1; // Exit with error code
    }

    // Check that the probability rate is within the valid range [0, 100]
    if (probability_rate < 0 || probability_rate > 100) {
        return 1; // Exit with error code
    }

    // Convert probability rate from percentage to decimal
    probability_rate /= 100.0;

    // Calculate the expected value
    double result = expected_value(position, probability_rate);

    // Output the result formatted to seven decimal places
    printf("%.7f\n", result);

    return 0;
}

// Recursive function to calculate expected value based on position and probability rate
double expected_value(int position, double probability_rate) {
    if (position % 2 == 0) {
        // For even positions, return position divided by probability rate
        return position / probability_rate;
    } else {
        // For odd positions, calculate using adjacent even positions
        double even_before = expected_value(position - 1, probability_rate);
        double even_after = expected_value(position + 1, probability_rate);
        // Calculate expected value for odd position
        return (even_before * probability_rate) + (even_after * (1 - probability_rate));
    }
}
