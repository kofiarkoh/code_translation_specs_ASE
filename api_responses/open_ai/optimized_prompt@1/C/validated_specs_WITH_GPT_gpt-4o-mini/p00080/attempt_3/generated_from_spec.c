#include <stdio.h>
#include <math.h>

#define EPS 1e-7 // Convergence criteria for cube root calculation

double cube_root(int n) {
    if (n == 0) return 0.0; // Handle the case for zero explicitly
    double guess = n / 3.0; // Initial guess
    double next_guess;

    // Iterative method based on Newton's method
    do {
        next_guess = (2.0 * guess + n / (guess * guess)) / 3.0; // Newton's update
        if (fabs(next_guess - guess) < EPS) break; // Check for convergence
        guess = next_guess;
    } while (1);

    return next_guess; // Return the converged result
}

int main() {
    int input;

    // Read integers until -1 is encountered
    while (1) {
        if (scanf("%d", &input) != 1) break; // Undefined behavior for non-integer input
        if (input == -1) break; // Termination condition

        if (input >= 0) { // Ensure non-negative input
            double result = cube_root(input);
            printf("%.6f\n", result); // Output the result formatted to six decimal places
        }
    }

    return 0; // Normal termination
}
