#include <stdio.h>

int main() {
    int num_pairs; // Number of pairs to process
    // Read the number of pairs; it must be non-negative
    if (scanf("%d", &num_pairs) != 1 || num_pairs < 0) {
        return 1; // Undefined behavior for negative input or read error
    }

    for (int i = 0; i < num_pairs; i++) {
        int first, second; // Variables to hold the pair of integers
        // Read the pair of integers
        if (scanf("%d %d", &first, &second) != 2) {
            return 1; // Undefined behavior for input format error
        }
        
        // Output derived value based on the second integer
        // The specification guarantees that the output is a positive integer
        // Here we simply output the second integer, assuming it is positive
        // as per the postconditions.
        printf("%d\n", second);
    }

    return 0; // Successful execution
}
