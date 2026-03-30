#include <stdio.h>

#define MODULO 10007

int main() {
    int N;
    scanf("%d", &N); // Read the number of characters

    // Preconditions check
    if (N < 0 || N > 1000) {
        return 1; // Exit if N is out of bounds
    }

    char input[N + 1]; // +1 for null terminator
    scanf("%s", input); // Read the input string

    // Schedule array to track combinations of keys
    int schedule[8] = {0}; // 2^3 = 8 combinations for 'J', 'O', 'I'
    int cumulative_value = 0;

    for (int i = 0; i < N; i++) {
        char key = input[i];
        int bit_value = 0;

        // Determine the bit value based on the key
        if (key == 'J') {
            bit_value = 1; // 2^0
        } else if (key == 'O') {
            bit_value = 2; // 2^1
        } else if (key == 'I') {
            bit_value = 4; // 2^2
        }

        // Update the schedule based on the current key
        for (int j = 0; j < 8; j++) {
            if (schedule[j] > 0) {
                schedule[j | bit_value] += schedule[j]; // Combine previous states
            }
        }

        // Initialize the current key's state
        schedule[bit_value]++; // Start a new combination with the current key
    }

    // Calculate the cumulative value from the schedule
    for (int j = 0; j < 8; j++) {
        cumulative_value += schedule[j]; // Sum all combinations
    }

    // Output the result modulo 10007
    printf("%d\n", cumulative_value % MODULO);
    return 0;
}
