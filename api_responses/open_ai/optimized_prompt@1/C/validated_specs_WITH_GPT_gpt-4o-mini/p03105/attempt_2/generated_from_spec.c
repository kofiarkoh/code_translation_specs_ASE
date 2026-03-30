#include <stdio.h>

int main() {
    int a, b, c; // Declare three integer variables for input
    int division_result; // Variable to store the result of the division

    // Read three integer values from input
    scanf("%d %d %d", &a, &b, &c);

    // Check precondition: a must be non-zero to avoid division by zero
    if (a != 0) {
        // Perform integer division of b by a
        division_result = b / a;

        // Output the smaller of the division result and c
        if (division_result < c) {
            printf("%d\n", division_result);
        } else {
            printf("%d\n", c);
        }
    }
    // If a is zero, the program does not produce a valid output,
    // but this scenario is not explicitly handled as per the specification.

    return 0; // Indicate successful completion
}
