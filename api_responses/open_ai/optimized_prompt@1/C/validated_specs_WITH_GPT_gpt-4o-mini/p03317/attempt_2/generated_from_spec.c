#include <stdio.h>

int main() {
    int n, k;

    // Read input values for n and k
    scanf("%d %d", &n, &k);

    // Preconditions check
    if (k <= 1) {
        // Undefined behavior if k is less than or equal to 1
        return 1; // Exit with an error code
    }

    // Calculate the number of groups
    int groups = 1; // Start with one group for the first element
    int remaining = n - 1; // Remaining elements after the first group

    // Calculate full groups of size k-1
    if (remaining > 0) {
        groups += remaining / (k - 1); // Full groups of size k-1
        if (remaining % (k - 1) != 0) {
            groups++; // Add an additional group for leftover elements
        }
    }

    // Output the result
    printf("%d\n", groups);

    return 0; // Successful execution
}
