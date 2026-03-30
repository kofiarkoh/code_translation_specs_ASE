#include <stdio.h>

#define MAX_LIMIT 2000
#define MODULO 1000000007 // Predefined constant for modulo operation

// Function to compute factorials and inverse factorials
void compute_factorials(long long factorial[], long long inv_factorial[]) {
    factorial[0] = 1;
    for (int i = 1; i <= MAX_LIMIT; i++) {
        factorial[i] = (factorial[i - 1] * i) % MODULO;
    }
    
    // Compute inverse factorial using Fermat's Little Theorem
    inv_factorial[MAX_LIMIT] = 1; // inv_factorial[MAX_LIMIT] = (factorial[MAX_LIMIT]^(MODULO-2)) % MODULO
    for (int i = MAX_LIMIT - 1; i >= 0; i--) {
        inv_factorial[i] = (inv_factorial[i + 1] * (i + 1)) % MODULO;
    }
}

// Function to compute combinations C(n, r) % MODULO
long long combination(int n, int r, long long factorial[], long long inv_factorial[]) {
    if (r < 0 || n < r) {
        return 0; // Return 0 for invalid combinations
    }
    return (factorial[n] * inv_factorial[r] % MODULO * inv_factorial[n - r] % MODULO) % MODULO;
}

// Custom modular exponentiation function
long long modular_exponentiation(long long base, long long exp, long long mod) {
    long long result = 1;
    base = base % mod;
    while (exp > 0) {
        if (exp % 2 == 1) { // If exp is odd
            result = (result * base) % mod;
        }
        exp = exp >> 1; // Divide exp by 2
        base = (base * base) % mod; // Square the base
    }
    return result;
}

// Main function to compute the result based on the specification
long long compute_result(int n, int m) {
    if (m > MAX_LIMIT) {
        return 0; // Return 0 if m exceeds the maximum limit
    }

    long long factorial[MAX_LIMIT + 1];
    long long inv_factorial[MAX_LIMIT + 1];
    compute_factorials(factorial, inv_factorial);

    long long result = 0;
    for (int i = 0; i <= m; i++) {
        long long term = combination(m, i, factorial, inv_factorial);
        long long exponentiation = modular_exponentiation(term, n, MODULO);
        
        // Adjust result based on the parity of (m - i)
        if ((m - i) % 2 == 0) {
            result = (result + exponentiation) % MODULO; // Add for even
        } else {
            result = (result - exponentiation + MODULO) % MODULO; // Subtract for odd, ensure non-negative
        }
    }
    return result; // Return the final computed result
}

// Example usage
int main() {
    int n = 5; // Example exponent
    int m = 10; // Example upper limit for combinations
    long long result = compute_result(n, m);
    printf("%lld\n", result); // Output the result
    return 0;
}
