#include <stdio.h>

int simulate(int n, int p) {
    // Preconditions check
    if (n <= 0) {
        return -1; // No valid output if n is zero or negative
    }

    int counts[n]; // Array to track counts, initialized to zero
    for (int i = 0; i < n; i++) {
        counts[i] = 0; // Initialize counts to zero
    }

    int w = p; // Decrementing counter initialized to p
    int index = 0; // Current index to process

    while (1) {
        // Check if decrementing counter has reached zero
        if (w == 0) {
            int all_other_zero = 1; // Flag to check if all other counts are zero
            for (int j = 0; j < n; j++) {
                if (j != index && counts[j] > 0) {
                    all_other_zero = 0; // Found an increment in another index
                    break;
                }
            }

            if (all_other_zero) {
                // If all other indices are zero, return the current index
                return index;
            } else {
                // Reset the decrementing counter based on the current index count
                w += counts[index];
                counts[index] = 0; // Reset the count for the current index
            }
        }

        // Simulate incrementing the count for the current index
        counts[index]++; // Increment the count for the current index
        w--; // Decrement the counter
        index = (index + 1) % n; // Move to the next index, wrap around if necessary
    }
}

int main() {
    int n, p;
    // Example input reading (assumed valid integers)
    scanf("%d %d", &n, &p);
    int result = simulate(n, p);
    if (result != -1) {
        printf("%d\n", result); // Output the result
    }
    return 0;
}
