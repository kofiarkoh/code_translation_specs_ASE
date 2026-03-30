#include <stdio.h>

int compute_count(int N, int X, int D) {
    // Handle special cases based on the specification
    if (X == 0 && D == 0) {
        return 1; // Both X and D are zero
    }
    if (D == 0) {
        return N + 1; // D is zero
    }
    if (X == 0) {
        // X is zero and D is non-zero
        return (N * (N - 1)) / 2 + 1; // Calculate based on the formula
    }

    // For other values of X and D, we need to compute the aggregated count
    int count = 0; // Initialize the count
    for (int i = 0; i < N; i++) {
        // Calculate L, R, and V based on the current iteration index
        int L = X + i * D; // Example calculation for L
        int R = L + D;     // Example calculation for R
        int V = L % D;     // Example calculation for V (modulus to ensure V is within range)

        // Update the count based on some condition involving L, R, and V
        // Here we assume a simple aggregation for demonstration
        if (V >= 0) { // Example condition
            count++; // Increment count for valid V
        }
    }

    return count; // Return the final computed count
}

int main() {
    // Example usage
    int N = 5; // Number of iterations
    int X = 2; // Initial value
    int D = 1; // Divisor

    int result = compute_count(N, X, D);
    printf("Computed count: %d\n", result); // Output the result
    return 0;
}
