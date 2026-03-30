#include <stdio.h>

#define MAX_VALUES 10

int main() {
    int n; // Number of values in the dataset
    float values[MAX_VALUES]; // Array to hold the dataset values

    while (1) {
        // Read the number of values for the dataset
        if (scanf("%d", &n) != 1 || n == 0) {
            // If input is not an integer or is zero, terminate gracefully
            break;
        }

        // Ensure we only process datasets with at least 4 values
        if (n < 4 || n > MAX_VALUES) {
            // Undefined behavior as per specification, but we assume valid input
            continue;
        }

        // Read the floating-point values into the array
        for (int i = 0; i < n; i++) {
            scanf("%f", &values[i]);
        }

        // Initialize variables to find the outlier
        float sum = 0.0f;
        float mean;
        int outlier_index = n - 1; // Default to the last index if no outlier is found

        // Calculate the sum of the values
        for (int i = 0; i < n; i++) {
            sum += values[i];
        }

        // Calculate the mean
        mean = sum / n;

        // Determine the outlier based on deviation from the mean
        for (int i = 0; i < n; i++) {
            if (values[i] > mean + 1.5f || values[i] < mean - 1.5f) {
                outlier_index = i; // Found an outlier
                break; // Exit loop after finding the first outlier
            }
        }

        // Output the index of the outlier
        printf("%d\n", outlier_index);
    }

    return 0;
}
