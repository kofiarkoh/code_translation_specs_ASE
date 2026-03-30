#include <stdio.h>

void rotate(int *arr, int b, int m, int e) {
    int len = e - b; // Length of the segment to rotate
    m = m % len; // Normalize m to avoid unnecessary full rotations
    if (m == 0) return; // No rotation needed if m is 0

    // Create a temporary array to hold the rotated values
    int temp[len];
    
    // Copy the segment to be rotated into the temporary array
    for (int i = 0; i < len; i++) {
        temp[i] = arr[b + i];
    }

    // Perform the rotation
    for (int i = 0; i < len; i++) {
        arr[b + i] = temp[(i + len - m) % len]; // Rotate right
    }
}

int main() {
    int n; // Number of elements in the array
    scanf("%d", &n); // Read the size of the array
    int arr[n]; // Declare the array

    // Read the array elements
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    int q; // Number of rotation queries
    scanf("%d", &q); // Read the number of queries

    // Process each rotation query
    for (int i = 0; i < q; i++) {
        int b, m, e; // Starting index, number of positions to rotate, ending index
        scanf("%d %d %d", &b, &m, &e); // Read the query parameters
        rotate(arr, b, m, e); // Perform the rotation
    }

    // Output the modified array
    for (int i = 0; i < n; i++) {
        printf("%d", arr[i]);
        if (i < n - 1) {
            printf(" "); // Print space between elements
        }
    }
    printf("\n"); // Newline at the end of output

    return 0; // Indicate successful completion
}
