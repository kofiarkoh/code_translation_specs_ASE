#include <stdio.h>
#include <stdlib.h>

#define N_MAX 1000 // Define maximum size for the arrays

// Function to compare two long integers for qsort
int compare(const void *a, const void *b) {
    return (*(long *)a > *(long *)b) - (*(long *)a < *(long *)b);
}

int main() {
    long arr1[N_MAX], arr2[N_MAX];
    int n, m;

    // Read sizes of the arrays
    scanf("%d %d", &n, &m);

    // Preconditions check (assumed valid input as per specification)
    if (n < 0 || m < 0 || n > N_MAX) {
        return 1; // Exit if preconditions are not met
    }

    // Read values into the arrays
    for (int i = 0; i < n; i++) {
        scanf("%ld", &arr1[i]);
    }
    for (int i = 0; i < m; i++) {
        scanf("%ld", &arr2[i]);
    }

    // Sort the input arrays
    qsort(arr1, n, sizeof(long), compare);
    qsort(arr2, m, sizeof(long), compare);

    // Initialize result variable
    long result = 0;

    // Compute the result based on the sorted arrays
    // Example logic: sum of maximums of both arrays
    if (n > 0 && m > 0) {
        result = arr1[n - 1] + arr2[m - 1]; // Sum of max elements
    }

    // Output condition based on the computed result
    // Example condition: output "Yes" if result is even, "No" if odd
    if (result % 2 == 0) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    // Print the computed result
    printf("%ld\n", result);

    return 0;
}
