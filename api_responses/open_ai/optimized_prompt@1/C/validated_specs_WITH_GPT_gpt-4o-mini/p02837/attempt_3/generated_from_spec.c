#include <stdio.h>

#define MAX_N 20
#define MAX_A 20

int max_set_bits(int n, int a[MAX_N], int x[MAX_N][MAX_A], int y[MAX_N][MAX_A]) {
    int max_bits = 0;

    // Iterate through all possible combinations of integers from 0 to (2^n - 1)
    for (int i = 0; i < (1 << n); i++) {
        int valid = 1; // Assume the combination is valid
        // Check validity against each group
        for (int j = 0; j < n; j++) {
            for (int k = 0; k < a[j]; k++) {
                // Check the validity condition
                int bit_x = (i >> (x[j][k] - 1)) & 1; // Check if bit x[j][k] is set
                int bit_y = (i >> (y[j][k] - 1)) & 1; // Check if bit y[j][k] is set

                // If bit_x is set, bit_y must also be set; if y[j][k] is 0, bit_y must not be set
                if (bit_x && !bit_y) {
                    valid = 0; // Invalid combination
                    break; // No need to check further constraints for this combination
                }
                if (!bit_x && bit_y) {
                    valid = 0; // Invalid combination
                    break; // No need to check further constraints for this combination
                }
            }
            if (!valid) break; // Exit if already invalid
        }

        // If the combination is valid, count the number of set bits
        if (valid) {
            int count = __builtin_popcount(i); // Count set bits in i
            if (count > max_bits) {
                max_bits = count; // Update max set bits
            }
        }
    }

    return max_bits; // Return the maximum number of set bits found
}

int main() {
    int n; // Number of groups
    int a[MAX_N]; // Array to hold the number of constraints for each group
    int x[MAX_N][MAX_A]; // Constraints x
    int y[MAX_N][MAX_A]; // Constraints y

    // Example input handling (not specified in the task, but necessary for testing)
    // This part should be replaced with actual input logic as needed.
    n = 3; // Example value
    a[0] = 2; // Example constraints for group 0
    a[1] = 1; // Example constraints for group 1
    a[2] = 1; // Example constraints for group 2
    x[0][0] = 1; y[0][0] = 1; // Example constraint
    x[0][1] = 2; y[0][1] = 0; // Example constraint
    x[1][0] = 3; y[1][0] = 1; // Example constraint
    x[2][0] = 1; y[2][0] = 0; // Example constraint

    int result = max_set_bits(n, a, x, y);
    printf("Maximum number of set bits in valid combinations: %d\n", result);

    return 0;
}
