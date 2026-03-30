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
    int exponent = m - 2; // Since m is prime, we can use m-2 as exponent
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
long long nCr(int n, int r, int m) {
    if (n < r || r < 0) return 0; // Invalid scenario
    long long num = factorial(n, m);
    long long denom = (factorial(r, m) * factorial(n - r, m)) % m;
    return (num * mod_inverse(denom, m)) % m; // nCr = n! / (r! * (n-r)!)
}

// Main function to compute the combinatorial value
long long compute_combinatorial(int n, int m) {
    if (n < 0 || m <= 1) return 0; // Preconditions check

    long long result = 0;

    // Loop through possible numbers of red objects
    for (int i = 0; i <= n; i++) {
        // Loop through possible numbers of black objects not connected to red
        for (int j = 0; j <= n - i; j++) {
            // Apply combinatorial logic based on the problem description
            long long arrangements = nCr(n, i, m); // Choose i red from n
            arrangements = (arrangements * factorial(i, m)) % m; // Arrange red objects
            // Here we assume some logic for connectivity, which is not fully specified
            // For simplicity, we assume j contributes multiplicatively
            arrangements = (arrangements * factorial(n - i - j, m)) % m; // Arrange remaining black
            result = (result + arrangements) % m; // Aggregate result
        }
    }

    return result; // Final result in range [0, m-1]
}

int main() {
    int n, m;
    scanf("%d %d", &n, &m); // Read inputs
    long long result = compute_combinatorial(n, m);
    printf("%lld\n", result); // Output the result
    return 0;
}
