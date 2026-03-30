#include <stdio.h>

void process_arrays(int n, int arr1[], int m, int arr2[], long arr3[]) {
    long cumulative_sum = 0; // To hold the cumulative sum of arr1
    int upper_bound; // To hold the upper bound index

    // Iterate through each element in the second array
    for (int j = 0; j < m; j++) {
        // Find the upper bound index for arr2[j]
        upper_bound = 0;
        while (upper_bound < n && arr1[upper_bound] <= arr2[j]) {
            upper_bound++;
        }

        // Calculate the cumulative sum up to the upper bound index
        cumulative_sum = 0;
        for (int k = 0; k < upper_bound; k++) {
            cumulative_sum += arr1[k];
        }

        // Compare cumulative sum with the corresponding long integer
        if (cumulative_sum >= arr3[j]) {
            printf("Yes\n");
        } else {
            printf("No\n");
        }
    }
}

int main() {
    // Example usage
    int n = 5;
    int arr1[] = {1, 2, 3, 4, 5}; // Sorted array
    int m = 3;
    int arr2[] = {3, 6, 1}; // Query array
    long arr3[] = {6, 15, 1}; // Long integer array

    process_arrays(n, arr1, m, arr2, arr3);
    return 0;
}
