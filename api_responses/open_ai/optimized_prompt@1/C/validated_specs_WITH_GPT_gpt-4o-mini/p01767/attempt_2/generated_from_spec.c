#include <stdio.h>

void process_arrays(int n, int arr1[], int m, int arr2[], long arr3[]) {
    long cumulative_sum = 0; // To hold the cumulative sum of arr1
    int upper_bound_index; // To hold the index for upper bound

    // Iterate through each element in the second array
    for (int j = 0; j < m; j++) {
        // Find the upper bound index for arr2[j] in arr1
        upper_bound_index = 0;
        while (upper_bound_index < n && arr1[upper_bound_index] <= arr2[j]) {
            cumulative_sum += arr1[upper_bound_index]; // Update cumulative sum
            upper_bound_index++; // Move to the next element
        }

        // Compare cumulative sum with the corresponding long integer
        if (cumulative_sum >= arr3[j]) {
            printf("Yes\n");
        } else {
            printf("No\n");
        }

        // Reset cumulative sum for the next query
        cumulative_sum = 0; // Reset for the next iteration
    }
}

int main() {
    // Example usage
    int n = 5;
    int arr1[] = {1, 2, 3, 4, 5}; // Sorted array
    int m = 3;
    int arr2[] = {3, 6, 1}; // Query array
    long arr3[] = {10, 15, 1}; // Long integer comparisons

    process_arrays(n, arr1, m, arr2, arr3);
    return 0;
}
