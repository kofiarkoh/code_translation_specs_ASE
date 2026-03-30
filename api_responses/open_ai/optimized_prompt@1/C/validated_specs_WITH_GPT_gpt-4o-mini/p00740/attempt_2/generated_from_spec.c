#include <stdio.h>

int simulate(int n, int p) {
    // Preconditions check
    if (n <= 0) {
        return -1; // No output if n is zero or negative
    }

    int counts[n]; // Array to track counts, initialized to zero
    for (int i = 0; i < n; i++) {
        counts[i] = 0; // Initialize counts to zero
    }

    int w = p; // Decrementing counter initialized to p
    int index = 0; // Current index to process

    while (1) {
        // Check if the decrementing counter has reached zero
        if (w == 0) {
            // Check if all other elements have been incremented
            int other_incremented = 0;
            for (int j = 0; j < n; j++) {
                if (j != index && counts[j] > 0) {
                    other_incremented = 1; // Found an increment in another index
                    break;
                }
            }
            if (!other_incremented) {
                // If no other indices have been incremented, return the current index
                return index;
            }
            // Reset the decrementing counter based on the current index's count
            w = counts[index];
        }

        // Increment the count for the current index
        counts[index]++;
        // Decrement the counter
        w--;

        // Move to the next index, wrapping around if necessary
        index = (index + 1) % n;
    }
}

int main() {
    int n, p;
    // Assume valid integers are provided for n and p
    scanf("%d %d", &n, &p);
    int result = simulate(n, p);
    if (result != -1) {
        printf("%d\n", result); // Output the result
    }
    return 0;
}
