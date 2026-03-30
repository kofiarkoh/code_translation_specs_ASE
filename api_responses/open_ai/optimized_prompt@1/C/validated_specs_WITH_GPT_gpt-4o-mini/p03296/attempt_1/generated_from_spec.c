#include <stdio.h>

int main() {
    int N; // Number of integers in the sequence
    int count = 0; // Count of pairs of consecutive identical integers
    int previous, current; // Variables to hold the current and previous integers

    // Read the number of integers
    scanf("%d", &N);

    // Preconditions: N must be a positive integer
    if (N <= 0) {
        return 1; // Exit if N is not positive (though not specified in error handling)
    }

    // Read the first integer
    scanf("%d", &previous); // Initialize previous with the first integer

    // Loop through the remaining integers
    for (int i = 1; i < N; i++) {
        scanf("%d", &current); // Read the current integer

        // Check for consecutive identical integers
        if (current == previous) {
            count++; // Increment count if they are identical
        }

        // Update previous to current for the next iteration
        previous = current;
    }

    // Output the count of pairs
    printf("%d\n", count);

    return 0; // Successful execution
}
