#include <stdio.h>

int compute_count(int n, int k) {
    long long transformed_n = n * 7; // Transform n by multiplying by 7
    int count = 0;

    // First loop: count for i from 1 to (n % 693)
    for (int i = 1; i <= transformed_n % 693; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count++;
        }
    }

    // Base count for i from 1 to 693
    for (int i = 1; i <= 693; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count++;
        }
    }

    // Decrement count for i from 1 to 7
    for (int i = 1; i <= 7; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count--;
        }
    }

    // Decrement count for i from (n - 6) to n
    for (int i = transformed_n - 6; i <= transformed_n; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count--;
        }
    }

    // Ensure count is non-negative as per the invariant
    if (count < 0) {
        count = 0;
    }

    return count; // Return the final computed count
}

int main() {
    int n, k;
    // Assume valid input is provided
    scanf("%d %d", &n, &k);
    int result = compute_count(n, k);
    printf("%d\n", result); // Output the result
    return 0;
}
