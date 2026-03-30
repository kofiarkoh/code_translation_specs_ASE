#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define THRESHOLD 10000000000L
#define SENTINEL_VALUE -1

// Function to check if a number is a perfect square
int is_perfect_square(long x) {
    long s = (long)sqrt(x);
    return s * s == x;
}

// Function to check if a number is a perfect cube
int is_perfect_cube(long x) {
    long c = (long)round(pow(x, 1.0 / 3.0));
    return c * c * c == x;
}

// Function to perform prime factorization and count based on the specification
long process_integers(int n, int *arr) {
    if (n <= 0) {
        return 0; // If the input array is empty, return zero
    }

    long count = 0;
    long *occurrences = (long *)calloc(n, sizeof(long)); // To track occurrences of each integer
    if (!occurrences) {
        return 0; // Handle memory allocation failure
    }

    for (int i = 0; i < n; i++) {
        occurrences[arr[i]]++; // Count occurrences of each integer
    }

    for (int i = 0; i < n; i++) {
        long value = arr[i];
        if (value == 1) {
            count++; // Count 1
            continue;
        }

        long product = 1;
        int is_valid = 1;

        // Prime factorization logic
        for (int j = 2; j <= sqrt(value); j++) {
            while (value % j == 0) {
                product *= j;
                value /= j;

                // Check for overflow
                if (product > THRESHOLD) {
                    product = SENTINEL_VALUE; // Set to sentinel value if exceeds threshold
                    is_valid = 0;
                    break;
                }
            }
            if (!is_valid) break;
        }
        if (value > 1) { // If there's a prime factor greater than sqrt(value)
            product *= value;
            if (product > THRESHOLD) {
                product = SENTINEL_VALUE; // Set to sentinel value if exceeds threshold
            }
        }

        // Count based on product and perfect square/cube conditions
        if (is_valid && product <= THRESHOLD) {
            count += occurrences[arr[i]]; // Increment based on occurrences
        } else if (product == SENTINEL_VALUE) {
            count += occurrences[arr[i]]; // Increment based on occurrences for sentinel
        }

        // Check for perfect square or cube
        if (is_perfect_square(arr[i]) || is_perfect_cube(arr[i])) {
            count += occurrences[arr[i]]; // Adjust count based on occurrences
        }
    }

    free(occurrences); // Free allocated memory
    return count; // Return the final count
}

int main() {
    int n;
    printf("Enter the number of elements: ");
    scanf("%d", &n);

    if (n <= 0) {
        printf("Count: 0\n");
        return 0; // Handle empty input case
    }

    int *arr = (int *)malloc(n * sizeof(int));
    if (!arr) {
        printf("Memory allocation failed\n");
        return 1; // Handle memory allocation failure
    }

    printf("Enter the elements:\n");
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    long result = process_integers(n, arr);
    printf("Count: %ld\n", result);

    free(arr); // Free allocated memory for input array
    return 0;
}
