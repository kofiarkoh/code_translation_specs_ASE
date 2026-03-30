#include <stdio.h>
#include <stdlib.h>

void stable_sort(int *arr, int n, int m) {
    // Create an array of indices to maintain original order for stable sorting
    int *indices = malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        indices[i] = i; // Initialize indices
    }

    // Perform sorting based on modulo results for each divisor from 1 to m
    for (int k = 1; k <= m; k++) {
        // Sort using the current divisor k
        for (int i = 0; i < n - 1; i++) {
            for (int j = 0; j < n - i - 1; j++) {
                // Compare based on modulo results
                int mod_a = arr[indices[j]] % k;
                int mod_b = arr[indices[j + 1]] % k;
                if (mod_a > mod_b) {
                    // Swap indices for stable sorting
                    int temp = indices[j];
                    indices[j] = indices[j + 1];
                    indices[j + 1] = temp;
                }
            }
        }
    }

    // Output the sorted array based on the final order of indices
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[indices[i]]);
    }
    printf("\n");

    // Free allocated memory for indices
    free(indices);
}

int main() {
    int n, m;
    // Read n and m
    scanf("%d %d", &n, &m);
    
    // Check preconditions for n and m
    if (n <= 0 || m <= 0) {
        return 1; // Undefined behavior, exit
    }

    int *arr = malloc(n * sizeof(int));
    
    // Read the array elements
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    // Call the sorting function
    stable_sort(arr, n, m);

    // Free allocated memory for the array
    free(arr);
    return 0;
}
