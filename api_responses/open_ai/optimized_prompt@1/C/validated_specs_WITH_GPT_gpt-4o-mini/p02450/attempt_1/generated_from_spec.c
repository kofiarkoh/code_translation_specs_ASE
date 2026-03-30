#include <stdio.h>
#include <stdlib.h>

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

void generate_permutations(int *array, int start, int end) {
    if (start == end) {
        // Print the current permutation
        for (int i = 0; i <= end; i++) {
            printf("%d", array[i]);
            if (i < end) {
                printf(" "); // Separate integers with a space
            }
        }
        printf("\n");
    } else {
        for (int i = start; i <= end; i++) {
            swap(&array[start], &array[i]); // Swap to create a new permutation
            generate_permutations(array, start + 1, end); // Recur for the next index
            swap(&array[start], &array[i]); // Backtrack to restore the original array
        }
    }
}

int main() {
    int n;
    if (scanf("%d", &n) != 1) {
        // Undefined behavior if input is not a valid integer
        return 1; // Exit with an error code
    }

    if (n < 0) {
        // Undefined behavior for negative n, no output is guaranteed
        return 1; // Exit with an error code
    }

    int *array = malloc(n * sizeof(int));
    if (array == NULL) {
        // Handle memory allocation failure
        return 1; // Exit with an error code
    }

    // Initialize the array with integers from 1 to n
    for (int i = 0; i < n; i++) {
        array[i] = i + 1;
    }

    generate_permutations(array, 0, n - 1); // Generate all permutations

    free(array); // Free allocated memory
    return 0; // Successful execution
}
