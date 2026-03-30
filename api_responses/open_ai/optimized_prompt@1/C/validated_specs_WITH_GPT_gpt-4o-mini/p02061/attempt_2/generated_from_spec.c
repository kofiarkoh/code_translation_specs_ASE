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

void compute_counts(int Q, int* inputs, int* outputs) {
    for (int i = 0; i < Q; i++) {
        int count = 0;
        for (int j = 2; j <= inputs[i]; j++) {
            long long product = product_of_prime_factors(j);
            // Check if the product of prime factors is >= 2 * j
            if (product >= 2 * j) {
                count++;
            }
        }
        outputs[i] = count; // Store the count for the current input
    }
}

int main() {
    int Q;
    if (scanf("%d", &Q) != 1 || Q <= 0) {
        return 1; // Error handling for invalid Q
    }

    int* inputs = malloc(Q * sizeof(int));
    int* outputs = malloc(Q * sizeof(int));
    if (inputs == NULL || outputs == NULL) {
        return 1; // Error handling for memory allocation failure
    }

    for (int i = 0; i < Q; i++) {
        if (scanf("%d", &inputs[i]) != 1 || inputs[i] < 0) {
            free(inputs);
            free(outputs);
            return 1; // Error handling for invalid input values
        }
    }

    compute_counts(Q, inputs, outputs);

    for (int i = 0; i < Q; i++) {
        printf("%d\n", outputs[i]); // Output the results
    }

    free(inputs);
    free(outputs);
    return 0;
}
