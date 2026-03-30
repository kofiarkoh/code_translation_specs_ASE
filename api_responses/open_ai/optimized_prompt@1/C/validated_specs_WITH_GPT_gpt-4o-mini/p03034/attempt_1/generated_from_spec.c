#include <stdio.h>

long long max_sum_with_distance(int n, long long arr[]) {
    // Preconditions check
    if (n <= 2) {
        // Undefined behavior as per specification
        return 0; // Placeholder for undefined behavior
    }

    long long max_sum = 0;

    // Iterate over possible distances
    for (int distance = 1; distance < n - 1; distance++) {
        // Evaluate pairs of elements with the given distance
        for (int i = 0; i < n - distance; i++) {
            int j = i + distance; // Ensure separation
            if (j < n) {
                long long current_sum = arr[i] + arr[j];
                if (current_sum > max_sum) {
                    max_sum = current_sum; // Update max sum if current is greater
                }
            }
        }
    }

    return max_sum; // Return the maximum sum found
}

int main() {
    // Example usage
    int n = 5;
    long long arr[] = {1, 2, 3, 4, 5};
    long long result = max_sum_with_distance(n, arr);
    printf("Maximum sum: %lld\n", result);
    return 0;
}
