#include <stdio.h>

int largest_prime_product(int n, int p) {
    if (n <= 0) {
        // Undefined behavior for non-positive n
        return -1; // Indicate error
    }
    if (p == 0) {
        // If p is zero, return zero as there are no prime factors
        return 0;
    }

    int ans = 1; // Initialize the product of prime factors
    int factor_count = 0; // Count of prime factors used
    int multiplicity; // To store the multiplicity of each prime factor

    // Factorization process
    for (int i = 2; i * i <= p; i++) {
        multiplicity = 0; // Reset multiplicity for each prime factor
        while (p % i == 0) {
            p /= i; // Divide p by the prime factor i
            multiplicity++; // Count the multiplicity
        }
        if (multiplicity > 0) {
            // If we have found a prime factor
            if (factor_count + multiplicity > n) {
                // If adding this factor exceeds n, only take the excess
                multiplicity = n - factor_count; // Limit to n
            }
            for (int j = 0; j < multiplicity; j++) {
                ans *= i; // Multiply the prime factor to the product
            }
            factor_count += multiplicity; // Update the total factor count
        }
    }

    // If p is still greater than 1, then it is a prime number
    if (p > 1) {
        if (factor_count < n) {
            ans *= p; // Include the remaining prime factor if under limit
            factor_count++; // Increment the factor count
        }
    }

    return ans; // Return the computed product
}

int main() {
    int n = 3; // Example input for number of factors
    int p = 28; // Example input for the integer to be factorized
    int result = largest_prime_product(n, p);
    printf("Largest product of prime factors: %d\n", result);
    return 0;
}
