#include <stdio.h>

#define MAX_N 2010

// Function to compute factorial modulo m
long long factorial(int n, int m) {
    long long result = 1;
    for (int i = 2; i <= n; i++) {
        result = (result * i) % m; // Maintain result within modulus
    }
    return result;
}

// Function to compute modular inverse using Fermat's Little Theorem
long long mod_inverse(long long a, int m) {
    long long result = 1;
    int exponent = m - 2; // Since m is prime, we can use Fermat's theorem
    while (exponent > 0) {
        if (exponent % 2 == 1) {
            result = (result * a) % m;
        }
        a = (a * a) % m;
        exponent /= 2;
    }
    return result;
}

// Function to compute n choose r modulo m
long long choose(int n, int r, int m) {
    if (n < 0 || r < 0 || n < r) {
        return 0; // Invalid scenario
    }
    long long num = factorial(n, m);
    long long denom = (factorial(r, m) * factorial(n - r, m)) % m;
    return (num * mod_inverse(denom, m)) % m; // Return result modulo m
}

// Main function to compute the combinatorial value
long long compute_combinatorial(int n, int m) {
    if (n < 0 || m <= 1) {
        return 0; // Invalid input based on preconditions
    }
    return choose(n, n / 2, m); // Example: choosing n/2 from n
}

int main() {
    int n, m;
    // Example input
    n = 10; // Total number of objects
    m = 1009; // Modulus

    long long result = compute_combinatorial(n, m);
    printf("%lld\n", result); // Output the result
    return 0;
}
