#include <stdio.h>

#define MAX_N 2010

// Function to compute factorial modulo m
long long factorial(int n, int m) {
    long long result = 1;
    for (int i = 2; i <= n; i++) {
        result = (result * i) % m; // Ensure result stays within modulus
    }
    return result;
}

// Function to compute modular inverse using Fermat's Little Theorem
long long mod_inverse(long long a, int m) {
    long long result = 1;
    int exponent = m - 2; // m is prime, so we can use this theorem
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
    if (n < 0 || r < 0 || n < r) {
        return 0; // Invalid scenario
    }
    long long num = factorial(n, m);
    long long denom = (factorial(r, m) * factorial(n - r, m)) % m;
    return (num * mod_inverse(denom, m)) % m; // nCr = n! / (r! * (n-r)!)
}

// Main function to compute the combinatorial value
long long compute_combinatorial(int n, int m) {
    long long result = 0;

    // Loop through possible numbers of red objects
    for (int i = 0; i <= n; i++) {
        // Loop through possible numbers of disconnected black objects
        for (int j = 0; j <= n - i; j++) {
            // Calculate the contribution of this configuration
            long long arrangements = nCr(n, i, m); // Choose i red from n
            arrangements = (arrangements * factorial(i, m)) % m; // Arrange red objects
            arrangements = (arrangements * factorial(n - i, m)) % m; // Arrange black objects
            result = (result + arrangements) % m; // Accumulate result
        }
    }

    return result; // Final result in range [0, m-1]
}

int main() {
    int n, m;
    // Input reading
    scanf("%d %d", &n, &m);
    
    // Preconditions check
    if (n < 0 || m <= 1) {
        return 1; // Invalid input
    }

    // Compute and print the combinatorial value
    long long result = compute_combinatorial(n, m);
    printf("%lld\n", result);
    
    return 0;
}
