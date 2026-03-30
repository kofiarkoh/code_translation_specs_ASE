#include <stdio.h>
#include <stdlib.h>

int is_prime(int n) {
    if (n <= 1) return 0; // 0 and 1 are not prime
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0; // Found a divisor, not prime
    }
    return 1; // n is prime
}

int product_of_prime_factors(int n) {
    int product = 1;
    for (int i = 2; i <= n; i++) {
        if (n % i == 0 && is_prime(i)) {
            product *= i; // Multiply by the prime factor
            while (n % i == 0) {
                n /= i; // Remove all occurrences of the prime factor
            }
        }
    }
    return product;
}

void compute_counts(int Q, int *input, int *output) {
    for (int j = 0; j < Q; j++) {
        int count = 0;
        for (int i = 2; i <= input[j]; i++) {
            int product = product_of_prime_factors(i);
            if (product >= 2 * i) {
                count++; // Increment count if condition is met
            }
        }
        output[j] = count; // Store the result for the current input
    }
}

int main() {
    int Q;
    if (scanf("%d", &Q) != 1 || Q <= 0) {
        return 1; // Error handling for invalid Q
    }

    int *input = malloc(Q * sizeof(int));
    if (input == NULL) {
        return 1; // Error handling for memory allocation failure
    }

    for (int i = 0; i < Q; i++) {
        if (scanf("%d", &input[i]) != 1 || input[i] < 0) {
            free(input);
            return 1; // Error handling for invalid input values
        }
    }

    int *output = malloc(Q * sizeof(int));
    if (output == NULL) {
        free(input);
        return 1; // Error handling for memory allocation failure
    }

    compute_counts(Q, input, output);

    for (int i = 0; i < Q; i++) {
        printf("%d\n", output[i]); // Output the results
    }

    free(input);
    free(output);
    return 0; // Successful termination
}
