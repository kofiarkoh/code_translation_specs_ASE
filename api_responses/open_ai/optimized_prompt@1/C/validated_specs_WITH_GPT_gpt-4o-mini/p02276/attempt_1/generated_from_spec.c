#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 1000 // Define maximum size of the array

void partition_and_highlight(int n, int *arr) {
    // Assuming the first element as the pivot for partitioning
    int pivot = arr[0];
    int i, j = 0;

    // Create a temporary array to hold the partitioned result
    int *temp = (int *)malloc(n * sizeof(int));
    if (temp == NULL) {
        // Handle memory allocation failure
        return; // Undefined behavior as per specification
    }

    // Partitioning the array
    for (i = 0; i < n; i++) {
        if (arr[i] <= pivot) {
            temp[j++] = arr[i];
        }
    }
    // Place the pivot in the middle
    temp[j++] = pivot;
    for (i = 0; i < n; i++) {
        if (arr[i] > pivot) {
            temp[j++] = arr[i];
        }
    }

    // Output the result with the pivot highlighted
    for (i = 0; i < n; i++) {
        if (i == (n - j + 1)) { // Check if it's the pivot position
            printf("[%d]", temp[i]); // Highlight the pivot
        } else {
            printf("%d", temp[i]);
        }
        if (i < n - 1) {
            printf(" "); // Print space between numbers
        }
    }
    printf("\n");

    free(temp); // Free allocated memory
}

int main() {
    int n;
    int arr[MAX_SIZE];

    // Read the number of elements
    if (scanf("%d", &n) != 1 || n <= 0 || n > MAX_SIZE) {
        // Undefined behavior for invalid input
        return 1; // Exit with error
    }

    // Read the sequence of integers
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &arr[i]) != 1) {
            // Undefined behavior if fewer than n integers are provided
            return 1; // Exit with error
        }
    }

    // Call the function to partition and highlight
    partition_and_highlight(n, arr);

    return 0;
}
