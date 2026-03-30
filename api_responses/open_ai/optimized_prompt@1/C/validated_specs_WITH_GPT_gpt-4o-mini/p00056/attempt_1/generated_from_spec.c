#include <stdio.h>
#include <stdlib.h>

#define MAX_VALUE 100 // Define the maximum value for prime generation

// Function to generate a table of prime numbers up to MAX_VALUE
void generate_primes(int prime_table[]) {
    for (int i = 0; i <= MAX_VALUE; i++) {
        prime_table[i] = 1; // Assume all numbers are prime initially
    }
    prime_table[0] = prime_table[1] = 0; // 0 and 1 are not prime

    for (int i = 2; i * i <= MAX_VALUE; i++) {
        if (prime_table[i]) {
            for (int j = i * i; j <= MAX_VALUE; j += i) {
                prime_table[j] = 0; // Mark multiples of i as non-prime
            }
        }
    }
}

// Function to count distinct pairs of primes that sum to n
int count_prime_pairs(int n, int prime_table[]) {
    int count = 0;
    for (int p1 = 0; p1 <= n / 2; p1++) {
        int p2 = n - p1;
        // Check if both p1 and p2 are prime and p1 <= p2
        if (p2 <= MAX_VALUE && prime_table[p1] && prime_table[p2] && p1 <= p2) {
            count++;
        }
    }
    return count;
}

int main() {
    int n;
    int prime_table[MAX_VALUE + 1];

    // Generate the prime number table
    generate_primes(prime_table);

    // Read input
    if (scanf("%d", &n) != 1 || n < 0 || n > MAX_VALUE) {
        return 0; // Invalid input, terminate without output
    }

    // If n is 0, terminate without output
    if (n == 0) {
        return 0;
    }

    // Count and print the number of distinct prime pairs
    int result = count_prime_pairs(n, prime_table);
    printf("%d\n", result);

    return 0;
}
