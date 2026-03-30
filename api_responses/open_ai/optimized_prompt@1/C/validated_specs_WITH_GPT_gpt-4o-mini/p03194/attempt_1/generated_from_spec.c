#include <stdio.h>

int largest_prime_product(int n, int p) {
    // Preconditions check
    if (n <= 0) {
        // Undefined behavior if n is zero or negative
        return -1; // Indicate error
    }
    if (p == 0) {
        // If p is zero, return zero as there are no prime factors
        return 0;
    }

    int ans = 1; // Initialize the product of prime factors
    int factor_count = 0; // To count the number of prime factors

    // Factorization process
    for (int i = 2; i * i <= p; i++) {
        while (p % i == 0) {
            p /= i; // Reduce p by the prime factor i
            factor_count++; // Count the multiplicity of the prime factor
            ans *= i; // Multiply the product by the prime factor
        }
    }
    // If p is still greater than 1, then it is a prime number
    if (p > 1) {
        ans *= p; // Include the last prime factor
        factor_count++; // Count this prime factor
    }

    // Calculate the final product based on the multiplicity divided by n
    // Using integer division to round down
    int final_product = 1;
    for (int i = 0; i < factor_count / n; i++) {
        final_product *= ans; // Raise to the power of multiplicity/n
    }

    return final_product; // Return the computed product
}

int main() {
    int n = 2; // Example input for number of factors
    int p = 30; // Example input for the integer to be factorized
    int result = largest_prime_product(n, p);
    printf("Result: %d\n", result); // Output the result
    return 0;
}
