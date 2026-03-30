#include <stdio.h>

long long max_sum_with_distance(int n, long long arr[]) {
    // Preconditions check
    if (n <= 2) {
        // Undefined behavior as per specification
        return 0; // Placeholder for undefined behavior
    }

    long long max_sum = 0;

    // Iterate over possible distances
    for (int d = 1; d * 2 < n - 1; d++) {
        int s = 0; // Start index
        int g = n - 1; // End index

        // Continue selecting elements while indices are valid
        while (s < g) {
            // Check if the next indices would overlap
            if (s + d < g - d) {
                // Calculate the current sum of selected elements
                long long current_sum = arr[s] + arr[g];
                if (current_sum > max_sum) {
                    max_sum = current_sum; // Update max sum if current is greater
                }
            }
            // Update indices based on the distance
            s += d; // Move start index forward
            g -= d; // Move end index backward
        }
    }

    return max_sum; // Return the maximum sum found
}

int main() {
    // Example usage
    long long arr[] = {1, 2, 3, 4, 5};
    int n = 5;
    long long result = max_sum_with_distance(n, arr);
    printf("Maximum sum: %lld\n", result);
    return 0;
}
