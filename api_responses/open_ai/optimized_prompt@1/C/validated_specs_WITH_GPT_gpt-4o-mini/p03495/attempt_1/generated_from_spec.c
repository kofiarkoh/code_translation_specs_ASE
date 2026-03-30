#include <stdio.h>
#include <stdlib.h>

long long sum_of_counts(int N, int K, int *array) {
    // Step 1: Initialize the count array
    int max_value = 100; // Assuming a valid range for input integers
    int *count = (int *)calloc(max_value + 1, sizeof(int)); // Count array initialized to zero

    // Step 2: Count occurrences of each integer
    for (int i = 0; i < N; i++) {
        count[array[i]]++;
    }

    // Step 3: Create an array to hold the counts for sorting
    int unique_counts[max_value + 1];
    int unique_count_size = 0;

    for (int i = 0; i <= max_value; i++) {
        if (count[i] > 0) {
            unique_counts[unique_count_size++] = count[i];
        }
    }

    // Step 4: Sort the unique counts (simple bubble sort for clarity)
    for (int i = 0; i < unique_count_size - 1; i++) {
        for (int j = 0; j < unique_count_size - i - 1; j++) {
            if (unique_counts[j] > unique_counts[j + 1]) {
                int temp = unique_counts[j];
                unique_counts[j] = unique_counts[j + 1];
                unique_counts[j + 1] = temp;
            }
        }
    }

    // Step 5: Calculate the sum of the first N - K unique counts
    long long sum = 0;
    for (int i = 0; i < unique_count_size && i < (N - K); i++) {
        sum += unique_counts[i];
    }

    // Free allocated memory for count array
    free(count);

    return sum;
}

int main() {
    // Example usage
    int N = 5;
    int K = 2;
    int array[] = {1, 2, 2, 3, 3};

    long long result = sum_of_counts(N, K, array);
    printf("Sum of counts: %lld\n", result);

    return 0;
}
