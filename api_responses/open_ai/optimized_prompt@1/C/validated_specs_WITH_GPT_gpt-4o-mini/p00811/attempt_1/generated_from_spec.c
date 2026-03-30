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
    int primes[MAX_PRIME];
    int prime_count;
    generate_primes(primes, &prime_count); // Generate primes once

    int m, a, b;
    while (true) {
        // Read three integers
        if (scanf("%d %d %d", &m, &a, &b) != 3) {
            // If not exactly three integers, skip to next input
            while (getchar() != '\n'); // Clear the input buffer
            continue;
        }
        
        // Termination condition
        if (m == 0 && a == 0 && b == 0) {
            break;
        }

        int largest_product = 0;
        int prime1 = 0, prime2 = 0;

        // Find the largest pair of primes whose product is within bounds
        for (int i = 0; i < prime_count; i++) {
            for (int j = i; j < prime_count; j++) {
                int product = primes[i] * primes[j];
                if (product <= m && product >= a && product <= b) {
                    if (product > largest_product) {
                        largest_product = product;
                        prime1 = primes[i];
                        prime2 = primes[j];
                    }
                }
            }
        }

        // Output the result if a valid pair is found
        if (largest_product > 0) {
            printf("%d %d\n", prime1, prime2);
        }
    }

    return 0;
}
