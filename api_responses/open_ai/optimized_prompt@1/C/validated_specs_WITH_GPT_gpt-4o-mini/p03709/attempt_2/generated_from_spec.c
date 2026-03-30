#include <stdio.h>
#include <stdlib.h>

#define N_MAX 1000  // Define maximum size for the arrays

// Function to process the input arrays and produce the output
void process_arrays(int n, int m, long *array1, long *array2) {
    // Check preconditions
    if (n < 0 || m < 0 || n > N_MAX) {
        // Invalid input, behavior is undefined as per specification
        return;
    }

    // Sort the arrays (using a simple bubble sort for demonstration)
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (array1[j] > array1[j + 1]) {
                long temp = array1[j];
                array1[j] = array1[j + 1];
                array1[j + 1] = temp;
            }
        }
    }

    for (int i = 0; i < m - 1; i++) {
        for (int j = 0; j < m - i - 1; j++) {
            if (array2[j] > array2[j + 1]) {
                long temp = array2[j];
                array2[j] = array2[j + 1];
                array2[j + 1] = temp;
            }
        }
    }

    // Compute a result based on the sorted arrays
    long result = 0;
    for (int i = 0; i < n; i++) {
        result += array1[i];  // Summing elements of the first array
    }
    for (int i = 0; i < m; i++) {
        result -= array2[i];  // Subtracting elements of the second array
    }

    // Determine output based on the computed result
    if (result > 0) {
        printf("Yes\n");  // Output success
    } else {
        printf("No\n");   // Output failure
    }

    // Print the computed result
    printf("%ld\n", result);  // Output the long integer result
}

int main() {
    // Example usage
    int n = 5;  // Size of the first array
    int m = 3;  // Size of the second array
    long array1[] = {5, 3, 2, 8, 1};  // Example input for the first array
    long array2[] = {4, 6, 7};        // Example input for the second array

    process_arrays(n, m, array1, array2);  // Call the processing function

    return 0;
}
