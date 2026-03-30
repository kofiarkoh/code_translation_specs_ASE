#include <stdio.h>

int main() {
    int n, k;

    // Read the total number of integers and the maximum group size
    if (scanf("%d %d", &n, &k) != 2) {
        // If input format is invalid, we may not produce a valid output
        return 1; // Exit with an error code
    }

    // Preconditions check
    if (n <= 0 || k <= 1) {
        // Undefined behavior if k <= 1
        return 1; // Exit with an error code
    }

    // Calculate the minimum number of groups
    // Each group can have at most k elements
    int groups = n / k; // Integer division gives the number of full groups
    if (n % k != 0) { // If there are remaining elements, we need one more group
        groups++;
    }

    // Postcondition: output must be at least 1
    if (groups < 1) {
        groups = 1; // Ensure output is at least 1
    }

    // Output the result
    printf("%d\n", groups);

    return 0; // Successful execution
}
