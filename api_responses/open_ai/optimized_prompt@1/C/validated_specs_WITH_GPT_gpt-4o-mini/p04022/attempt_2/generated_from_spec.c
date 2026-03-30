#include <stdio.h>
#include <stdlib.h>

long count_integers_with_prime_factors(int n, int *arr) {
    if (n <= 0) {
        return 0; // Return 0 if the input array is empty
    }

    long count = 0; // Initialize count of integers meeting criteria
    const long THRESHOLD = 10000000000; // Define the threshold for sentinel value

    // Function to perform prime factorization and check conditions
    for (int i = 0; i < n; i++) {
        int num = arr[i];
        if (num < 0) {
            num = -num; // Consider absolute value for factorization
        }

        int original_count = 0; // Count occurrences of the original integer
        // Count occurrences of the integer in the array
        for (int j = 0; j < n; j++) {
            if (arr[j] == arr[i]) {
                original_count++;
            }
        }

        // Perform prime factorization
        int factor_count = 0; // Count of prime factors
        for (int factor = 2; factor * factor <= num; factor++) {
            while (num % factor == 0) {
                num /= factor;
                factor_count++;
            }
        }
        if (num > 1) {
            factor_count++; // Count the last prime factor if num is still greater than 1
        }

        // Check conditions based on prime factors
        if (factor_count > 0) {
            count += original_count; // Increment count based on original occurrences
        }

        // Handle threshold condition
        if (count > THRESHOLD) {
            count = THRESHOLD; // Set to sentinel value if exceeding threshold
        }
    }

    return count; // Return the final count
}

int main() {
    int n;
    printf("Enter the number of integers: ");
    scanf("%d", &n);

    int *arr = (int *)malloc(n * sizeof(int));
    printf("Enter the integers:\n");
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    long result = count_integers_with_prime_factors(n, arr);
    printf("Count of integers meeting criteria: %ld\n", result);

    free(arr); // Free allocated memory
    return 0;
}
