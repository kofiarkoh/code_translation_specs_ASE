#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 1000 // Assuming a maximum defined size for the arrays

// Function to process two arrays and compute a result
void compute_result(int size1, int size2, int *array1, int *array2, char *output, long *result) {
    // Check preconditions
    if (size1 < 0 || size2 < 0 || size1 > MAX_SIZE || size2 > MAX_SIZE) {
        // If preconditions are not met, we assume undefined behavior
        return;
    }

    // Initialize result
    *result = 0;

    // Example computation: sum of products of corresponding elements
    for (int i = 0; i < size1 && i < size2; i++) {
        *result += (long)array1[i] * (long)array2[i]; // Use long to prevent overflow
    }

    // Determine output based on the result
    if (*result > 0) {
        snprintf(output, 4, "Yes"); // Output "Yes" if result is positive
    } else {
        snprintf(output, 4, "No"); // Output "No" otherwise
    }
}

int main() {
    int size1, size2;
    int array1[MAX_SIZE], array2[MAX_SIZE];
    char output[4]; // Buffer for output string
    long result;

    // Example input (to be replaced with actual input handling)
    size1 = 3; // Example size
    size2 = 3; // Example size
    array1[0] = 1; array1[1] = 2; array1[2] = 3; // Example values
    array2[0] = 4; array2[1] = 5; array2[2] = 6; // Example values

    // Call the computation function
    compute_result(size1, size2, array1, array2, output, &result);

    // Print the output
    printf("%s\n", output);
    printf("%ld\n", result);

    return 0;
}
