#include <stdio.h>

#define MAX_LIMIT 2000
#define MODULO 1000000007 // Predefined constant for modulo operation

// Function to compute factorial modulo MODULO
long long factorial(int x) {
    long long result = 1;
    for (int i = 2; i <= x; i++) {
        result = (result * i) % MODULO;
    }
    return result;
}

// Function to compute modular exponentiation
long long modular_exponentiation(long long base, int exp, long long mod) {
    long long result = 1;
    base = base % mod; // Ensure base is within mod
    while (exp > 0) {
        if (exp % 2 == 1) { // If exp is odd
            result = (result * base) % mod;
        }
        exp = exp >> 1; // Divide exp by 2
        base = (base * base) % mod; // Square the base
    }
    return result;
}

// Function to compute combinations C(n, r) modulo MODULO
long long combination(int n, int r) {
    if (r < 0 || n < r) {
        return 0; // Return 0 for invalid combinations
    }
    long long num = factorial(n);
    long long denom = (factorial(r) * factorial(n - r)) % MODULO;
    // Using Fermat's Little Theorem for modular inverse
    long long denom_inv = modular_exponentiation(denom, MODULO - 2, MODULO);
    return (num * denom_inv) % MODULO;
}

// Main function to compute the result based on the specification
long long compute_result(int n, int m) {
    if (n < 0 || m < 0 || m > MAX_LIMIT) {
        return 0; // Return 0 for invalid inputs
    }
    
    long long result = 0;
    for (int i = 0; i <= m; i++) {
        long long comb = combination(m, i);
        long long term = modular_exponentiation(comb, n, MODULO);
        result = (result + term) % MODULO; // Accumulate result with modulo
    }
    
    return result; // Return the final computed result
}

int main() {
    int n, m;
    // Example input; in practice, you would get these from user input or other sources
    n = 5; // Example exponent
    m = 10; // Example upper limit for combinations
    long long result = compute_result(n, m);
    printf("%lld\n", result); // Output the result
    return 0;
}
