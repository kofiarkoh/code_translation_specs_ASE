#include <stdio.h>

int main() {
    long long n, k; // Input values
    scanf("%lld %lld", &n, &k); // Read inputs

    n *= 7; // Transform n by multiplying by 7
    int count = 0; // Initialize count

    // First loop: count for i from 1 to n % 693
    for (int i = 1; i <= n % 693; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count++; // Increment count if conditions are met
        }
    }

    // Second loop: count for i from 1 to 693
    for (int i = 1; i <= 693; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count++; // Increment count if conditions are met
        }
    }

    // Third loop: decrement count for i from 1 to 7
    for (int i = 1; i <= 7; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count--; // Decrement count if conditions are met
        }
    }

    // Fourth loop: decrement count for i from n - 6 to n
    for (int i = n - 6; i <= n; i++) {
        if (i % 7 > 1 && (9 * i) % 11 == k) {
            count--; // Decrement count if conditions are met
        }
    }

    printf("%d\n", count); // Output the final count
    return 0; // Return success
}
