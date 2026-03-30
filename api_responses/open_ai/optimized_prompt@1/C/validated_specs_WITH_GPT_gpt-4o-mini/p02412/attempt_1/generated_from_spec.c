#include <stdio.h>

int main() {
    int n, x;

    // Read input pairs until both n and x are zero
    while (1) {
        scanf("%d %d", &n, &x);
        
        // Termination condition
        if (n == 0 && x == 0) {
            break;
        }

        // If n is less than 3, output is always zero
        if (n < 3) {
            printf("0\n");
            continue;
        }

        int count = 0;

        // Iterate through all combinations of three distinct integers
        for (int i = 1; i <= n - 2; i++) {
            for (int j = i + 1; j <= n - 1; j++) {
                for (int k = j + 1; k <= n; k++) {
                    // Check if the sum of the three integers equals x
                    if (i + j + k == x) {
                        count++;
                    }
                }
            }
        }

        // Output the count divided by 6
        printf("%d\n", count / 6);
    }

    return 0;
}
