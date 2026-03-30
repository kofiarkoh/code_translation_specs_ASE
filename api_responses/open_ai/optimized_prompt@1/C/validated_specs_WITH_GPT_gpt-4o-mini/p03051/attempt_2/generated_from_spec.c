#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

// Function to compute power of a number under modulo
long long power(long long base, long long exp, long long mod) {
    long long result = 1;
    while (exp > 0) {
        if (exp % 2 == 1) {
            result = (result * base) % mod;
        }
        base = (base * base) % mod;
        exp /= 2;
    }
    return result;
}

int compute_result(int N, int* sequence) {
    // Handle the case where N is zero
    if (N == 0) {
        return 0; // Default result for empty sequence
    }

    int cumulative_xor = 0;
    int zero_count = 0;

    // Array to track cumulative XOR results
    int* xor_results = (int*)malloc(N * sizeof(int));
    if (xor_results == NULL) {
        // Undefined behavior if memory allocation fails
        return -1; // Indicate failure (not specified in the task)
    }

    // Compute cumulative XOR and count occurrences of zero
    for (int i = 0; i < N; i++) {
        cumulative_xor ^= sequence[i];
        xor_results[i] = cumulative_xor;

        if (cumulative_xor == 0) {
            zero_count++;
        }
    }

    // Final result computation based on cumulative XOR
    int result;
    if (cumulative_xor == 0) {
        // If cumulative XOR is zero, adjust result using zero_count
        result = power(2, zero_count, MOD);
    } else {
        // If cumulative XOR is non-zero, derive result from last XOR value
        result = xor_results[N - 1] % MOD;
    }

    free(xor_results); // Free allocated memory
    return result;
}

int main() {
    int N;
    scanf("%d", &N); // Read number of elements

    // Allocate memory for the sequence
    int* sequence = (int*)malloc(N * sizeof(int));
    if (sequence == NULL) {
        // Undefined behavior if memory allocation fails
        return -1; // Indicate failure (not specified in the task)
    }

    // Read the sequence of integers
    for (int i = 0; i < N; i++) {
        scanf("%d", &sequence[i]);
    }

    // Compute the result based on the sequence
    int result = compute_result(N, sequence);
    printf("%d\n", result); // Output the result

    free(sequence); // Free allocated memory for the sequence
    return 0;
}
