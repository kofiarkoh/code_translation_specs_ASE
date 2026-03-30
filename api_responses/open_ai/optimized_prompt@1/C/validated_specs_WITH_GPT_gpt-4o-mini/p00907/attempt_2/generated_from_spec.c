#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAX_VALUES 10
#define OUTLIER_THRESHOLD 0.1

// Function to calculate the expected value using Lagrange interpolation
double lagrange_interpolation(double values[], int n, int exclude_index) {
    double expected_value = 0.0;
    for (int i = 0; i < n; i++) {
        if (i != exclude_index) {
            double term = values[i];
            for (int j = 0; j < n; j++) {
                if (j != i && j != exclude_index) {
                    term *= (0.0 - j) / (i - j); // Using j as the index for interpolation
                }
            }
            expected_value += term;
        }
    }
    return expected_value;
}

int main() {
    int n;
    while (1) {
        // Read the number of values in the dataset
        if (scanf("%d", &n) != 1 || n <= 0) {
            break; // Terminate if input is invalid or zero
        }

        // Ensure we do not exceed the maximum allowed values
        if (n > MAX_VALUES) {
            n = MAX_VALUES;
        }

        double values[MAX_VALUES];
        for (int i = 0; i < n; i++) {
            scanf("%lf", &values[i]); // Read floating-point numbers
        }

        int outlier_index = n - 1; // Default to the last index
        for (int i = 0; i < n; i++) {
            double expected_value = lagrange_interpolation(values, n, i);
            // Check if the current value is an outlier
            if (fabs(expected_value - values[i]) > OUTLIER_THRESHOLD) {
                outlier_index = i; // Update outlier index if found
                break; // Exit loop on first outlier found
            }
        }

        // Output the index of the outlier or the last value
        printf("%d\n", outlier_index);
    }
    return 0;
}
