#include <stdio.h>
#include <stdlib.h>

// Function to compute the greatest common divisor (GCD) of two integers
int gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// Function to compute the product of GCDs of each element and its index, modulo a prime
int compute_gcd_product(int N, int *sequence, int prime) {
    long long product = 1; // Use long long to prevent overflow during multiplication

    for (int i = 0; i < N; i++) {
        int index = i + 1; // Indexing starts from 1
        int current_gcd = gcd(sequence[i], index);
        product = (product * current_gcd) % prime; // Compute product modulo prime
    }

    return (int)product; // Return the product as an integer
}

int main() {
    int N; // Number of elements in the sequence
    int prime; // Specified prime number

    // Read N and prime from input
    scanf("%d %d", &N, &prime);

    // Undefined behavior if N is non-positive
    if (N <= 0) {
        return 1; // Exit with error code
    }

    // Allocate memory for the sequence
    int *sequence = (int *)malloc(N * sizeof(int));
    if (sequence == NULL) {
        // Memory allocation failed, terminate without output
        return 1; // Exit with error code
    }

    // Read the sequence from input
    for (int i = 0; i < N; i++) {
        scanf("%d", &sequence[i]);
    }

    // Compute the product of GCDs
    int result = compute_gcd_product(N, sequence, prime);

    // Output the result
    printf("%d\n", result);

    // Free allocated memory
    free(sequence);

    return 0;
}
