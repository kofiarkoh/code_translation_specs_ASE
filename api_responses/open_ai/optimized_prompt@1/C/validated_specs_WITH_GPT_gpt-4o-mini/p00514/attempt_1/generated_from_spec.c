#include <stdio.h>
#include <stdlib.h>

#define MAX_DIGITS 10000 // Maximum digits to store large integers

// Function to multiply a large number represented as an array by an integer
void multiply(int *result, int multiplier, int *size) {
    int carry = 0;
    for (int i = 0; i < *size; i++) {
        long long product = (long long)result[i] * multiplier + carry;
        result[i] = product % 10; // Store last digit
        carry = product / 10; // Carry over
    }
    while (carry) { // Handle remaining carry
        result[*size] = carry % 10;
        carry /= 10;
        (*size)++;
    }
}

// Function to compute the binomial coefficient (n+r-1)C(n-1)
void binomial_coefficient(int n, int r, int *result, int *size) {
    if (n == 0 || r == 0) {
        result[0] = 1; // (n+r-1)C(n-1) = 1 when r = 0 or n = 0
        *size = 1;
        return;
    }

    int N = n + r - 1; // Compute n + r - 1
    int K = n - 1; // Compute n - 1

    // Initialize result array
    result[0] = 1; // Start with 1
    *size = 1; // Size of the result

    // Calculate (N! / (K! * (N-K)!))
    for (int i = 0; i < K; i++) {
        multiply(result, N - i, size); // Multiply by (N - i)
        // Divide by (i + 1)
        for (int j = 0; j < *size; j++) {
            if (result[j] < (i + 1)) {
                // Handle division with carry
                int carry = 0;
                for (int k = *size - 1; k >= 0; k--) {
                    long long current = carry * 10 + result[k];
                    result[k] = current / (i + 1);
                    carry = current % (i + 1);
                }
                // Remove leading zeros
                while (*size > 1 && result[*size - 1] == 0) {
                    (*size)--;
                }
                break;
            }
        }
    }
}

// Main function to read input and compute the binomial coefficient
int main() {
    int n, m, r;
    scanf("%d %d %d", &n, &m, &r); // Read inputs

    // Check preconditions
    if (n < 0 || m < 0 || r < 0 || r < n * m) {
        printf("0\n"); // Output 0 for invalid conditions
        return 0;
    }

    // Handle special cases
    if (r == 0) {
        printf("1\n"); // Output 1 if r is 0
        return 0;
    }

    int result[MAX_DIGITS] = {0}; // Array to hold the result
    int size = 0; // Size of the result

    // Compute the binomial coefficient
    binomial_coefficient(n, r, result, &size);

    // Print the result in standard numerical format
    for (int i = size - 1; i >= 0; i--) {
        printf("%d", result[i]);
    }
    printf("\n");

    return 0;
}
