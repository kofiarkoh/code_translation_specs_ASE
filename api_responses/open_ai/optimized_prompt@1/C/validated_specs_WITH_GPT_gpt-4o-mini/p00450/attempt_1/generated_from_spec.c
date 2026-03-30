#include <stdio.h>

#define MAX_SIZE 100000

int process_sequence(int *sequence) {
    int count_zeros = 0;
    int n = sequence[0]; // First integer indicates the number of subsequent integers

    // Process the sequence according to the specified rules
    for (int i = 1; i <= n; i++) {
        // Replace values based on specific conditions
        if (sequence[i] < 0) {
            sequence[i] = 0; // Replace negative values with zero
        }
        // Count zeros in the modified sequence
        if (sequence[i] == 0) {
            count_zeros++;
        }
    }

    return count_zeros; // Return the count of zeros in the modified sequence
}

int main() {
    int sequence[MAX_SIZE + 1]; // +1 to accommodate the first integer

    while (1) {
        // Read the first integer to determine the number of subsequent integers
        scanf("%d", &sequence[0]);
        if (sequence[0] == 0) {
            break; // Termination condition
        }

        // Read the subsequent integers
        for (int i = 1; i <= sequence[0]; i++) {
            scanf("%d", &sequence[i]);
        }

        // Process the sequence and get the count of zeros
        int zero_count = process_sequence(sequence);
        printf("%d\n", zero_count); // Output the count of zeros
    }

    return 0;
}
