#include <stdio.h>
#include <limits.h>

#define MAX_N 110
#define ROWS 3

int f(int a[ROWS][MAX_N], int n, int current) {
    int count = 0;
    for (int j = 0; j < n; j++) {
        if (a[current][j] > a[current][0]) {
            count++;
        }
    }
    return count;
}

int main() {
    int n;
    int x[MAX_N];
    int a[ROWS][MAX_N];
    
    // Read input for n, x, and a (assumed to be populated correctly)
    // For example purposes, we will assume they are filled correctly.
    
    // Initialize the minimum difference to a large value
    int min_diff = INT_MAX;
    int found_valid_difference = 0;

    // Iterate over each row of the 2D array a
    for (int i = 0; i < ROWS; i++) {
        int s[MAX_N] = {0}; // Reset s to zero at the beginning of each iteration

        // Iterate over each element in the array x
        for (int j = 0; j < n; j++) {
            // Count how many elements in the current row of a exceed x[j]
            int count = f(a, n, i);
            if (count <= 7) { // Ensure the count does not exceed 7
                int modified_value = a[i][0] + count; // Modify the first element of a
                int diff = modified_value - x[j]; // Calculate the difference
                if (diff < min_diff) {
                    min_diff = diff; // Update minimum difference
                    found_valid_difference = 1; // Mark that we found a valid difference
                }
            }
        }
    }

    // Output the result
    if (found_valid_difference) {
        printf("%d\n", min_diff); // Print the minimum difference found
    } else {
        printf("Saiko\n"); // Print "Saiko" if no valid difference exists
    }

    return 0;
}
