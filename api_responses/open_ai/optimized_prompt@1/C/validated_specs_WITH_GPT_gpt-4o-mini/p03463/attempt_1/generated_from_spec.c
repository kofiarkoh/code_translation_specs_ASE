#include <stdio.h>

int main() {
    int N, A, B; // Declare three integers as specified

    // Read input values from a single line
    // Assumes valid integer input as per the specification
    scanf("%d %d %d", &N, &A, &B);

    // Calculate the difference between B and A
    int difference = B - A;

    // Check the parity of the difference
    if (difference % 2 == 0) {
        // If even, output "Alice"
        printf("Alice\n");
    } else {
        // If odd, output "Borys"
        printf("Borys\n");
    }

    return 0; // Indicate successful completion
}
