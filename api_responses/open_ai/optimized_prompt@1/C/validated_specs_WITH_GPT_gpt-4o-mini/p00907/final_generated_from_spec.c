#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAX_VALUES 10
#define OUTLIER_THRESHOLD 0.1

// Function to calculate the expected value using Lagrange interpolation
double lagrange_interpolation(double values[], int n, int skip_index) {
    double expected_value = 0.0;
    for (int i = 0; i < n; i++) {
        if (i == skip_index) continue; // Skip the outlier candidate
        double term = values[i];
        for (int j = 0; j < n; j++) {
            if (j == i || j == skip_index) continue; // Skip the current and the skipped index
            term *= (0.0 - j) / (i - j); // Lagrange basis polynomial
        }
        expected_value += term;
    }
    return expected_value;
}

int main() {
    int n;
    while (1) {
        // Read the number of values
        if (scanf("%d", &n) != 1 || n <= 0) break; // Terminate on zero or invalid input
        if (n < 4 || n > MAX_VALUES) continue; // Ignore datasets that do not meet the size requirement

        double values[MAX_VALUES];
        for (int i = 0; i < n; i++) {
            scanf("%lf", &values[i]); // Read the floating-point numbers
        }

        int outlier_index = n - 1; // Default to the last index
        for (int i = 0; i < n; i++) {
            double expected_value = lagrange_interpolation(values, n, i);
            if (fabs(expected_value - values[i]) > OUTLIER_THRESHOLD) {
                outlier_index = i; // Found an outlier
                break; // Report the first outlier found
            }
        }

        printf("%d\n", outlier_index); // Output the index of the outlier or last value
    }
    return 0; // Graceful termination
}
