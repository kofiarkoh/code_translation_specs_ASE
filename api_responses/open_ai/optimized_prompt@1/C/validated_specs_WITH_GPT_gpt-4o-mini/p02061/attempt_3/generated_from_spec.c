#include <stdio.h>
#include <stdlib.h>

int is_prime(int n) {
    if (n <= 1) return 0;
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return 0;
    }
    return 1;
}

long long product_of_prime_factors(int n) {
    long long product = 1;
    for (int i = 2; i <= n; i++) {
        if (is_prime(i) && n % i == 0) {
            int count = 0;
            while (n % i == 0) {
                n /= i;
                count++;
            }
            // Multiply the prime factor raised to its power
            for (int j = 0; j < count; j++) {
                product *= i;
            }
        }
    }
    return product;
}

void compute_counts(int Q, int *input, int *output) {
    for (int i = 0; i < Q; i++) {
        int count = 0;
        for (int j = 2; j <= input[i]; j++) {
            long long product = product_of_prime_factors(j);
            // Check the multiplicative condition
            if (product >= 2 * j) {
                count++;
            }
        }
        output[i] = count;
    }
}

int main() {
    int Q;
    if (scanf("%d", &Q) != 1 || Q <= 0) {
        // Error handling for invalid input
        return 1; // Terminate with error code
    }

    int *input = malloc(Q * sizeof(int));
    int *output = malloc(Q * sizeof(int));
    if (input == NULL || output == NULL) {
        // Error handling for memory allocation failure
        return 1; // Terminate with error code
    }

    for (int i = 0; i < Q; i++) {
        if (scanf("%d", &input[i]) != 1 || input[i] < 0) {
            // Error handling for invalid input
            free(input);
            free(output);
            return 1; // Terminate with error code
        }
    }

    compute_counts(Q, input, output);

    for (int i = 0; i < Q; i++) {
        printf("%d\n", output[i]);
    }

    free(input);
    free(output);
    return 0;
}
