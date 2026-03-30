#include <stdio.h>

void generate_sequence(int k) {
    if (k >= 50) {
        // Calculate base value and number of increments
        int base_value = 49 + (k / 50);
        int t = k % 50; // Number of increments
        for (int i = 0; i < 50; i++) {
            if (i < t) {
                // First t integers incremented by 50
                printf("%d\n", base_value + 50);
            } else {
                // Remaining integers decremented accordingly
                printf("%d\n", base_value);
            }
        }
    } else if (k >= 2) {
        // Output k integers, all equal to k
        for (int i = 0; i < k; i++) {
            printf("%d\n", k);
        }
    } else if (k == 0) {
        // Output four integers, each equal to 3
        for (int i = 0; i < 4; i++) {
            printf("%d\n", 3);
        }
    } else { // k < 0
        // Output three integers: 1, 0, and 3
        printf("1\n0\n3\n");
    }
}

int main() {
    int k;
    // Read input value k
    if (scanf("%d", &k) != 1) {
        // Undefined behavior if input is not a valid integer
        return 1; // Exit with error code
    }
    generate_sequence(k); // Generate and output the sequence based on k
    return 0; // Successful execution
}
