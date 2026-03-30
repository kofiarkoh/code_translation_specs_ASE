#include <stdio.h>

int simulate(int n, int p) {
    // Preconditions check
    if (n <= 0) {
        // If n is zero or negative, terminate without output
        return -1; // Indicating no valid output
    }

    int counts[n]; // Array to track counts, initialized to zero
    for (int i = 0; i < n; i++) {
        counts[i] = 0; // Initialize counts to zero
    }

    int index = 0; // Start from the first index
    while (p > 0) { // Continue until the decrementing counter reaches zero
        // Simulate processing of the current index
        counts[index]++; // Increment the count for the current index

        // Check if the current index meets the termination condition
        if (counts[index] >= p) {
            // If the count for this index meets or exceeds p, we terminate
            return index; // Return the current index
        }

        // Move to the next index, wrapping around if necessary
        index = (index + 1) % n; // Wrap around using modulo
        p--; // Decrement the counter
    }

    // If we exit the loop without returning, we return the last index checked
    return index; // Return the last index checked
}

int main() {
    int n, p;
    // Example input reading (assumed valid integers)
    printf("Enter number of elements (n): ");
    scanf("%d", &n);
    printf("Enter initial count to decrement (p): ");
    scanf("%d", &p);

    int result = simulate(n, p);
    if (result != -1) {
        printf("Output index: %d\n", result);
    } else {
        printf("No valid output due to n being zero or negative.\n");
    }

    return 0;
}
