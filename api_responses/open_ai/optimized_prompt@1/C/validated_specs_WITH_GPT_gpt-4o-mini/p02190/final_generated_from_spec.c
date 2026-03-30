#include <stdio.h>
#include <stdlib.h>

int compare(const void *a, const void *b) {
    // Comparison function for qsort
    return (*(int *)a - *(int *)b);
}

int count_distinct_integers(int n, int *list) {
    // If the input integer is zero, return zero as specified
    if (n == 0) {
        return 0;
    }

    // Sort the list to bring duplicates together
    qsort(list, n, sizeof(int), compare);

    int distinct_count = 1; // Start with one distinct integer

    // Iterate through the sorted list to count distinct integers
    for (int i = 1; i < n; i++) {
        // If the current integer is different from the previous one, increment the count
        if (list[i] != list[i - 1]) {
            distinct_count++;
        }
    }

    return distinct_count; // Return the count of distinct integers
}

int main() {
    int n;
    // Read the number of elements in the list
    scanf("%d", &n);

    // Allocate memory for the list based on the input size
    int *list = (int *)malloc(n * sizeof(int));

    // Read the elements of the list
    for (int i = 0; i < n; i++) {
        scanf("%d", &list[i]);
    }

    // Call the function to count distinct integers
    int result = count_distinct_integers(n, list);

    // Output the result
    printf("%d\n", result);

    // Free the allocated memory
    free(list);

    return 0;
}
