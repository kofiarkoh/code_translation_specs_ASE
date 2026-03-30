#include <stdio.h>

int main() {
    int a, b, c; // Declare three integers
    // Read three integers from input
    if (scanf("%d %d %d", &a, &b, &c) != 3) {
        // Undefined behavior if input is not exactly three integers
        return 1; // Exit with error code
    }

    // Calculate the modified first integer as the absolute difference
    a = (a > b) ? (a - b) : (b - a); // Ensure a is non-negative

    // Decrement c based on the modified value of a
    while (a > 0 && c > 0) {
        c--; // Decrement c
        a--; // Decrement a
    }

    // Output the final value of c
    printf("%d\n", c); // Output the result
    return 0; // Successful execution
}
