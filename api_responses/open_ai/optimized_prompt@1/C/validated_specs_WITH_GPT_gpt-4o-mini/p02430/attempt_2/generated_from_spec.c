#include <stdio.h>
#include <stdlib.h>

void print_combination(int n, int k, int pattern) {
    // Print the binary pattern with leading zeros
    for (int i = n - 1; i >= 0; i--) {
        putchar((pattern & (1 << i)) ? '1' : '0');
    }
    
    // Print the indices corresponding to the set bits
    printf(":");
    for (int i = 0; i < n; i++) {
        if (pattern & (1 << i)) {
            printf(" %d", i);
        }
    }
    putchar('\n');
}

void generate_combinations(int n, int k) {
    int total_patterns = 1 << n; // 2^n possible patterns

    for (int pattern = 0; pattern < total_patterns; pattern++) {
        // Count the number of set bits (1s) in the pattern
        int count = __builtin_popcount(pattern); // GCC/Clang specific function
        
        if (count == k) {
            print_combination(n, k, pattern);
        }
    }
}

int main() {
    int n, k;

    // Read input values
    if (scanf("%d %d", &n, &k) != 2) {
        return -1; // Input reading failed
    }

    // Check preconditions
    if (n < 0 || k < 0 || k > n) {
        return -1; // Undefined behavior for invalid inputs
    }

    // Generate and print combinations
    generate_combinations(n, k);

    return 0;
}
