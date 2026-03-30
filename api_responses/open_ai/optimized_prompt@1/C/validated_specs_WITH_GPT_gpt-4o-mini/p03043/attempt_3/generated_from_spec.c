#include <stdio.h>
#include <stdlib.h>

int main() {
    int N, K;

    // Read input values for N and K
    scanf("%d %d", &N, &K);

    // Preconditions: N and K must be positive
    if (N <= 0 || K <= 0) {
        return 1; // Exit if preconditions are not met
    }

    // Array to hold denominators
    double *denominators = (double *)malloc(N * sizeof(double));
    if (denominators == NULL) {
        return 1; // Exit if memory allocation fails
    }

    // Initialize denominators to zero
    for (int i = 0; i < N; i++) {
        denominators[i] = 0.0;
    }

    // Populate the denominators array
    for (int i = 0; i < N; i++) {
        double multiplier = 1.0;
        // Double the multiplier until it exceeds K * i
        while (multiplier <= K * i) {
            multiplier *= 2.0;
        }
        // Set the denominator based on the computed multiplier
        denominators[i] = multiplier;
    }

    // Calculate the cumulative sum of ratios
    double first_denominator = denominators[0];
    double cumulative_sum = 0.0;

    // If the first denominator is zero, the output will be undefined
    if (first_denominator != 0.0) {
        for (int i = 1; i < N; i++) {
            cumulative_sum += denominators[i] / first_denominator;
        }
    }

    // Output the result formatted to twelve decimal places
    printf("%.12f\n", cumulative_sum);

    // Free allocated memory
    free(denominators);

    return 0;
}
