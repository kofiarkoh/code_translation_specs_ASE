#include <stdio.h>

int largest_prime_product(int n, int p) {
    if (n <= 0) {
        // Undefined behavior if n is zero or negative
        return -1; // Indicating an error
    }
    if (p == 0) {
        // If p is zero, the output will be zero
        return 0;
    }

    int ans = 1; // Initialize the product of prime factors
    int factor_count = 0; // Count of prime factors used

    // Factorization process
    for (int i = 2; i * i <= p; i++) {
        int count = 0; // Count of the current prime factor
        while (p % i == 0) {
            p /= i;
            count++;
        }
        if (count > 0) {
            // If we have found a prime factor
            factor_count += count; // Update total factor count
            // Calculate how many times to include this prime factor
            int use_count = count / n; // Integer division
            for (int j = 0; j < use_count; j++) {
                ans *= i; // Multiply the prime factor
            }
        }
    }

    // If p is still greater than 1, then it is a prime number
    if (p > 1) {
        factor_count++; // Count this prime factor
        ans *= p; // Include this prime factor in the product
    }

    // If the total count of prime factors exceeds n, we need to adjust
    if (factor_count > n) {
        // We can only use up to n factors, so we need to adjust
        // This is handled by the way we calculate use_count above
        // The product is already maximized by the multiplicities considered
    }

    return ans; // Return the final product
}

int main() {
    int n = 3; // Example input for number of factors
    int p = 28; // Example input for the integer to be factorized
    int result = largest_prime_product(n, p);
    printf("Largest product of prime factors: %d\n", result);
    return 0;
}
