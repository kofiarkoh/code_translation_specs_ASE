#include <stdio.h>
#include <stdlib.h>

int main() {
    int T1, T2, R1, R2;

    // Read four integers from input
    if (scanf("%d %d %d %d", &T1, &T2, &R1, &R2) != 4) {
        // Terminate if the input is not exactly four integers
        return 1; // Exit with an error code
    }

    // Determine output based on the sign of result values R1 and R2
    if (R1 < 0 || R2 < 0) {
        // If either result value is negative, compare time values
        if (T1 < T2) {
            printf("Alice\n");
        } else if (T1 > T2) {
            printf("Bob\n");
        } else {
            printf("Draw\n");
        }
    } else {
        // If both result values are non-negative, compare result values
        if (R1 > R2) {
            printf("Bob\n");
        } else if (R1 < R2) {
            printf("Alice\n");
        } else {
            printf("Draw\n");
        }
    }

    return 0; // Successful execution
}
