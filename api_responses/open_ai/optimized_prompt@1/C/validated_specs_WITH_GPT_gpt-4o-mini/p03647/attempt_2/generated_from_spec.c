#include <stdio.h>

int main() {
    int n; // Number of conditions to evaluate
    scanf("%d", &n); // Read the number of conditions

    int a, b; // Variables to hold the pairs of integers
    int count = n; // Initialize count to the number of conditions

    // Iterate over the number of conditions
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &a, &b); // Read each pair of integers

        // Evaluation logic based on the specification
        if (a != 0 && b != count) {
            count--; // Decrease count if conditions are met
        }
    }

    // Determine the output based on the final count
    if (count > 0) {
        printf("POSSIBLE\n"); // If count is still positive, output "POSSIBLE"
    } else {
        printf("IMPOSSIBLE\n"); // If count is zero or less, output "IMPOSSIBLE"
    }

    return 0; // Return success
}
