#include <stdio.h>
#include <stdlib.h>

// Function to compute the GCD of two integers
int gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// Function to compute the product of GCDs modulo a prime number
int compute_gcd_product(int N, int *sequence, int prime) {
    long long product = 1; // Use long long to prevent overflow during multiplication

    for (int i = 0; i < N; i++) {
        int index = i; // zero-based index
        int current_gcd = gcd(sequence[i], index);
        product = (product * current_gcd) % prime; // Compute product modulo prime
    }

    return (int)product; // Return the product as an integer
}

int main() {
    int N; // Number of elements in the sequence
    int prime; // Specified prime number

    // Read N and prime from input
    if (scanf("%d %d", &N, &prime) != 2 || N <= 0) {
        // Undefined behavior if N is non-positive
        return 1; // Exit with error code
    }

    // Allocate memory for the sequence
    int *sequence = (int *)malloc(N * sizeof(int));
    if (sequence == NULL) {
        // Memory allocation failed
        return 1; // Exit with error code
    }

    // Read the sequence from input
    for (int i = 0; i < N; i++) {
        if (scanf("%d", &sequence[i]) != 1) {
            free(sequence); // Free allocated memory
            return 1; // Exit with error code
        }
    }

    // Compute the product of GCDs
    int result = compute_gcd_product(N, sequence, prime);

    // Output the result
    printf("%d\n", result);

    // Free allocated memory
    free(sequence);
    return 0;
}
