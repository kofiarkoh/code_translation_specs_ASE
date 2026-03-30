#include <stdio.h>

int main() {
    int N; // Number of integers to read
    scanf("%d", &N); // Read the number of integers

    // Preconditions state that N must be positive
    // We assume valid input as per the specification

    int sum = 0; // Initialize sum to zero
    int input; // Variable to hold each input integer

    // Read N integers and accumulate the sum of (input - 1)
    for (int i = 0; i < N; i++) {
        scanf("%d", &input); // Read each integer
        sum += (input - 1); // Decrement input by 1 and add to sum
    }

    printf("%d\n", sum); // Output the final sum
    return 0; // Return success
}
