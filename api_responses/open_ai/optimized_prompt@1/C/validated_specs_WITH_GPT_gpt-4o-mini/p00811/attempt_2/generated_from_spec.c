#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_PRIME 100000

// Function to check if a number is prime
bool is_prime(int num) {
    if (num < 2) return false;
    for (int i = 2; i * i <= num; i++) {
        if (num % i == 0) return false;
    }
    return true;
}

// Function to generate a list of prime numbers up to MAX_PRIME
void generate_primes(int *primes, int *count) {
    *count = 0;
    for (int i = 2; i <= MAX_PRIME; i++) {
        if (is_prime(i)) {
            primes[(*count)++] = i;
        }
    }
}

int main() {
    int primes[10000]; // Array to hold prime numbers
    int prime_count;   // Number of primes generated
    generate_primes(primes, &prime_count); // Generate primes

    int m, a, b;
    while (true) {
        // Read input until termination condition (0, 0, 0)
        if (scanf("%d %d %d", &m, &a, &b) != 3 || (m == 0 && a == 0 && b == 0)) {
            break; // Exit on termination condition or invalid input
        }

        int largest_product = -1; // To track the largest valid product
        int prime1 = -1, prime2 = -1; // To store the best pair of primes

        // Check pairs of primes for valid products
        for (int i = 0; i < prime_count; i++) {
            for (int j = i; j < prime_count; j++) {
                int product = primes[i] * primes[j];
                // Check if product is within the specified bounds
                if (product <= m && product >= a && product <= b) {
                    if (product > largest_product) {
                        largest_product = product; // Update largest product
                        prime1 = primes[i]; // Update the best pair
                        prime2 = primes[j];
                    }
                }
            }
        }

        // Output the largest pair of primes if found
        if (largest_product != -1) {
            printf("%d %d\n", prime1, prime2);
        }
    }

    return 0;
}
